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
//! scalar field of the curve BN183. This allows defining cryptographic
//! primitives that use elliptic curves over the scalar field of the latter curve.
//!
//! Curve information:
//! * Base field: q =
//!   6804759748846355405830582786011032970784946075266449409
//! * Scalar field: r =
//!   850594968605794425728822847691900677428791383617892397
//! * Valuation(q - 1, 2) = 30
//! * Valuation(r - 1, 2) = 2
//! * Curve equation: ax^2 + y^2 =1 + dx^2y^2, where
//!    * a = 98032
//!    * d = 98028

#[cfg(feature = "r1cs")]
pub mod constraints;

mod curves;
mod fields;

pub use curves::*;
pub use fields::*;
