use std::path::PathBuf;
use std::error::Error;
use reqwest::blocking::{Client, multipart};
use crate::merkle::MerkleResponse;
use crate::config::Config;

pub struct Api;

impl Api {
    pub fn upload_files(files: &[PathBuf]) -> Result<(), Box<dyn Error>> {
        let client = Client::new();
        let base_url = Config::config().server_url.clone();
        let uri = format!("{}/files/upload", base_url);

        let mut form = multipart::Form::new();

        for file_path in files {
            let file = multipart::Part::file(file_path)?;
            form = form.part("files", file);
        }

        let response = client.post(uri).multipart(form).send()?;
        
        if response.status().is_success() {
            println!("Files uploaded successfully");
            Ok(())
        } else {
            println!("Failed to upload files. Status: {:?}", response.status());
            Err("Request failed".into())
        }
    }

    pub fn download_file(file_name: &str) -> Result<Vec<u8>, Box<dyn Error>> {
        let base_url = Config::config().server_url.clone();
        let uri = format!("{}/files/{}/download", base_url, file_name);
    
        let response = reqwest::blocking::get(&uri)?;

        if response.status().is_success() {            
            let bytes = response.bytes()?;
            println!("File '{}' downloaded successfully", file_name);
            Ok(bytes.to_vec())
        } else {
            println!("Failed to download file. Status code: {}", response.status());
            Err("Request failed".into())
        }
    }

    pub fn get_merkle_proof(file_name: &str) -> Result<MerkleResponse, Box<dyn Error>> {
        let base_url = Config::config().server_url.clone();
        let uri = format!("{}/files/{}", base_url, file_name);

        let response = reqwest::blocking::get(&uri)?;

        if response.status().is_success() {
            let response_body = response.text()?;
            let parsed_response: MerkleResponse = serde_json::from_str(&response_body)?;
            Ok(parsed_response)
        } else {
            println!("Failed to get Merkle Proof. Status code: {}", response.status());
            Err("Request failed".into())
        }
    }
}
