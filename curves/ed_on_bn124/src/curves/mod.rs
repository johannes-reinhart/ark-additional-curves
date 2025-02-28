use ark_ec::{
    models::CurveConfig,
    twisted_edwards::{Affine, MontCurveConfig, Projective, TECurveConfig},
};
use ark_ff::{Field, MontFp};

use crate::{Fq, Fr};

#[cfg(test)]
mod tests;

pub type EdwardsAffine = Affine<EdwardsConfig>;
pub type EdwardsProjective = Projective<EdwardsConfig>;

/// ED-ON-ED124 is a twisted Edwards curve. These curves have equations of the
/// form: ax² + y² = 1 + dx²y².
/// over some base finite field Fq.
///
/// ED-ON-ED124's curve equation: 419140² + y² = 1 + (419136)x²y²
///
/// q = 17000133324792832058895897937997463553
#[derive(Clone, Default, PartialEq, Eq)]
pub struct EdwardsConfig;

impl CurveConfig for EdwardsConfig {
    type BaseField = Fq;
    type ScalarField = Fr;

    /// COFACTOR = 8
    const COFACTOR: &'static [u64] = &[8];

    /// COFACTOR^(-1) mod r =
    /// 1859389582399216006355621899049396905
    const COFACTOR_INV: Fr =
        MontFp!("1859389582399216006355621899049396905");
}

impl TECurveConfig for EdwardsConfig {
    /// COEFF_A = 419140
    const COEFF_A: Fq = MontFp!("419140");

    /// COEFF_D = 419136
    const COEFF_D: Fq =
        MontFp!("419136");

    /// AFFINE_GENERATOR_COEFFS = (GENERATOR_X, GENERATOR_Y)
    const GENERATOR: EdwardsAffine = EdwardsAffine::new_unchecked(GENERATOR_X, GENERATOR_Y);

    type MontCurveConfig = EdwardsConfig;
}

impl MontCurveConfig for EdwardsConfig {
    /// COEFF_A = 419138
    const COEFF_A: Fq = MontFp!("419138");
    /// COEFF_B = 1
    const COEFF_B: Fq = Fq::ONE;

    type TECurveConfig = EdwardsConfig;
}

/// GENERATOR_X =
/// 6909407964054054886647187949591941645
pub const GENERATOR_X: Fq =
    MontFp!("6909407964054054886647187949591941645");

/// GENERATOR_Y =
/// 5471589801131278201635494029953342812
pub const GENERATOR_Y: Fq =
    MontFp!("5471589801131278201635494029953342812");
