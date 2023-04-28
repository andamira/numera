// numera::rational
//
//! All kinds of rationals, subsets of $\Bbb{Q}$.
//!
//! A *rational* ([w][0w]/[m][0m]) is a number that can be expressed as a
//! fraction $\frac{a}{b}$ where a and b are integers and $ b \ne 0 $.
//!
// For example, $ 21 , 4 , 0 , −2048 $ are integers,
// while $9.75, \dfrac{1}{2} , \sqrt{2} $ are not.
//!
//! [0w]: https://en.wikipedia.org/wiki/Rational_number
//! [0m]: https://mathworld.wolfram.com/RationalNumber.html
//

// pub(crate) mod macros;

mod family;
mod trait_rational;

// pub mod n0q;
// pub mod nnq;
// pub mod npq;
// pub mod nq;
// pub mod pq;
pub mod q;

/// Abbreviations for all rationals.
pub mod abbr {
    // n0q::abbr::*, nnq::abbr::*, npq::abbr::*, nq::abbr::*, pq::abbr::*, q::abbr::*,
    #[doc(inline)]
    pub use super::q::abbr::*;
}

pub use all::*;
pub(crate) mod all {
    #[doc(inline)]
    pub use super::{family::AnyRationals, q::*, trait_rational::Rational};
}
