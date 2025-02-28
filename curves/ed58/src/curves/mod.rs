use crate::{Fq, Fq3Config, Fq6Config};
use ark_ec_ed::{ed6};
use ark_ec_ed::ed6::{Ed6, Ed6Config};

pub mod g1;

pub mod g2;

#[cfg(test)]
mod tests;

#[derive(PartialEq, Eq)]
pub struct Config;

impl Ed6Config for Config {
    const ATE_LOOP_COUNT: &'static [i8] = &[-1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0, 0, -1, 0, -1, 0, 1, 0, 0, 1, 1];
    const FINAL_EXPONENT_W0: &'static [u64] = &[0x000000018ae00003];
    const FINAL_EXPONENT_W0_IS_NEGATIVE: bool = true;
    const FINAL_EXPONENT_W1: &'static [u64] = &[4];
    type Fp = Fq;
    type Fp3Config = Fq3Config;
    type Fp6Config = Fq6Config;
    type G1Config = g1::Config;
    type G2Config = g2::Config;
}

pub type Ed58 = Ed6<Config>;


pub type G1Affine = ed6::G1Affine<Config>;
pub type G1Projective = ed6::G1Projective<Config>;
pub type G2Affine = ed6::G2Affine<Config>;
pub type G2Projective = ed6::G2Projective<Config>;






