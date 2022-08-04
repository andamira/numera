// numera::lib
//
//! Numbers allows us to express and compare magnitudes and quantities.
//!
//!

#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(
    clippy::float_arithmetic,
    clippy::implicit_return,
    clippy::needless_return,
    clippy::blanket_clippy_restriction_lints,
    clippy::pattern_type_mismatch,
    clippy::module_name_repetitions,
    clippy::wildcard_imports,
    clippy::enum_glob_use,
    clippy::must_use_candidate
)]
#![forbid(unsafe_code)]
#![cfg_attr(not(feature = "std"), no_std)]

pub(crate) mod macros;

pub mod error;
pub mod integer;
pub mod rational;
pub mod real;
pub mod traits;

// #[doc(inline)]
// pub use {error::Result, integer::Integer, rational::Rational, real::Real, traits::Number};
