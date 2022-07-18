// numera::rational
//
//! Rational numbers.
//!
//! A *rational number* is ([0w]/[0m])…
//!
//! [0w]: https://en.wikipedia.org/wiki/Rational_number
//! [0m]: https://mathworld.wolfram.com/RationalNumber.html
//

use crate::integer::{Integer, NonZeroInteger};
use crate::traits::{Number, Signed};

mod impl_number;

/// Acronyms for rational types ([`Q`][a::Q]).
// /// Acronyms for rational types ([`Q`][a::Q], [`Pq`][a::Pq]).
pub mod a {
    use super::*;
    /// Acronym for [`Rational`].
    pub type Q<N, D> = Rational<N, D>;

    // /// Acronym for [`PositiveRational`].
    // pub type Pq<N, D> = PositiveRational<N, D>;
}

/// A rational number, from the set $\Bbb{Q}$.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Rational<N, D>
where
    N: Number + Signed,
    D: Number + Signed,
{
    /// The numerator.
    pub num: Integer<N>, // <=> 0

    /// The denominator.
    pub den: NonZeroInteger<D>, // != 0
    // pub den: PositiveInteger<D>, // > 0 TODO
}

// /// A variation of a [`Rational`] where the component must always be positive.
// #[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
// pub struct PositiveRational<N, D>
// where
//     N: Number,
//     D: Number,
// {
//     /// The numerator.
//     pub num: NonNegativeInteger<N>, // >= 0
//     /// The denominator.
//     pub den: PositiveInteger<D>, // > 0
// }

// /// A variation of a [`Rational`] where the component must always be negative.
// #[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
// pub struct NegativeRational<N, D>
// where
//     N: Number,
//     D: Number,
// {
//     /// The numerator.
//     pub num: NonPositiveInteger<N>, // <= 0
//     /// The denominator.
//     pub den: NegativeInteger<D>, // < 0
// }
