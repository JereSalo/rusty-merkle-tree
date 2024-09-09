use std::vec;
use hex;
use sha2::{Sha256, Digest};

/// A Hash is a String, just to differentiate it from a normal element String.
type Hash = String;

#[derive(Debug)]
pub struct MerkleTree{
    tree: Vec<Vec<String>>,
    even_elem: bool,
}

impl MerkleTree{
    // Considerations for implementing the most basic thing:
    //  Not going to hash elements, assuming n of elements are even
    pub fn build(elements: Vec<String>) -> Self{
        // New empty Merkle Tree
        let mut merkle_tree = MerkleTree::new();

        // Add elements to it, but clone the last one if qty of elements is odd.
        merkle_tree.even_elem = elements.len() % 2 == 0;
        let mut elements_to_push = elements.clone();
        
        if !merkle_tree.even_elem{
            let last = elements.last().expect("Empty list").clone();
            elements_to_push.push(last);
        }
        
        merkle_tree.tree.push(elements_to_push.clone());
        
        // While merkle root is not reached (length 1)
        while elements_to_push.len() > 1{
            elements_to_push = MerkleTree::calculate_upper_level(&elements_to_push);

            if elements_to_push.len() > 1{
                let is_even = elements_to_push.len() % 2 == 0;
                if !is_even { 
                    elements_to_push.push(elements_to_push.last().unwrap().clone());
                }
            }
            
            merkle_tree.tree.push(elements_to_push.clone());
        }
        
        merkle_tree
    }

    pub fn new() -> MerkleTree{
        MerkleTree { tree: vec![], even_elem: true}
    }

    // Given a level N of the tree it calculates and returns the upper level of it.
    // Note: Consider the case of odd number of elements in one level, the last one should be repeated.
    pub fn calculate_upper_level(actual_level: &Vec<Hash>) -> Vec<Hash>{
        let mut next_level: Vec<Hash> = vec![];

        // Iterate list and calculate hashes
        for (i, s_left) in actual_level.iter().enumerate().step_by(2){
            // Considering even!
            let s_right = &actual_level[i+1];

            let combined_hashes = format!("{}{}", s_left, s_right);

            let result_hash = Self::hash(combined_hashes);
            next_level.push(result_hash);
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
        let expected_hash = "ca978112ca1bbdcafac231b39a23dc4da786eff8147c4e72b9807785afee48bb".to_string();

        let str_test = String::from("a");
        let calc_hash = MerkleTree::hash(str_test);

        assert_eq!(expected_hash, calc_hash);
    }

    // This one is with a trivial example
    #[test]
    fn calculate_next_level_even(){
        // Hash of "ab" is fb8e20fc2e4c3f248c60c39bd652f3c1347298bb977b8b4d5903b85055620603
        // Hash of "cd" is 21e721c35a5823fdb452fa2f9f0a612c74fb952e06927489c6b27a43b817bed4
        let expected_next_level = vec!["fb8e20fc2e4c3f248c60c39bd652f3c1347298bb977b8b4d5903b85055620603".to_string(), "21e721c35a5823fdb452fa2f9f0a612c74fb952e06927489c6b27a43b817bed4".to_string()];

        // SHA256 of the other hashes concatenated is this one.
        let expected_following_level = vec!["12a40550c10c6339bf6f271445270e49b844d6c9e8abc36b9b642be532befe94".to_string()];

        let current_level = vec!["a".to_string(), "b".to_string(), "c".to_string(), "d".to_string()];

        let next_level = MerkleTree::calculate_upper_level(&current_level);

        let following_level = MerkleTree::calculate_upper_level(&next_level);

        assert_eq!(expected_next_level, next_level);
        assert_eq!(expected_following_level, following_level);
    }


    #[test]
    fn calculate_next_level_odd(){
        // Hash of "a" is ca978112ca1bbdcafac231b39a23dc4da786eff8147c4e72b9807785afee48bb
        // We expect hash of "aa" as result because current level is going to be odd: 961b6dd3ede3cb8ecbaacbd68de040cd78eb2ed5889130cceb4c49268ea4d506
        let expected_next_level = vec!["961b6dd3ede3cb8ecbaacbd68de040cd78eb2ed5889130cceb4c49268ea4d506".to_string()];

        let current_level = vec!["a".to_string()];

        let next_level = MerkleTree::calculate_upper_level(&current_level);

        assert_eq!(expected_next_level, next_level);
    }
}
