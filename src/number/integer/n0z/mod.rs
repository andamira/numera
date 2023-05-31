// numera::number::integer::n0z
//
//! *Non-zero* integer numbers ([o][0o]), from the set $\Z \setminus 0$
//! (`!= 0`).
//!
//! $ \Z = \lbrace …, -2, -1, 1, 2, … \rbrace $
//!
//! This type corresponds with the *signed non-zero* primitives
//! ([`NonZeroI8`][core::num::NonZeroI8]…[`NonZeroI128`][core::num::NonZeroI128]).
//!
//! [0o]: https://oeis.org/wiki/Nonzero_integers
//

mod convert;
mod family;
mod integer;
mod ops;
mod sized;

pub use family::NonZeroInteger;
pub use sized::{
    NonZeroInteger128, NonZeroInteger16, NonZeroInteger32, NonZeroInteger64, NonZeroInteger8,
};

use crate::number::macros::define_abbreviations;
define_abbreviations![N0z, NonZeroInteger, 8, 16, 32, 64, 128];
