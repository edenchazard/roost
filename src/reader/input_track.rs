use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Deserialize, Serialize, Debug)]
pub struct InputTrack {
    pub path: String,
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub track_number: Option<i32>,
    pub media: Option<Vec<models::NewMedia>>,
}
