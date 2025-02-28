use crate::ed6::Ed6Config;
use ark_ec::twisted_edwards::{Affine, Projective};
use ark_ec::{AffineRepr, CurveGroup};
use ark_ff::Field;
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use ark_std::vec::*;
use educe::Educe;

pub type G1Affine<P> = Affine<<P as Ed6Config>::G1Config>;
pub type G1Projective<P> = Projective<<P as Ed6Config>::G1Config>;

#[derive(Educe, CanonicalSerialize, CanonicalDeserialize)]
#[educe(Clone, Debug, PartialEq, Eq)]
pub struct G1Prepared<P: Ed6Config> {
    pub p_xy: P::Fp,
    pub p_xz: P::Fp,
    pub p_zzplusyz: P::Fp,
}

impl<P: Ed6Config> From<G1Affine<P>> for G1Prepared<P> {
    fn from(other: G1Affine<P>) -> Self {
        Self {
            p_xy: other.x * other.y,
            p_xz: other.x,
            p_zzplusyz: other.y + P::Fp::ONE,
        }
    }
}

impl<P: Ed6Config> From<G1Projective<P>> for G1Prepared<P> {
    fn from(q: G1Projective<P>) -> Self {
        q.into_affine().into()
    }
}

impl<'a, P: Ed6Config> From<&'a G1Affine<P>> for G1Prepared<P> {
    fn from(other: &'a G1Affine<P>) -> Self {
        Self::from(*other)
    }
}

impl<'a, P: Ed6Config> From<&'a G1Projective<P>> for G1Prepared<P> {
    fn from(q: &'a G1Projective<P>) -> Self {
        q.into_affine().into()
    }
}

impl<P: Ed6Config> Default for G1Prepared<P> {
    fn default() -> Self {
        Self::from(G1Affine::<P>::generator())
    }
}
