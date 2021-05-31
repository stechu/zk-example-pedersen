mod groth_api;
mod marlin;
mod pedersen;
mod r1cs;

use ark_crypto_primitives::commitment::pedersen::Randomness;
use ark_ed_on_bls12_381::*;
use ark_ff::UniformRand;
use groth_api::*;
//use marlin::*;
use pedersen::*;
use r1cs::*;
use ark_serialize::CanonicalSerialize;
use std::time::Instant;


fn main() {
    let start = Instant::now();
    let mut rng = rand::thread_rng();
    let len = 128;
    let pedersen_param = pedersen_setup(&[0u8; 32]);
    let input = vec![2u8; len];
    let open = Randomness::<JubJub>(Fr::rand(&mut rng));
    let commit = pedersen_commit(&input, &pedersen_param, &open);

    let circuit = PedersenComCircuit {
        param: pedersen_param.clone(),
        input,
        open,
        commit,
    };

    //sanity_check();
    let elapse = start.elapsed();
    let start2 = Instant::now();

    let zk_param = groth_param_gen(pedersen_param);
    
    let elapse2 = start2.elapsed();
    let start3 = Instant::now();
    // This is the part that we want to benchmark:
    let proof = groth_proof_gen(&zk_param, circuit, &[0u8; 32]);
    let elapse3 = start3.elapsed();
    println!("time to prepare comm: {:?}", elapse);
    println!("time to gen groth param: {:?}", elapse2);
    println!("time to gen proof: {:?}", elapse3);
    let mut zkparam_buf: Vec<u8> = vec! [];
    zk_param.serialize_uncompressed(&mut zkparam_buf).unwrap();
    println!("vk size: {}", zkparam_buf.len());
    //println!("vk: {:?}", zkparam_buf);

    let mut proof_buf: Vec<u8> = vec! [];
    proof.serialize_uncompressed(&mut proof_buf).unwrap();
    println!("proof size: {}", proof_buf.len());

    assert!(groth_verify(&zk_param, &proof, &commit));

}
