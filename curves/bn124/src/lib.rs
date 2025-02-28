#![cfg_attr(not(feature = "std"), no_std)]
#![deny(
    warnings,
    unused,
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms
)]
#![forbid(unsafe_code)]

//! This library implements a Barreto-Naehring curve of embedding degree 12,
//! defined over a 124-bit (prime) field with a 124-bit scalar field. The scalar field is highly 2-adic.
//!
//! #CAUTION
//! **This curve does not satisfy the 128-bit security level**
//!
//!
//! Curve information:
//! * Base field: q =
//!   17000133324792832063019019729102503239
//! * Scalar field: r =
//!   17000133324792832058895897937997463553
//! * valuation(q - 1, 2) = 1
//! * valuation(r - 1, 2) = 25
//! * G1 curve equation: y^2 = x^3 + 3
//! * G2 curve equation: y^2 = x^3 + B, where
//!    * B = 3/(u+5) where Fq2 is represented as Fq\[u\]/(u^2+1)

#[cfg(feature = "curve")]
mod curves;

mod fields;

#[cfg(feature = "curve")]
pub use curves::*;

pub use fields::*;

#[cfg(feature = "r1cs")]
pub mod constraints;
