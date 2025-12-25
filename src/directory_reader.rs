use std::fs;

use crate::reader::input_track::InputTrack;
use std::path::Path;
pub fn list_files_in_dir(dir_path: &str) -> Result<(Vec<InputTrack>, Vec<String>), std::io::Error> {
    let mut supported_files: Vec<InputTrack> = Vec::new();
    let mut unsupported_paths: Vec<String> = Vec::new();

    fn visit_dir(
        path: &Path,
        supported_files: &mut Vec<InputTrack>,
        unsupported_paths: &mut Vec<String>,
    ) -> Result<(), std::io::Error> {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                visit_dir(&path, supported_files, unsupported_paths)?;
                continue;
            }

            if !path.is_file() {
                continue;
            }

            let file_path: &str = match path.to_str() {
                Some(p) => p,
                None => {
                    continue;
                }
            };

            match crate::reader::new(file_path) {
                Ok(reader) => match reader.read(file_path) {
                    Ok(track) => {
                        supported_files.push(track);
                    }
                    Err(e) => eprintln!("Error reading file {}: {}", file_path, e),
                },
                Err(_) => unsupported_paths.push(file_path.to_string()),
            }
        }
        Ok(())
    }

    visit_dir(
        Path::new(dir_path),
        &mut supported_files,
        &mut unsupported_paths,
    )?;

    Ok((supported_files, unsupported_paths))
}
