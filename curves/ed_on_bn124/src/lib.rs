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
//! scalar field of the curve BN124. This allows defining cryptographic
//! primitives that use elliptic curves over the scalar field of the latter curve.
//!
//! Curve information:
//! * Base field: q =
//!   17000133324792832058895897937997463553
//! * Scalar field: r =
//!   2125016665599104007263567884627882177
//! * Valuation(q - 1, 2) = 25
//! * Valuation(r - 1, 2) = 6
//! * Curve equation: ax^2 + y^2 =1 + dx^2y^2, where
//!    * a = 419140
//!    * d = 419136

#[cfg(feature = "r1cs")]
pub mod constraints;

mod curves;
mod fields;

pub use curves::*;
pub use fields::*;
