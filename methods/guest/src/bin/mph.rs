#![no_main]
#![no_std]

extern crate alloc;

use alloc::vec::Vec;
use risc0_zkvm_guest::env;
use bitcoin_hashes::{sha256, Hash};

risc0_zkvm_guest::entry!(main);

pub fn main() {
    // Load the preimage
    let num_preimages: usize = env::read();

    
    let mut res:Vec<u8> = Vec::new();

    for n in 0..num_preimages {
        let preimage: [u8; 32] = env::read();
        
        // hash the preimage
        let payment_hash = sha256::Hash::hash(&preimage[..]).into_inner();

        // commit the payment hash
        env::commit(&payment_hash);

        res.extend(preimage.iter().cloned())
    }


    // hash the concatinated preimages to a single preimage to the cost of revealing the preimage
    let final_payment_preimage = sha256::Hash::hash(res.as_slice()).into_inner();
    
    // hash the final preimage to the final payment hash
    let final_payment_hash = sha256::Hash::hash(&final_payment_preimage).into_inner();
    
    env::commit(&final_payment_hash);
}
