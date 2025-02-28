use ark_ec::{
    bn,
    bn::{Bn, BnConfig, TwistType},
};
use ark_ff::MontFp;

use crate::*;

pub mod g1;
pub mod g2;

#[cfg(test)]
mod tests;

pub struct Config;

impl BnConfig for Config {
    const X: &'static [u64] = &[828967409];
    /// `x` is positive.
    const X_IS_NEGATIVE: bool = false;
    const ATE_LOOP_COUNT: &'static [i8] = &[0, 0, 0, 1, 0, 1, 0, -1, 0, 0, -1, 0, 0, 0, 1, 0, 0, -1,
        0, -1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 1];

    const TWIST_MUL_BY_Q_X: Fq2 = Fq2::new(
        MontFp!("11057421525851115170548223099100677627"),
        MontFp!("5501413929240230020448532534432497469"),
    );
    const TWIST_MUL_BY_Q_Y: Fq2 = Fq2::new(
        MontFp!("11217235539836039461521625978614135522"),
        MontFp!("5085777724801701118551070705763167893"),
    );
    const TWIST_TYPE: TwistType = TwistType::D;
    type Fp = Fq;
    type Fp2Config = Fq2Config;
    type Fp6Config = Fq6Config;
    type Fp12Config = Fq12Config;
    type G1Config = g1::Config;
    type G2Config = g2::Config;
}

pub type Bn124 = Bn<Config>;

pub type G1Affine = bn::G1Affine<Config>;
pub type G1Projective = bn::G1Projective<Config>;
pub type G2Affine = bn::G2Affine<Config>;
pub type G2Projective = bn::G2Projective<Config>;
