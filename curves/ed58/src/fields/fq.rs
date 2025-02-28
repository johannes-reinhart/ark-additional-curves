use ark_ff::fields::{Fp64, MontBackend, MontConfig};

#[derive(MontConfig)]
#[modulus = "844025809322115073"]
#[generator = "10"]
pub struct FqConfig;
pub type Fq = Fp64<MontBackend<FqConfig, 1>>;