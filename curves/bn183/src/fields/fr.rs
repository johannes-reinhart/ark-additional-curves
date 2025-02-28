use ark_ff::fields::{Fp192, MontBackend, MontConfig};

#[derive(MontConfig)]
#[modulus = "6804759748846355405830582786011032970784946075266449409"]
#[generator = "22"]
pub struct FrConfig;
pub type Fr = Fp192<MontBackend<FrConfig, 3>>;
