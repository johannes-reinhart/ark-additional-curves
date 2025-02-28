use ark_ff::fields::{Fp128, MontBackend, MontConfig};

#[derive(MontConfig)]
#[modulus = "17000133324792832063019019729102503239"]
#[generator = "3"]
pub struct FqConfig;
pub type Fq = Fp128<MontBackend<FqConfig, 2>>;
