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
//! scalar field of the curve ED97. This allows defining cryptographic
//! primitives that use elliptic curves over the scalar field of the latter curve.
//!
//! Curve information:
//! * Base field: q =
//!   141455844224742490147094691841
//! * Scalar field: r =
//!   17681980528092752387406989869
//! * Valuation(q - 1, 2) = 15
//! * Valuation(r - 1, 2) = 2
//! * Curve equation: ax^2 + y^2 =1 + dx^2y^2, where
//!    * a = 39328
//!    * d = 39324

#[cfg(feature = "r1cs")]
pub mod constraints;

mod curves;
mod fields;

pub use curves::*;
pub use fields::*;
