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

/// ED-ON-ED181 is a twisted Edwards curve. These curves have equations of the
/// form: ax² + y² = 1 + dx²y².
/// over some base finite field Fq.
///
/// ED-ON-ED181's curve equation: 81860x² + y² = 1 + (81856)x²y²
///
/// q = 1552511030102430251236801561344621993261920897571225601
#[derive(Clone, Default, PartialEq, Eq)]
pub struct EdwardsConfig;

impl CurveConfig for EdwardsConfig {
    type BaseField = Fq;
    type ScalarField = Fr;

    /// COFACTOR = 8
    const COFACTOR: &'static [u64] = &[8];

    /// COFACTOR^(-1) mod r =
    /// 169805893917453308729025170731262915023598102857529716
    const COFACTOR_INV: Fr =
        MontFp!("169805893917453308729025170731262915023598102857529716");
}

impl TECurveConfig for EdwardsConfig {
    /// COEFF_A = 81860
    const COEFF_A: Fq = MontFp!("81860");

    /// COEFF_D = 81856
    const COEFF_D: Fq =
        MontFp!("81856");

    /// AFFINE_GENERATOR_COEFFS = (GENERATOR_X, GENERATOR_Y)
    const GENERATOR: EdwardsAffine = EdwardsAffine::new_unchecked(GENERATOR_X, GENERATOR_Y);

    type MontCurveConfig = EdwardsConfig;
}

impl MontCurveConfig for EdwardsConfig {
    /// COEFF_A = 81858
    const COEFF_A: Fq = MontFp!("81858");
    /// COEFF_B = 1
    const COEFF_B: Fq = MontFp!("1");

    type TECurveConfig = EdwardsConfig;
}

/// GENERATOR_X =
/// 6342750575829190867836500271983213213419799629817996
pub const GENERATOR_X: Fq =
    MontFp!("6342750575829190867836500271983213213419799629817996");

/// GENERATOR_Y =
/// 483659551475197198402892488930464626792170795987821102
pub const GENERATOR_Y: Fq =
    MontFp!("483659551475197198402892488930464626792170795987821102");
