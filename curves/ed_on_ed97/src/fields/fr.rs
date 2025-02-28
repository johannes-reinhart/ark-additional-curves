use ark_ff::fields::{Fp128, MontBackend, MontConfig};

#[derive(MontConfig)]
#[modulus = "17681980528092752387406989869"]
#[generator = "6"]
pub struct FrConfig;
pub type Fr = Fp128<MontBackend<FrConfig, 2>>;
