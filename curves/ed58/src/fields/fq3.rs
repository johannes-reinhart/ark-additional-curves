use ark_ff::{fields::*, MontFp};

use crate::*;

pub type Fq3 = Fp3<Fq3Config>;

pub struct Fq3Config;

impl Fp3Config for Fq3Config {
    type Fp = Fq;

    /// NONRESIDUE = 10
    const NONRESIDUE: Fq = MontFp!("10");

    /// Coefficients for the Frobenius automorphism.
    const FROBENIUS_COEFF_FP3_C1: &'static [Fq] = &[
        // NONRESIDUE**(((q^0) - 1) / 3)
        Fq::ONE,
        // NONRESIDUE**(((q^1) - 1) / 3)
        MontFp!("126784148197437870"),
        // NONRESIDUE**(((q^2) - 1) / 3)
        MontFp!("717241661124677202"),
    ];

    const FROBENIUS_COEFF_FP3_C2: &'static [Fq] = &[
        // NONRESIDUE**(((2q^0) - 1) / 3)
        Fq::ONE,
        // NONRESIDUE**(((2q^1) - 1) / 3)
        MontFp!("717241661124677202"),
        // NONRESIDUE**(((2q^2) - 1) / 3)
        MontFp!("126784148197437870"),
    ];

    const TWO_ADICITY: u32 = 19;

    const TRACE_MINUS_ONE_DIV_TWO: &'static [u64] = &[
        0xb034a352fa5c155b,
        0xfc19fa3956eea07c,
        0x000000006470b65f
    ];

    const QUADRATIC_NONRESIDUE_TO_T: Fq3 = Fq3::new(
        MontFp!("628058373490290683"),
        Fq::ZERO,
        Fq::ZERO,
    );

}