// numera::number::rational::q
//
//! Rational numbers ([w][0w]/[m][0m]), from the set $\Bbb{Q}$.
//!
//! $\Bbb{Q} = \lbrace …, -1, …, \frac{-1}{2}, …, 0, …, \frac{1}{2}, …, 1, … \rbrace$
//!
//! [0w]: https://en.wikipedia.org/wiki/Rational_number
//! [0m]: https://mathworld.wolfram.com/RationalNumber.html
//

use crate::number::macros::define_abbreviations;

mod define_sized;
mod family;
mod from;
mod impl_rational;
mod ops;

pub use define_sized::{Rational128, Rational16, Rational32, Rational64, Rational8};
pub use family::Rationals;

define_abbreviations![many Q, Rational, 8, 16, 32, 64, 128];
