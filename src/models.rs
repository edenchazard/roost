use diesel::prelude::*;
use gents_derives::TS;

#[derive(Queryable, Selectable, TS, Clone)]
#[ts(file_name = "album.ts", rename_all = "camelCase")]
#[diesel(table_name = crate::schema::albums)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Album {
    pub id: i32,
    pub title: String,
    pub artist: String,
    pub picture_url: Option<String>,
}

#[derive(Queryable, Selectable, TS, Clone)]
#[ts(file_name = "track.ts", rename_all = "camelCase")]
#[diesel(table_name = crate::schema::tracks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Track {
    pub id: i32,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub track_number: i32,
    pub picture_url: Option<String>,
}

#[ignore]
#[test]
fn generate_ts_files() {
    let mut g = gents::FileGroup::new();
    g.add::<Album>();
    g.add::<Track>();
    g.gen_files("./client/app/types/generated", true);
}
