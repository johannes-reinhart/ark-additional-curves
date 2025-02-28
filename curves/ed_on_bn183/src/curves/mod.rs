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

/// ED-ON-ED183 is a twisted Edwards curve. These curves have equations of the
/// form: ax² + y² = 1 + dx²y².
/// over some base finite field Fq.
///
/// ED-ON-ED183's curve equation: 98032² + y² = 1 + (98028)x²y²
///
/// q = 6804759748846355405830582786011032970784946075266449409
#[derive(Clone, Default, PartialEq, Eq)]
pub struct EdwardsConfig;

impl CurveConfig for EdwardsConfig {
    type BaseField = Fq;
    type ScalarField = Fr;

    /// COFACTOR = 8
    const COFACTOR: &'static [u64] = &[8];

    /// COFACTOR^(-1) mod r =
    /// 318973113227172909648308567884462754035796768856709649
    const COFACTOR_INV: Fr =
        MontFp!("318973113227172909648308567884462754035796768856709649");
}

impl TECurveConfig for EdwardsConfig {
    /// COEFF_A = a: 98032
    const COEFF_A: Fq = MontFp!("98032");

    /// COEFF_D = 98028
    const COEFF_D: Fq =
        MontFp!("98028");

    /// AFFINE_GENERATOR_COEFFS = (GENERATOR_X, GENERATOR_Y)
    const GENERATOR: EdwardsAffine = EdwardsAffine::new_unchecked(GENERATOR_X, GENERATOR_Y);

    type MontCurveConfig = EdwardsConfig;
}

impl MontCurveConfig for EdwardsConfig {
    /// COEFF_A = 98030
    const COEFF_A: Fq = MontFp!("98030");
    /// COEFF_B = 1
    const COEFF_B: Fq = Fq::ONE;

    type TECurveConfig = EdwardsConfig;
}

/// GENERATOR_X =
pub const GENERATOR_X: Fq =
    MontFp!("3104089982041410053686572563144207251924860028807892115");

/// GENERATOR_Y =
pub const GENERATOR_Y: Fq =
    MontFp!("4933170495199545959259650164466459533050671935663371647");
