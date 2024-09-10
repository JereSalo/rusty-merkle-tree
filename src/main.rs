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

    let hash = "2e7d2c03a9507ae265ecf5b5356885a53393a2029d241394997265a1a25aefc6".to_string();
    let calc_root = proof.iter().fold(hash, |acc, next_hash| { 
            println!("acc {}, next hash: {}", acc, next_hash); 
            if index % 2 == 0 {
                MerkleTree::hash(&(acc + &next_hash))
            } else {
                MerkleTree::hash(&(next_hash.to_owned() + &acc))
            }
            });

    // In concat the order is from left to right, here I'm not specifying an order
    // I need hash index for this...
    // Even index: left, odd index: right

    println!("Proof: {:?}",proof);
    println!("Calculated Root: {}", calc_root);
}
