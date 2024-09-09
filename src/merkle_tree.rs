
use hex_literal::hex;
use sha2::{Sha256, Sha512, Digest};

/// A Hash is a String, just to differentiate it from a normal element String.
type Hash = String;

#[derive(Debug)]
pub struct MerkleTree{
    tree: Vec<Vec<String>>,
}

impl MerkleTree{
    // elements should be hashed
    pub fn build(elements: Vec<String>) -> Self{
        // 1. hash elements
        // 2. add hashes as leafs of the tree
        // 3. calculate upper level based on lower level iteratively (using other function)
        //      until the current level is of size 1 (Merkle Root). It adds levels to MerkleTree

        todo!();
        MerkleTree { tree: vec![elements] }
    }

    // Given a level N of the tree it calculates and returns the upper level of it.
    // Note: Consider the case of odd number of elements in one level, the last one should be repeated.
    fn calculate_upper_level(lower_level: Vec<Hash>) -> Vec<Hash>{
        todo!();
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

    fn hash(element: String) -> Hash{
        todo!();
    }
}
