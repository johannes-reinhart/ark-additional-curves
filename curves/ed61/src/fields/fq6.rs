use ark_ff::{fields::fp6_2over3::{Fp6, Fp6Config}, AdditiveGroup, Field, MontFp};

use crate::{Fq, Fq3, Fq3Config};

pub type Fq6 = Fp6<Fq6Config>;

#[derive(Clone, Copy)]
pub struct Fq6Config;

impl Fp6Config for Fq6Config {
    type Fp3Config = Fq3Config;

    /// NONRESIDUE = (0, 1, 0)
    const NONRESIDUE: Fq3 = Fq3::new(Fq::ZERO, Fq::ONE, Fq::ZERO);

    const FROBENIUS_COEFF_FP6_C1: &'static [Fq] = &[
        Fq::ONE,
        MontFp!("875438387576607506"),
        MontFp!("875438387576607505"),
        MontFp!("-1"),
        MontFp!("4066505744086822127"),
        MontFp!("4066505744086822128"),
    ];
}