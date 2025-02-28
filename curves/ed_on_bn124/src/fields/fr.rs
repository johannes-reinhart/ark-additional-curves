use ark_ff::fields::{Fp128, MontBackend, MontConfig};

#[derive(MontConfig)]
#[modulus = "2125016665599104007263567884627882177"]
#[generator = "5"]
pub struct FrConfig;
pub type Fr = Fp128<MontBackend<FrConfig, 2>>;
