use crate::schema::tracks::dsl::*;
use axum::{Json, Router, http::StatusCode, response::IntoResponse, routing::get};
use diesel::RunQueryDsl;

use crate::models;

pub fn track_controller() -> Router<()> {
    Router::<()>::new().route("/", get(index))
}

async fn index() -> Result<impl IntoResponse, StatusCode> {
    let conn = &mut crate::establish_connection();

    let results = tracks.load::<models::Track>(conn);

    match results {
        Ok(results) => Ok(Json(results).into_response()),
        Err(e) => {
            eprintln!("Error fetching tracks: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
