// numera::number::integer::nnz
//
//! *Non-negative* integer numbers ([m][0m]/[o][0o]), from the set $\Z^*$
//! ($\N _0$) (`>= 0`).
//!
//! $ \Z^* = \lbrace 0, 1, 2, … \rbrace $ ([oeis])
//!
//! This type corresponds with the *unsigned* primitives ([`u8`]…[`u128`]).
//!
//! In this library they are also known as *natural numbers* ($\N _0$)
//! ([m][1m]/[w][1w]). In ISO 80000-2:2019 they are also known as $\N$ ([I][2i]).
//!
//! [0m]: https://mathworld.wolfram.com/NonnegativeInteger.html
//! [0o]: https://oeis.org/wiki/Nonnegative_integers
//! [oeis]: http://oeis.org/A001477
//! [1m]: https://mathworld.wolfram.com/NaturalNumber.html
//! [1w]: https://en.wikipedia.org/wiki/Natural_number
//! [2i]: https://en.wikipedia.org/wiki/ISO/IEC_80000#cite_note-80000-2:2019-4
//

mod define_sized;
mod family;
mod impl_from;
mod impl_integer;
mod ops;

pub use define_sized::{
    NonNegativeInteger128, NonNegativeInteger16, NonNegativeInteger32, NonNegativeInteger64,
    NonNegativeInteger8,
};
pub use family::NonNegativeIntegers;

use crate::number::macros::{define_abbreviations, define_aliases};
define_abbreviations![many Nnz, NonNegativeInteger, 8, 16, 32, 64, 128];
define_aliases![many Natural, NonNegativeInteger, 8, 16, 32, 64, 128];
