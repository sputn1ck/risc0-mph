#![feature(array_chunks)]

use methods::{MPH_ID, MPH_PATH};
use risc0_zkvm::host::Prover;
use risc0_zkvm::serde::{to_vec};
use chrono::Utc;
use rand::{Rng};
use bitcoin_hashes::{sha256, Hash};

fn main() {

    let start_time = Utc::now().time();

    // First, we make the prover, loading the 'mph' method
    let mut prover = Prover::new(&std::fs::read(MPH_PATH).unwrap(), MPH_ID).unwrap();
    
    // Set the number of preimage we want to create
    let num_preimages: usize = 3;

    // Add the number of preimages to the prover
    prover.add_input(to_vec(&num_preimages).unwrap().as_slice()).unwrap();


    let mut preimages:Vec<u8> = Vec::new();

    for n in 0..num_preimages {
         // Pick a random preimage
        let random_bytes = rand::thread_rng().gen::<[u8; 32]>();
        println!("preimage {}: {}", n, hex::encode(random_bytes));

        // Next we add the preimage to the prover
        prover.add_input(to_vec(&random_bytes).unwrap().as_slice()).unwrap();

        preimages.extend(random_bytes.iter().cloned())
    }
    
   

    // Run prover & generate receipt
    let receipt = prover.run().unwrap();

    // Extract journal of receipt
    let payment_hashes = receipt.get_journal_vec().unwrap();

    let mut v2 : Vec<Vec<u8>> = vec![];
    for i in 0..num_preimages {
        v2.push(payment_hashes[i*32..(i+1)*32].into_iter().map(|x| *x as u8).collect());

        // print the individual payment hashes
        println!("payment hash {}: {}",i ,hex::encode(v2[i].as_slice()));
    }


    // hash the concatenated preimages to the final preimage
    let final_payment_preimage = sha256::Hash::hash(preimages.as_slice()).into_inner();
    println!("final preimage: {}",hex::encode(final_payment_preimage));

    // print the final payment hash
    v2.push(payment_hashes[(num_preimages)*32..(num_preimages+1)*32].into_iter().map(|x| *x as u8).collect());
    println!("final payment hash: {}",hex::encode(v2[num_preimages].as_slice()));
        
    // Here is where one would send 'receipt' over the network...

    // print the si
    let prove_size = receipt.get_seal().unwrap().len()*4 + payment_hashes.len();
    println!("size of prove: {:?} bytes", prove_size);

    // Verify receipt, panic if it's wrong  
    receipt.verify(MPH_ID).unwrap();

    let end_time = Utc::now().time();
    let diff = end_time - start_time;
    println!("Total time taken to run is {}ms", diff.num_milliseconds());
}
