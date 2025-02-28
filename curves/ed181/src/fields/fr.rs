use ark_ff::fields::{Fp192, MontBackend, MontConfig};

#[derive(MontConfig)]
#[modulus = "1552511030102430251236801561344621993261920897571225601"]
#[generator = "19"]
pub struct FrConfig;
pub type Fr = Fp192<MontBackend<FrConfig, 3>>;