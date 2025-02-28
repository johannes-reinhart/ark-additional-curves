#![cfg_attr(not(feature = "std"), no_std)]
#![deny(
    warnings,
    unused,
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms
)]
#![forbid(unsafe_code)]

//! This library implements a twisted Edwards curve whose base field is the
//! scalar field of the curve ED181. This allows defining cryptographic
//! primitives that use elliptic curves over the scalar field of the latter curve.
//!
//! Curve information:
//! * Base field: q =
//!   1552511030102430251236801561344621993261920897571225601
//! * Scalar field: r =
//!   194063878762803781404600195121443331455540688980033961
//! * Valuation(q - 1, 2) = 31
//! * Valuation(r - 1, 2) = 3
//! * Curve equation: ax^2 + y^2 =1 + dx^2y^2, where
//!    * a = 81860
//!    * d = 81856

#[cfg(feature = "r1cs")]
pub mod constraints;

mod curves;
mod fields;

pub use curves::*;
pub use fields::*;
