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

mod define_sized;
mod family;
mod impl_from;
mod impl_integer;
mod impl_ops;

#[cfg(feature = "ibig")]
mod define_big;

pub use define_sized::{Integer128, Integer16, Integer32, Integer64, Integer8};
pub use family::Integers;

#[cfg(feature = "ibig")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "ibig")))]
pub use define_big::IntegerBig;

use crate::number::macros::define_abbreviations;
define_abbreviations![many Z, Integer, 8, 16, 32, 64, 128];

/// Abbreviation of [`IntegerBig`].
#[cfg(feature = "ibig")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "ibig")))]
pub type ZBig = IntegerBig;
