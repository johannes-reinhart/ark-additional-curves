use ark_ff::Fp3;
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use ark_std::vec::*;
use educe::Educe;

use crate::ed6::{Ed6, Ed6Config};
use ark_ec::twisted_edwards::{Affine, Projective};
use ark_ec::AffineRepr;

pub type G2Affine<P> = Affine<<P as Ed6Config>::G2Config>;
pub type G2Projective<P> = Projective<<P as Ed6Config>::G2Config>;

#[derive(Educe, CanonicalSerialize, CanonicalDeserialize)]
#[educe(Clone, Debug, PartialEq, Eq)]
pub struct G2Prepared<P: Ed6Config> {
    pub ell_coeffs: Vec<EllCoeff<P>>,
}

#[derive(Educe, CanonicalSerialize, CanonicalDeserialize)]
#[educe(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EllCoeff<P: Ed6Config> {
    pub c_zz: Fp3<P::Fp3Config>,
    pub c_xy: Fp3<P::Fp3Config>,
    pub c_xz: Fp3<P::Fp3Config>,
}

impl<P: Ed6Config> Default for G2Prepared<P> {
    fn default() -> Self {
        Self::from(G2Affine::<P>::generator())
    }
}

impl<P: Ed6Config> From<G2Projective<P>> for G2Prepared<P> {
    fn from(q: G2Projective<P>) -> Self {
        let mut ell_coeffs = vec![];

        let mut r = q.clone();
        let base = r.clone();
        let neg_base = -base;

        for bit in P::ATE_LOOP_COUNT.iter().rev().skip(1) {
            // doubling step
            let c = Ed6::doubling_for_flipped_miller_loop(&mut r);
            ell_coeffs.push(c);

            match bit {
                1 => ell_coeffs.push(Ed6::addition_for_flipped_miller_loop(&base, &mut r)),
                -1 => ell_coeffs.push(Ed6::addition_for_flipped_miller_loop(&neg_base, &mut r)),
                _ => continue,
            }
        }

        Self { ell_coeffs }
    }
}

impl<P: Ed6Config> From<G2Affine<P>> for G2Prepared<P> {
    fn from(q: G2Affine<P>) -> Self {
        Self::from(G2Projective::<P>::from(q))
    }
}

impl<'a, P: Ed6Config> From<&'a G2Affine<P>> for G2Prepared<P> {
    fn from(q: &'a G2Affine<P>) -> Self {
        Self::from(G2Projective::<P>::from(*q))
    }
}

impl<'a, P: Ed6Config> From<&'a G2Projective<P>> for G2Prepared<P> {
    fn from(other: &'a G2Projective<P>) -> Self {
        Self::from(*other)
    }
}
