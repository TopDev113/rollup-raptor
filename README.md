![raptor-retrievers](https://github.com/jonas089/noir-cli-rollup/blob/master/resources/icon.webp)
# What is

This application is under development and strives to be a client side zk rollup service that will be able to generate proofs for state transitions in the
shape of on-chain transaction data. Consensus and sequalization are less of a priority than the generation of sensible proofs and the overall structure of the project.

# Special dependencies

This client application depends on my library `ecdsa-circuit-input-lib`. The library is used to generate byte inputs for the noir circuit to then generate proof for secp256k1 signature validity.

It also utilizes a special merkle tree that's essentially a Rust implementation of a blockchain privacy transaction service's merkle tree.

The merkle tree resides in my `noir-research` crate/project. But was also added to this crate for sake of simplicity [here](https://github.com/jonas089/noir-cli-rollup/blob/master/merkle-tree/src/tornado.rs)

# Circuits

The rollup circuit is a composition of 3 sub-circuits, one for each `merkle proofs`, `signatures`, `transfer hash` and `state transitions`.

## Inputs and Outputs

`Base set` of inputs for each circuit:

| merkle proof | signature | transfer hash | state transition  |
|--------------|-----------|---------------|-------------------|
| merkle path  | tx hash   | sender        | sender balance    |
| merkle root  | sender x  | recipient     | recipient balance |
|              | sender y  | amount        | transfer amount   |
|              |           |               | sender y          |
|              |           |               | recipient y       |


Additional inputs:

- `nonce`
- `timestamp`

For the first stage of the `POC`, only the `base set` of inputs will be considered.
