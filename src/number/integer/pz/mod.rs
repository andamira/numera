// numera::number::integer::pz
//
//! *Positive* integer numbers ([m][0m]), from the set $\Z^+$ ($\N _1$) (`> 0`).
//!
//! $ \Z^+ = \lbrace 1, 2, … \rbrace $
//!
//! Sometimes called *Natural numbers* ([w][1w]) or *counting numbers*,
//! but in that case they could be confused with the
//! [`NonNegativeIntegers`][super::nnz].
//!
//! This type corresponds with the *unsigned* `NonZero` primitives
//! ([`NonZeroU8`][core::num::NonZeroU8]…[`NonZeroU128`][core::num::NonZeroU128]).
//!
//! [0m]: https://mathworld.wolfram.com/PositiveInteger.html
//! [1w]: https://en.wikipedia.org/wiki/Natural_number
//
mod define_sized;
mod family;
mod impl_from;
mod impl_integer;
mod impl_ops;

pub use define_sized::{
    PositiveInteger128, PositiveInteger16, PositiveInteger32, PositiveInteger64, PositiveInteger8,
};
pub use family::PositiveIntegers;

/// Abbreviations for positive integers.
pub mod abbr {
    use super::{super::macros::define_abbreviations, *};

    define_abbreviations![many Pz, PositiveInteger, 8, 16, 32, 64, 128];
}
