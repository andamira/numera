// numera::lib
//
//! Numbers allows us to express and compare magnitudes and quantities.
//!
//

#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(
    clippy::enum_glob_use,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::needless_return,
    clippy::redundant_closure_for_method_calls,
    clippy::wildcard_imports
)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "nightly", feature(doc_cfg))]

pub mod error;
pub mod number;

#[doc(inline)]
pub use number::{integer::Integer, traits::Number};

/// Everything is directly available in here.
pub mod all {
    #[doc(inline)]
    pub use super::{error::*, number::all::*};
}
