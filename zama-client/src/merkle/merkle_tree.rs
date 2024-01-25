use super::merkle_node::MerkleNode;

#[derive(Debug, Clone)]
pub struct MerkleTree {
    pub root_hash: String,
}

impl MerkleTree {
    pub fn create(files: &Vec<(Vec<u8>, Option<String>)>) -> Option<MerkleNode> {
        if files.is_empty() {
            return None;
        }
    
        let leaf_nodes: Vec<MerkleNode> = files
            .iter()
            .map(|(file, file_name)| MerkleNode::create_leaf(file, file_name.clone()))
            .collect();        
        
        let mut nodes = leaf_nodes;
        
        while nodes.len() > 1 {
            let mut new_nodes = Vec::new();
            
            for i in (0..nodes.len()).step_by(2) {
                let left = nodes[i].clone();
                let right = if i + 1 < nodes.len() {
                    nodes[i + 1].clone()
                } else {
                    nodes[i].clone()
                };
                
                let branch = MerkleNode::create_branch(left, right);
                new_nodes.push(branch.clone());
            }            
            nodes = new_nodes;
        }

        nodes.pop()
    }

    // TODO: positioning of the node needed for verification
    // pub fn verify_merkle_proof(target_hash: &str, proof: &[String], merkle_root_hash: &str) -> Result<bool, String> {
    //     let mut calculated_hash = target_hash.to_owned();
    //     for sibling_hash in proof {
    //         calculated_hash = utils::hash_nodes(&calculated_hash, sibling_hash);
    //     }

    //     if calculated_hash == merkle_root_hash {
    //         Ok(true)
    //     } else {
    //         Err("Merkle Tree verification failed".to_string())
    //     }
    // }
}
