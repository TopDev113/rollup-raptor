use std::collections::HashMap;
use std::thread::current;
use uint::construct_uint;
use crate::helpers::{hash_bytes, hashLeftRight};
use crate::config::{DEFAULT_DEPTH};
use crate::error::{MerkleTreeError};

#[derive(Debug, Clone, PartialEq)]
struct MerkleNode{
    data: Vec<u8>,
    left: Option<Box<MerkleNode>>,
    right: Option<Box<MerkleNode>>
}

impl MerkleNode{
    fn is_left(&self, right: &MerkleNode) -> bool{
        match &self.left{
            Some(node) => {
                **node == *right
            },
            None => false
        }
    }
    fn is_right(&self, left: &MerkleNode) -> bool{
        match &self.right{
            Some(node) => {
                **node == *left
            },
            None => false
        }
    }
}

struct MerkleTree{
    root: Option<MerkleNode>,
    zero_levels: Option<Vec<Vec<u8>>>,
    depth: u32
}

impl MerkleTree{
    fn calculate_zero_levels(&mut self, zero_node: Vec<u8>){
        let mut zero_levels: Vec<Vec<u8>> = vec![zero_node];
        for level in 0..self.depth - 2{
            zero_levels.push(hashLeftRight(zero_levels[zero_levels.len()-1].clone(), zero_levels[zero_levels.len()-1].clone()))
        };
        self.zero_levels = Some(zero_levels);
    }
    fn build(&mut self, mut data: Vec<Vec<u8>>) -> Result<(), MerkleTreeError>{
        let zero_node: Vec<u8> = hash_bytes(vec![0;32]);
        self.calculate_zero_levels(zero_node.clone());
        let mut zero_levels: Vec<Vec<u8>> = Vec::new();
        let result = match &self.zero_levels {
            Some(levels) => {
                zero_levels = levels.clone();
                Ok(())
            },
            None => Err(MerkleTreeError::ZeroLevelsMissing)
        }?;

        let mut input_length: u32 = data.len() as u32;
        if input_length % 2 != 0{
            data.push(zero_node.clone());
            input_length += 1;
        };
        let mut current_level: Vec<MerkleNode> = data
            .into_iter()
            .map(|data| MerkleNode{data: data, left: None, right: None})
            .collect();
        current_level.reverse();
        let mut height = 1;
        while current_level.len() > 1{
            if current_level.len() % 2 != 0{
                current_level.push(MerkleNode { data: zero_levels[height].clone(), left: None, right: None });
            };
            let mut next_level: Vec<MerkleNode> = Vec::new();
            while current_level.len() > 1{
                let left = current_level.pop().expect("Missing left node!");
                let right = current_level.pop().expect("Missing right node!");
                if &left.data == &right.data{
                    next_level.push(MerkleNode { 
                        data: zero_levels[height].clone(), 
                        left: Some(Box::new(left)),
                        right: Some(Box::new(right))
                    });
                }
                else{
                    next_level.push(MerkleNode { 
                        data: hashLeftRight(left.clone().data, right.clone().data), 
                        left: Some(Box::new(left)), 
                        right: Some(Box::new(right)) 
                    });
                };
            }
            current_level = next_level;
            height += 1;
        };
        self.root = Some(current_level.pop().expect("Failed to pop root from stack!"));
        Ok(())
    }
}


#[test]
fn optimized_merkle_tree(){
    let mut tree = MerkleTree{
        root: None,
        zero_levels: None,
        depth: 5
    };
    let transactions: Vec<Vec<u8>> = vec![vec![1;32]];
    tree.build(transactions);
    println!("Tree root: {:?}", tree.root);
}



#[derive(Debug, Clone, PartialEq)]
struct OpTree{
    levels: Vec<Vec<u8>>
}
