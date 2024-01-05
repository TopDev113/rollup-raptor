/*
This circuit takes as input the balances of two accounts, a new merkle hash (that's computed off-chain based on current state), a signed transacion object
and generates a proof for the validity of the transaction and its inclusion in the merkle tree for the given merkle root.

The new merkle hash is a public input to the circuit, and  the previous state (accounts, balances) are private inputs.

Public outputs of this circuit are the balances after state mutation.
*/

use dep::std;
mod hash;
mod merkle;
mod helpers;
mod secp256k1;

use hash::Transfer;
use merkle::Merkle;
use secp256k1::Signature;

use helpers::{u8_array_to_u64};
/* Batch prover - 10 transactions
    * Inputs:
        x: [[u8;32];10],
        y: [[u8;32];10],
        leaf: [[u8;32];10],
        leaf_preimage: [[u8;32];10]
        ...
*/

/* CLI design
    * client must generate Prover file
    * client may generate tempdir for proving and verification process
    * client batch processes transactions (set of n transactions) -> for testing, n = 10

*/

pub fn main(
    x: [u8;32],
    y: [u8;32],
    // sender = y,
    recipient: [u8;32],
    // hash of the tx
    leaf: [u8;32],
    // preimage of the tx hash
    leaf_preimage: [u8;32],
    // transaction signature
    signature: [u8;64],
    // transaction amount
    amount: [u8;8],
    // LoR hashing indicator
    position: [u8;4],
    // merkle path
    path: [[u8;32];4],
    // state root
    root: [u8;32],

    /* TBD
        * balances
    
    */
    balances: pub [u64;2]

    // output state (sender, recipient balances)
) -> pub [u64;2]{
    // prove merkle path
    let merkle = Merkle{
        leaf: leaf,
        path: path,
        position: position,
        root: root
    };
    assert(merkle.reconstruct() == root);
    // prove secp256k1 signature
    let signature = Signature{
        message: leaf_preimage,
        x: x,
        y: y,
        signature: signature
    };
    assert(signature.verify() == true);
    let transfer = Transfer{
        sender: y, 
        recipient: recipient, 
        amount: amount,
        hash: leaf
    };
    let preimage = transfer.Hash();
    assert(preimage == leaf);

    // very simple for now
    let balance_sender = balances[0] - u8_array_to_u64(amount);
    let balance_recipient = balances[1] + u8_array_to_u64(amount);
    [balance_sender,balance_recipient]
    
    // apply state transition
}

#[test]
fn test_main(){
    let state = main(
        // x
        [42, 20, 27, 7, 166, 238, 115, 118, 231, 70, 250, 155, 101, 211, 192, 140, 19, 27, 144, 177, 226, 5, 17, 160, 24, 56, 8, 156, 29, 165, 234, 121],
        // y
        [83, 7, 164, 2, 157, 234, 12, 147, 193, 122, 238, 77, 240, 96, 153, 248, 232, 84, 4, 37, 135, 204, 5, 238, 210, 29, 134, 226, 211, 89, 183, 124],
        // recipient
        [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        // leaf
        [184, 96, 186, 248, 151, 5, 50, 112, 190, 121, 152, 130, 62, 252, 38, 192, 227, 149, 129, 61, 160, 5, 142, 20, 202, 244, 21, 204, 178, 251, 155, 134],
        // leaf preimage
        [69, 205, 19, 157, 170, 214, 175, 23, 226, 45, 70, 231, 78, 125, 30, 215, 51, 189, 117, 41, 176, 197, 78, 145, 216, 51, 199, 225, 184, 194, 100, 233],
        // signature
        [143, 120, 218, 20, 203, 25, 70, 17, 179, 89, 25, 235, 159, 155, 75, 92, 201, 96, 125, 72, 29, 28, 37, 245, 213, 130, 204, 231, 129, 97, 147, 69, 57, 115, 176, 248, 226, 176, 243, 101, 93, 59, 210, 101, 160, 109, 194, 208, 250, 126, 166, 163, 101, 47, 72, 205, 18, 163, 34, 81, 170, 98, 136, 207],
        // amount
        [0, 0, 0, 0, 0, 0, 0, 10],
        // position
        [0,0,0,0],
        // path
        [[102, 104, 122, 173, 248, 98, 189, 119, 108, 143, 193, 139, 142, 159, 142, 32, 8, 151, 20, 133, 110, 226, 51, 179, 144, 42, 89, 29, 13, 95, 41, 37],
        [46, 235, 116, 166, 23, 127, 88, 141, 128, 192, 199, 82, 185, 149, 86, 144, 45, 223, 150, 130, 208, 185, 6, 245, 170, 42, 219, 175, 132, 102, 164, 233],
        [18, 35, 52, 154, 64, 210, 238, 16, 189, 27, 235, 181, 136, 158, 248, 1, 140, 139, 193, 51, 89, 237, 148, 179, 135, 129, 10, 249, 108, 110, 66, 104],
        [91, 130, 182, 149, 167, 172, 38, 104, 225, 136, 183, 95, 125, 79, 167, 159, 170, 80, 65, 23, 209, 253, 252, 190, 138, 70, 145, 92, 26, 138, 81, 145]],
        // root
        [175, 190, 72, 137, 14, 91, 195, 230, 208, 249, 38, 141, 52, 215, 126, 6, 252, 20, 33, 24, 205, 197, 166, 220, 130, 110, 24, 125, 197, 226, 21, 72],
        // balances (sender: 10, recipient: 0)
        [10,0]
    );
}
