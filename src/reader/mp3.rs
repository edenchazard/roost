use std::str;

use id3::{Tag, TagLike};

use crate::reader::{ReaderTrait, track::Track};

pub struct Mp3 {}

impl ReaderTrait for Mp3 {
    fn read(&self, file_path: &str) -> Result<Track, Box<dyn std::error::Error>> {
        let tag: Tag = Tag::read_from_path(file_path)?;

        Ok(Track {
            path: file_path.to_string(),
            title: tag.title().unwrap_or("").to_string(),
            artist: tag.artist().unwrap_or("").to_string(),
            album: tag.album().unwrap_or("").to_string(),
        })
    }
}
