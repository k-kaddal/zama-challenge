use std::fs;
use std::error::Error;
use anyhow::{Result, Context};
use structopt::StructOpt;
use crate::utils;
use crate::api::Api;
use crate::merkle::{MerkleResponse, merkle_tree::MerkleTree};

#[derive(Debug, StructOpt)]
pub enum CliCommand {
    #[structopt(name = "upload", about = "Upload files to the server")]
    Upload {
        #[structopt(short = "p", long = "folder_path")]
        folder_path: String,
    },
    #[structopt(name = "download", about = "Download a file from the server")]
    Download {
        #[structopt(short = "f", long = "file_name")]
        file_name: String,
    },
}

impl CliCommand {
    pub fn execute(&self) -> Result<(), Box<dyn Error>> {
        match self {
            CliCommand::Upload { folder_path } => Self::upload_files(folder_path),
            CliCommand::Download { file_name } => Self::download_file(file_name),
        }
    }

    fn upload_files(folder_path: &str) -> Result<(), Box<dyn Error>> {
        let files_paths = utils::get_files_paths(folder_path)?;
        let files_content = utils::get_files_content(&files_paths)?;

        Api::upload_files(&files_paths)?;

        let files_with_names: Vec<(Vec<u8>, Option<String>)> = files_content
            .into_iter()
            .map(|(content, path)| (content, path.file_name().map(|os_str| os_str.to_string_lossy().to_string())))
            .collect();

        if let Some(merkle_root) = MerkleTree::create(&files_with_names) {
            let merkle_root_hash = merkle_root.hash;
            let directory_path = format!("{}/{}", folder_path, merkle_root_hash);

            fs::create_dir_all(&directory_path)
                .with_context(|| format!("Failed to create directory: {}", directory_path))?;

            let file_path = format!("{}/merkle_root.txt", directory_path);
            utils::save_to_file(&file_path, &merkle_root_hash.as_bytes())?;

            println!("Merkle hash: {}", merkle_root_hash);
        } else {
            println!("No files to build the Merkle tree.");
        }

        utils::delete_files(folder_path)?;

        Ok(())
    }

    fn download_file(file_name: &str) -> Result<(), Box<dyn Error>> {
        let file_bytes: Vec<u8> = Api::download_file(file_name)?;
        let merkle_response: MerkleResponse = Api::get_merkle_proof(file_name)?;

        let merkle_proof: Vec<String> = merkle_response.merkle_proof;
        let merkle_root_hash: String = merkle_response.merkle_root_hash;
        println!("Merkle Proof: {:?}", merkle_proof);

        // TODO: If valid, store file successfully
        // let target_hash: String = utils::hash(&file_bytes);
        // let is_valid = MerkleTree::verify_merkle_proof(&target_hash, &merkle_proof, &merkle_root_hash);

        let download_dir = format!("/app/data/uploads/{}", merkle_root_hash);
        let file_path = format!("{}/{}", download_dir, file_name);
        utils::save_to_file(&file_path, &file_bytes)?;

        Ok(())
    }
}
