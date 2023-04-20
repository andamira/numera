// numera::lib
//
//! Numbers allows us to express and compare magnitudes and quantities.
//!
//

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
#![cfg_attr(not(feature = "std"), no_std)]

pub mod error;
pub mod number;

#[doc(inline)]
pub use number::{integer::Integer, traits::Number};

/// Everything is directly available in here.
pub mod all {
    #[doc(inline)]
    pub use super::{error::*, number::all::*};
}
