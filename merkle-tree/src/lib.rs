use std::{collections::HashMap};
use uint::construct_uint;
pub mod helpers;
pub mod tornado;



/* Artifacts that I want to keep for now */


// mod deprecated;
// mod optimized;
//use helpers::{hash_bytes, hashLeftRight};
// mod config;
//use config::{DEFAULT_DEPTH, TEST_DEPTH};
// mod error;


/*
fn main(){
    panic!("Run cargo test -- --nocapture instead!");
}

#[derive(Debug, Clone)]
pub struct MerkleNode{
    pub data: Vec<u8>,
    pub left: Option<Box<MerkleNode>>,
    pub right: Option<Box<MerkleNode>>
}

#[derive(Debug, Clone)]
pub struct MerkleTree{
    pub root: Option<MerkleNode>,
    pub depth: u32
}

impl MerkleTree{
    pub fn build(&mut self, transactions: Vec<Vec<u8>>){
        // zero val for merkle roots
        let size: u32 = 2_u32.pow(self.depth-1_u32);
        let zero_val: Vec<u8> = hash_bytes(vec![0]);
        let mut zero_levels: Vec<Vec<u8>> = Vec::new();
        let mut current_level = zero_val.clone();
        zero_levels.push(zero_val.clone());
        // calculate zero levels
        for level in 0..self.depth - 2{
            let _hash = hashLeftRight(current_level.clone(), current_level.clone());
            zero_levels.push(_hash.clone());
            current_level = _hash;
        };

        let mut levels: Vec<Vec<MerkleNode>> = Vec::new();
        let mut bottom_level: Vec<MerkleNode> = Vec::new();
        // fill bottom level of the tree with data-nodes
        for tx in transactions{
            bottom_level.push(MerkleNode { 
                data: tx, 
                left: None, 
                right: None })
        };
        // fill rest of bottom level with empty nodes
        while bottom_level.len() < size as usize{
            bottom_level.push(MerkleNode { data: 
                zero_val.clone(), 
                left: None, 
                right: None });
        }

        let mut current_level: Vec<MerkleNode> = bottom_level.clone();
        let mut current_level_height: usize = 1;

        // calculate and fill all other levels i=1 => i=len
        while current_level.len() > 1{
            // ensure the amount of nodes in the level is even
            while current_level.len() % 2 != 0{
                current_level.push(MerkleNode { data: zero_levels[current_level_height].clone(), left: None, right: None });
            }
            let mut new_level: Vec<MerkleNode> = Vec::new();
            for i in (0..current_level.len()).step_by(2){
                let left: MerkleNode = current_level[i].clone();
                let right: MerkleNode = current_level[i+1].clone();
                // take the zero hash and push new node
                if left.data == right.data{
                    new_level.push(MerkleNode { 
                        data: zero_levels[current_level_height].clone(), 
                        left: Some(Box::new(left.clone())), 
                        right: Some(Box::new(right.clone())) 
                    });
                }
                // hash the children and push new node
                else{
                    new_level.push(MerkleNode { 
                        data: hashLeftRight(left.clone().data, right.clone().data), 
                        left: Some(Box::new(left.clone())), 
                        right: Some(Box::new(right.clone()))
                    });
                }
            };

            levels.push(new_level.clone());
            current_level = new_level.clone();
            current_level_height += 1;
        };
        self.root = Some(levels.pop().unwrap()[0].clone());

        // return here
    }
    // find the sibling of a leaf -> takes parent as input
    pub fn find_leaf_sibling(&self, parent: MerkleNode, target: Vec<u8>) -> Option<MerkleNode>{
        if let Some(ref left) = parent.left{
            if parent.clone().left.unwrap().data == target{
                return Some(*parent.clone().right.unwrap());
            }
            else{
                return Some(*parent.clone().left.unwrap());
            }
        }
        else{
            return None;
        }
    }
    // find the parent for a leaf in the tree
    pub fn find_leaf_parent(&self, root: MerkleNode, target: Vec<u8>) -> Option<MerkleNode>{
        // check if target in children
        if let Some(ref left) = root.left{
            if root.clone().left.unwrap().data == target{
                return Some(root);
            }
        }
        if let Some(ref right) = root.right{
            if root.clone().right.unwrap().data == target{
                return Some(root);
            }
        }
        /*
        if root.clone().left.unwrap().data == target || root.clone().right.unwrap().data == target{
            // return node with child
            return Some(root);
        };
        */
        if let Some(ref left) = root.left{
            let left_node: Option<MerkleNode> = self.find_leaf_parent(*root.clone().left.unwrap(), target.clone());
            if !left_node.is_none(){
                return  left_node;
            }
        }
        if let Some(ref right) = root.right{
            let right_node: Option<MerkleNode> = self.find_leaf_parent(*root.clone().right.unwrap(), target.clone());
            if !right_node.is_none(){
                return right_node;
            }
        }
        return None
    }
    // recursive function to find the path to a leaf
    pub fn find_leaf_path(&self, root: MerkleNode, target: Vec<u8>, mut path: Vec<Vec<u8>>) -> Option<Vec<Vec<u8>>>{
        if root.data == target{
            path.push(target);
            let mut proof_path: Vec<Vec<u8>> = Vec::new();
            for leaf in &path.clone()[1..path.len()]{
                //proof_path.push(leaf.clone());
                let leaf_parent: MerkleNode = self.find_leaf_parent(self.clone().root.unwrap(), leaf.clone()).unwrap();
                let leaf_sibling: MerkleNode = self.find_leaf_sibling(leaf_parent.clone(), leaf.clone()).unwrap();
                //println!("Leaf: {}, sibling: {:?}", leaf, leaf_sibling.data);
                proof_path.push(leaf_sibling.data);
            };
            return Some(proof_path);
        }
        let mut path_cp_left: Vec<Vec<u8>> = path.clone();
        path_cp_left.push(root.data.clone());
        let mut path_cp_right: Vec<Vec<u8>> = path.clone();
        path_cp_right.push(root.data.clone());
        if let Some(ref left) = root.left{
            let left_path: Option<Vec<Vec<u8>>> = self.find_leaf_path(*root.clone().left.unwrap(), target.clone(), path_cp_left);
            if !left_path.is_none(){
                return left_path;
            }
        }
        if let Some(ref right) = root.right{
            let right_path: Option<Vec<Vec<u8>>> = self.find_leaf_path(*root.clone().right.unwrap(), target, path_cp_right);
            if !right_path.is_none(){
                return right_path;
            }
        }
        return None;
    }
    pub fn merkle_path_in_order(&self, merkle_root: Vec<u8>, merkle_tree: MerkleNode, mut proof_path: Vec<Vec<u8>>) -> Vec<(Vec<u8>, bool)>{
        let mut in_order: Vec<(Vec<u8>, bool)> = Vec::new();
        let mut sibling: Vec<u8> = proof_path.pop().unwrap();
        let parent: MerkleNode = self.find_leaf_parent(merkle_tree.clone(), sibling.clone()).unwrap();
        in_order.push((sibling.clone(), false));
        while !proof_path.is_empty() {
            sibling = proof_path.pop().unwrap();
            let parent: MerkleNode = self.find_leaf_parent(merkle_tree.clone(), sibling.clone()).unwrap();
            if let Some(ref left) = parent.left{
                if left.clone().data == sibling{
                    // is left child
                    in_order.push((sibling.clone(), true));
                }
                else{
                    in_order.push((sibling.clone(), false));
                }
            }
        }
        in_order
    }
}


#[test]
fn produce_merkle_proof(){
    let mut tree = MerkleTree{
        root: None,
        depth: DEFAULT_DEPTH
    };

    let transactions = vec![
        vec![1;32], 
        vec![2;32], 
        vec![3;32], 
        vec![4;32], 
        vec![5;32], 
        vec![6;32], 
        vec![7;32], 
        vec![8;32], 
        vec![9;32], 
        vec![10;32], 
        vec![11;32], 
        vec![12;32]
    ];
    tree.build(transactions.clone());
    let parent: Option<MerkleNode> = tree.find_leaf_parent(tree.root.clone().unwrap(), transactions.clone()[10].clone());
    let sibling: Option<MerkleNode> = tree.find_leaf_sibling(parent.clone().unwrap(), transactions.clone()[10].clone());
    let mut path: Vec<Vec<u8>> = tree.find_leaf_path(tree.root.clone().unwrap(), transactions.clone()[10].clone(), Vec::new()).unwrap();
    // True -> is left, False -> is right
    let result: Vec<(Vec<u8>, bool)> = tree.merkle_path_in_order(tree.clone().root.unwrap().data, tree.clone().root.unwrap(), path);
    // compute merkle hash
    let mut current_hash: Vec<u8> = transactions.clone()[10].clone();
    for (sibling, indicator) in result.clone(){
        if indicator == false{
            current_hash = hashLeftRight(current_hash, sibling);
            let current_hash_hex: Vec<String> = current_hash.clone().iter()
            .map(|byte| format!("0x{:02x}", byte))
            .collect();
        }
        else{
            current_hash = hashLeftRight(sibling, current_hash);
        }
    }
    
    assert_eq!(current_hash, tree.clone().root.unwrap().data);
    // output merkle proof information
    println!("Transaction to prove: {:?}", transactions[0]);
    for (index, (sibling, indicator)) in result.into_iter().enumerate(){
        println!("Sibling #{:?}: {:?} : {:?}", index, sibling, indicator);
    }
    println!("Merkle root: {:?}", tree.clone().root.unwrap().data);

}

#[test]
fn build_merkle_tree(){
    let mut tree: MerkleTree = MerkleTree{
        root: None,
        depth: TEST_DEPTH
    };
    let transactions: Vec<Vec<u8>> = vec![vec![1;32]];
    tree.build(transactions.clone());
}
*/