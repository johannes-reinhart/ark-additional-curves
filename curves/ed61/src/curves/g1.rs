use ark_ec::{
    models::{CurveConfig},
    AdditiveGroup
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
    /// 926614525438328833
    const COFACTOR_INV: Fr =
        MontFp!("926614525438328833");
}

impl TECurveConfig for Config {
    /// COEFF_A = 5
    const COEFF_A: Fq = MontFp!("5");

    /// COEFF_D = 2154376507894293213
    const COEFF_D: Fq =
        MontFp!("2154376507894293213");


    const GENERATOR: G1Affine = G1Affine::new_unchecked(GENERATOR_X, GENERATOR_Y);

    type MontCurveConfig = Config;

    /// Multiplication by `a`
    #[inline(always)]
    fn mul_by_a(elem: Self::BaseField) -> Self::BaseField {
        let elem4 = elem.double().double();
        elem4 + elem
    }
}

impl MontCurveConfig for Config {
    const COEFF_A: Fq = MontFp!("3261940957443743925");

    const COEFF_B: Fq =
        MontFp!("1640777017821434712");

    type TECurveConfig = Config;
}

/// GENERATOR_X =
pub const GENERATOR_X: Fq =
    MontFp!("2564941438079622667");

/// GENERATOR_Y =
pub const GENERATOR_Y: Fq =
    MontFp!("268572741021024558");
