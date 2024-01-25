use crate::utils;

#[derive(Debug, Clone)]
pub struct MerkleNode {
    pub hash: String,
    pub left: Option<Box<MerkleNode>>,
    pub right: Option<Box<MerkleNode>>,
    pub file_name: Option<String>, 
}

impl MerkleNode {
    pub fn create_leaf (data: &[u8], file_name: Option<String>) -> MerkleNode {
        let hash = utils::hash(data);

        MerkleNode {
            hash,
            left: None,
            right: None,
            file_name,
        }
    }
    
    pub fn create_branch(left: MerkleNode, right: MerkleNode) -> MerkleNode {    
        let left_bytes = match hex::decode(&left.hash) {
            Ok(bytes) => bytes,
            Err(err) => panic!("{}", err),
        };
    
        let right_bytes = match hex::decode(&right.hash) {
            Ok(bytes) => bytes,
            Err(err) => panic!("{}", err),
        };
    
        let mut combined_bytes = left_bytes;
        combined_bytes.extend(&right_bytes);

        let hashed_branch =  utils::hash(&combined_bytes);
    
        MerkleNode {
            hash: hashed_branch,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
            file_name: None, 
        }
    }
}
