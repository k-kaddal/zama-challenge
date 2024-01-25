pub mod merkle_node;
pub mod merkle_tree;
use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub struct MerkleResponse {
    pub file_name: String,
    pub merkle_proof: Vec<String>,
    pub merkle_root_hash: String,
}