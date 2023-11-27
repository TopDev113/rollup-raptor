use dep::std;
//std::hash::sha256(self.message)

struct Transfer{
    sender: [u8;32],
    recipient: [u8;32],
    amount: [u8;8],
    hash: [u8;32]
}

impl Transfer{
    fn Hash(self) -> [u8;32]{
        //let preimage = self.sender.as_slice();
        let mut current_preimage = self.sender.as_slice();
        for byte in self.recipient{
            current_preimage = current_preimage.push_back(byte);
        };
        for byte in self.amount{
            current_preimage = current_preimage.push_back(byte);
        };
        let preimage_sha256: [u8;32] = std::hash::sha256(current_preimage);
        preimage_sha256
    }
}
#[test]
fn test(){
    let transfer = Transfer{
        sender: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
        recipient: [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 
        amount: [0, 0, 0, 0, 0, 0, 0, 10],
        hash: [205, 46, 23, 231, 250, 174, 164, 51, 242, 170, 114, 211, 48, 56, 191, 177, 214, 40, 36, 97, 162, 90, 42, 155, 37, 167, 7, 33, 149, 179, 214, 178]
    };
    
    //let preimage = transfer.Hash();
    //let target = [102, 104, 122, 173, 248, 98, 189, 119, 108, 143, 193, 139, 142, 159, 142, 32, 8, 151, 20, 133, 110, 226, 51, 179, 144, 42, 89, 29, 13, 95, 41, 37];
    let target: [u8;32] = [205, 46, 23, 231, 250, 174, 164, 51, 242, 170, 114, 211, 48, 56, 191, 177, 214, 40, 36, 97, 162, 90, 42, 155, 37, 167, 7, 33, 149, 179, 214, 178];
    let preimage = transfer.Hash();
    assert(preimage == target);     
}
/*
Circuit inputs: Transfer_G1 { 
    sender: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
    recipient: [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 
    amount: [0, 0, 0, 0, 0, 0, 0, 10] }, 
[205, 46, 23, 231, 250, 174, 164, 51, 242, 170, 114, 211, 48, 56, 191, 177, 214, 40, 36, 97, 162, 90, 42, 155, 37, 167, 7, 33, 149, 179, 214, 178]
*/