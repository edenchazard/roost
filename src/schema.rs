// @generated automatically by Diesel CLI.

diesel::table! {
    albums (id) {
        id -> Int4,
        #[max_length = 255]
        title -> Nullable<Varchar>,
        #[max_length = 255]
        artist -> Nullable<Varchar>,
        #[max_length = 512]
        picture_url -> Nullable<Varchar>,
    }
}

diesel::table! {
    tracks (id) {
        id -> Int4,
        #[max_length = 255]
        title -> Nullable<Varchar>,
        #[max_length = 255]
        album -> Nullable<Varchar>,
        #[max_length = 255]
        artist -> Nullable<Varchar>,
        track_number -> Nullable<Int4>,
        #[max_length = 512]
        picture_url -> Nullable<Varchar>,
        #[max_length = 512]
        audio_url -> Nullable<Varchar>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(albums, tracks,);
