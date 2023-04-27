// numera::number::integer::npz
//
//! *Non-positive* integer numbers ([m][0m]), from the set $\Z^- \cup {0}$
//! (`<= 0`).
//!
//! $ {0} \cup Z^- = \lbrace …, -2, -1, 0 \rbrace $
//!
//! [0m]: https://mathworld.wolfram.com/NonpositiveInteger.html
//

mod define_sized;
mod family;
mod impl_from;
mod impl_integer;
mod impl_ops;

pub use define_sized::{
    NonPositiveInteger128, NonPositiveInteger16, NonPositiveInteger32, NonPositiveInteger64,
    NonPositiveInteger8,
};
pub use family::NonPositiveIntegers;

/// Abbreviations for non-positive integers.
pub mod abbr {
    use super::{super::macros::define_abbreviations, *};

    define_abbreviations![many Npz, NonPositiveInteger, 8, 16, 32, 64, 128];
}