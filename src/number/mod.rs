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

pub mod integer;
mod macros;
pub mod traits;

#[doc(inline)]
pub use {
    integer::{
        Integer, NegativeInteger, NonNegativeInteger, NonPositiveInteger, NonZeroInteger,
        PositiveInteger,
    },
    traits::{Bounded, InnerNumber, Number, One, Sign, Zero},
};
