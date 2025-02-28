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
    const X: &'static [u64] = &[20851032918513];
    /// `x` is positive.
    const X_IS_NEGATIVE: bool = false;
    const ATE_LOOP_COUNT: &'static [i8] = &[0, 0, 0, 1, 0, 1, 0, -1, 0, 0, -1, 0, 0, 0, 1, 0, 0, -1,
        0, -1, 0, 0, 0, 1, 0, -1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, -1, 0, 0, 1, 0, 0, -1, 0, 0, 1];

    const TWIST_MUL_BY_Q_X: Fq2 = Fq2::new(
        MontFp!("2514746150702782023387381073984499065469793711673933027"),
        MontFp!("3170995372818140140778075409082321578161037698420449167"),
    );
    const TWIST_MUL_BY_Q_Y: Fq2 = Fq2::new(
        MontFp!("2785947042921859904240819706842983078799881943539725802"),
        MontFp!("5571894085843719808481639413685966157599763887079451604"),
    );
    const TWIST_TYPE: TwistType = TwistType::D;
    type Fp = Fq;
    type Fp2Config = Fq2Config;
    type Fp6Config = Fq6Config;
    type Fp12Config = Fq12Config;
    type G1Config = g1::Config;
    type G2Config = g2::Config;
}

pub type Bn183 = Bn<Config>;

pub type G1Affine = bn::G1Affine<Config>;
pub type G1Projective = bn::G1Projective<Config>;
pub type G2Affine = bn::G2Affine<Config>;
pub type G2Projective = bn::G2Projective<Config>;
