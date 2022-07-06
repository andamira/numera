// numera::number
//
//! Number definitions.
//!
//! A *number* ([w][1w]/[m][1m]) is a mathematical object used to count, measure,
//! and label. A general term which refers to a member of a given set.
//!
//! [1w]: https://en.wikipedia.org/wiki/Number
//! [1m]: https://mathworld.wolfram.com/Number.html
//!

#![allow(non_camel_case_types)]

use num_traits::{One, Zero};

mod macros;

pub mod integer;
pub mod rational;
pub mod real;

#[doc(inline)]
pub use {
    integer::{Integer, Negative, NonNegative, NonPositive, Positive},
    real::Real,
};

/// A common trait for all numbers.
pub trait Number {
    /// The inner numeric value must support the numeric identities.
    type Value: One + Zero;

    /// Returns a new number of the current type.
    ///
    /// This method must ensure the inner value is in a correct format.
    fn new(value: Self::Value) -> Self;

    /// Returns the largest number that can be represented with the current type.
    fn largest() -> Self;

    /// Returns the smallest number that can be represented with the current type.
    fn smallest() -> Self;
}
