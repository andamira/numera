// numera::number::integer::z
//
//! Integer numbers ([m][0m]/[o][0o]/[w][0w]), from the set $\Z$.
//!
//! $ \Z = \lbrace …, -2, -1, 0, 1, 2, … \rbrace $
//!
//! This type corresponds with the *signed* primitives ([`i8`]…[`i128`]).
//!
//! [0m]: https://mathworld.wolfram.com/Integer.html
//! [0o]: https://oeis.org/wiki/Integers
//! [0w]: https://en.wikipedia.org/wiki/Integer
//

mod convert;
mod family;
mod integer;
mod ops;
mod sized;

#[cfg(feature = "dashu-int")]
mod big;

pub use family::Integer;
pub use sized::{Integer128, Integer16, Integer32, Integer64, Integer8};

#[cfg(feature = "dashu-int")]
#[cfg_attr(
    feature = "nightly",
    doc(cfg(any(feature = "big", feature = "dashu-int")))
)]
pub use big::IntegerBig;

use crate::number::macros::define_abbreviations;
define_abbreviations![Z, Integer, 8, 16, 32, 64, 128];

/// Abbreviation of [`IntegerBig`].
#[cfg(feature = "dashu-int")]
#[cfg_attr(
    feature = "nightly",
    doc(cfg(any(feature = "big", feature = "dashu-int")))
)]
pub type ZBig = IntegerBig;
