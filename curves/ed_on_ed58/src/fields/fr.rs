use ark_ff::fields::{Fp64, MontBackend, MontConfig};

#[derive(MontConfig)]
#[modulus = "26375806633482721"]
#[generator = "22"]
pub struct FrConfig;
pub type Fr = Fp64<MontBackend<FrConfig, 1>>;
