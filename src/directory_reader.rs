use std::fs;

use crate::reader::track::Track;

pub fn list_files_in_dir(dir_path: &str) -> Result<(Vec<Track>, Vec<String>), std::io::Error> {
    let mut supported_files: Vec<Track> = Vec::new();
    let mut unsupported_paths: Vec<String> = Vec::new();
    let mut failed_reads: Vec<String> = Vec::new();

    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        let file_path: &str = match path.to_str() {
            Some(p) => p,
            None => {
                failed_reads.push(path.to_string_lossy().to_string());
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

    Ok((supported_files, unsupported_paths))
}
