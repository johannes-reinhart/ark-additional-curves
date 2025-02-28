use ark_ec::{
    models::{CurveConfig},
};
use ark_ec::twisted_edwards::{Affine, MontCurveConfig, Projective, TECurveConfig};
use ark_ff::{AdditiveGroup, MontFp};

use crate::{Fq, Fq3, Fr};

pub type G2Affine = Affine<Config>;
pub type G2Projective = Projective<Config>;

#[derive(Clone, Default, PartialEq, Eq)]
pub struct Config;

impl CurveConfig for Config {
    type BaseField = Fq3;
    type ScalarField = Fr;

    /// COFACTOR =
    /// 97691247122707960797919188776040529920
    #[rustfmt::skip]
    const COFACTOR: &'static [u64] = &[
         0x0784b0014ac00000,
        0x497ea65a32bec5fb
    ];

    /// COFACTOR^(-1) mod r =
    /// 145152730903179600
    const COFACTOR_INV: Fr = MontFp!("145152730903179600");
}

impl TECurveConfig for Config {
    /// COEFF_A = 5u
    const COEFF_A: Fq3 = Fq3::new(Fq::ZERO, MontFp!("5"), Fq::ZERO);

    /// COEFF_B = 2154376507894293213*u
    const COEFF_D: Fq3 = Fq3::new(
        Fq::ZERO,
        MontFp!("2154376507894293213"),
        Fq::ZERO
    );

    /// AFFINE_GENERATOR_COEFFS = (G2_GENERATOR_X, G2_GENERATOR_Y)
    const GENERATOR: G2Affine = G2Affine::new_unchecked(G2_GENERATOR_X, G2_GENERATOR_Y);

    type MontCurveConfig = Config;

    #[inline(always)]
    fn mul_by_a(elem: Self::BaseField) -> Self::BaseField {
        let f1 = MontFp!("5");
        let f2 = MontFp!("25");
        Fq3::new(f2 * elem.c2, f1*elem.c0, f1*elem.c1)
    }
}

impl MontCurveConfig for Config {
    /// COEFF_A = 3261940957443743925
    const COEFF_A: Fq3 = Fq3::new(
        MontFp!("3261940957443743925"),
        Fq::ZERO,
        Fq::ZERO
    );

    /// COEFF_B = 1316544229896972869*u^2
    const COEFF_B: Fq3 = Fq3::new(
        Fq::ZERO,
        Fq::ZERO,
        MontFp!("1316544229896972869"),
    );

    type TECurveConfig = Config;
}

/// G2_GENERATOR_X =
///  1723522398605076730*u^2 + 2756072976377383446*u + 1052827796159481622
pub const G2_GENERATOR_X: Fq3 = Fq3::new(
    MontFp!("1052827796159481622"),
    MontFp!("2756072976377383446"),
    MontFp!("1723522398605076730"),
);

/// G2_GENERATOR_Y =
/// 3968026883264007206*u^2 + 2638401689630552874*u + 1536872126468765608
pub const G2_GENERATOR_Y: Fq3 = Fq3::new(
    MontFp!("1536872126468765608"),
    MontFp!("2638401689630552874"),
    MontFp!("3968026883264007206"),
);