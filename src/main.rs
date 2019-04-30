#![allow(unused_imports)]
#![allow(unused_variables)]
extern crate bellman;
extern crate pairing;
extern crate rand;

mod square;

use pairing::{Engine, Field, PrimeField};

use pairing::bls12_381::{
    Bls12,
    Fr,
};

use bellman::groth16::{
    Proof,
    generate_random_parameters,
    prepare_verifying_key,
    create_random_proof,
    verify_proof,
};

use rand::thread_rng;

fn main() {
    let rng = &mut thread_rng();
    
    println!("generating setup...");
    
    let params = {
        let b = square::Square::<Bls12> {
            a: None,
            b: None
        };

        generate_random_parameters(b, rng).unwrap()
    };
    
    let pvk = prepare_verifying_key(&params.vk);

    println!("calculating proofs...");
    
    let public_input = Fr::from_str("9");
    
    let c = square::Square::<Bls12> {
        a: Fr::from_str("3"),
        b: public_input
    };
    
    let proof = create_random_proof(c, &params, rng).unwrap();
    
    println!("vering proof...");

    assert!(verify_proof(
        &pvk,
        &proof,
        &[public_input.unwrap()]
    ).unwrap());

    println!("proof is valid");
}