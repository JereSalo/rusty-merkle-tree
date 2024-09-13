use crate::{hash::Hash, side::Side};

#[derive(Debug, Clone, PartialEq)]
pub struct ProofElement {
    pub hash: Hash,
    pub side: Side,
}

impl ProofElement {
    pub fn new_from_index(hash: String, index: usize) -> Self {
        let side = if index % 2 == 0 {
            Side::Left
        } else {
            Side::Right
        };

        ProofElement { hash, side }
    }
}
