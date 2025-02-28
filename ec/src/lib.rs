#![cfg_attr(not(feature = "std"), no_std)]
#![warn(
    unused,
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms,
    rust_2021_compatibility
)]
#![forbid(unsafe_code)]
#![allow(
    clippy::op_ref,
    clippy::suspicious_op_assign_impl,
    clippy::many_single_char_names
)]
#![doc = include_str!("../README.md")]

#[macro_use]
extern crate ark_std;

pub use ark_ec::scalar_mul::{variable_base::VariableBaseMSM, ScalarMul};

pub use ark_ff::AdditiveGroup;

pub mod models;
pub use self::models::*;
