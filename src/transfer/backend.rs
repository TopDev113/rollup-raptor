use merkle_tree::{tornado::TornadoTree, helpers::{hashLeftRight, hash_bytes}};
use crate::transfer::Transfer_G1;
// A mock node with account state
pub struct MockNode{
    tree: Option<TornadoTree>,
    // representing account balances.
    state: Vec<(Vec<u8>, u64)>
}
impl MockNode{
    fn init(mut self){
        // default merkle tree for testing has a depth of 5.
        let mut tree = TornadoTree{
            zero_node: hash_bytes(vec![0;32]),
            zero_levels: Vec::new(),
            root_history: Vec::new(),
            filled: vec![vec![], vec![], vec![], vec![], vec![]],
            index: 0,
            depth: 5
        };
        self.tree = Some(tree);
    }
    // add new leaf to the simulated L2's merkle tree
    fn add_leaf(self, transfer: Transfer_G1) -> Vec<(Vec<u8>, bool)>{
        let transfer_perimage: Vec<u8> = transfer.hash();
        // add transaction preimage to the tree
        self.tree.unwrap().add_leaf(transfer_perimage)
    }
}