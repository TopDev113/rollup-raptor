use transfer::Transfer_G1;

use crate::helpers::u64_to_u8_array;

pub mod helpers;
pub mod transfer;
fn main() {
    todo!("Implement client service")
    /*
        * Instantiate a transfer backend with state (e.g. non-empty set of balances)
        * generate a valid transfer with signature using the ecdsa-lib
        * add the transfer preimage to the merkle tree and record the merkle proof
        * output all inputs to the circuit and run the prover

        => initially the prover will be run manually, later this client should make
        use of the nargo binary (nix env).
    */
}

#[test]
fn u64(){
    use helpers::u64_to_u8_array;
    let x: u64 = 10031;
    let a = u64_to_u8_array(x);
    println!("u8 array: {:?}", a);
}
#[test]
fn new_key(){
    use ecdsa_circuit_input_lib::{core, keys, db};
    use std::path::PathBuf;
    let key_manager = keys::ecdsa::EcdsaKeyManager{
        slice: vec![]
    };
    let key_serialized = key_manager.new();
    let store_manager = db::StoreManager{
        path: PathBuf::from("./keys.db")
    };
    store_manager.init()
        .expect("[Error] Failed to initialize keystore");
    // store serialized key
    store_manager.insert_key(
        "SOME_KEY_UID_0".to_string(), 
        key_serialized)
            .expect("[Error] Failed to store key!");
}

#[test]
fn generate_signature_circuit_inputs(){
    use std::path::PathBuf;
    use ecdsa_circuit_input_lib::db::StoreManager;
    use ecdsa_circuit_input_lib::keys::ecdsa::EcdsaKeyManager;
    use ecdsa_circuit_input_lib::core::signatures::InputGenerator;
    use k256::{
        ecdsa::{SigningKey}, FieldBytes
    };
    use helpers::u64_to_u8_array;
    // initialize keystore
    let store_manager: StoreManager = StoreManager{
        path: PathBuf::from("./keys.db")
    };
    // get key
    let key: ecdsa_circuit_input_lib::db::Response = store_manager
        .get_key_by_uid("SOME_KEY_UID_0".to_string())
        .expect("[Error] Failed to get the key!");
    // deserialize SigningKey from Response object
    let key_slice: Vec<u8> = key.deserialize();
    let key_manager: EcdsaKeyManager = EcdsaKeyManager{
        slice: key_slice
    };
    // signing key ready for use with input generator
    let deserialized_signing_key = key_manager.deserialize();
    /* 
        * define a message that is to be signed
        * a message can be a serialized struct
        * an example for a message struct would be a blockchain transfer/transaction/deploy
        #[derive(Serialize, Deserialize)]
        struct Transfer{
            sender: 
            recipient:
            amount:
            signature:
            nonce:
            timestamp:
            ...
        }
    */
    // for this test, a placeholder message will be used
    // let message: Vec<u8> = vec![0;32];
    let transfer = Transfer_G1{
        sender: vec![0;32],
        recipient: vec![1;32],
        // convert amount from u64 to Vec<u8>
        amount: u64_to_u8_array(10 as u64).to_vec()
    };
    let message: Vec<u8> = transfer.hash();
    // initialize the input generator
    let input_generator = InputGenerator{
        sk: deserialized_signing_key,
        message: message
    };
    let inputs = input_generator.generate();
    println!("Circuit Inputs: {:?}", inputs);
}

#[test]
fn generate_merkle_circuit_inputs(){
    use transfer::backend::MockNode;
    use transfer::Transfer_G1;
    let mock_backend: MockNode = MockNode{
        tree: None,
        state: Vec::new()
    };
    let mock_backend: MockNode = mock_backend.init();
    /* 
        create transfer message
        hash transfer message, 
        sign transfer hash,
        add leaf
    */
    use helpers::u64_to_u8_array;
    let transfer: Transfer_G1 = Transfer_G1{
        sender: vec![0;32],
        recipient: vec![1;32],
        // convert amount from u64 to Vec<u8>
        amount: u64_to_u8_array(10 as u64).to_vec()
    };
    // add the leaf to the merkle tree
    let inputs: Vec<(Vec<u8>, bool)> = mock_backend.add_leaf(transfer);
    println!("Circuit Inputs: {:?}", inputs);
}

#[test]
fn generate_transfer_hash_circuit_inputs(){
    use transfer::Transfer_G1;
    let transfer: Transfer_G1 = Transfer_G1 { 
        sender: vec![0;32], 
        recipient: vec![1;32], 
        amount: u64_to_u8_array(10 as u64).to_vec() 
    };
    let transfer_hash = transfer.hash();
    println!("Circuit inputs: {:?}, {:?}", transfer, transfer_hash);
    // todo! for verifier
    /*
        sender_bytes.append(recipient_bytes).append(amount_bytes) => use u8_to_u64 for amount
        => compare transfer_hash & generate proof    
    */

}