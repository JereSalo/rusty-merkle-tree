use std::fmt;

#[derive(Debug)]
pub enum MerkleError {
    EmptyList(String),
    NotFound(String),
}

impl fmt::Display for MerkleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MerkleError::EmptyList(element) => write!(f, "'{}' list is empty", element),
            MerkleError::NotFound(element) => write!(f, "'{}' wasn't found in merkle tree", element),
        }
    }
}

impl std::error::Error for MerkleError {}
