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

mod macros;
mod traits;

pub mod integer;

#[doc(inline)]
pub use {
    integer::{Integer, Negative, NonNegative, NonPositive, Positive},
    traits::{Number, InnerNumber, Sign, Identities, MaxMin},
};
