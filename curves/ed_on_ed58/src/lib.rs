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
//!   211006452744585217
//! * Scalar field: r =
//!   26375806633482721
//! * Valuation(q - 1, 2) = 19
//! * Valuation(r - 1, 2) = 5
//! * Curve equation: ax^2 + y^2 =1 + dx^2y^2, where
//!    * a = 78956
//!    * d = 78952

#[cfg(feature = "r1cs")]
pub mod constraints;

mod curves;
mod fields;

pub use curves::*;
pub use fields::*;
