use merkle_tree::merkle_tree::MerkleTree;

fn main() {
    // TESTING PURPOSES
    let elements = vec!["a".to_string(), "b".to_string(),"c".to_string(), "d".to_string(), "e".to_string(),"f".to_string()];
    let mktree = MerkleTree::build(elements);

    
    println!("{:?}",mktree);
}
