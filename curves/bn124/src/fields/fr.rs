use ark_ff::fields::{Fp128, MontBackend, MontConfig};

#[derive(MontConfig)]
#[modulus = "17000133324792832058895897937997463553"]
#[generator = "7"]
pub struct FrConfig;
pub type Fr = Fp128<MontBackend<FrConfig, 2>>;
