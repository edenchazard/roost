use std::{collections::HashSet, fs};

use diesel::{Connection, ExpressionMethods, RunQueryDsl, insert_into};
use sha2::Digest;

use crate::{models, reader::input_track::InputTrack, schema};

type MediaFile<'a> = (String, &'a Vec<u8>);

fn save_media_files_to_disk(files: Vec<MediaFile<'_>>) {
    for (path, data) in files {
        if let Err(e) = fs::write(&path, &data) {
            eprintln!("Error writing media file {}: {}", path, e);
        }
    }
}

/**
 * TODO: something like alerting a web socket so users can see progress.
 * TODO: better error handling and reporting.
 * TODO: third step to create optimal sizes of media files.
 */
pub fn insert_media_records(paths: Vec<InputTrack>) {
    let albums: Vec<models::NewAlbum> = paths
        .iter()
        // Filter out tracks without an album name
        .filter_map(|track| track.album.clone())
        .collect::<HashSet<String>>()
        .into_iter()
        .map(|album_name| models::NewAlbum {
            title: Some(album_name),
            artist: None,
            picture_url: None,
        })
        .collect();

    let mut files_to_save: Vec<MediaFile> = vec![];

    let conn = &mut crate::establish_connection();

    let result = conn.transaction(|conn| {
        insert_into(schema::albums::table)
            .values(&albums)
            .on_conflict_do_nothing()
            .execute(conn)?;

        for track in &paths {
            use schema::tracks::dsl::*;

            let inserted_track = insert_into(schema::tracks::table)
                .values((
                    title.eq(&track.title),
                    artist.eq(&track.artist),
                    album.eq(&track.album),
                    track_number.eq(&track.track_number),
                    audio_url.eq(&track.path),
                ))
                .on_conflict_do_nothing()
                .returning(id)
                .get_result::<i32>(conn);

            let mediable_id = match inserted_track {
                Ok(insert_id) => insert_id,
                Err(_) => continue,
            };

            if track.media.is_none() {
                continue;
            }

            for media_item in track.media.as_ref().unwrap() {
                let art_path = format!(
                    "assets/album-art/{:x}.media",
                    sha2::Sha256::digest(&media_item.data)
                );

                insert_into(schema::media::table)
                    .values((
                        schema::media::path.eq(&art_path),
                        schema::media::mime_type.eq(&media_item.mime_type),
                        schema::media::type_.eq(&media_item.media_type),
                        schema::media::mediable_id.eq(mediable_id),
                        schema::media::mediable_type.eq("Track"),
                    ))
                    .on_conflict_do_nothing()
                    .execute(conn)?;

                files_to_save.push((art_path, &media_item.data));
            }
        }

        diesel::result::QueryResult::Ok(())
    });

    match result {
        Ok(_) => {
            save_media_files_to_disk(files_to_save);
        }
        Err(e) => {
            eprintln!("Error inserting media records: {}", e);
        }
    }
}
