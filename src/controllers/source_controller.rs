use crate::{directory_reader, filesystem::sync_tracks};
use axum::{Router, http::StatusCode, response::IntoResponse, routing::get};
use tokio::task;

pub fn source_controller() -> Router<()> {
    Router::<()>::new().route("/scan", get(scan))
}

async fn scan() -> Result<impl IntoResponse, StatusCode> {
    let input_dir: &str = "/root/roost/in";

    let (supported, _unsupported) = match directory_reader::list_files_in_dir(&input_dir) {
        Ok((s, u)) => (s, u),
        Err(e) => {
            eprintln!("Error reading directory: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    task::spawn(sync_tracks(supported));

    Ok(StatusCode::OK)
}
