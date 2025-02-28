#![cfg_attr(not(feature = "std"), no_std)]
#![deny(
    warnings,
    unused,
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms
)]
#![forbid(unsafe_code)]

//! This library implements the ed97 twisted Edwards curve.
//!
//! Curve information:
//! * Base field: q =
//!   565823376898968604518330826753
//! * Scalar field: r =
//!   141455844224742490147094691841
//! * Curve equation: ax^2 + y^2 =1 + dx^2y^2, where
//!    * a = 5
//!    * d = 482996825047815773983380486779

#[cfg(feature = "curve")]
mod curves;

#[cfg(feature = "scalar_field")]
mod fields;

#[cfg(feature = "curve")]
pub use curves::*;

#[cfg(feature = "scalar_field")]
pub use fields::*;