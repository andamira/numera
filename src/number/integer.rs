// numera::number::integer
//
//!
//

//! An *integer* ([w][0w]/[m][0m]) is a number that can be written
//! without a fractional component.
//!
//! For example, 21, 4, 0, and −2048 are integers, while 9.75, 5+1/2, and √2 are not.
//!
//! # Integer subsets
//!
//! *Natural numbers* ([m][1m]), *Counting numbers* ([m][2m]) and *Whole numbers*
//! ([m][3m]) are tradicitonal ambiguous ways to refer to different subsets of
//! integers, without consensus on whether *zero* ([m][4m]) is included in
//! any of those sets.
//!
//! This is why the integer types defined here are named using a more explicit,
//! unambiguous notation.
//!
//! [0w]: https://en.wikipedia.org/wiki/Integer
//! [0m]: https://mathworld.wolfram.com/Integer.html
//! [1m]: https://mathworld.wolfram.com/NaturalNumber.html
//! [2m]: https://mathworld.wolfram.com/NaturalNumber.html
//! [3m]: https://mathworld.wolfram.com/NaturalNumber.html
//! [4m]: https://mathworld.wolfram.com/Zero.html

use core::ops::Neg;
use min_max_traits::{Max, Min};
use num_integer::Integer as NumInt;

/// An `Integer` number ([w][w0]/[m][m0]), from the set $\Z$.
///
/// $ \Z = \lbrace …, -2, -1, 0, 1, 2, … \rbrace $
///
/// This type exactly corresponds to the signed primitives (i8…i128)
///
/// [w0]: https://en.wikipedia.org/wiki/Integer
/// [m0]: https://mathworld.wolfram.com/Integer.html
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Integer<I>(I)
where
    // NumInt already includes:
    // Sized + PartialEq + Eq + PartialOrd + Ord + Zero + One + NumOps + NumCast
    I: NumInt + Max + Min + Neg;

/// A *non-negative* `Integer` number ([m][0m]/[o][0o]), from the set $\Z^*$.
///
/// $ \Z^* = \lbrace 0, 1, 2, … \rbrace $
///
/// Also called *Natural*.
///
/// This type exactly corresponds to the unsigned primitives (u8…u128).
///
/// [0m]: https://mathworld.wolfram.com/NonnegativeInteger.html
/// [0o]: http://oeis.org/A001477
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct NonNegative<I>(I)
where
    I: NumInt + Max + Min;

/// A *positive* `Integer` number ([m][0m]), from the set $\Z^+$.
///
/// $ \Z^+ = \lbrace 1, 2, … \rbrace $
///
/// Doesn't include 0.
///
/// [0m]: https://mathworld.wolfram.com/PositiveInteger.html
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Positive<I>(I)
where
    I: NumInt + Max + Min;

/// A *negative* `Integer` number ([m][0m]/[o][0o]), from the set $\Z^-$.
///
/// $ \Z^- = \lbrace …, -2, -1 \rbrace $
///
/// Doesn't include 0.
///
/// [0m]: https://mathworld.wolfram.com/NegativeInteger.html
/// [0o]: http://oeis.org/A001478
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Negative<I>(I)
where
    I: NumInt + Max + Min + Neg;

/// A *non-positive* `Integer` number ([m][0m]), from the set ${0} \cup \Z^-$.
///
/// $ {0} \cup Z^- = \lbrace …, -2, -1, 0 \rbrace $
///
/// [0m]: https://mathworld.wolfram.com/NonpositiveInteger.html
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct NonPositive<I>(I)
where
    I: NumInt + Max + Min + Neg;

/// implements the Number trait
mod number_impls {
    use super::{Integer, Max, Min, Neg, Negative, NonNegative, NonPositive, NumInt, Positive};
    use crate::number::Number;

    impl<I: NumInt + Max + Min + Neg> Number for Integer<I> {
        type Value = I;
        /// Returns a new `Integer`.
        #[inline]
        fn new(value: Self::Value) -> Self {
            Self(value)
        }
        #[inline]
        fn largest() -> Self {
            Self::new(Self::Value::MAX)
        }
        #[inline]
        fn smallest() -> Self {
            Self::new(Self::Value::MIN)
        }
    }
    impl<I: NumInt + Max + Min> Number for NonNegative<I> {
        type Value = I;
        /// Returns a new *non-negative* `Integer`.
        /// The smallest value saturates to 0.
        #[inline]
        fn new(value: Self::Value) -> Self {
            Self(if value < Self::Value::zero() {
                Self::Value::zero()
            } else {
                value
            })
        }
        #[inline]
        fn largest() -> Self {
            Self::new(Self::Value::MAX)
        }
        #[inline]
        fn smallest() -> Self {
            Self::new(Self::Value::zero())
        }
    }
    impl<I: NumInt + Max + Min> Number for Positive<I> {
        type Value = I;
        /// Returns a new *positive* `Integer`.
        /// The smallest value saturates to 1.
        #[inline]
        fn new(value: Self::Value) -> Self {
            Self(if value < Self::Value::one() {
                Self::Value::one()
            } else {
                value
            })
        }
        #[inline]
        fn largest() -> Self {
            Self::new(Self::Value::MAX)
        }
        #[inline]
        fn smallest() -> Self {
            Self::new(Self::Value::one())
        }
    }
    impl<I: NumInt + Max + Min + Neg> Number for NonPositive<I> {
        type Value = I;
        /// Returns a new *positive* `Integer`.
        /// The largest value Saturates to 0.
        #[inline]
        fn new(value: Self::Value) -> Self {
            Self(if value > Self::Value::zero() {
                Self::Value::zero()
            } else {
                value
            })
        }
        #[inline]
        fn largest() -> Self {
            Self::new(Self::Value::zero())
        }
        #[inline]
        fn smallest() -> Self {
            Self::new(Self::Value::MIN)
        }
    }
    impl<I: NumInt + Max + Min + Neg<Output = I>> Number for Negative<I> {
        type Value = I;
        /// Returns a new *positive* `Integer`.
        /// The largest value saturates to -1.
        #[inline]
        fn new(value: Self::Value) -> Self {
            Self(if value > Self::Value::zero() {
                Self::Value::one().neg()
            } else {
                value
            })
        }
        #[inline]
        fn largest() -> Self {
            Self::new(Self::Value::one().neg())
        }
        #[inline]
        fn smallest() -> Self {
            Self::new(Self::Value::MIN)
        }
    }
}

/// implements std traits: Default, From…
mod std_impls {
    use super::{Integer, Max, Min, Neg, Negative, NonNegative, NonPositive, NumInt, Positive};
    use crate::number::Number;
    use num_traits::{One, Zero};

    use core::hash::{Hash, Hasher};
    use std::fmt;

    // impl Default

    /// Default: 0.
    impl<I: NumInt + Max + Min + Neg> Default for Integer<I> {
        #[inline]
        fn default() -> Self {
            Self(Zero::zero())
        }
    }
    /// Default: 0.
    impl<I: NumInt + Max + Min> Default for NonNegative<I> {
        #[inline]
        fn default() -> Self {
            Self(Zero::zero())
        }
    }
    /// Default: 1.
    impl<I: NumInt + Max + Min> Default for Positive<I> {
        #[inline]
        fn default() -> Self {
            Self(One::one())
        }
    }
    /// Default: -1.
    impl<I: NumInt + Max + Min + Neg<Output = I>> Default for Negative<I> {
        #[inline]
        fn default() -> Self {
            <Self as Number>::largest()
        }
    }
    /// Default: 0.
    impl<I: NumInt + Max + Min + Neg> Default for NonPositive<I> {
        #[inline]
        fn default() -> Self {
            Self::new(Zero::zero())
        }
    }

    /// Derives several traits when the inner type supports them (neg version).
    macro_rules! derive_traits_neg {
        (all: $($int:ident),+) => {
            $( derive_traits_neg!($int); )+
        };
        ($int:ident) => {
            impl<I: NumInt + Max + Min + Neg + Copy> Copy for $int<I> {}
            impl<I: NumInt + Max + Min + Neg + Clone> Clone for $int<I> {
                fn clone(&self) -> Self{ Self(self.0.clone()) }
            }
            // This is OK since both PartialEq & Hash are derived from the inner type:
            #[allow(clippy::derive_hash_xor_eq)]
            impl<I: NumInt + Max + Min + Neg + Hash> Hash for $int<I> {
                fn hash<H: Hasher>(&self, hasher: &mut H) {
                    self.0.hash(hasher);
                }
            }
            impl<I: NumInt + Max + Min + Neg + fmt::Display> fmt::Display for $int<I> {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "{}", self.0)
                }
            }
        };
    }

    /// Derives several traits when the inner type supports them (non-neg version).
    macro_rules! derive_traits_non_neg {
        (all: $($int:ident),+) => {
            $( derive_traits_non_neg!($int); )+
        };
        ($int:ident) => {
            impl<I: NumInt + Max + Min + Copy> Copy for $int<I> {}
            impl<I: NumInt + Max + Min + Clone> Clone for $int<I> {
                fn clone(&self) -> Self{ Self(self.0.clone()) }
            }
            // This is OK since both PartialEq & Hash are derived from the inner type:
            #[allow(clippy::derive_hash_xor_eq)]
            impl<I: NumInt + Max + Min + Hash> Hash for $int<I> {
                fn hash<H: Hasher>(&self, hasher: &mut H) {
                    self.0.hash(hasher);
                }
            }
            impl<I: NumInt + Max + Min + fmt::Display> fmt::Display for $int<I> {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "{}", self.0)
                }
            }
        };
    }

    derive_traits_neg!(all: Integer, Negative, NonPositive);
    derive_traits_non_neg!(all: Positive, NonNegative);
}

#[cfg(test)]
mod tests {
    use super::{Integer, Negative, NonNegative, NonPositive, Positive};
    use crate::number::Number;

    #[test]
    fn number_api() {
        // new

        assert_eq![Negative::new(23), Negative::new(-1)];
        assert_eq![NonPositive::new(23), NonPositive::new(0)];
        assert_eq![NonNegative::new(-23), NonNegative::new(0)];
        assert_eq![Positive::new(-23), Positive::new(1)];

        // scope

        assert_eq![Integer::new(i8::MIN), Integer::<i8>::smallest()];
        assert_eq![Integer::new(i8::MAX), Integer::<i8>::largest()];

        assert_eq![Negative::new(i8::MIN), Negative::<i8>::smallest()];
        assert_eq![Negative::new(-1), Negative::<i8>::largest()];

        assert_eq![NonPositive::new(i8::MIN), NonPositive::<i8>::smallest()];
        assert_eq![NonPositive::new(0), NonPositive::<i8>::largest()];

        assert_eq![Positive::new(1), Positive::<i8>::smallest()];
        assert_eq![Positive::new(i8::MAX), Positive::<i8>::largest()];
        assert_eq![Positive::new(1), Positive::<u8>::smallest()];
        assert_eq![Positive::new(u8::MAX), Positive::<u8>::largest()];

        assert_eq![NonNegative::new(0), NonNegative::<i8>::smallest()];
        assert_eq![NonNegative::new(i8::MAX), NonNegative::<i8>::largest()];
        assert_eq![NonNegative::new(0), NonNegative::<u8>::smallest()];
        assert_eq![NonNegative::new(u8::MAX), NonNegative::<u8>::largest()];
    }
}
