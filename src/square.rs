#![allow(unused_imports)]
#![allow(unused_variables)]
extern crate bellman;
extern crate pairing;
extern crate rand;

use self::rand::{thread_rng};

use self::pairing::{
    Engine,
    Field,
    PrimeField
};

use self::pairing::bls12_381::{
    Bls12,
    Fr
};

use self::bellman::{
    Circuit,
    ConstraintSystem,
    SynthesisError
};

use self::bellman::groth16::{
    Proof,
    generate_random_parameters,
    prepare_verifying_key,
    create_random_proof,
    verify_proof,
};

// proving that I know a such that a * a = b
pub struct Square<E: Engine> {
    pub a: Option<E::Fr>,
    pub b: Option<E::Fr>
}

impl <E: Engine> Circuit<E> for Square<E> {
    fn synthesize<CS: ConstraintSystem<E>>(
        self, 
        cs: &mut CS
    ) -> Result<(), SynthesisError>
    {
        
        let a = cs.alloc(|| "a", || {
            self.a.ok_or(SynthesisError::AssignmentMissing)
        })?;
        
        let b = cs.alloc_input(|| "b", || {
            self.b.ok_or(SynthesisError::AssignmentMissing)
        })?;

        cs.enforce(
            || "mult",
            |lc| lc + a,
            |lc| lc + a,
            |lc| lc + b
        );
        
        Ok(())
    }
}

#[test]
fn test_square(){
    let rng = &mut thread_rng();
    
    println!("generating setup...");
    
    let params = {
        let b = Square::<Bls12> {
            a: None,
            b: None
        };

        generate_random_parameters(b, rng).unwrap()
    };
    
    let pvk = prepare_verifying_key(&params.vk);

    println!("calculating proofs...");
    
    let public_input = Fr::from_str("9");
    
    let c = Square::<Bls12> {
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




