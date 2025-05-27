#![no_main]

use risc0_zkvm::guest::env;
use blockchain::blockchain::{Block, Transaction, Blockchain};
use serde::{Serialize, Deserialize};



risc0_zkvm::guest::entry!(main);
fn main() {
    
    // read the input
    let blockchain_state: String = env::read();

    let deserialize: Blockchain = serde_json::from_str(&blockchain_state).unwrap();

    let result: bool = deserialize.is_valid();

    // write public output to the receipt journal
    env::commit(&result);
}
