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
use crate::traits::NumberAble;

/// A rational number.
///
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Rational<N, D>
where
    N: NumberAble,
    D: NumberAble,
{
    /// The numerator.
    pub num: Integer<N>,
    /// The denominator.
    pub den: NonZeroInteger<D>,
}

/// A variation of a [`Rational`] where the component must always be positive.
pub struct PositiveRational<N, D>
where
    N: NumberAble,
    D: NumberAble,
{
    /// The numerator.
    pub num: NonNegativeInteger<N>,
    /// The denominator.
    pub den: PositiveInteger<D>,
}
