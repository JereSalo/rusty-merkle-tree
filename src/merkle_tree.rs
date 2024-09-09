
use std::vec;

use hex;
use sha2::{Sha256, Digest};

use crate::merkle_tree;

/// A Hash is a String, just to differentiate it from a normal element String.
type Hash = String;

#[derive(Debug)]
pub struct MerkleTree{
    tree: Vec<Vec<String>>,
}

impl MerkleTree{
    // Considerations for implementing the most basic thing:
    //  Not going to hash elements, will assume elements are pow of 2
    pub fn build(elements: Vec<String>) -> Self{
        let mut merkle_tree = MerkleTree::new();
        // tree elements.clone();
        merkle_tree.tree.push(elements.clone());

        // While merkle root is not reached (length 1)
        let upper_level = MerkleTree::calculate_upper_level(elements.clone());
        merkle_tree.tree.push(upper_level);

        
        todo!();
    }

    pub fn new() -> MerkleTree{
        MerkleTree { tree: vec![vec![]]}
    }

    // Given a level N of the tree it calculates and returns the upper level of it.
    // Note: Consider the case of odd number of elements in one level, the last one should be repeated.
    pub fn calculate_upper_level(actual_level: Vec<Hash>) -> Vec<Hash>{
        let mut next_level: Vec<String> = vec![];

        // Iterate list (assuming even) and calculate hashes
        for (i, s_left) in actual_level.iter().enumerate().step_by(2){
            let s_right = actual_level[i+1].clone();

            // let result = format!("{}{}", s_left, s_right);
            let combined_hashes = format!("{}{}", s_left, s_right);

            let result_hash = Self::hash(combined_hashes);
            next_level.push(result_hash)
        }

        next_level
    }

    pub fn verify(&self, hash: Hash, proof: Vec<Hash>) -> bool{
        todo!();
    }

    pub fn gen_proof(&self, hash: Hash) -> Vec<String>{
        todo!();
    }

    pub fn add_element(&mut self, element: String){
        todo!();
    }

    pub fn hash(element: String) -> Hash{
        let mut hasher = Sha256::new();
        hasher.update(element);
        let result = hasher.finalize();
        hex::encode(result)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn calculate_hash(){
        // SHA256 of "a" is ca978112ca1bbdcafac231b39a23dc4da786eff8147c4e72b9807785afee48bb
        let str_test = String::from("a");
        let hard_hash = "ca978112ca1bbdcafac231b39a23dc4da786eff8147c4e72b9807785afee48bb".to_string();
        let calc_hash = MerkleTree::hash(str_test);

        assert_eq!(hard_hash, calc_hash);
    }
}
