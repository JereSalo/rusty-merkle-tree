use merkle_tree::merkle_tree::MerkleTree;

fn main() {
    // TESTING PURPOSES
    // let elements = vec!["a".to_string(), "b".to_string(),"c".to_string(), "d".to_string(), "e".to_string(),"f".to_string()];
    let a = MerkleTree::hash("a".to_string());
    let b = MerkleTree::hash("b".to_string());
    let c = MerkleTree::hash("c".to_string());
    let d = MerkleTree::hash("d".to_string());

    let elements = vec![a, b];
    let mut mktree = MerkleTree::build(elements);

    println!("{:?}",mktree);

    mktree.add_element(c);

    println!("{:?}",mktree);

    mktree.add_element(d);

    println!("{:?}",mktree);
}
