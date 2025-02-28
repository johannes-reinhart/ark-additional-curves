use ark_ff::fields::{Fp192, MontBackend, MontConfig};

#[derive(MontConfig)]
#[modulus = "6804759748846355405830582788619626413398422602255236423"]
#[generator = "6"]
pub struct FqConfig;
pub type Fq = Fp192<MontBackend<FqConfig, 3>>;
