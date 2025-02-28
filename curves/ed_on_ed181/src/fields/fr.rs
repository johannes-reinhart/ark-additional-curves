use ark_ff::fields::{Fp192, MontBackend, MontConfig};

#[derive(MontConfig)]
#[modulus = "194063878762803781404600195121443331455540688980033961"]
#[generator = "7"]
pub struct FrConfig;
pub type Fr = Fp192<MontBackend<FrConfig, 3>>;
