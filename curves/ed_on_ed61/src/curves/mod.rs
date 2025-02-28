use ark_ec::{
    models::CurveConfig,
    twisted_edwards::{Affine, MontCurveConfig, Projective, TECurveConfig},
};
use ark_ff::{MontFp};

use crate::{Fq, Fr};

#[cfg(test)]
mod tests;

pub type EdwardsAffine = Affine<EdwardsConfig>;
pub type EdwardsProjective = Projective<EdwardsConfig>;

/// ED-ON-ED61 is a twisted Edwards curve. These curves have equations of the
/// form: ax² + y² = 1 + dx²y².
/// over some base finite field Fq.
///
/// ED-ON-ED61's curve equation: 27252x² + y² = 1 + (27248)x²y²
///
/// q = 1235486033917771777
#[derive(Clone, Default, PartialEq, Eq)]
pub struct EdwardsConfig;

impl CurveConfig for EdwardsConfig {
    type BaseField = Fq;
    type ScalarField = Fr;

    /// COFACTOR = 8
    const COFACTOR: &'static [u64] = &[8];

    /// COFACTOR^(-1) mod r =
    /// 135131284933659169
    const COFACTOR_INV: Fr =
        MontFp!("135131284933659169");
}

impl TECurveConfig for EdwardsConfig {
    /// COEFF_A = 27252
    const COEFF_A: Fq = MontFp!("27252");

    /// COEFF_D = 27248
    const COEFF_D: Fq =
        MontFp!("27248");

    /// AFFINE_GENERATOR_COEFFS = (GENERATOR_X, GENERATOR_Y)
    const GENERATOR: EdwardsAffine = EdwardsAffine::new_unchecked(GENERATOR_X, GENERATOR_Y);

    type MontCurveConfig = EdwardsConfig;
}

impl MontCurveConfig for EdwardsConfig {
    /// COEFF_A = 27250
    const COEFF_A: Fq = MontFp!("27250");
    /// COEFF_B = 1
    const COEFF_B: Fq = MontFp!("1");

    type TECurveConfig = EdwardsConfig;
}

/// GENERATOR_X =
/// 686458358783095410
pub const GENERATOR_X: Fq =
    MontFp!("686458358783095410");

/// GENERATOR_Y =
/// 941103428444444802
pub const GENERATOR_Y: Fq =
    MontFp!("941103428444444802");
