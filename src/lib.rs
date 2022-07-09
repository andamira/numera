// numera::lib
//
//! Numbers allows us to express and compare magnitudes and quantities.
//!
//!

#![cfg_attr(not(feature = "std"), no_std)]

pub mod integer;
pub mod rational;
pub mod traits;

#[doc(inline)]
pub use {integer::Integer, rational::Rational, traits::Number};
