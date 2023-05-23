// numera::number::integer::nz
//
//! *Negative* integer numbers ([m][0m]/[o][0o]), from the set $\Z^-$
//! (`< 0`).
//!
//! $ \Z^- = \lbrace …, -2, -1 \rbrace $ ([oeis])
//!
//! Doesn't include 0.
//!
//! [0m]: https://mathworld.wolfram.com/NegativeInteger.html
//! [0o]: https://oeis.org/wiki/Negative_integers
//! [oeis]: http://oeis.org/A001478
//

mod define_sized;
mod family;
mod from;
mod impl_integer;
mod ops;

pub use define_sized::{
    NegativeInteger128, NegativeInteger16, NegativeInteger32, NegativeInteger64, NegativeInteger8,
};
pub use family::NegativeIntegers;

use crate::number::macros::define_abbreviations;
define_abbreviations![many Nz, NegativeInteger, 8, 16, 32, 64, 128];
