use std::collections::HashSet;

use crate::{
    directory_reader,
    models::{self},
    schema::{self, tracks::dsl::*},
};
use axum::{Json, Router, http::StatusCode, response::IntoResponse, routing::get};
use diesel::{Connection, RunQueryDsl, insert_into};

pub fn source_controller() -> Router<()> {
    Router::<()>::new().route("/scan", get(scan))
}

async fn scan() -> Result<impl IntoResponse, StatusCode> {
    let input_dir: &str = "/root/roost/in";
    let conn = &mut crate::establish_connection();

    let (supported, _unsupported) = match directory_reader::list_files_in_dir(&input_dir) {
        Ok((s, u)) => (s, u),
        Err(e) => {
            eprintln!("Error reading directory: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let inputs: Vec<models::NewTrack> = supported
        .into_iter()
        .map(|track| models::NewTrack {
            album: track.album,
            artist: track.artist,
            title: track.title,
            audio_url: track.path,
            picture_url: None,
            track_number: None,
        })
        .collect();

    let albums: Vec<models::NewAlbum> = inputs
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

    let result = conn.transaction(|conn| {
        insert_into(schema::albums::table)
            .values(&albums)
            .on_conflict_do_nothing()
            .execute(conn)?;

        insert_into(tracks)
            .values(&inputs)
            .on_conflict_do_nothing()
            .execute(conn)?;

        diesel::result::QueryResult::Ok(())
    });

    match result {
        Ok(results) => Ok(Json(results).into_response()),
        Err(e) => {
            eprintln!("Error inserting tracks: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
