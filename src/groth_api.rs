use crate::pedersen::*;
use crate::r1cs::*;
use ark_bls12_381::Bls12_381;
use ark_crypto_primitives::commitment::pedersen::Randomness;
use ark_crypto_primitives::SNARK;
use ark_ed_on_bls12_381::Fq;
use ark_ed_on_bls12_381::Fr;
use ark_ff::UniformRand;
use ark_groth16::*;
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;

/// generate CRS given parameter of pedersen hash
#[allow(dead_code)]
pub fn groth_param_gen(
    pedersen_param: PedersenParam,
) -> <Groth16<Bls12_381> as SNARK<Fq>>::ProvingKey {
    let mut rng = rand::thread_rng();
    let len = 128;
    let input = vec![0u8; len];
    let open = Randomness::<JubJub>(Fr::rand(&mut rng));
    let commit = pedersen_commit(&input, &pedersen_param, &open);

    let circuit = PedersenComCircuit {
        param: pedersen_param,
        input,
        open,
        commit,
    };
    generate_random_parameters::<Bls12_381, _, _>(circuit, &mut rng).unwrap()
}

#[allow(dead_code)]
pub fn groth_proof_gen(
    param: &<Groth16<Bls12_381> as SNARK<Fq>>::ProvingKey,
    circuit: PedersenComCircuit,
    seed: &[u8; 32],
) -> <Groth16<Bls12_381> as SNARK<Fq>>::Proof {
    let mut rng = ChaCha20Rng::from_seed(*seed);
    create_random_proof(circuit, &param, &mut rng).unwrap()
}

#[allow(dead_code)]
pub fn groth_verify(
    param: &<Groth16<Bls12_381> as SNARK<Fq>>::ProvingKey,
    proof: &<Groth16<Bls12_381> as SNARK<Fq>>::Proof,
    commit: &PedersenCommitment,
) -> bool {
    let pvk = prepare_verifying_key(&param.vk);
    let inputs = [commit.x, commit.y];
    verify_proof(&pvk, &proof, &inputs[..]).unwrap()
}
