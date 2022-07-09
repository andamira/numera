// numera::lib
//
//! Numbers allows us to express and compare magnitudes and quantities.
//!
//! Number definitions.
//!
//! A *number* ([w][1w]/[m][1m]) is a mathematical object used to count, measure,
//! and label. A general term which refers to a member of a given set.
//!
//! [1w]: https://en.wikipedia.org/wiki/Number
//! [1m]: https://mathworld.wolfram.com/Number.html
//!

#![cfg_attr(not(feature = "std"), no_std)]

pub mod integer;
pub mod rational;
pub mod traits;

#[doc(inline)]
pub use {integer::Integer, rational::Rational};
