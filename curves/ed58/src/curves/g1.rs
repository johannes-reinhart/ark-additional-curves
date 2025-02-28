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
    /// 158254839558438913
    const COFACTOR_INV: Fr =
        MontFp!("158254839558438913");
}

impl TECurveConfig for Config {
    /// COEFF_A = 5
    const COEFF_A: Fq = MontFp!("5");

    /// COEFF_D = 579073710274753001
    const COEFF_D: Fq =
        MontFp!("579073710274753001");


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
    const COEFF_A: Fq = MontFp!("481817660492266597");

    const COEFF_B: Fq =
        MontFp!("433973855827299349");

    type TECurveConfig = Config;
}

/// GENERATOR_X =
pub const GENERATOR_X: Fq =
    MontFp!("135119008168998470");

/// GENERATOR_Y =
pub const GENERATOR_Y: Fq =
    MontFp!("793956801732934748");
