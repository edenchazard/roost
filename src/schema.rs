// @generated automatically by Diesel CLI.

diesel::table! {
    albums (id) {
        id -> Int4,
        #[max_length = 255]
        title -> Varchar,
        #[max_length = 255]
        artist -> Varchar,
        #[max_length = 512]
        picture_url -> Nullable<Varchar>,
    }
}

diesel::table! {
    tracks (id) {
        id -> Int4,
        #[max_length = 255]
        title -> Varchar,
        #[max_length = 255]
        album -> Varchar,
        #[max_length = 255]
        artist -> Varchar,
        track_number -> Int4,
        #[max_length = 512]
        picture_url -> Nullable<Varchar>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(albums, tracks,);
