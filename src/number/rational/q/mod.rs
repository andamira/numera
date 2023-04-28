// numera::number::rational::q
//
//! Rational numbers ([w][0w]/[m][0m]), from the set $\Bbb{Q}$.
//!
//! $\Bbb{Q} = \lbrace …, -1, …, \frac{-1}{2}, …, 0, …, \frac{1}{2}, …, 1, … \rbrace$
//!
//! [0w]: https://en.wikipedia.org/wiki/Rational_number
//! [0m]: https://mathworld.wolfram.com/RationalNumber.html
//

mod define_sized;
mod family;
// mod impl_from; // TODO
mod impl_rational;
// mod impl_ops; // TODO

pub use define_sized::{Rational128, Rational16, Rational32, Rational64, Rational8};
pub use family::Rationals;

/// Abbreviations for rationals.
pub mod abbr {
    use super::*;
    use crate::number::macros::define_abbreviations;

    define_abbreviations![many Q, Rational, 8, 16, 32, 64, 128];
}
