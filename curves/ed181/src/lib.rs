#![cfg_attr(not(feature = "std"), no_std)]
#![deny(
    warnings,
    unused,
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms
)]
#![forbid(unsafe_code)]

//! This library implements the ed181 twisted Edwards curve.
//!
//! Curve information:
//! * Base field: q =
//!   6210044120409721004947206240885978274523751269793792001
//! * Scalar field: r =
//!   1552511030102430251236801561344621993261920897571225601
//! * Curve equation: ax^2 + y^2 =1 + dx^2y^2, where
//!    * a = 1
//!    * d = 600581931845324488256649384912508268813600056237543024

#[cfg(feature = "curve")]
mod curves;

#[cfg(feature = "scalar_field")]
mod fields;

#[cfg(feature = "curve")]
pub use curves::*;

#[cfg(feature = "scalar_field")]
pub use fields::*;
