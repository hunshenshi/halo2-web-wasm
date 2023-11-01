mod generator;
mod circuits;

use wasm_bindgen::prelude::*;

use halo2_curves::bn256::Fr;
use halo2_proofs::circuit::Value;
use snark_verifier::loader::evm::encode_calldata;

use serde_json::Value as JsonValue;

use crate::{circuits::simple::SimpleCircuit, generator::{gen_srs, gen_pk, gen_proof}};

#[wasm_bindgen]
pub fn generator_proof(input: String) -> i32 {
    let v: JsonValue = serde_json::from_str(&input).unwrap();
    let private_a = Fr::from(v["private_a"].as_str().unwrap().parse::<u64>().unwrap());
    let private_b = Fr::from(v["private_b"].as_str().unwrap().parse::<u64>().unwrap());
    let constant = Fr::from(3);

    // 1. gen params or use w3b params
    let params = gen_srs(4);
    let empty_circuit = SimpleCircuit {
        constant,
        a: Value::unknown(),
        b: Value::unknown(),
    };

    // 2. gen pk
    let pk = gen_pk(&params, &empty_circuit);

    let c = constant * private_a.square() * private_b.square();
    // println!("{:?}", c);
    let circuit = SimpleCircuit {
        constant,
        a: Value::known(private_a),
        b: Value::known(private_b),
    };
    let instances = vec![vec![c]];

    // 3. gen proof
    let proof = gen_proof(&params, &pk, circuit.clone(), &instances);

    // 4. gen proof calldata
    let calldata = encode_calldata(&instances, &proof);
    println!("{:?}", hex::encode(&calldata));
    
    0
}
