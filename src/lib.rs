// numera::lib
//
#![doc = include_str!("./Lib.md")]
//

// warnings
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(
    clippy::doc_markdown,
    clippy::enum_glob_use,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::needless_return,
    clippy::redundant_closure_for_method_calls,
    clippy::wildcard_imports
)]
// environment
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "safe", forbid(unsafe_code))]
#![cfg_attr(feature = "nightly", feature(doc_cfg))]
#[cfg(feature = "alloc")]
extern crate alloc;

// safeguards
#[cfg(all(feature = "std", feature = "no_std"))]
compile_error!("You can't enable the `std` and `no_std` features at the same time.");
#[cfg(all(feature = "safe", feature = "unsafe"))]
compile_error!("You can't enable the `safe` and `unsafe` features at the same time.");
// deprecated
devela::deprecate_feature![old: "no-std", new: "no_std", since: "0.4.0"];
devela::deprecate_feature![old: "non-safe", new: "unsafe", since: "0.4.0"];
devela::deprecate_feature![old: "all", new: "full", since: "0.5.0"];

pub mod error;
pub mod number;

/// All items are reexported here.
pub mod all {
    #[doc(inline)]
    pub use super::{error::*, number::all::*};
}
