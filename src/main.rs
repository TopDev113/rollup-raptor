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
    let message: String = "SOME_SERIALIZED_STRUCT".to_string();
    // initialize the input generator
    let input_generator = InputGenerator{
        sk: deserialized_signing_key,
        message: message
    };
    let inputs = input_generator.generate();
    println!("Circuit Inputs: {:?}", inputs);
}