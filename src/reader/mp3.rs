use std::str;

use id3::{Tag, TagLike, frame::Picture};

use crate::{
    models,
    reader::{ReaderTrait, input_track::InputTrack},
};

pub struct Mp3;

impl ReaderTrait for Mp3 {
    fn read(&self, file_path: &str) -> Result<InputTrack, Box<dyn std::error::Error>> {
        let tag: Tag = Tag::read_from_path(file_path)?;

        let media: Vec<Picture> = tag.pictures().cloned().collect();

        Ok(InputTrack {
            path: file_path.to_string(),
            title: Some(tag.title().unwrap_or("").to_string()),
            artist: Some(tag.artist().unwrap_or("").to_string()),
            album: Some(tag.album().unwrap_or("").to_string()),
            track_number: Some(tag.track().unwrap_or(0) as i32),
            media: Some(
                media
                    .into_iter()
                    .map(|picture| models::NewMedia {
                        mime_type: picture.mime_type,
                        media_type: picture.picture_type.to_string(),
                        data: picture.data,
                    })
                    .collect(),
            ),
        })
    }
}
