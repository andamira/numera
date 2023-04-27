// numera::number::integer::nz
//
//! *Negative* integer numbers ([m][0m]/[o][0o]), from the set $\Z^-$
//! (`< 0`).
//!
//! $ \Z^- = \lbrace …, -2, -1 \rbrace $
//!
//! Doesn't include 0.
//!
//! [0m]: https://mathworld.wolfram.com/NegativeInteger.html
//! [0o]: http://oeis.org/A001478
//! An only *positive* integer number ([m][0m]), from the set $\Z^+$ (`> 0`).
//!
//
mod define_sized;
mod family;
mod impl_from;
mod impl_integer;
mod impl_ops;

pub use define_sized::{
    NegativeInteger128, NegativeInteger16, NegativeInteger32, NegativeInteger64, NegativeInteger8,
};
pub use family::NegativeIntegers;

/// Abbreviations for negative integers.
pub mod abbr {
    use super::{super::macros::define_abbreviations, *};

    define_abbreviations![many Nz, NegativeInteger, 8, 16, 32, 64, 128];
}