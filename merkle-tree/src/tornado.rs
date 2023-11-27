use std::thread::current;

use crate::helpers::{hashLeftRight, hash_bytes};

#[derive(Debug, Clone, PartialEq)]
pub struct TornadoTree{
    pub zero_node: Vec<u8>,
    pub zero_levels: Vec<Vec<u8>>,
    pub root_history: Vec<Vec<u8>>,
    pub filled: Vec<Vec<u8>>,
    pub index: usize,
    pub depth: usize
}

impl TornadoTree{
    pub fn calculate_zero_levels(&mut self){
        let mut zero_levels: Vec<Vec<u8>> = vec![self.zero_node.clone()];
        for i in 0..self.depth - 1{
            zero_levels.push(hashLeftRight(zero_levels[zero_levels.len()-1].clone(), zero_levels[zero_levels.len()-1].clone()));
        };
        self.zero_levels = zero_levels;
    }
    pub fn add_leaf(&mut self, leaf: Vec<u8>) -> Vec<(Vec<u8>, bool)> {
        let mut proof_path: Vec<(Vec<u8>, bool)> = vec![(leaf.clone(), false)];
        let mut current_index = self.index;
        let mut current_hash: Vec<u8> = leaf.clone();

        for i in 0..self.depth {
            if current_index % 2 == 0 {
                // If index is even, left is the new leaf preimage, right is the zero node at this level
                self.filled[i] = current_hash.clone(); // push the leaf or its hash
                current_hash = hashLeftRight(current_hash, self.zero_levels[i].clone());
                if i < self.depth - 1{
                    proof_path.push((self.zero_levels[i].clone(), true));
                }
            } else {
                // If index is odd, left is the previous leaf or its hash at this level, right is the new leaf or its hash
                let left = self.filled[i].clone();
                current_hash = hashLeftRight(left.clone(), current_hash.clone());
                proof_path.push((left, false));
            }
            current_index /= 2;
        }
        self.index += 1;
        proof_path
    }
}

#[test]
fn test_tornado(){
    let mut tree = TornadoTree{
        zero_node: hash_bytes(vec![0;32]),
        zero_levels: Vec::new(),
        root_history: Vec::new(),
        filled: vec![vec![], vec![], vec![], vec![], vec![]],
        index: 0,
        depth: 5
    };
    tree.calculate_zero_levels();
    let mut proof_path: Vec<(Vec<u8>, bool)> = tree.add_leaf(vec![242, 69, 81, 38, 252, 95, 197, 129, 177, 105, 42, 137, 129, 73, 125, 148, 130, 204, 83, 82, 126, 104, 106, 71, 156, 96, 55, 233, 132, 103, 128, 11]);
    println!("Proof path: {:?}", proof_path);
    let merkle_root = &tree.filled.pop().unwrap();
    // true -> right, false -> left
    println!("Merkle root: {:?}", &merkle_root);

    let mut current_hash = proof_path[0].clone().0;
    for i in 1..proof_path.len(){
        if &proof_path[i].1 == &true{
            current_hash = hashLeftRight(current_hash, proof_path[i].clone().0);
        }
        else{
            current_hash = hashLeftRight(proof_path[i].clone().0, current_hash);
        }
    }

    assert_eq!(&current_hash, merkle_root);
}

/*
- If the index is even, the sibling is the next node (index + 1) at the same level.
- If the index is odd, the sibling is the previous node (index - 1).
*/

/* Information needed for merkle proof
- index of leaf
- sibling at each level

- if leaf index is even, check
*/