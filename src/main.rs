use merkle_tree::merkle_tree::MerkleTree;

fn main() {
    // TESTING PURPOSES
    let mktree = MerkleTree::new();

    let a = "a".to_string();
    
    let test_level = vec![a.clone(), a.clone(), a.clone(), a.clone()];

    println!("{:?}",test_level);

    let next_level = MerkleTree::calculate_upper_level(&test_level);

    println!("{:?}",next_level);
}
