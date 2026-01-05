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

impl Ledger{//Create a new and empty ledger
    fn new() -> Self{//constructor
        Self{
            utxos: HashMap::new(),
        }
    }

    fn apply_tx(&mut self, tx: Transaction) -> Result<(), String> {
        let mut input_sum = 0;

        for input in &tx.inputs {
            let utxo = self
                .utxos
                .get(&input.utxo_id)
                .ok_or_else(|| format!("UTXO {} does not exist (double spend?)", input.utxo_id))?;

            input_sum += utxo.amount;
        }

        // 2. Sum all outputs
        let output_sum: u64 = tx.outputs.iter().map(|o| o.amount).sum();

        // 3. Enforce conservation of value
        if input_sum != output_sum {
            return Err(format!(
                "Input sum ({}) does not match output sum ({})",
                input_sum, output_sum
            ));
        }

        // 4. Remove spent UTXOs
        for input in tx.inputs {
            self.utxos.remove(&input.utxo_id);
        }

        // 5. Add new UTXOs
        for (index, output) in tx.outputs.into_iter().enumerate() {
            let utxo_id = format!("utxo_{}", self.utxos.len() + index + 1);

            let new_utxo = Utxo {
                id: utxo_id.clone(),
                owner: output.owner,
                amount: output.amount,
            };

            self.utxos.insert(utxo_id, new_utxo);
        }

        Ok(())
    }
}

fn main() {
    let mut ledger = Ledger::new();

    ledger.utxos.insert( //<String, Utxo>
        "genesis_1".to_string(),
        Utxo { 
            id: "genesis_1".to_string(), 
            owner: "Alice".to_string(), 
            amount: 50, 
        },
    );
    println!("Initial ledger: \n{:#?}", ledger);

    //Alice sends 30 to Bob, keeps 20 as change
    let tx = Transaction {
        inputs: vec![TxInput{
            utxo_id: "genesis_1".to_string(),
        }],
        outputs: vec![
            TxOutput{
                owner: "Bob".to_string(), 
                amount: 30,
            }, 
            TxOutput{
                owner: "Alice".to_string(), 
                amount: 20,
            },

        ],
    };

    match ledger.apply_tx(tx) {
        Ok(_) => println!("Transaction applied successfully."),
        Err(e) => println!("Transaction failed: {}", e),
    }

    println!("Ledger after transaction:\n{:#?}", ledger);
    
}


