use crate::helpers::u8_array_to_u64;
// struct to prove and apply state transitions
struct StateEngine{
    // 2 accounts (sender, recipient)
    accounts: [[u8;32];2],
    // 2 balances (sender, recipient)
    balances: [u64;2]
}

impl StateEngine{
    
}