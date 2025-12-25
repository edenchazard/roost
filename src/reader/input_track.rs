use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct InputTrack {
    pub path: String,
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub track_number: Option<i32>,
}
