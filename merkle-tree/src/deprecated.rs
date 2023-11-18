/* String implementation, kept for reference

extern crate sha256;
use sha256::{digest, try_digest};
// insert, get
use std::{collections::HashMap};
use uint::construct_uint;
fn main(){
    panic!("Run cargo test -- --nocapture instead!");
}

fn hash_string(input: String) -> String{
    digest(input)
}

fn hashLeftRight(left: String, right: String) -> String{
    hash_string(left + &right)
}

#[derive(Debug, Clone)]
struct MerkleNode{
    data: String,
    left: Option<Box<MerkleNode>>,
    right: Option<Box<MerkleNode>>
}

#[derive(Debug, Clone)]
struct MerkleTree{
    root: Option<MerkleNode>,
    depth: u32
}

impl MerkleTree{
    // not optimized, but sufficient for circuit research with a small set of transactions.
    fn build(&mut self, transactions: Vec<String>){
        // zero val for merkle roots
        let size = 2_u32.pow(self.depth-1_u32);
        let zero_val = String::from("casper");
        let mut zero_levels: Vec<String> = Vec::new();
        let mut current_level = zero_val.clone();
        zero_levels.push(zero_val.clone());
        for level in 0..self.depth - 2{
            let _hash = hashLeftRight(current_level.clone(), current_level.clone());
            zero_levels.push(_hash.clone());
            current_level = _hash;
        };
        println!("Zero levels: {:?}", zero_levels);
        let mut levels: Vec<Vec<MerkleNode>> = Vec::new();
        let mut bottom_level: Vec<MerkleNode> = Vec::new();
        for tx in transactions{
            bottom_level.push(MerkleNode { 
                data: tx, 
                left: None, 
                right: None })
        };

        while bottom_level.len() < size as usize{
            bottom_level.push(MerkleNode { data: 
                zero_val.clone(), 
                left: None, 
                right: None });
        }

        let mut current_level = bottom_level.clone();
        println!("Bottom level: {:?}", &bottom_level);
        // start at first hash (one level above tx data)
        let mut current_level_height = 1;
        while current_level.len() > 1{
            println!("Current Level: {:?}", &current_level);


            while current_level.len() % 2 != 0{
                current_level.push(MerkleNode { data: zero_levels[current_level_height].clone(), left: None, right: None });
            }
            let mut new_level: Vec<MerkleNode> = Vec::new();



            println!("Len of current level: {:?}", &current_level.len());
            for i in (0..current_level.len()).step_by(2){
                let left: MerkleNode = current_level[i].clone();
                let right: MerkleNode = current_level[i+1].clone();

                /*
                    Don't hash if L === R, instead push zero_level node.
                    * create nodes for each level
                    * push the level node 
                */
                new_level.push(MerkleNode { 
                    data: hashLeftRight(left.clone().data, right.clone().data), 
                    left: Some(Box::new(left.clone())), 
                    right: Some(Box::new(right.clone()))}
                );
                levels.push(new_level.clone());
            };
            current_level = new_level.clone();
            current_level_height += 1;
        };
        self.root = Some(levels.pop().unwrap()[0].clone());

        // return here
    }
    // find the sibling of a leaf -> takes parent as input
    fn find_leaf_sibling(&self, parent: MerkleNode, target: String) -> Option<MerkleNode>{
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
    fn find_leaf_parent(&self, root: MerkleNode, target: String) -> Option<MerkleNode>{
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
            let left_node = self.find_leaf_parent(*root.clone().left.unwrap(), target.clone());
            if !left_node.is_none(){
                return  left_node;
            }
        }
        if let Some(ref right) = root.right{
            let right_node = self.find_leaf_parent(*root.clone().right.unwrap(), target.clone());
            if !right_node.is_none(){
                return right_node;
            }
        }
        return None
    }
    // recursive function to find the path to a leaf
    fn find_leaf_path(&self, root: MerkleNode, target: String, mut path: Vec<String>) -> Option<Vec<String>>{
        if root.data == target{
            path.push(target);
            let mut proof_path: Vec<String> = Vec::new();
            for leaf in &path.clone()[1..path.len()]{
                //proof_path.push(leaf.clone());
                let leaf_parent = self.find_leaf_parent(self.clone().root.unwrap(), leaf.clone()).unwrap();
                let leaf_sibling = self.find_leaf_sibling(leaf_parent.clone(), leaf.clone()).unwrap();
                //println!("Leaf: {}, sibling: {:?}", leaf, leaf_sibling.data);
                proof_path.push(leaf_sibling.data);
            };
            return Some(proof_path);
        }
        let mut path_cp_left = path.clone();
        path_cp_left.push(root.data.clone());
        let mut path_cp_right = path.clone();
        path_cp_right.push(root.data.clone());
        if let Some(ref left) = root.left{
            let left_path = self.find_leaf_path(*root.clone().left.unwrap(), target.clone(), path_cp_left);
            if !left_path.is_none(){
                return left_path;
            }
        }
        if let Some(ref right) = root.right{
            let right_path = self.find_leaf_path(*root.clone().right.unwrap(), target, path_cp_right);
            if !right_path.is_none(){
                return right_path;
            }
        }
        return None;
    }
    fn merkle_path_in_order(&self, merkle_root: String, merkle_tree: MerkleNode, mut proof_path: Vec<String>) -> Vec<(String, bool)>{
        let mut in_order: Vec<(String, bool)> = Vec::new();
        let mut sibling = proof_path.pop().unwrap();
        let parent = self.find_leaf_parent(merkle_tree.clone(), sibling.clone()).unwrap();
        in_order.push((sibling.clone(), false));
        while !proof_path.is_empty() {
            sibling = proof_path.pop().unwrap();
            let parent = self.find_leaf_parent(merkle_tree.clone(), sibling.clone()).unwrap();
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
fn test_string(){
    let mut tree = MerkleTree{
        root: None,
        depth: 5
    };

    let transactions = vec![String::from("tx01"), String::from("tx02")];
    tree.build(transactions);
    println!("Tree root: {:?}", tree.root);

    let parent = tree.find_leaf_parent(tree.root.clone().unwrap(), String::from("tx01"));
    println!("tx01 parent: {:?}", &parent);

    let sibling = tree.find_leaf_sibling(parent.clone().unwrap(), String::from("tx01"));
    println!("tx01 sibling: {:?}", &sibling);

    let mut path = tree.find_leaf_path(tree.root.clone().unwrap(), String::from("tx01"), Vec::new()).unwrap();
    println!("Path: {:?}", &path);
    /*
    println!("Path: {:?}", path);
    let mut proof_path: Vec<String> = Vec::new();
    for leaf in &path.clone()[1..path.len()]{
        //proof_path.push(leaf.clone());
        let leaf_parent = tree.find_leaf_parent(tree.clone().root.unwrap(), leaf.clone()).unwrap();
        let leaf_sibling = tree.find_leaf_sibling(leaf_parent.clone(), leaf.clone()).unwrap();
        //println!("Leaf: {}, sibling: {:?}", leaf, leaf_sibling.data);
        proof_path.push(leaf_sibling.data);
    };
    */
    let result = tree.merkle_path_in_order(tree.clone().root.unwrap().data, tree.clone().root.unwrap(), path);

    println!("Proof_path: {:?}", result);
}
*/