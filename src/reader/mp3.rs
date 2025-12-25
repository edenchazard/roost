use std::str;

use id3::{Tag, TagLike};

use crate::reader::{ReaderTrait, input_track::InputTrack};

pub struct Mp3 {}

impl ReaderTrait for Mp3 {
    fn read(&self, file_path: &str) -> Result<InputTrack, Box<dyn std::error::Error>> {
        let tag: Tag = Tag::read_from_path(file_path)?;

        Ok(InputTrack {
            path: file_path.to_string(),
            title: Some(tag.title().unwrap_or("").to_string()),
            artist: Some(tag.artist().unwrap_or("").to_string()),
            album: Some(tag.album().unwrap_or("").to_string()),
            track_number: Some(tag.track().unwrap_or(0) as i32),
        })
    }
}
