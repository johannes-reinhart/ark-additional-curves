#![cfg_attr(not(feature = "std"), no_std)]
#![deny(
    warnings,
    unused,
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms
)]
#![forbid(unsafe_code)]

//! This library implements a Barreto-Naehring curve of embedding degree 12,
//! defined over a 183-bit (prime) field with a 183-bit scalar field. The scalar field is highly 2-adic.
//!
//! #CAUTION
//! **This curve does not satisfy the 128-bit security level**
//!
//!
//! Curve information:
//! * Base field: q =
//!   6804759748846355405830582788619626413398422602255236423
//! * Scalar field: r =
//!   6804759748846355405830582786011032970784946075266449409
//! * valuation(q - 1, 2) = 1
//! * valuation(r - 1, 2) = 30
//! * G1 curve equation: y^2 = x^3 + 3
//! * G2 curve equation: y^2 = x^3 + B, where
//!    * B = 3/(u+2) where Fq2 is represented as Fq\[u\]/(u^2+1)

#[cfg(feature = "curve")]
mod curves;

mod fields;

#[cfg(feature = "curve")]
pub use curves::*;

pub use fields::*;

#[cfg(feature = "r1cs")]
pub mod constraints;
