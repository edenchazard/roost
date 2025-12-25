use std::path::Path;

use crate::reader::input_track::InputTrack;

pub mod input_track;
mod mp3;

pub fn new(file_path: &str) -> Result<Box<dyn ReaderTrait>, ()> {
    let ext = Path::new(file_path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");
    match ext {
        "mp3" => Ok(Box::new(crate::reader::mp3::Mp3 {})),
        _ => Err(()),
    }
}

pub trait ReaderTrait {
    fn read(&self, file_path: &str) -> Result<InputTrack, Box<dyn std::error::Error>>;
}
