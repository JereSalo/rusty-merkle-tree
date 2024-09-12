use std::fmt;

#[derive(Debug)]
pub enum MerkleError {
    EmptyList(String),
    NotFound(String),
    DuplicateElement,
    ParsingError(String)
}

impl fmt::Display for MerkleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MerkleError::EmptyList(element) => write!(f, "'{}' list is empty", element),
            MerkleError::NotFound(element) => {
                write!(f, "'{}' wasn't found in merkle tree", element)
            }
            MerkleError::DuplicateElement => {
                write!(f, "You can't insert duplicate elements into the tree!")
            }
            MerkleError::ParsingError(msg) =>{
                write!(f, "Proof file parsing error - {}", msg)
            }
        }
    }
}

impl std::error::Error for MerkleError {}
