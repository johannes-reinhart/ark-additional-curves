use ark_ff::fields::{Fp192, MontBackend, MontConfig};

#[derive(MontConfig)]
#[modulus = "850594968605794425728822847691900677428791383617892397"]
#[generator = "5"]
pub struct FrConfig;
pub type Fr = Fp192<MontBackend<FrConfig, 3>>;
