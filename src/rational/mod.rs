// numera::rational
//
//! Rational numbers.
//!
//! A *rational number* is ([0w]/[0m])…
//!
//! [0w]: https://en.wikipedia.org/wiki/Rational_number
//! [0m]: https://mathworld.wolfram.com/RationalNumber.html
//

mod fraction;
pub use fraction::Fraction;

use crate::integer::{Integer, NonNegativeInteger, NonZeroInteger, PositiveInteger};
use crate::traits::{Number, Signed};

/// Acronyms for rational types ([`Q`][a::Q], [`Pq`][a::Pq]).
pub mod a {
    use super::*;
    /// Acronym for [`Rational`].
    pub type Q<N, D> = Rational<N, D>;
    /// Acronym for [`PositiveRational`].
    pub type Pq<N, D> = PositiveRational<N, D>;
}

/// A rational number.
///
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Rational<N, D>
where
    N: Number + Signed,
    D: Number + Signed,
{
    /// The numerator.
    pub num: Integer<N>,
    /// The denominator.
    pub den: NonZeroInteger<D>,
}

/// A variation of a [`Rational`] where the component must always be positive.
pub struct PositiveRational<N, D>
where
    N: Number,
    D: Number,
{
    /// The numerator.
    pub num: NonNegativeInteger<N>,
    /// The denominator.
    pub den: PositiveInteger<D>,
}
