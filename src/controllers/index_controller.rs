use axum::{Router, routing::get};
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;

use crate::controllers::{album_controller, track_controller};

pub fn index_controller() -> Router<()> {
    let static_assets = ServeDir::new("./assets");

    let cors_layer = CorsLayer::new().allow_origin(Any);

    Router::<()>::new()
        .route("/", get(index))
        .nest("/albums", album_controller())
        .nest("/tracks", track_controller())
        .nest_service("/assets", static_assets)
        .layer(cors_layer)
}

async fn index() -> &'static str {
    "Index Page"
}
