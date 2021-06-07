mod blake;

use crate::blake::{PRFCircuit, groth_param_gen, groth_proof_gen, groth_verify};
use ark_crypto_primitives::{
	prf::{Blake2s, PRF},
};

use std::time::Instant;


fn main() {
    let start = Instant::now();
    //  fix a secret key
	let sk = [4u8; 32];

	// pk = PRF(sk, 0); which is also the address
	let pk = <Blake2s as PRF>::evaluate(&sk, &[0u8; 32]).unwrap();

    // build the circuit
    let circuit = PRFCircuit {
        seed: sk,
        input: [0u8; 32],
        output: pk,
    };

    let elapse = start.elapsed();
    let start2 = Instant::now();

    // generate a zkp parameters
    let zk_param = groth_param_gen();

    let elapse2 = start2.elapsed();
    let start3 = Instant::now();
    
    let proof = groth_proof_gen(&zk_param, circuit, &[0u8; 32]);

    let elapse3 = start3.elapsed();
    println!("time to prepare comm: {:?}", elapse);
    println!("time to gen groth param: {:?}", elapse2);
    println!("time to gen proof: {:?}", elapse3);
    
    assert!(groth_verify(&zk_param, &proof, &pk));

}