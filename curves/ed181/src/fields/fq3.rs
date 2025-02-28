use ark_ff::{fields::*, MontFp};

use crate::*;

pub type Fq3 = Fp3<Fq3Config>;

pub struct Fq3Config;

impl Fp3Config for Fq3Config {
    type Fp = Fq;

    /// NONRESIDUE = 61
    const NONRESIDUE: Fq = MontFp!("61");

    /// Coefficients for the Frobenius automorphism.
    const FROBENIUS_COEFF_FP3_C1: &'static [Fq] = &[
        // NONRESIDUE**(((q^0) - 1) / 3)
        Fq::ONE,
        // NONRESIDUE**(((q^1) - 1) / 3)
        MontFp!("1073752683758513276629212192812154536507607213288832061"),
        // NONRESIDUE**(((q^2) - 1) / 3)
        MontFp!("5136291436651207728317994048073823738016144056504959939"),
    ];

    const FROBENIUS_COEFF_FP3_C2: &'static [Fq] = &[
        // NONRESIDUE**(((2q^0) - 1) / 3)
        Fq::ONE,
        // NONRESIDUE**(((2q^1) - 1) / 3)
        MontFp!("5136291436651207728317994048073823738016144056504959939"),
        // NONRESIDUE**(((2q^2) - 1) / 3)
        MontFp!("1073752683758513276629212192812154536507607213288832061"),
    ];

    const TWO_ADICITY: u32 = 31;

    const TRACE_MINUS_ONE_DIV_TWO: &'static [u64] = &[
        0x8342d651e4c23b22,
        0xea84a0a49d4f15a5,
        0x024fd9e9d8f1048a,
        0xbde0e9eaca04fb54,
        0xf94253de46487722,
        0xda4ff5e9f0ef2d1d,
        0xd50f203bb8c60fc9,
        0x28a619bc8433204a,
        0x0000000000000004
    ];

    const QUADRATIC_NONRESIDUE_TO_T: Fq3 = Fq3::new(
        MontFp!("104810943629412208121981114244673004633270996333237516"),
        Fq::ZERO,
        Fq::ZERO,
    );

}
