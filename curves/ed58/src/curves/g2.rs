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
    /// 2849518261615816147846566658477916160
    #[rustfmt::skip]
    const COFACTOR: &'static [u64] = &[
        0x64730b0088b00000,
        0x0224cc18e28468cc
];

    /// COFACTOR^(-1) mod r =
    /// 69758963944332641
    const COFACTOR_INV: Fr = MontFp!("69758963944332641");
}

impl TECurveConfig for Config {
    /// COEFF_A = 5u
    const COEFF_A: Fq3 = Fq3::new(Fq::ZERO, MontFp!("5"), Fq::ZERO);

    /// COEFF_B = 579073710274753001*u
    const COEFF_D: Fq3 = Fq3::new(
        Fq::ZERO,
        MontFp!("579073710274753001"),
        Fq::ZERO
    );

    /// AFFINE_GENERATOR_COEFFS = (G2_GENERATOR_X, G2_GENERATOR_Y)
    const GENERATOR: G2Affine = G2Affine::new_unchecked(G2_GENERATOR_X, G2_GENERATOR_Y);

    type MontCurveConfig = Config;

    #[inline(always)]
    fn mul_by_a(elem: Self::BaseField) -> Self::BaseField {
        let f1 = MontFp!("5");
        let f2 = MontFp!("50");
        Fq3::new(f2 * elem.c2, f1*elem.c0, f1*elem.c1)
    }
}

impl MontCurveConfig for Config {
    /// COEFF_A = 481817660492266597
    const COEFF_A: Fq3 = Fq3::new(
        MontFp!("481817660492266597"),
        Fq::ZERO,
        Fq::ZERO
    );

    /// COEFF_B = 634215452108210486*u^2
    const COEFF_B: Fq3 = Fq3::new(
        Fq::ZERO,
        Fq::ZERO,
        MontFp!("634215452108210486"),
    );

    type TECurveConfig = Config;
}

/// G2_GENERATOR_X =
///  173440355679943349*u^2 + 369364789740391425*u + 96752087320423969
pub const G2_GENERATOR_X: Fq3 = Fq3::new(
    MontFp!("96752087320423969"),
    MontFp!("369364789740391425"),
    MontFp!("173440355679943349"),
);

/// G2_GENERATOR_Y =
/// 720092750626437816*u^2 + 613665718875394571*u + 17436120953877613
pub const G2_GENERATOR_Y: Fq3 = Fq3::new(
    MontFp!("17436120953877613"),
    MontFp!("613665718875394571"),
    MontFp!("720092750626437816"),
);