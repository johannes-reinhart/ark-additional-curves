use ark_ff::{fields::*, MontFp};

use crate::*;

pub type Fq3 = Fp3<Fq3Config>;

pub struct Fq3Config;

impl Fp3Config for Fq3Config {
    type Fp = Fq;

    /// NONRESIDUE = 5
    const NONRESIDUE: Fq = MontFp!("5");

    /// Coefficients for the Frobenius automorphism.
    const FROBENIUS_COEFF_FP3_C1: &'static [Fq] = &[
        // NONRESIDUE**(((q^0) - 1) / 3)
        Fq::ONE,
        // NONRESIDUE**(((q^1) - 1) / 3)
        MontFp!("875438387576607505"),
        // NONRESIDUE**(((q^2) - 1) / 3)
        MontFp!("4066505744086822127"),
    ];

    const FROBENIUS_COEFF_FP3_C2: &'static [Fq] = &[
        // NONRESIDUE**(((2q^0) - 1) / 3)
        Fq::ONE,
        // NONRESIDUE**(((2q^1) - 1) / 3)
        MontFp!("4066505744086822127"),
        // NONRESIDUE**(((2q^2) - 1) / 3)
        MontFp!("875438387576607505"),
    ];

    const TWO_ADICITY: u32 = 21;

    const TRACE_MINUS_ONE_DIV_TWO: &'static [u64] = &[
        0x31ffcb200d700ceb,
        0xab6ed0f3f013a24d,
        0x00000013b081deef
    ];

    const QUADRATIC_NONRESIDUE_TO_T: Fq3 = Fq3::new(
        MontFp!("4129413696311215253"),
        Fq::ZERO,
        Fq::ZERO,
    );

}