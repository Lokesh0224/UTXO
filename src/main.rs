use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Utxo {//unspent txn output
    id: String,
    owner: String,
    amount: u64,
}

#[derive(Debug)]
struct TxInput {//the amt that's going to consumed
    utxo_id: String,
}

#[derive(Debug)]
struct TxOutput {//amt after consumed
    owner: String,
    amount: u64,
}

#[derive(Debug)]
struct Transaction {
    inputs: Vec<TxInput>,
    outputs: Vec<TxOutput>,
}

#[derive(Debug)]
struct Ledger {
    utxos: HashMap<String, Utxo>,
}

fn main() {
    println!("UTXO Ledger starting...");
}
