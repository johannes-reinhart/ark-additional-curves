use ark_ff::fields::{Fp64, MontBackend, MontConfig};

#[derive(MontConfig)]
#[modulus = "211006452744585217"]
#[generator = "7"]
pub struct FrConfig;
pub type Fr = Fp64<MontBackend<FrConfig, 1>>;