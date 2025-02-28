use ark_ec::{
    models::{CurveConfig}
};
use ark_ec::twisted_edwards::{Affine, MontCurveConfig, Projective, TECurveConfig};
use ark_ff::{MontFp};

use crate::{Fq, Fr};

pub type G1Affine = Affine<Config>;
pub type G1Projective = Projective<Config>;


#[derive(Clone, Default, PartialEq, Eq)]
pub struct Config;

impl CurveConfig for Config {
    type BaseField = Fq;
    type ScalarField = Fr;

    /// COFACTOR = 4
    const COFACTOR: &'static [u64] = &[4];

    /// COFACTOR_INV (mod r) =
    /// 1164383272576822688427601171008466494946440673178419201
    const COFACTOR_INV: Fr =
        MontFp!("1164383272576822688427601171008466494946440673178419201");
}

impl TECurveConfig for Config {
    /// COEFF_A = 1
    const COEFF_A: Fq = MontFp!("1");

    /// COEFF_D = 600581931845324488256649384912508268813600056237543024
    const COEFF_D: Fq =
        MontFp!("600581931845324488256649384912508268813600056237543024");


    const GENERATOR: G1Affine = G1Affine::new_unchecked(GENERATOR_X, GENERATOR_Y);

    type MontCurveConfig = Config;

    /// Multiplication by `a` is the identity.
    #[inline(always)]
    fn mul_by_a(elem: Self::BaseField) -> Self::BaseField {
        elem
    }
}

impl MontCurveConfig for Config {
    const COEFF_A: Fq = MontFp!("3633873325914583757878111665460483522889356159676216271");

    const COEFF_B: Fq =
        MontFp!("3633873325914583757878111665460483522889356159676216273");

    type TECurveConfig = Config;
}

/// GENERATOR_X =
pub const GENERATOR_X: Fq =
    MontFp!("3713709671941291996998665608188072510389821008693530490");

/// GENERATOR_Y =
pub const GENERATOR_Y: Fq =
    MontFp!("4869953702976555123067178261685365085639705297852816679");
