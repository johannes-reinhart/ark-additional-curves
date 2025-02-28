use ark_ff::fields::{Fp128, MontBackend, MontConfig};

#[derive(MontConfig)]
#[modulus = "141455844224742490147094691841"]
#[generator = "13"]
pub struct FrConfig;
pub type Fr = Fp128<MontBackend<FrConfig, 2>>;