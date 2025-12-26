use diesel::prelude::*;
use gents_derives::TS;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Clone, TS)]
#[diesel(table_name = crate::schema::albums)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[ts(file_name = "album.ts", rename_all = "camelCase")]
pub struct Album {
    pub id: i32,
    pub title: Option<String>,
    pub artist: Option<String>,
    pub picture_url: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::albums)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewAlbum {
    pub title: Option<String>,
    pub artist: Option<String>,
    pub picture_url: Option<String>,
}

#[derive(Queryable, Selectable, Clone, TS)]
#[diesel(table_name = crate::schema::tracks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[ts(file_name = "track.ts", rename_all = "camelCase")]
pub struct Track {
    pub id: i32,
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub track_number: Option<i32>,
    pub picture_url: Option<String>,
    pub audio_url: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NewTrack {
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub track_number: Option<i32>,
    pub audio_url: String,
    pub media: Option<Vec<NewMedia>>,
}

#[derive(Queryable, Selectable, Clone, TS)]
#[diesel(table_name = crate::schema::media)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[ts(file_name = "media.ts", rename_all = "camelCase")]
pub struct Media {
    pub id: i32,
    pub path: String,
    pub mime_type: String,
    pub type_: String,
    pub mediable_id: i32,
    pub mediable_type: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NewMedia {
    pub mime_type: String,
    pub media_type: String,
    pub data: Vec<u8>,
}

#[ignore]
#[test]
fn generate_ts_files() {
    let mut g = gents::FileGroup::new();
    g.add::<Album>();
    g.add::<Track>();
    g.add::<Media>();
    g.gen_files(&"./client/app/types/generated", true);
}
