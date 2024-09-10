use merkle_tree::merkle_tree::MerkleTree;

fn main() {
    // TESTING PURPOSES
    // let elements = vec!["a".to_string(), "b".to_string(),"c".to_string(), "d".to_string(), "e".to_string(),"f".to_string()];
    let a = "a".to_string();
    let b = "b".to_string();
    let c = "c".to_string();
    let d = "d".to_string();

    let elements = vec![a, b,c,d];
    let mut mktree = MerkleTree::build(elements).unwrap();

    let proof = mktree.gen_proof("2e7d2c03a9507ae265ecf5b5356885a53393a2029d241394997265a1a25aefc6".to_string()).unwrap();

    println!("{:?}",mktree);


    println!("Proof: {:?}",proof);
}
