// numera::number::integer::pz
//
//! *Positive* integer numbers ([m][0m]/[o][0o]), from the set $\Z^+$ ($\N _1$) (`> 0`).
//!
//! $ \Z^+ = \lbrace 1, 2, … \rbrace $ ([oeis])
//!
//! This type corresponds with the *unsigned non-zero* primitives
//! ([`NonZeroU8`][core::num::NonZeroU8]…[`NonZeroU128`][core::num::NonZeroU128]).
//!
//! In this library they are also known as *counting numbers* ($\N _1$)
//! ([m][1m]). In ISO 80000-2:2019 they are also known as $\N^*$ ([I][2i]).
//!
//! [0m]: https://mathworld.wolfram.com/PositiveInteger.html
//! [0o]: https://oeis.org/wiki/Positive_integers
//! [oeis]: https://oeis.org/A000027
//! [1m]: https://mathworld.wolfram.com/CountingNumber.html
//! [2i]: https://en.wikipedia.org/wiki/ISO/IEC_80000#cite_note-80000-2:2019-4
//

mod convert;
mod family;
mod integer;
mod ops;
mod sized;

pub use sized::{
    PositiveInteger128, PositiveInteger16, PositiveInteger32, PositiveInteger64, PositiveInteger8,
};
pub use family::PositiveIntegers;

use crate::number::macros::{define_abbreviations, define_aliases};
define_abbreviations![many Pz, PositiveInteger, 8, 16, 32, 64, 128];
define_aliases![many Counting, PositiveInteger, 8, 16, 32, 64, 128];
