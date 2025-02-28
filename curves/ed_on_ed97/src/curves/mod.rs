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

/// ED-ON-ED97 is a twisted Edwards curve. These curves have equations of the
/// form: ax² + y² = 1 + dx²y².
/// over some base finite field Fq.
///
/// ED-ON-ED97's curve equation: 39328x² + y² = 1 + (39324)x²y²
///
/// q = 141455844224742490147094691841
#[derive(Clone, Default, PartialEq, Eq)]
pub struct EdwardsConfig;

impl CurveConfig for EdwardsConfig {
    type BaseField = Fq;
    type ScalarField = Fr;

    /// COFACTOR = 8
    const COFACTOR: &'static [u64] = &[8];

    /// COFACTOR^(-1) mod r =
    /// 6630742698034782145277621201
    const COFACTOR_INV: Fr =
        MontFp!("6630742698034782145277621201");
}

impl TECurveConfig for EdwardsConfig {
    /// COEFF_A = 39328
    const COEFF_A: Fq = MontFp!("39328");

    /// COEFF_D = 39324
    const COEFF_D: Fq =
        MontFp!("39324");

    /// AFFINE_GENERATOR_COEFFS = (GENERATOR_X, GENERATOR_Y)
    const GENERATOR: EdwardsAffine = EdwardsAffine::new_unchecked(GENERATOR_X, GENERATOR_Y);

    type MontCurveConfig = EdwardsConfig;
}

impl MontCurveConfig for EdwardsConfig {
    /// COEFF_A = 39326
    const COEFF_A: Fq = MontFp!("39326");
    /// COEFF_B = 1
    const COEFF_B: Fq = MontFp!("1");

    type TECurveConfig = EdwardsConfig;
}

/// GENERATOR_X =
/// 91710717517069496517222348537
pub const GENERATOR_X: Fq =
    MontFp!("91710717517069496517222348537");

/// GENERATOR_Y =
/// 141066233644195222309696025391
pub const GENERATOR_Y: Fq =
    MontFp!("141066233644195222309696025391");
