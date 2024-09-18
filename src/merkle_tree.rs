use crate::{merkle_error::MerkleError, proof_element::ProofElement, side::Side};
use hex;
use sha2::{Digest, Sha256};
use std::{collections::HashSet, fmt, vec};

#[derive(Debug, PartialEq, Default)]
pub struct MerkleTree {
    tree: Vec<Vec<String>>,
}

impl fmt::Display for MerkleTree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Reverse the tree levels for printing, starting with the root node.
        for (level, nodes) in self.tree.iter().rev().enumerate() {
            writeln!(f, "Level {}:\n  {}", level, nodes.join("\n  "))?;
        }
        Ok(())
    }
}

impl MerkleTree {
    pub fn new_empty() -> MerkleTree {
        MerkleTree { tree: vec![vec![]] }
    }

    /// Builds merkle tree from elements list, hashing them first if specified. Hashing can be skipped if user wants to build a tree with hashes directly instead of elements.
    pub fn build(elements: &[&str], hashed: bool) -> Result<Self, MerkleError> {
        if has_duplicates(elements) {
            return Err(MerkleError::DuplicateElement);
        }

        let mut merkle_tree = MerkleTree { tree: vec![] };

        // Hash elements if not hashed
        let mut elements_to_push: Vec<String> = if !hashed {
            elements.iter().map(|e| Self::hash(e)).collect()
        } else {
            elements.iter().map(|e| e.to_string()).collect()
        };

        // Push every level to the tree (cloning last element if necessary) until root is reached.
        while elements_to_push.len() > 1 {
            if let (true, Some(last_element)) = (elements_to_push.len() % 2 != 0, elements_to_push.last()) {
                elements_to_push.push(last_element.clone())
            }

            merkle_tree.tree.push(elements_to_push.clone());

            elements_to_push = Self::calculate_upper_level(&elements_to_push);
        }

        merkle_tree.tree.push(elements_to_push.clone()); // Push the root node.

        Ok(merkle_tree)
    }

    pub fn verify(&self, hash: String, proof: Vec<ProofElement>) -> Result<bool, MerkleError> {
        // Calculates root with element hash (leaf node) and it's proof
        let calc_root = proof.iter().fold(hash, |cur_hash, partner| {
            let combined_hash = if partner.side == Side::Left {
                format!("{}{}", partner.hash, cur_hash)
            } else {
                format!("{}{}", cur_hash, partner.hash)
            };
            MerkleTree::hash(&combined_hash)
        });

        let real_root = self.get_root()?;

        Ok(calc_root == *real_root)
    }

    pub fn gen_proof(&self, hash: String) -> Result<Vec<ProofElement>, MerkleError> {
        let mut proof: Vec<ProofElement> = vec![];

        // 1. Find index of given hash in leaves.
        let mut i = self.find_hash_index(hash)?;

        // 2. Push the elements, climbing up on every level. Stopping right before reaching the root node.
        let mut level = 0;
        while level < self.tree.len() - 1 {
            let i_partner = Self::get_partner_index(i);
            let proof_elem =
                ProofElement::new_from_index(self.tree[level][i_partner].clone(), i_partner);
            proof.push(proof_elem);
            level += 1;
            i /= 2; // Next level will have half the nodes
        }
        Ok(proof)
    }

    /// Adds an element/hash to the merkle tree, if 'hashed' is false it hashes it, otherwise it only adds it to the tree
    pub fn add(&mut self, element: String, hashed: bool) -> Result<(), MerkleError> {
        let hash = if hashed {
            element
        } else {
            Self::hash(&element)
        };

        // Not allowing to insert an element that's already in the tree.
        if self.tree[0].contains(&hash) {
            return Err(MerkleError::DuplicateElement);
        }

        // If last 2 elements are equal, the last one is a clone.
        if let Some((last, all_but_last)) = self.tree[0].split_last() {
            let second_to_last = all_but_last.last();
            if second_to_last == Some(last) {
                self.tree[0].pop();
            }
        };

        self.tree[0].push(hash);
        
        let slice_of_strs: Vec<&str> = self.tree[0].iter().map(String::as_str).collect();
        *self = MerkleTree::build(&slice_of_strs, true)?; // Rebuilds tree from sliceit Work

        Ok(())
    }

    /// Given a level N of the tree it calculates and returns the upper level of it.
    /// > Note: Case of level with odd quantity of elements is not considered because
    /// > Merkle Tree always has an even quantity of elements (last one duplicated if necessary)
    fn calculate_upper_level(actual_level: &[String]) -> Vec<String> {
        let mut next_level: Vec<String> = vec![];

        // Iterate list and calculate hashes
        for (i, left_hash) in actual_level.iter().enumerate().step_by(2) {
            let right_hash = &actual_level[i + 1];

            let combined_hashes = format!("{}{}", left_hash, right_hash);

            let result_hash = Self::hash(&combined_hashes);
            next_level.push(result_hash);
        }

        next_level
    }

    /// Returns SHA256 of a given element.
    fn hash(element: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(element);
        let result = hasher.finalize();
        hex::encode(result)
    }

    /// If tree is not empty, returns it's root hash.
    fn get_root(&self) -> Result<&String, MerkleError> {
        let root = self
            .tree
            .last()
            .ok_or(MerkleError::EmptyTree)?
            .first()
            .ok_or(MerkleError::EmptyTree)?;
        Ok(root)
    }

    /// Tries to find the index of a given hash. Returns error if not found.
    fn find_hash_index(&self, hash: String) -> Result<usize, MerkleError> {
        self.tree.get(0)
            .ok_or(MerkleError::EmptyTree)?
            .iter()
            .position(|element| *element == hash)
            .ok_or(MerkleError::NotFound(hash))
    }

    /// Given an element's index, gets it's partner's index.
    fn get_partner_index(my_index: usize) -> usize {
        if my_index % 2 == 0 {
            my_index + 1
        } else {
            my_index - 1
        }
    }
}

/// Checks for duplicate strings in a list
fn has_duplicates(vec: &[&str]) -> bool {
    let mut seen = HashSet::new();
    vec.iter().any(|item| !seen.insert(item))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_basic_tree() -> MerkleTree {
        let elements = ["a","b","c","d"];

        MerkleTree::build(&elements, false).unwrap()
    }

    #[test]
    fn calculate_hash() {
        // SHA256 of "a" is ca978112ca1bbdcafac231b39a23dc4da786eff8147c4e72b9807785afee48bb
        let expected_hash = "ca978112ca1bbdcafac231b39a23dc4da786eff8147c4e72b9807785afee48bb";

        let calc_hash = MerkleTree::hash("a");

        assert_eq!(expected_hash, calc_hash);
    }

    // This one is with a trivial example
    #[test]
    fn calculate_next_level() {
        // EXPECTED RESULTS
        //      Hash of "ab" is fb8e20fc2e4c3f248c60c39bd652f3c1347298bb977b8b4d5903b85055620603
        //      Hash of "cd" is 21e721c35a5823fdb452fa2f9f0a612c74fb952e06927489c6b27a43b817bed4
        let expected_next_level = vec![
            "fb8e20fc2e4c3f248c60c39bd652f3c1347298bb977b8b4d5903b85055620603".to_string(),
            "21e721c35a5823fdb452fa2f9f0a612c74fb952e06927489c6b27a43b817bed4".to_string(),
        ];
        //      SHA256 of the other hashes concatenated is this one.
        let expected_following_level =
            vec!["12a40550c10c6339bf6f271445270e49b844d6c9e8abc36b9b642be532befe94".to_string()];

        let current_level = vec![
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "d".to_string(),
        ];

        let next_level = MerkleTree::calculate_upper_level(&current_level);

        let following_level = MerkleTree::calculate_upper_level(&next_level);

        assert_eq!(expected_next_level, next_level);
        assert_eq!(expected_following_level, following_level);
    }

    #[test]
    fn build_tree() {
        // Im just going to check if the root of this tree is the expected, if that's right then it is well built.
        let mktree = build_basic_tree();

        let merkle_root = mktree.get_root().unwrap();
        let expected_root = "58c89d709329eb37285837b042ab6ff72c7c8f74de0446b091b6a0131c102cfd";

        assert_eq!(merkle_root, expected_root);
    }

    #[test]
    fn build_large_tree() {
        // Tree is going to have 4 levels, it's elements will be a b c d e f g h.
        // Expected root is 5d2a8967adb92f46e3266c0cddef844418e95fc6dbe733029e8a7da6145a5afe
        // If the tree built has that root then it means that all levels of it are well built. 
        let elements = ["a", "b", "c", "d", "e", "f", "g", "h"];
        let mktree = MerkleTree::build(&elements, false).unwrap();

        assert_eq!(mktree.get_root().unwrap(), "5d2a8967adb92f46e3266c0cddef844418e95fc6dbe733029e8a7da6145a5afe");
    }

    #[test]
    fn make_proof() {
        let mktree = build_basic_tree();

        let hash = "2e7d2c03a9507ae265ecf5b5356885a53393a2029d241394997265a1a25aefc6".to_string();
        let expected_proof = [
            ProofElement {
                hash: "18ac3e7343f016890c510e93f935261169d9e3f565436429830faf0934f4f8e4"
                    .to_string(),
                side: Side::Right,
            },
            ProofElement {
                hash: "62af5c3cb8da3e4f25061e829ebeea5c7513c54949115b1acc225930a90154da"
                    .to_string(),
                side: Side::Left,
            },
        ]
        .to_vec();

        let generated_proof = mktree.gen_proof(hash).unwrap();

        assert_eq!(expected_proof, generated_proof);
    }

    #[test]
    fn verify_proof_true() {
        let mktree = build_basic_tree();

        // Provided the right proof for a tree hash, calculate and return true.
        let hash = "2e7d2c03a9507ae265ecf5b5356885a53393a2029d241394997265a1a25aefc6".to_string();
        let proof = [
            ProofElement {
                hash: "18ac3e7343f016890c510e93f935261169d9e3f565436429830faf0934f4f8e4"
                    .to_string(),
                side: Side::Right,
            },
            ProofElement {
                hash: "62af5c3cb8da3e4f25061e829ebeea5c7513c54949115b1acc225930a90154da"
                    .to_string(),
                side: Side::Left,
            },
        ]
        .to_vec();

        let validation = mktree.verify(hash, proof).unwrap();

        assert!(validation);
    }

    #[test]
    fn verify_proof_false() {
        let mktree = build_basic_tree();

        // Provided the wrong proof for a tree hash, calculate and return false.
        let hash = "2e7d2c03a9507ae265ecf5b5356885a53393a2029d241394997265a1a25aefc6".to_string();
        // WRONG PROOF FOR THIS ELEMENT
        let proof = [
            ProofElement {
                hash: "18ac3e7343f016890c51ThisIsNotTheRightProof436429830faf0934f4f8e4"
                    .to_string(),
                side: Side::Right,
            },
            ProofElement {
                hash: "62af5c3cb8da3e4f25061esJEREeea5c7513c54949115b1acc225930a90154da"
                    .to_string(),
                side: Side::Left,
            },
        ]
        .to_vec();

        let validation = mktree.verify(hash, proof).unwrap();

        assert!(!validation);
    }

    #[test]
    fn add_element() -> Result<(), MerkleError> {
        let expected_tree = build_basic_tree();
        let mut mktree = MerkleTree { tree: vec![vec![]] };

        mktree.add(
            "ca978112ca1bbdcafac231b39a23dc4da786eff8147c4e72b9807785afee48bb".to_string(),
            true,
        )?; // hash of 'a'
        mktree.add("b".to_string(), false)?;
        mktree.add("c".to_string(), false)?;
        mktree.add("d".to_string(), false)?;

        assert_eq!(mktree, expected_tree);
        Ok(())
    }
}
