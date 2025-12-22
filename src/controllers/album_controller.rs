use crate::models::{self, Album, Track};
use crate::schema::albums::dsl::*;
use axum::{Json, Router, extract::Path, http::StatusCode, response::IntoResponse, routing::get};
use diesel::ExpressionMethods;
use diesel::query_dsl::methods::FilterDsl;
use diesel::{
    OptionalExtension, RunQueryDsl, SelectableHelper,
    query_dsl::methods::{FindDsl, SelectDsl},
};

pub fn album_controller() -> Router<()> {
    Router::<()>::new()
        .route("/", get(index))
        .route("/{id}", get(show))
        .route("/{id}/tracks", get(show_tracks))
}

async fn index() -> Result<impl IntoResponse, StatusCode> {
    let conn = &mut crate::establish_connection();

    let results = albums.load::<models::Album>(conn);

    match results {
        Ok(results) => Ok(Json(results).into_response()),
        Err(e) => {
            eprintln!("Error fetching albums: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn show(Path(album_id): Path<i32>) -> Result<impl IntoResponse, StatusCode> {
    use crate::schema::albums::dsl::*;

    let conn = &mut crate::establish_connection();

    let result = albums
        .find(album_id)
        .select(Album::as_select())
        .first(conn)
        .optional();

    match result {
        Ok(album) => Ok(Json(album).into_response()),
        Err(diesel::result::Error::NotFound) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            eprintln!("Error fetching album: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn show_tracks(Path(album_id): Path<i32>) -> Result<impl IntoResponse, StatusCode> {
    use crate::schema::tracks;

    let conn = &mut crate::establish_connection();

    let album_result = albums
        .find(album_id)
        .select(Album::as_select())
        .first::<models::Album>(conn)
        .optional();

    let album: Album = match album_result {
        Ok(Some(a)) => a,
        Ok(None) => return Err(StatusCode::NOT_FOUND),
        Err(e) => {
            eprintln!("Error fetching album: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let results = tracks::table
        .select(Track::as_select())
        .filter(tracks::album.eq(album.title))
        .load::<models::Track>(conn);

    match results {
        Ok(album) => Ok(Json(album).into_response()),
        Err(diesel::result::Error::NotFound) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            eprintln!("Error fetching album: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
