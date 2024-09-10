use std::vec;
use hex;
use sha2::{Digest, Sha256};
use crate::merkle_error::MerkleError;
use crate::merkle_proof::ProofElement;

#[derive(Debug, PartialEq)]
pub struct MerkleTree{
    tree: Vec<Vec<String>>
}

impl MerkleTree{
    /// Builds merkle tree from elements list, hashing them first.
    pub fn build(elements: Vec<String>) -> Result<Self, MerkleError>{
        // 1. New merkle tree
        let mut merkle_tree = MerkleTree::new();

        // 2. Hash elements and push them into tree
        let mut elements_to_push = vec![];
        for element in elements{
            let hash = Self::hash(&element);
            elements_to_push.push(hash);
        }

        // clone the last one if qty of elements is odd
        if elements_to_push.len() % 2 != 0 {
            let last = elements_to_push.last().ok_or(MerkleError::EmptyList)?.clone();
            elements_to_push.push(last);
        }
        
        merkle_tree.tree.push(elements_to_push.clone());
        
        // 3. Calculate upper levels and push them to the tree until root is reached.
        while elements_to_push.len() > 1 {
            elements_to_push = Self::calculate_upper_level(&elements_to_push);

            // If qty of elements in a non-root node is uneven clone the last one.
            if elements_to_push.len() % 2 != 0 && elements_to_push.len() > 1 { 
                let last = elements_to_push.last().ok_or(MerkleError::EmptyList)?.clone();
                elements_to_push.push(last);
            }
            
            merkle_tree.tree.push(elements_to_push.clone());
        }

        Ok(merkle_tree)
    }

    pub fn build_without_hashing(elements: Vec<String>) -> Result<Self, MerkleError>{
        // 1. New merkle tree
        let mut merkle_tree = MerkleTree::new();

        // 2. Hash elements and push them into tree
        let mut elements_to_push = elements.clone();

        // clone the last one if qty of elements is odd
        if elements_to_push.len() % 2 != 0 {
            let last = elements_to_push.last().ok_or(MerkleError::EmptyList)?.clone();
            elements_to_push.push(last);
        }
        
        merkle_tree.tree.push(elements_to_push.clone());
        
        // 3. Calculate upper levels and push them to the tree until root is reached.
        while elements_to_push.len() > 1 {
            elements_to_push = Self::calculate_upper_level(&elements_to_push);

            // If qty of elements in a non-root node is uneven clone the last one.
            if elements_to_push.len() % 2 != 0 && elements_to_push.len() > 1 { 
                let last = elements_to_push.last().ok_or(MerkleError::EmptyList)?.clone();
                elements_to_push.push(last);
            }
            
            merkle_tree.tree.push(elements_to_push.clone());
        }

        Ok(merkle_tree)
    }

    pub fn new() -> MerkleTree{
        MerkleTree { tree: vec![] }
    }

    
    pub fn verify(&self, hash: String, proof: Vec<ProofElement>) -> Result<bool, MerkleError>{
        // Calculates root with element hash (leaf node) and it's proof
        let calc_root = proof.iter().fold(hash, |cur_hash, partner| { 
            let combined_hashes = if partner.left {
                partner.hash.clone() + &cur_hash
            } else {
                cur_hash + &partner.hash
            };

            MerkleTree::hash(&combined_hashes)
        });

        let real_root = self.get_root()?;

        Ok(calc_root == *real_root)
    }
    
    pub fn gen_proof(&self, hash: String) -> Result<Vec<ProofElement>, MerkleError> {
        let mut proof: Vec<ProofElement> = vec![];
        
        // 1. Find hash index
        let mut i: Option<usize> = None;
        for (index, element) in self.tree[0].iter().enumerate() {
            if *element == hash {
                i = Some(index);
                break;
            }
        }
        
        let mut i = i.ok_or(MerkleError::NotFound)?;
        
        // 2. Push it's partner in the same level to the proof
        
        // If even, the partner's index is i + 1; if odd, it is i - 1
        let i_partner = if i % 2 == 0 {
            i + 1
        }
        else{
            i - 1
        };
        
        let proof_elem = ProofElement::new_from_index(self.tree[0][i_partner].clone(), i_partner);
        proof.push(proof_elem);
        
        // 3. Now push the elements, climbing up on every level
        let mut level = 1;
        while level < self.tree.len() - 1 { // While root hasn't been reached
            // Math for getting the next element in the proof:
            //      floor(n/2) + 1 if floor(n/2) even
            //      floor(n/2) - 1 if floor(n/2) odd
            let idx= if (i/2) % 2 == 0 {
                i/2 + 1
            }
            else{
                i/2 - 1
            };
            let proof_elem = ProofElement::new_from_index(self.tree[level][idx].clone(), idx);
            proof.push(proof_elem);
            level += 1;
            i = idx;
        }
        Ok(proof)
    }

    /// Adds element and rebuilds the tree.
    pub fn add_element(&mut self, element: String) -> Result<(), MerkleError>{
        
        // Check if last 2 elements are equal, which if true would mean the last one was cloned
        if self.tree[0].len() >= 2 {
            let second_to_last = self.tree[0].get(self.tree[0].len()-2).ok_or(MerkleError::LastElementErr)?;
            let last = self.tree[0].last().ok_or(MerkleError::LastElementErr)?;

            if last == second_to_last {
                // replace last element because it's a clone, not a concrete element.
                self.tree[0].pop();
            }
        }
        
        let hashed_element = Self::hash(&element); 
        self.tree[0].push(hashed_element);

        *self = MerkleTree::build_without_hashing(self.tree[0].clone())?; // Rebuilds the tree from scratch, not efficient but Make it Work
        Ok(())
    }

    // Given a level N of the tree it calculates and returns the upper level of it.
    // Note: I'm not considering the case of odd qty of elements being sent because it is something that won't happen. The tree will always have e2ven number of nodes on each sub-root level.
    fn calculate_upper_level(actual_level: &Vec<String>) -> Vec<String>{
        let mut next_level: Vec<String> = vec![];

        // Iterate list and calculate hashes
        for (i, s_left) in actual_level.iter().enumerate().step_by(2){
            let s_right = &actual_level[i+1];

            let combined_hashes = format!("{}{}", s_left, s_right);

            let result_hash = Self::hash(&combined_hashes);
            next_level.push(result_hash);
        }

        next_level
    }

    fn hash(element: &str) -> String{
        let mut hasher = Sha256::new();
        hasher.update(element);
        let result = hasher.finalize();
        hex::encode(result)
    }

    fn get_root(&self) -> Result<&String, MerkleError>{
        let root = self.tree.last().ok_or(MerkleError::EmptyList)?.get(0).ok_or(MerkleError::EmptyList)?;
        Ok(root)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    fn build_basic_tree() -> MerkleTree{
        let a = "a".to_string();
        let b = "b".to_string();
        let c = "c".to_string();
        let d = "d".to_string();
        let elements = vec![a, b, c, d];

        MerkleTree::build(elements).unwrap()
    }

    #[test]
    fn calculate_hash(){
        // SHA256 of "a" is ca978112ca1bbdcafac231b39a23dc4da786eff8147c4e72b9807785afee48bb
        let expected_hash = "ca978112ca1bbdcafac231b39a23dc4da786eff8147c4e72b9807785afee48bb";

        let calc_hash = MerkleTree::hash("a");

        assert_eq!(expected_hash, calc_hash);
    }

    // This one is with a trivial example
    #[test]
    fn calculate_next_level(){
        // EXPECTED RESULTS
        //      Hash of "ab" is fb8e20fc2e4c3f248c60c39bd652f3c1347298bb977b8b4d5903b85055620603
        //      Hash of "cd" is 21e721c35a5823fdb452fa2f9f0a612c74fb952e06927489c6b27a43b817bed4
        let expected_next_level = vec!["fb8e20fc2e4c3f248c60c39bd652f3c1347298bb977b8b4d5903b85055620603".to_string(), "21e721c35a5823fdb452fa2f9f0a612c74fb952e06927489c6b27a43b817bed4".to_string()];
        //      SHA256 of the other hashes concatenated is this one.
        let expected_following_level = vec!["12a40550c10c6339bf6f271445270e49b844d6c9e8abc36b9b642be532befe94".to_string()];


        let current_level = vec!["a".to_string(), "b".to_string(), "c".to_string(), "d".to_string()];

        let next_level = MerkleTree::calculate_upper_level(&current_level);

        let following_level = MerkleTree::calculate_upper_level(&next_level);

        assert_eq!(expected_next_level, next_level);
        assert_eq!(expected_following_level, following_level);
    }

    #[test]
    fn build_tree(){
        // Im just going to check if the root of this tree is the expected, if that's right then it is well built.
        let mktree = build_basic_tree();


        let merkle_root = mktree.get_root().unwrap();
        let expected_root = "58c89d709329eb37285837b042ab6ff72c7c8f74de0446b091b6a0131c102cfd";

        assert_eq!(merkle_root,expected_root);
    }

    #[test]
    fn make_proof(){
        let mktree = build_basic_tree();

        let hash = "2e7d2c03a9507ae265ecf5b5356885a53393a2029d241394997265a1a25aefc6".to_string();
        let proof = [ProofElement { hash: "18ac3e7343f016890c510e93f935261169d9e3f565436429830faf0934f4f8e4".to_string(), left: false }, ProofElement { hash: "62af5c3cb8da3e4f25061e829ebeea5c7513c54949115b1acc225930a90154da".to_string(), left: true }].to_vec();

        let generated_proof = mktree.gen_proof(hash).unwrap();

        assert_eq!(proof, generated_proof);
    }

    #[test]
    fn verify_proof_true(){
        let mktree = build_basic_tree();

        // Provided the right proof for a tree return true.
        let hash = "2e7d2c03a9507ae265ecf5b5356885a53393a2029d241394997265a1a25aefc6".to_string();
        let proof = [ProofElement { hash: "18ac3e7343f016890c510e93f935261169d9e3f565436429830faf0934f4f8e4".to_string(), left: false }, ProofElement { hash: "62af5c3cb8da3e4f25061e829ebeea5c7513c54949115b1acc225930a90154da".to_string(), left: true }].to_vec();


        let validation = mktree.verify(hash, proof).unwrap();

        assert!(validation);
    }

    #[test]
    fn verify_proof_false(){
        let mktree = build_basic_tree();

        // Provided the wrong proof for a tree return false.
        let hash = "2e7d2c03a9507ae265ecf5b5356885a53393a2029d241394997265a1a25aefc6".to_string();
        // WRONG PROOF FOR THIS ELEMENT
        let proof = [ProofElement { hash: "18ac3e7343f016890c510e93f935261169d9e3f565436429830faf0934f4f8e4".to_string(), left: false }, ProofElement { hash: "62af5c3cb8da3e4f25061JEREebeea5c7513c54949115b1acc225930a90154da".to_string(), left: true }].to_vec();


        let validation = mktree.verify(hash, proof).unwrap();

        assert!(!validation);
    }

    #[test]
    fn add_element(){
        let expected_tree = build_basic_tree();
        let mut mktree = MerkleTree {tree: vec![vec![]]};

        let _ = mktree.add_element("a".to_string());
        let _ = mktree.add_element("b".to_string());
        let _ = mktree.add_element("c".to_string());
        let _ = mktree.add_element("d".to_string());

        assert_eq!(mktree, expected_tree);
    }
}
