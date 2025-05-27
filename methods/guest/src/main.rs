#![no_main]

use risc0_zkvm::guest::env;


risc0_zkvm::guest::entry!(main);
fn main() {
    
    // read the input
    let input: String = env::read();

    // TODO: do something with the input

    // write public output to the journal
    env::commit(&input);
}
