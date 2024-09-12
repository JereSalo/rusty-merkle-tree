use std::fmt;

#[derive(Debug)]
pub enum MerkleError {
    EmptyTree,
    NotFound(String),
    DuplicateElement,
    ParsingError(String)
}

impl fmt::Display for MerkleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MerkleError::EmptyTree => write!(f, "Error: Tree is empty!"),
            MerkleError::NotFound(element) => {
                write!(f, "'{}' wasn't found in merkle tree", element)
            }
            MerkleError::DuplicateElement => {
                write!(f, "You can't insert duplicate elements into the tree!")
            }
            MerkleError::ParsingError(msg) =>{
                write!(f, "Proof file error - {}", msg)
            }
        }
    }
}

impl std::error::Error for MerkleError {}
