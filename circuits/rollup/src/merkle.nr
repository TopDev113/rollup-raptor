// Struct to prove merkle paths
struct Merkle{
    leaf: [u8;32],
    path: [[u8;32];4],
    position: [u8;4],
    root: [u8;32]
}
impl Merkle{
    fn try_prove(self){
        let mut current_hash: [u8;32] = [0;32];
        if self.position[0] == 0{
            current_hash = std::hash::sha256(combine(self.leaf, self.path[0]));
        }
        else{
            current_hash = std::hash::sha256(combine(self.path[0], self.leaf));
        };
        for i in 1..self.path.len(){
            if self.position[i] == 0{
                current_hash = std::hash::sha256(combine(current_hash, self.path[i]));
            }
            else{
                current_hash = std::hash::sha256(combine(self.path[i], current_hash));
            }
        };
        // assertion will trigger if path is invalid
        assert(current_hash == self.root);
    }
}