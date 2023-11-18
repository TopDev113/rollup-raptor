use sha2::{Digest, Sha256};
use serde_json;
pub mod backend;
// Generation 1 Transfer struct
pub struct Transfer_G1{
    pub sender: Vec<u8>,
    pub recipient: Vec<u8>,
    pub amount: Vec<u8>,

    pub signature: Vec<u8>
}

impl Transfer_G1{
    // experimental feature, could cause trouble in circuit
    pub fn hash(&self) -> Vec<u8>{
        let mut message = self.sender.clone();
        message.append(&mut self.recipient.clone());
        message.append(&mut self.amount.clone());
        message.append(&mut self.signature.clone());
        let mut sha_256 = Sha256::new();
        sha_256.update(message);
        sha_256.finalize().to_vec()
    }
}

// Generation 2 Transfer struct
struct Transfer_G2{
    sender: Vec<u8>,
    recipient: Vec<u8>,
    amount: Vec<u8>,
    nonce: Vec<u8>,
    timestamp: Vec<u8>,

    signature: Vec<u8>
}