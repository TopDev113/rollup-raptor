use crate::helpers::u8_array_to_u64;
// struct to prove and apply state transitions
struct StateEngine{
    // 2 accounts (sender, recipient)
    accounts: [[u8;32];2],
    // 2 balances (sender, recipient)
    balances: [u64;2],
    // amount
    amount: [u8;8]
}

impl StateEngine{
    /* state transition
        * find index of sender and recipient
        * reduce balance[s] and increase balance[r] by amount if sufficient funds
        * add new state to outputs
        *
        *
        *
    */
}