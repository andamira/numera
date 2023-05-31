// numera::number::integer
//
//! Integer numbers, subsets of $\Z$.
//!
//! An *integer* ([m][0m]/[w][0w]) is a number that can be written
//! without a fractional component.
//!
//! [0m]: https://mathworld.wolfram.com/Integer.html
//! [0w]: https://en.wikipedia.org/wiki/Integer
//!
//!
//! # Integer subsets
//!
//! Different integer implementations are classified mathematically by:
//! - Whether they only have positive sign (`PositiveInteger`, `NonNegativeInteger`),
//!   only negative sign (`NegativeInteger`, `NonPositiveInteger`),
//!   or both (`Integer`, `NonZeroInteger`).
//! - Whether they support 0 (`Integer`, `NonNegativeInteger`, `NonPositiveInteger`)
//!   or not (`NonZeroInteger`, `PositiveInteger`, `NegativeInteger`).
//!
//! This makes for 6 different kinds of integers.
//!
//! They all implement the [`Integer`] trait, and are all variants of the
//! [`Integers`] family enum, which includes also the special `Prime` kind.
//!
//! They are also classified computationally, by 6 choices on their bit-size
//! representation: (`8`, `16`, `32`, `64`, `128` or `Big`). Combined makes for
//! 36 choices of integers. (Except `Big` is only implemented for `Integer`
//! for now).
//!
//! Note that numbers that can represent both positive an negative integers can
//! represent half the max positive or negative value at the same bit-size than
//! their only negative or positive counterparts. E.g.:
//!
// TODO
//! ```ignore
//! # use numera::all::*;
//! assert_eq![[-128, 127], [Integer8::MIN, Integer8::MAX]];
//! assert_eq![[-255, 255], [NonPositiveInteger8::MIN.into(), NonNegativeInteger8::MAX.into()]];
//! ```
//!
//! # Operations
//!
//! Not all types have the same support for arithmetic operations, specially if
//! we want the operation to return an integer of the same type.
//!
//! We say that a group is closed over an operation when that operation will
//! always return a number of the same type. the following table summarizes
//! the closed operations for each kind of integer:
//!
//! ||[`Integer`][z]|[`NonZero`][n0z]|[`NonNegative`][nnz]|[`Positive`][pz]|[`Nonpositive`][npz]|[`Negative`][nz]|[`Prime`][prime]|
//! |:-:|:------:|:----------:|:----------:|:--------:|:----------:|:--------:|:----------:|
//! |sign|  + -  |     + -    |     +      |     +    |      -     |     -    |      +     |
//! |zero|   0   |            |     0      |          |      0     |          |            |
//! ||
//! | add|  yes  |            |    yes     |    yes   |     yes    |   yes    |            |
//! | sub|  yes  |            |            |          |            |          |            |
//! | mul|  yes  |    yes     |    yes     |    yes   |            |          |            |
//! | div|  yes  |    yes     |    yes     |    yes   |            |          |            |
//! | rem|  yes  |            |    yes     |          |     yes    |          |            |
//! | neg|  yes  |    yes     |            |          |            |          |            |
//!
//

pub(crate) mod macros;

mod family;
mod fns;
mod integers;
pub mod prime;

pub mod n0z;
pub mod nnz;
pub mod npz;
pub mod nz;
pub mod pz;
pub mod z;

pub use all::*;
pub(crate) mod all {
    #[doc(inline)]
    pub use super::{
        family::*, integers::Integers, n0z::*, nnz::*, npz::*, nz::*, prime::*, pz::*,
        z::*,
    };

    #[doc(inline)]
    #[cfg(feature = "ibig")]
    pub use super::{
        fns::{bit_len, bit_len_next_power},
        z::IntegerBig,
    };
}
