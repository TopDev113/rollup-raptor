use std;

#[derive(Debug)]
pub enum MerkleTreeError{
    ZeroLevelsMissing
}

impl std::fmt::Display for MerkleTreeError{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MerkleTreeError::ZeroLevelsMissing => write!(f, "The zero levels are missing."),
        }
    }
}

impl std::error::Error for MerkleTreeError {}
