use dep::std;
// struct to prove message signatures
pub struct Signature{
    pub message: [u8;32],
    pub x: [u8;32],
    pub y: [u8;32],
    pub signature: [u8;64]
}

impl Signature{
    pub fn verify(self) -> bool{
        let message_hash = std::hash::sha256(self.message);
        std::ecdsa_secp256k1::verify_signature(self.x, self.y, self.signature, message_hash)
    }
}