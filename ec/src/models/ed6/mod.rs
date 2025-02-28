use ark_ec::{
    models::{twisted_edwards::TECurveConfig, CurveConfig},
    pairing::{MillerLoopOutput, Pairing, PairingOutput},
};
use ark_ff::{
    fields::{
        fp3::Fp3Config,
        fp6_2over3::{Fp6, Fp6Config},
        Field, Fp3, PrimeField,
    },
    AdditiveGroup, CyclotomicMultSubgroup,
};
use ark_std::{cfg_chunks_mut, marker::PhantomData, vec::*};
use educe::Educe;
use itertools::Itertools;
use num_traits::One;

use crate::ed6::g2::EllCoeff;
#[cfg(feature = "parallel")]
use rayon::prelude::*;

pub mod g1;
pub mod g2;

pub use self::{
    g1::{G1Affine, G1Prepared, G1Projective},
    g2::{G2Affine, G2Prepared, G2Projective},
};

pub trait Ed6Config: 'static + Sized {
    /// Trace - 1
    const ATE_LOOP_COUNT: &'static [i8];

    const FINAL_EXPONENT_W0: &'static [u64];
    const FINAL_EXPONENT_W0_IS_NEGATIVE: bool;
    const FINAL_EXPONENT_W1: &'static [u64];

    type Fp: PrimeField + Into<<Self::Fp as PrimeField>::BigInt>;
    type Fp3Config: Fp3Config<Fp = Self::Fp>;
    type Fp6Config: Fp6Config<Fp3Config = Self::Fp3Config>;
    type G1Config: TECurveConfig<BaseField = Self::Fp>;
    type G2Config: TECurveConfig<
        BaseField = Fp3<Self::Fp3Config>,
        ScalarField = <Self::G1Config as CurveConfig>::ScalarField,
    >;

    #[inline(always)]
    fn mul_by_twist(v: Fp3<Self::Fp3Config>) -> Fp3<Self::Fp3Config> {
        // Default implementation assumes twist = u
        Fp3::new(Self::Fp3Config::NONRESIDUE * v.c2, v.c0, v.c1)
    }
}

#[derive(Educe)]
#[educe(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct Ed6<P: Ed6Config>(PhantomData<fn() -> P>);

impl<P: Ed6Config> Ed6<P> {
    /// Doubling step of the flipped miller loop
    /// From: C. Arene et. al.: "Faster Computation of the Tate Pairing", Section 5.2
    fn doubling_for_flipped_miller_loop(r: &mut G2Projective<P>) -> EllCoeff<P> {
        let a = r.x.square();
        let b = r.y.square();
        let c = r.z.square();
        let d = (r.x + r.y).square();
        let e = (r.y + r.z).square();
        let f = d - (a + b);
        let g = e - (b + c);
        let h = P::G2Config::mul_by_a(a);
        let i = h + b;
        let j = c - i;
        let k = j + c;

        let mut c_zz = r.y * (r.t - r.x);
        c_zz.double_in_place();
        let c_xy = j + j + g;
        let mut c_xz = P::G2Config::mul_by_a(r.x * r.t) - b;
        c_xz.double_in_place();

        r.x = f * k;
        r.y = i * (b - h);
        r.z = i * k;
        r.t = f * (b - h);

        EllCoeff { c_zz, c_xy, c_xz }
    }

    /// addition step of the flipped miller loop
    /// From: C. Arene et. al.: "Faster Computation of the Tate Pairing", Section 5.1
    fn addition_for_flipped_miller_loop(
        base: &G2Projective<P>,
        r: &mut G2Projective<P>,
    ) -> EllCoeff<P> {
        let a = r.x * base.x;
        let b = r.y * base.y;
        let c = r.z * base.t;
        let d = r.t * base.z;
        let e = d + c;
        let f = (r.x - r.y) * (base.x + base.y) + b - a;
        let g = b + P::G2Config::mul_by_a(a);
        let h = d - c;
        let i = r.t * base.t;

        let c_zz = P::mul_by_twist((r.t - r.x) * (base.t + base.x) - i + a);
        let c_xy = r.x * base.z - base.x * r.z + f;
        let c_xz = (r.y - r.t) * (base.y + base.t) - b + i - h;

        r.x = e * f;
        r.y = g * h;
        r.z = f * g;
        r.t = e * h;

        EllCoeff { c_zz, c_xy, c_xz }
    }

    #[allow(clippy::let_and_return)]
    fn final_exponentiation_first_part(
        elt: Fp6<P::Fp6Config>,
        elt_inv: Fp6<P::Fp6Config>,
    ) -> Fp6<P::Fp6Config> {
        // First part: result = elt^(q^3-1)*(q+1).
        let elt_q3 = elt.frobenius_map(3);
        let elt_q3_m_1 = elt_q3 * elt_inv;
        let elt_q3_m_1_q = elt_q3_m_1.frobenius_map(1);
        let elt_q3_m_f_q_p_1 = elt_q3_m_1_q * elt_q3_m_1;
        elt_q3_m_f_q_p_1
    }

    #[allow(clippy::let_and_return)]
    fn final_exponentiation_last_part_w1(elt: Fp6<P::Fp6Config>) -> Fp6<P::Fp6Config> {
        // Last part w1:
        let elt_q = elt.frobenius_map(1);
        let w1 = elt_q.cyclotomic_exp(P::FINAL_EXPONENT_W1);

        w1
    }

    #[allow(clippy::let_and_return)]
    fn final_exponentiation_last_part_w0(elt: Fp6<P::Fp6Config>) -> Fp6<P::Fp6Config> {
        // Last part w0:
        elt.cyclotomic_exp(P::FINAL_EXPONENT_W0)
    }
}

impl<P: Ed6Config> Pairing for Ed6<P> {
    type BaseField = <P::G1Config as CurveConfig>::BaseField;
    type ScalarField = <P::G1Config as CurveConfig>::ScalarField;
    type G1 = G1Projective<P>;
    type G1Affine = G1Affine<P>;
    type G1Prepared = G1Prepared<P>;
    type G2 = G2Projective<P>;
    type G2Affine = G2Affine<P>;
    type G2Prepared = G2Prepared<P>;
    type TargetField = Fp6<P::Fp6Config>;

    fn multi_miller_loop(
        a: impl IntoIterator<Item = impl Into<Self::G1Prepared>>,
        b: impl IntoIterator<Item = impl Into<Self::G2Prepared>>,
    ) -> MillerLoopOutput<Self> {
        let mut pairs = a
            .into_iter()
            .zip_eq(b)
            .map(|(p, q)| {
                let (p, q) = (p.into(), q.into());
                (p, q.ell_coeffs.into_iter())
            })
            .collect::<Vec<_>>();

        let f = cfg_chunks_mut!(pairs, 4)
            .map(|pairs| {
                let mut f = <Ed6<P> as Pairing>::TargetField::one();
                for i in (1..P::ATE_LOOP_COUNT.len()).rev() {
                    if i != P::ATE_LOOP_COUNT.len() - 1 {
                        f.square_in_place();
                    }

                    for (p, coeffs) in pairs.iter_mut() {
                        let c = coeffs.next().unwrap();
                        let g_rr = Fp6::new(
                            c.c_xy.mul_by_base_prime_field(&p.p_xy)
                                + c.c_xz.mul_by_base_prime_field(&p.p_xz),
                            c.c_zz.mul_by_base_prime_field(&p.p_zzplusyz),
                        );
                        f *= &g_rr;
                    }

                    let bit = P::ATE_LOOP_COUNT[i - 1];
                    if bit == 1 || bit == -1 {
                        for (p, coeffs) in pairs.iter_mut() {
                            let c = coeffs.next().unwrap();
                            let g_rq = Fp6::new(
                                c.c_zz.mul_by_base_prime_field(&p.p_zzplusyz),
                                c.c_xy.mul_by_base_prime_field(&p.p_xy)
                                    + c.c_xz.mul_by_base_prime_field(&p.p_xz),
                            );
                            f *= &g_rq;
                        }
                    }
                }
                f
            })
            .product::<<Ed6<P> as Pairing>::TargetField>();

        MillerLoopOutput(f)
    }

    fn final_exponentiation(f: MillerLoopOutput<Self>) -> Option<PairingOutput<Self>> {
        let f = f.0;
        let f_inv = f.inverse()?;
        let f_first_part = Self::final_exponentiation_first_part(f, f_inv);
        let f_inv_first_part = Self::final_exponentiation_first_part(f_inv, f);

        let w1 = Self::final_exponentiation_last_part_w1(f_first_part);
        let w0 = match P::FINAL_EXPONENT_W0_IS_NEGATIVE {
            true => Self::final_exponentiation_last_part_w0(f_inv_first_part),
            false => Self::final_exponentiation_last_part_w0(f_first_part),
        };
        Some(PairingOutput(w1 * w0))
    }
}
