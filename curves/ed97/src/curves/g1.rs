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
    /// 106091883168556867610321018881
    const COFACTOR_INV: Fr =
        MontFp!("106091883168556867610321018881");
}

impl TECurveConfig for Config {
    /// COEFF_A = 5
    const COEFF_A: Fq = MontFp!("5");

    /// COEFF_D = 482996825047815773983380486779
    const COEFF_D: Fq =
        MontFp!("482996825047815773983380486779");


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
    const COEFF_A: Fq = MontFp!("113278357329480347952692529053");

    const COEFF_B: Fq =
        MontFp!("22655671465896069590538505811");

    type TECurveConfig = Config;
}

/// GENERATOR_X =
pub const GENERATOR_X: Fq =
    MontFp!("35853984911660288509491339153");

/// GENERATOR_Y =
pub const GENERATOR_Y: Fq =
    MontFp!("383408730566446498428080402051");
