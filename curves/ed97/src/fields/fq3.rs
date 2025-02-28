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
        MontFp!("394610617619779289602567691434"),
        // NONRESIDUE**(((q^2) - 1) / 3)
        MontFp!("171212759279189314915763135318"),
    ];

    const FROBENIUS_COEFF_FP3_C2: &'static [Fq] = &[
        // NONRESIDUE**(((2q^0) - 1) / 3)
        Fq::ONE,
        // NONRESIDUE**(((2q^1) - 1) / 3)
        MontFp!("171212759279189314915763135318"),
        // NONRESIDUE**(((2q^2) - 1) / 3)
        MontFp!("394610617619779289602567691434"),
    ];

    const TWO_ADICITY: u32 = 15;

    const TRACE_MINUS_ONE_DIV_TWO: &'static [u64] = &[
        0xd46a97d8be296d2f,
        0xa7028d1c557274a9,
        0xe9238302994e1a6e,
        0xf3782d3cbf94a3d7,
        0x00000000016c40ee
    ];

    const QUADRATIC_NONRESIDUE_TO_T: Fq3 = Fq3::new(
        MontFp!("179411943930830432587917996150"),
        Fq::ZERO,
        Fq::ZERO,
    );

}