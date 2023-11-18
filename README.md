# Client side of a potential noir zero knowledge rollup application that proves state transitions in a blockchain environment
State transitions limited to transfers.

## Concept / Background
For a zero knowledge rollup to be sound, the client will query state from the L1 and add new transactions to a merkle tree.

For each transaction (or batches of transactions) in the merkle tree, the circuit will generate proofs for 3 statements.

### Statement 1: Signature integrity
Every transfer in the merkle tree must be associated with a valid Signature

### Statement 2: Merkle proof validity
The merkle path is recalculated in the circuit and a merkle proof is generated.

### Statement 3: State transition integrity
The integrity of all state transitions is ensured by conditionally applying those state transitions inside the circuit

### Statement 4: Message hash / tx hash

