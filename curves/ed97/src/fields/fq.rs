use ark_ff::fields::{Fp128, MontBackend, MontConfig};

#[derive(MontConfig)]
#[modulus = "565823376898968604518330826753"]
#[generator = "5"]
pub struct FqConfig;
pub type Fq = Fp128<MontBackend<FqConfig, 2>>;