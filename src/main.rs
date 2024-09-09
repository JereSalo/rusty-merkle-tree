use merkle_tree::merkle_tree::MerkleTree;

fn main() {
    // TESTING PURPOSES
    let mktree = MerkleTree::new();

    let h = MerkleTree::hash("a".to_string());
    println!("{}",h);
    // let test_level = vec!["a".to_string(), "b".to_string()];

    // let next_level = MerkleTree::calculate_upper_level(test_level);
}
