use std::fs;

use diesel::{
    Connection, ExpressionMethods, IntoSql, PgConnection, RunQueryDsl,
    dsl::{exists, not},
    insert_into,
    query_dsl::methods::{FilterDsl, SelectDsl},
    sql_types::*,
    update,
};
use sha2::Digest;
use tokio::task;

use crate::{
    models::{self, NewAlbum},
    reader::input_track::InputTrack,
    schema::{self},
};

struct Media {
    path: String,
    media_type: String,
    data: Vec<u8>,
}

fn save_media_to_disk(file_path: &String, data: &Vec<u8>) -> Result<(), std::io::Error> {
    fs::write(&file_path, &data)?;
    Ok(())
}

fn extract_medias_from_tracks(file_path: &String) -> Result<Vec<Media>, ()> {
    let read = crate::reader::new(&file_path)?;
    let medias = read.media(&file_path).unwrap_or(vec![]);
    let mut files_to_save: Vec<Media> = vec![];

    for media in medias {
        files_to_save.push(Media {
            path: format!(
                "assets/album-art/{:x}.media",
                sha2::Sha256::digest(&media.data)
            ),
            data: media.data,
            media_type: media.media_type,
        });
    }

    Ok(files_to_save)
}

fn populate_media_to_db(media: &Media, conn: &mut PgConnection) -> Result<(), ()> {
    let result = insert_into(schema::media::table)
        .values((
            schema::media::path.eq(&media.path),
            schema::media::type_.eq(&media.media_type),
        ))
        .on_conflict_do_nothing()
        .execute(conn);

    match result {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("Error inserting media records: {}", e);
            Err(())
        }
    }
}

fn populate_albums_for_newly_added_tracks_to_db(
    track_ids: &Vec<i32>,
    conn: &mut PgConnection,
) -> Result<Vec<i32>, diesel::result::Error> {
    let sub_query = schema::albums::table
        .select(0.into_sql::<Integer>())
        .filter(schema::albums::title.eq(schema::tracks::album))
        .filter(schema::albums::artist.eq(schema::tracks::artist));

    let tracks_missing_album_parent = schema::tracks::table
        .filter(schema::tracks::id.eq_any(track_ids))
        .filter(not(exists(sub_query)))
        .load::<models::Track>(conn)?;

    let results = insert_into(schema::albums::table)
        .values(
            tracks_missing_album_parent
                .into_iter()
                .map(|t| NewAlbum {
                    title: t.album.clone(),
                    artist: t.album_artist.clone(),
                    picture_url: None,
                })
                .collect::<Vec<NewAlbum>>(),
        )
        .returning(schema::albums::id)
        .on_conflict_do_nothing()
        .get_results::<i32>(conn);

    match results {
        Ok(insert_ids) => Ok(insert_ids),
        Err(e) => {
            eprintln!("Error populating albums for newly added tracks: {}", e);
            Err(e)
        }
    }
}

fn populate_tracks_to_db(
    tracks_in: &Vec<InputTrack>,
    conn: &mut PgConnection,
) -> Result<Vec<i32>, ()> {
    let mut track_ids: Vec<i32> = vec![];

    for track in tracks_in {
        let inserted_track = insert_into(schema::tracks::table)
            .values((
                schema::tracks::title.eq(&track.title),
                schema::tracks::artist.eq(&track.artist),
                schema::tracks::album.eq(&track.album),
                schema::tracks::album_artist.eq(&track.album_artist),
                schema::tracks::track_number.eq(&track.track_number),
                schema::tracks::audio_url.eq(&track.path),
            ))
            .on_conflict_do_nothing()
            .returning(schema::tracks::id)
            .get_result::<i32>(conn);

        let track_id = match inserted_track {
            Ok(insert_id) => insert_id,
            Err(_) => continue,
        };

        track_ids.push(track_id);
    }

    Ok(track_ids)
}

/**
 * Outline:
 * 1. Insert Tracks (collect file arts)
 * 2. Derive Albums from Tracks and insert Albums
 * 3. Save media files to disk
 * TODO: something like alerting a web socket so users can see progress.
 * TODO: better error handling and reporting.
 * TODO: third step to create optimal sizes of media files.
 */
pub async fn sync_tracks(tracks_in: Vec<InputTrack>) {
    let conn: &mut PgConnection = &mut crate::establish_connection();

    let result = conn.transaction(|conn| {
        let new_track_ids = populate_tracks_to_db(&tracks_in, conn).unwrap();
        let new_album_ids =
            populate_albums_for_newly_added_tracks_to_db(&new_track_ids, conn).unwrap();

        diesel::result::QueryResult::Ok((new_track_ids, new_album_ids))
    });

    let (track_ids, album_ids) = match result {
        Ok((t_ids, a_ids)) => (t_ids, a_ids),
        Err(e) => {
            eprintln!("Error during track sync transaction: {}", e);
            return;
        }
    };

    // Collect media files in a separate  task.
    task::spawn_blocking(move || {
        let conn: &mut PgConnection = &mut crate::establish_connection();

        let inserted_tracks = schema::tracks::table
            .filter(schema::tracks::id.eq_any(&track_ids))
            .load::<models::Track>(conn)
            .unwrap();

        for insert in inserted_tracks.iter() {
            let medias = extract_medias_from_tracks(&insert.audio_url).unwrap();

            let result = conn.transaction(|conn| {
                let mut attached_to_album: bool = false;

                for media in &medias {
                    // Save media to disk to ensure it's available before populating DB.
                    save_media_to_disk(&media.path, &media.data).unwrap();
                    populate_media_to_db(media, conn).unwrap();

                    // Sync the front cover art to the track.
                    // If this was a newly created album that doesn't yet have cover art attached,
                    // also attach it to the album.
                    if &media.media_type == "Front cover" {
                        update(schema::tracks::table)
                            .filter(schema::tracks::id.eq(insert.id))
                            .set(schema::tracks::picture_url.eq(Some(media.path.clone())))
                            .execute(conn)
                            .unwrap();

                        if attached_to_album {
                            continue;
                        }

                        let associated_album = schema::albums::table
                            .filter(schema::albums::title.eq(insert.album.clone()))
                            .filter(schema::albums::artist.eq(insert.album_artist.clone()))
                            .first::<models::Album>(conn)
                            .unwrap();

                        if associated_album.picture_url.is_none()
                            && album_ids.contains(&associated_album.id)
                        {
                            update(schema::albums::table)
                                .filter(schema::albums::id.eq(associated_album.id))
                                .set(schema::albums::picture_url.eq(Some(media.path.clone())))
                                .execute(conn)
                                .unwrap();

                            attached_to_album = true;
                        }
                    }
                }

                diesel::result::QueryResult::Ok(())
            });

            if let Err(e) = result {
                eprintln!("Error populating media for track ID {}: {}", insert.id, e);
                // TODO: Probably do some logging / informing here.
            }
        }
    });
}
