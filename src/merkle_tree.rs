
use hex;
use sha2::{Sha256, Digest};

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
        
        
        todo!();
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
        let mut hasher = Sha256::new();
        hasher.update(element);
        let result = hasher.finalize();
        hex::encode(result)
    }
}
