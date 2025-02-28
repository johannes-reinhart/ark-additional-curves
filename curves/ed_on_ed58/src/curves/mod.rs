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

/// ED-ON-ED58 is a twisted Edwards curve. These curves have equations of the
/// form: ax² + y² = 1 + dx²y².
/// over some base finite field Fq.
///
/// ED-ON-ED58's curve equation: 78956x² + y² = 1 + (78952)x²y²
///
/// q = 211006452744585217
#[derive(Clone, Default, PartialEq, Eq)]
pub struct EdwardsConfig;

impl CurveConfig for EdwardsConfig {
    type BaseField = Fq;
    type ScalarField = Fr;

    /// COFACTOR = 8
    const COFACTOR: &'static [u64] = &[8];

    /// COFACTOR^(-1) mod r =
    /// 23078830804297381
    const COFACTOR_INV: Fr =
        MontFp!("23078830804297381");
}

impl TECurveConfig for EdwardsConfig {
    /// COEFF_A = 78956
    const COEFF_A: Fq = MontFp!("78956");

    /// COEFF_D = 78952
    const COEFF_D: Fq =
        MontFp!("78952");

    /// AFFINE_GENERATOR_COEFFS = (GENERATOR_X, GENERATOR_Y)
    const GENERATOR: EdwardsAffine = EdwardsAffine::new_unchecked(GENERATOR_X, GENERATOR_Y);

    type MontCurveConfig = EdwardsConfig;
}

impl MontCurveConfig for EdwardsConfig {
    /// COEFF_A = 78954
    const COEFF_A: Fq = MontFp!("78954");
    /// COEFF_B = 1
    const COEFF_B: Fq = MontFp!("1");

    type TECurveConfig = EdwardsConfig;
}

/// GENERATOR_X =
/// 136172154314006764
pub const GENERATOR_X: Fq =
    MontFp!("136172154314006764");

/// GENERATOR_Y =
/// 153203390242712599
pub const GENERATOR_Y: Fq =
    MontFp!("153203390242712599");
