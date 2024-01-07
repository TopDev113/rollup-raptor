#![no_main]

mod error;
use error::NoirError;
use casper_contract::contract_api::{noir::noir_verifier, runtime, storage};
use casper_types::{CLType, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Key, contracts::NamedKeys};
use serde_json_wasm;
use serde::{Serialize, Deserialize};

// tbd: import type from contract api?
#[derive(Default, Clone, Serialize, Deserialize)]
pub struct NoirProof {
    pub verifier: String,
    pub proof: String,
}

#[cfg(not(feature = "casper-circom"))]
#[no_mangle]
pub extern "C" fn call_verifier(){
    let proof_payload: &str = include_str!("../rollup.proof");
    let verifier_payload: &str = include_str!("../Verifier.toml");
    let noir_proof: NoirProof = NoirProof{
        verifier: verifier_payload.to_string(),
        proof: proof_payload.to_string()
    };
    if noir_verifier(&serde_json_wasm::to_vec(&noir_proof).unwrap()) != [1]{
        runtime::revert(NoirError::InvalidProof);
    }
}

#[no_mangle]
pub extern "C" fn call(){
    // entry point definitions
    let mut entry_points: EntryPoints = EntryPoints::new();
    let call_verifier: EntryPoint = EntryPoint::new(
        "call_verifier",
        vec![],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract
    );
    entry_points.add_entry_point(call_verifier);
    // named keys definitions
    let mut named_keys: std::collections::BTreeMap<String, Key> = NamedKeys::new();
    // contract package
    let package_key_name: String = "noir_rollup_prover".to_string();
    let (contract_hash, _) = storage::new_contract(
        entry_points,
        Some(named_keys),
        Some(package_key_name),
        Some("noir_rollup_prover".to_string()),
    );
    let contract_hash_key: Key = Key::from(contract_hash);
    // store contract package key
    runtime::put_key("noir_rollup_prover", contract_hash_key);
}