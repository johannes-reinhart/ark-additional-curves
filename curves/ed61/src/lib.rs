#![cfg_attr(not(feature = "std"), no_std)]
#![deny(
warnings,
unused,
future_incompatible,
nonstandard_style,
rust_2018_idioms
)]
#![forbid(unsafe_code)]

//! This library implements the ed61 twisted Edwards curve.
//!
//! Curve information:
//! * Base field: q =
//!   4941944131663429633
//! * Scalar field: r =
//!   1235486033917771777
//! * Curve equation: ax^2 + y^2 =1 + dx^2y^2, where
//!    * a = 5
//!    * d = 2154376507894293213

#[cfg(feature = "curve")]
mod curves;

#[cfg(feature = "scalar_field")]
mod fields;

#[cfg(feature = "curve")]
pub use curves::*;

#[cfg(feature = "scalar_field")]
pub use fields::*;