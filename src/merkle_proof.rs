
#[derive(Debug, Clone, PartialEq)]
pub struct ProofElement{
    pub hash: String,
    pub left: bool,
}

impl ProofElement{
    pub fn new_from_index(hash: String, index: usize) -> Self{
        let left = index % 2 == 0;
        
        ProofElement{hash, left}
    }

}
