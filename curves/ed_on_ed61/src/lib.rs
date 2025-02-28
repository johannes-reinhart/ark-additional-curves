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
//! scalar field of the curve ED61. This allows defining cryptographic
//! primitives that use elliptic curves over the scalar field of the latter curve.
//!
//! Curve information:
//! * Base field: q =
//!   1235486033917771777
//! * Scalar field: r =
//!   154435754209896193
//! * Valuation(q - 1, 2) = 21
//! * Valuation(r - 1, 2) = 8
//! * Curve equation: ax^2 + y^2 =1 + dx^2y^2, where
//!    * a = 27252
//!    * d = 27248

#[cfg(feature = "r1cs")]
pub mod constraints;

mod curves;
mod fields;

pub use curves::*;
pub use fields::*;
