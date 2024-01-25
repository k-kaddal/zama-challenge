use crypto::digest::Digest;
use crypto::sha2::Sha256;
use std::fs;
use std::path::PathBuf;
use std::error::Error;
use std::io::{self, Read, Write};
use anyhow::Result;

pub fn hash(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.input(data);
    hasher.result_str()
}

// pub fn hash_nodes(left: &str, right: &str) -> String {
//     let left_bytes = decode(left).expect("Failed to decode left hash");
//     let right_bytes = decode(right).expect("Failed to decode right hash");

//     let mut hasher = Sha256::new();
//     hasher.input(&left_bytes);
//     hasher.input(&right_bytes);

//     hasher.result_str()
// }

pub fn delete_files(folder_path: &str) -> Result<(), Box<dyn Error>> {
    let files = fs::read_dir(folder_path)?;

    for file in files {
        let path = file?.path();

        if path.is_file() {
            fs::remove_file(&path)?;
        }
    }

    Ok(())
}

pub fn get_files_paths(folder_path: &str) -> Result<Vec<PathBuf>, io::Error> {
    let files: Vec<PathBuf> = fs::read_dir(folder_path)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().map(|ft| ft.is_file()).unwrap_or(false))
        .map(|entry| entry.path())
        .collect();

    Ok(files)
}

pub fn get_files_content(file_paths: &[PathBuf]) -> Result<Vec<(Vec<u8>, PathBuf)>, io::Error> {
    let file_contents: Result<Vec<(Vec<u8>, PathBuf)>, io::Error> = file_paths
        .iter()
        .map(|file_path| {
            let mut file_content = Vec::new();
            let mut file = fs::File::open(file_path)?;
            file.read_to_end(&mut file_content)?;
            Ok((file_content, file_path.clone()))
        })
        .collect();

    file_contents
}

pub fn save_to_file(file_path: &str, content: &[u8]) -> Result<(), io::Error> {
    let mut file = fs::File::create(file_path)?;
    file.write_all(content)?;
    Ok(())
}
