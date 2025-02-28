#![cfg_attr(not(feature = "std"), no_std)]
#![deny(
warnings,
unused,
future_incompatible,
nonstandard_style,
rust_2018_idioms
)]
#![forbid(unsafe_code)]

//! This library implements the ed58 twisted Edwards curve.
//!
//! Curve information:
//! * Base field: q =
//!   844025809322115073
//! * Scalar field: r =
//!   211006452744585217
//! * Curve equation: ax^2 + y^2 =1 + dx^2y^2, where
//!    * a = 5
//!    * d = 579073710274753001

#[cfg(feature = "curve")]
mod curves;

#[cfg(feature = "scalar_field")]
mod fields;

#[cfg(feature = "curve")]
pub use curves::*;

#[cfg(feature = "scalar_field")]
pub use fields::*;