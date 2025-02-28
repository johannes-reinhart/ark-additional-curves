use ark_ff::fields::{Fp64, MontBackend, MontConfig};

#[derive(MontConfig)]
#[modulus = "4941944131663429633"]
#[generator = "5"]
pub struct FqConfig;
pub type Fq = Fp64<MontBackend<FqConfig, 1>>;