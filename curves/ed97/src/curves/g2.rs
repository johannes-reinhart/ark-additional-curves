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
    /// 1280624375381406043860609276660876115836617427099203367272448
    #[rustfmt::skip]
    const COFACTOR: &'static [u64] = &[
        0x74d59a8e12130000,
        0xfc858cb964cfe5c8,
        0x03e655673d8b9565,
        0x00000000000000cc
    ];

    /// COFACTOR^(-1) mod r =
    /// 97782318548797452557720164655
    const COFACTOR_INV: Fr = MontFp!("97782318548797452557720164655");
}

impl TECurveConfig for Config {
    /// COEFF_A = 5u
    const COEFF_A: Fq3 = Fq3::new(Fq::ZERO, MontFp!("5"), Fq::ZERO);

    /// COEFF_B = 482996825047815773983380486779*u
    const COEFF_D: Fq3 = Fq3::new(
        Fq::ZERO,
        MontFp!("482996825047815773983380486779"),
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
    /// COEFF_A = 113278357329480347952692529053
    const COEFF_A: Fq3 = Fq3::new(
        MontFp!("113278357329480347952692529053"),
        Fq::ZERO,
        Fq::ZERO
    );

    /// COEFF_B = 344025160432560376629106197214*u^2
    const COEFF_B: Fq3 = Fq3::new(
        Fq::ZERO,
        Fq::ZERO,
        MontFp!("344025160432560376629106197214"),
    );

    type TECurveConfig = Config;
}

/// G2_GENERATOR_X =
///  253816576543536698648272458255*u^2 + 119444391335210967012930975948*u + 14090155217658213991591903921
pub const G2_GENERATOR_X: Fq3 = Fq3::new(
    MontFp!("14090155217658213991591903921"),
    MontFp!("119444391335210967012930975948"),
    MontFp!("253816576543536698648272458255"),
);

/// G2_GENERATOR_Y =
/// 466686531549704834086607277409*u^2 + 334014561784152932012374906068*u + 314473696778825319568436450096
pub const G2_GENERATOR_Y: Fq3 = Fq3::new(
    MontFp!("314473696778825319568436450096"),
    MontFp!("334014561784152932012374906068"),
    MontFp!("466686531549704834086607277409"),
);