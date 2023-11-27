use merkle_tree::{tornado::TornadoTree, helpers::{hashLeftRight, hash_bytes}};
use crate::transfer::Transfer_G1;
// A mock node with account state
#[derive(Clone)]
pub struct MockNode{
    pub tree: Option<TornadoTree>,
    // representing account balances.
    pub state: Vec<(Vec<u8>, u64)>
}
impl MockNode{
    pub fn init(mut self) -> MockNode{
        // default merkle tree for testing has a depth of 5.
        let mut tree = TornadoTree{
            zero_node: hash_bytes(vec![0;32]),
            zero_levels: Vec::new(),
            root_history: Vec::new(),
            filled: vec![vec![], vec![], vec![], vec![], vec![]],
            index: 0,
            depth: 5
        };
        tree.calculate_zero_levels();
        self.tree = Some(tree);
        self
    }
    // add new leaf to the simulated L2's merkle tree
    pub fn add_leaf(&mut self, transfer: Transfer_G1) -> Vec<(Vec<u8>, bool)>{
        let transfer_preimage: Vec<u8> = transfer.hash();
        // add transaction preimage to the tree
        self.tree.as_mut().unwrap().add_leaf(transfer_preimage)
    }
}