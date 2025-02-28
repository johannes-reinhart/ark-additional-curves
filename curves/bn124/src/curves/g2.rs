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
    /// 17000133324792832067142141520207542925
    #[rustfmt::skip]
    const COFACTOR: &'static [u64] = &[
        0x41af0810f2f9fa8d,
        0x0cca1b4903fd4618,
    ];

    /// COFACTOR_INV = COFACTOR^{-1} mod r
    const COFACTOR_INV: Fr =
        MontFp!("8500066672650216804247105437771916169");
}

impl SWCurveConfig for Config {
    /// COEFF_A = [0, 0]
    const COEFF_A: Fq2 = Fq2::ZERO;

    /// COEFF_B = 3/(u+9)
    const COEFF_B: Fq2 = Fq2::new(
        MontFp!("653851281722801233193039220350096279"),
        MontFp!("3269256408614006165965196101750481392"),
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
        MontFp!("10253800776860717366812606462"),
        Fq::ZERO,
    )];

    const LAMBDA: Self::ScalarField =
        MontFp!("17000133304285230517543828587634978595");

    const SCALAR_DECOMP_COEFFS: [(bool, <Self::ScalarField as PrimeField>::BigInt); 4] = [
        (false, BigInt!("4123121792762974504")),
        (true, BigInt!("1657934819")),
        (false, BigInt!("1657934819")),
        (false, BigInt!("4123121794420909323")),
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
/// 14880959035855674610313492209367066422
pub const G2_GENERATOR_X_C0: Fq =
    MontFp!("14880959035855674610313492209367066422");

/// G2_GENERATOR_X_C1 =
/// 5742857096359631272514000993617690531
pub const G2_GENERATOR_X_C1: Fq =
    MontFp!("5742857096359631272514000993617690531");

/// G2_GENERATOR_Y_C0 =
/// 14898980375260182662452300601323602433
pub const G2_GENERATOR_Y_C0: Fq =
    MontFp!("14898980375260182662452300601323602433");

/// G2_GENERATOR_Y_C1 =
/// 2528144427111297799162338239765869074
pub const G2_GENERATOR_Y_C1: Fq =
    MontFp!("2528144427111297799162338239765869074");

// PSI_X = (u+9)^((p-1)/3) = TWIST_MUL_BY_Q_X
const P_POWER_ENDOMORPHISM_COEFF_0: Fq2 = Fq2::new(
    MontFp!("11057421525851115170548223099100677627"),
    MontFp!("5501413929240230020448532534432497469"),
);

// PSI_Y = (u+9)^((p-1)/2) = TWIST_MUL_BY_Q_Y
const P_POWER_ENDOMORPHISM_COEFF_1: Fq2 = Fq2::new(
    MontFp!("11217235539836039461521625978614135522"),
    MontFp!("5085777724801701118551070705763167893"),
);

// Integer representation of 6x^2 = t - 1
const SIX_X_SQUARED: [u64; 1] = [0x3938456c7c7cfd46];

/// psi(P) is the untwist-Frobenius-twist endomorphism on E'(Fq2)
fn p_power_endomorphism(p: &Affine<Config>) -> Affine<Config> {
    // Maps (x,y) -> (x^p * (u+9)^((p-1)/3), y^p * (u+9)^((p-1)/2))

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
