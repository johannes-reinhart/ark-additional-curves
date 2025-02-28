use ark_ff::fields::{Fp192, MontBackend, MontConfig};

#[derive(MontConfig)]
#[modulus = "6210044120409721004947206240885978274523751269793792001"]
#[generator = "61"]
pub struct FqConfig;
pub type Fq = Fp192<MontBackend<FqConfig, 3>>;