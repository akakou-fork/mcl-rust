// env RUSTFLAGS="-L <mcl>/lib" cargo run
use mcl_rust::*;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Sample {
    fp1: Fp,
    fp2: Fp,
}

#[allow(non_snake_case)]
fn main() {
    init(CurveType::BN254);

    let mut fp1 = Fp::zero();
    fp1.set_int(123456);

    let mut fp2 = Fp::zero();
    fp2.set_int(7890);

    let sample = Sample {
        fp1: fp1.clone(),
        fp2: fp2.clone(),
    };
    let s = serde_json::to_string(&sample).unwrap();

    println!("{}", s);

    let sample2: Sample = serde_json::from_str(&s).unwrap();

    print!("{}", sample2.fp1 == fp1);
    print!("{}", sample2.fp2 == fp2);
}
