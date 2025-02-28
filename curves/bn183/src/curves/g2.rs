use ark_ec::AffineRepr;
use ark_ec::{
    models::{short_weierstrass::SWCurveConfig, CurveConfig},
    scalar_mul::glv::GLVConfig,
    short_weierstrass::{Affine, Projective},
};
use ark_ff::{AdditiveGroup, BigInt, Field, MontFp, PrimeField, Zero};

use crate::{Fq, Fq2, Fr};

pub type G2Affine = Affine<Config>;

#[derive(Clone, Default, PartialEq, Eq)]
pub struct Config;

impl CurveConfig for Config {
    type BaseField = Fq2;
    type ScalarField = Fr;

    /// COFACTOR = (36 * X^4) + (36 * X^3) + (30 * X^2) + 6*X + 1
    /// 6804759748846355405830582791228219856011899129244023437
    #[rustfmt::skip]
    const COFACTOR: &'static [u64] = &[
        0xa47533ccd0f9fa8d,
        0xd5aa16d763652bfe,
        0x00470b858f450c16
    ];

    /// COFACTOR_INV = COFACTOR^{-1} mod r
    const COFACTOR_INV: Fr =
        MontFp!("3402379874423340878518520251781352264607001911826473865");
}

impl SWCurveConfig for Config {
    /// COEFF_A = [0, 0]
    const COEFF_A: Fq2 = Fq2::ZERO;

    /// COEFF_B = 3/(u+2)
    const COEFF_B: Fq2 = Fq2::new(
        MontFp!("4082855849307813243498349673171775848039053561353141855"),
        MontFp!("1360951949769271081166116557723925282679684520451047284"),
    );

    /// AFFINE_GENERATOR_COEFFS = (G2_GENERATOR_X, G2_GENERATOR_Y)
    const GENERATOR: G2Affine = G2Affine::new_unchecked(G2_GENERATOR_X, G2_GENERATOR_Y);

    #[inline(always)]
    fn mul_by_a(_: Self::BaseField) -> Self::BaseField {
        Self::BaseField::zero()
    }

    fn is_in_correct_subgroup_assuming_on_curve(point: &G2Affine) -> bool {
        // Subgroup check from section 4.3 of https://eprint.iacr.org/2022/352.pdf.
        //
        // Checks that [p]P = [6X^2]P

        let x_times_point = point.mul_bigint(SIX_X_SQUARED);
        let p_times_point = p_power_endomorphism(point);
        x_times_point.eq(&p_times_point)
    }
}

impl GLVConfig for Config {
    const ENDO_COEFFS: &'static [Self::BaseField] = &[Fq2::new(
        MontFp!("163175603228860080132500521329690786398206"),
        Fq::ZERO,
    )];

    const LAMBDA: Self::ScalarField =
        MontFp!("6804759748846029054624125073676548297582966487055036195");

    const SCALAR_DECOMP_COEFFS: [(bool, <Self::ScalarField as PrimeField>::BigInt); 4] = [
        (false, BigInt!("2608593442613518229054624040")),
        (true, BigInt!("41702065837027")),
        (false, BigInt!("41702065837027")),
        (false, BigInt!("2608593442613559931120461067")),
    ];

    fn endomorphism(p: &Projective<Self>) -> Projective<Self> {
        let mut res = (*p).clone();
        res.x *= Self::ENDO_COEFFS[0];
        res
    }

    fn endomorphism_affine(p: &Affine<Self>) -> Affine<Self> {
        let mut res = (*p).clone();
        res.x *= Self::ENDO_COEFFS[0];
        res
    }
}

pub const G2_GENERATOR_X: Fq2 = Fq2::new(G2_GENERATOR_X_C0, G2_GENERATOR_X_C1);
pub const G2_GENERATOR_Y: Fq2 = Fq2::new(G2_GENERATOR_Y_C0, G2_GENERATOR_Y_C1);

/// G2_GENERATOR_X_C0 =
pub const G2_GENERATOR_X_C0: Fq =
    MontFp!("5918765072744050273221227385107771003170856254397331867");

/// G2_GENERATOR_X_C1 =
pub const G2_GENERATOR_X_C1: Fq =
    MontFp!("4901218678230392619274717698348105178349606046410571296");

/// G2_GENERATOR_Y_C0 =
pub const G2_GENERATOR_Y_C0: Fq =
    MontFp!("832061610063624276916684208076330867301923647802765078");

/// G2_GENERATOR_Y_C1 =
pub const G2_GENERATOR_Y_C1: Fq =
    MontFp!("487297600131553948827424152042953421947568131462965400");

// PSI_X = (u+2)^((p-1)/3) = TWIST_MUL_BY_Q_X
const P_POWER_ENDOMORPHISM_COEFF_0: Fq2 = Fq2::new(
    MontFp!("2514746150702782023387381073984499065469793711673933027"),
    MontFp!("3170995372818140140778075409082321578161037698420449167"),
);

// PSI_Y = (u+2)^((p-1)/2) = TWIST_MUL_BY_Q_Y
const P_POWER_ENDOMORPHISM_COEFF_1: Fq2 = Fq2::new(
    MontFp!("2785947042921859904240819706842983078799881943539725802"),
    MontFp!("5571894085843719808481639413685966157599763887079451604"),
);

// Integer representation of 6x^2 = t - 1
const SIX_X_SQUARED: [u64; 2] = [0xe1723006887cfd46, 0x00000000086dc725];

/// psi(P) is the untwist-Frobenius-twist endomorphism on E'(Fq2)
fn p_power_endomorphism(p: &Affine<Config>) -> Affine<Config> {
    // Maps (x,y) -> (x^p * (u+2)^((p-1)/3), y^p * (u+2)^((p-1)/2))

    let mut res = *p;
    res.x.frobenius_map_in_place(1);
    res.y.frobenius_map_in_place(1);

    res.x *= P_POWER_ENDOMORPHISM_COEFF_0;
    res.y *= P_POWER_ENDOMORPHISM_COEFF_1;

    res
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::g2;
    use ark_std::{rand::Rng, UniformRand};

    fn sample_unchecked() -> Affine<g2::Config> {
        let mut rng = ark_std::test_rng();
        loop {
            let x1 = Fq::rand(&mut rng);
            let x2 = Fq::rand(&mut rng);
            let greatest = rng.gen();
            let x = Fq2::new(x1, x2);

            if let Some(p) = Affine::get_point_from_x_unchecked(x, greatest) {
                return p;
            }
        }
    }

    fn naive_is_in_subgroup_assuming_on_curve(p: &Affine<g2::Config>) -> bool {
        <g2::Config as SWCurveConfig>::mul_affine(
            p,
            <g2::Config as CurveConfig>::ScalarField::characteristic(),
        )
        .is_zero()
    }

    #[test]
    fn test_is_in_subgroup_assuming_on_curve() {
        const SAMPLES: usize = 100;
        for _ in 0..SAMPLES {
            let p: Affine<g2::Config> = sample_unchecked();
            assert!(p.is_on_curve());

            assert_eq!(
                naive_is_in_subgroup_assuming_on_curve(&p),
                p.is_in_correct_subgroup_assuming_on_curve()
            );

            let cleared = p.clear_cofactor();
            assert!(cleared.is_in_correct_subgroup_assuming_on_curve());
        }
    }
}
