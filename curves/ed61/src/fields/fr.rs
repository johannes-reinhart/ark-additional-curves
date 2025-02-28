use ark_ff::fields::{Fp64, MontBackend, MontConfig};

#[derive(MontConfig)]
#[modulus = "1235486033917771777"]
#[generator = "13"]
pub struct FrConfig;
pub type Fr = Fp64<MontBackend<FrConfig, 1>>;