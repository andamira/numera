// numera::number::integer
//
//! An *integer* ([w][0w]/[m][0m]) is a number that can be written
//! without a fractional component.
//!
//! For example, 21, 4, 0, and −2048 are integers, while 9.75, 5+1/2, and √2 are not.
//!
//! # Integer subsets
//!
//! *Natural numbers* ([w][1w]/[m][1m]), *Counting numbers* ([m][2m]) and *Whole numbers*
//! ([m][3m]) are tradicitonal ambiguous ways to refer to different subsets of
//! integers, without consensus on whether *zero* ([m][4m]) is included in
//! any of those sets.
//!
//! This is why the integer types defined here are named using a more explicit,
//! unambiguous notation.
//!
//! [0w]: https://en.wikipedia.org/wiki/Integer
//! [0m]: https://mathworld.wolfram.com/Integer.html
//! [1w]: https://en.wikipedia.org/wiki/Natural_number
//! [1m]: https://mathworld.wolfram.com/NaturalNumber.html
//! [2m]: https://mathworld.wolfram.com/NaturalNumber.html
//! [3m]: https://mathworld.wolfram.com/NaturalNumber.html
//! [4m]: https://mathworld.wolfram.com/Zero.html
//

use crate::number::InnerNumber;

/// An integer number ([w][w0]/[m][m0]), from the set $\Z$.
///
/// $ \Z = \lbrace …, -2, -1, 0, 1, 2, … \rbrace $
///
/// This type perfectly encapsulates the signed primitives ([`i8`] … [`i128`]).
///
/// [w0]: https://en.wikipedia.org/wiki/Integer
/// [m0]: https://mathworld.wolfram.com/Integer.html
#[repr(transparent)]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Integer<I: InnerNumber>(I);

/// A *non-negative* integer number ([m][0m]/[o][0o]), from the set $\Z^*$.
///
/// $ \Z^* = \lbrace 0, 1, 2, … \rbrace $
///
/// Sometimes called *Natural number* ([w][1w]).
///
/// This type exactly corresponds to the unsigned primitives (u8…u128).
///
/// [0m]: https://mathworld.wolfram.com/NonnegativeInteger.html
/// [0o]: http://oeis.org/A001477
/// [1w]: https://en.wikipedia.org/wiki/Natural_number
#[repr(transparent)]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct NonNegative<I: InnerNumber>(I);

/// A *positive* integer number ([m][0m]), from the set $\Z^+$.
///
/// $ \Z^+ = \lbrace 1, 2, … \rbrace $
///
/// Doesn't include 0.
///
/// Sometimes called *Natural number* ([w][1w]).
///
/// [0m]: https://mathworld.wolfram.com/PositiveInteger.html
/// [1w]: https://en.wikipedia.org/wiki/Natural_number
#[repr(transparent)]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Positive<I: InnerNumber>(I);

/// A *negative* integer number ([m][0m]/[o][0o]), from the set $\Z^-$.
///
/// $ \Z^- = \lbrace …, -2, -1 \rbrace $
///
/// Doesn't include 0.
///
/// [0m]: https://mathworld.wolfram.com/NegativeInteger.html
/// [0o]: http://oeis.org/A001478
#[repr(transparent)]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Negative<I: InnerNumber>(I);

/// A *non-positive* integer number ([m][0m]), from the set ${0} \cup \Z^-$.
///
/// $ {0} \cup Z^- = \lbrace …, -2, -1, 0 \rbrace $
///
/// [0m]: https://mathworld.wolfram.com/NonpositiveInteger.html
#[repr(transparent)]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct NonPositive<I: InnerNumber>(I);

/// implements the Number trait
mod number_impls {
    use super::{Integer, Negative, NonNegative, NonPositive, Positive};
    use crate::number::{InnerNumber, Number};
    use core::ops::Neg;

    #[rustfmt::skip]
    impl<I: InnerNumber> Number for Integer<I> {
        type Inner = I;
        /// Returns a new `Integer`.
        #[inline]
        fn new(value: Self::Inner) -> Self { Self(value) }
        #[inline]
        fn new_max() -> Self { Self::new(Self::Inner::MAX) }
        #[inline]
        fn new_min() -> Self { Self::new(Self::Inner::MIN) }
        #[inline]
        fn can_negative() -> bool { true }
        #[inline]
        fn is_negative(&self) -> bool { self.0.is_negative() }
        #[inline]
        fn can_positive() -> bool { true }
        #[inline]
        fn is_positive(&self) -> bool { self.0.is_positive() }
        #[inline]
        fn can_zero() -> bool { true }
        #[inline]
        fn is_zero(&self) -> bool { self.0.is_zero() }
        #[inline]
        fn can_one() -> bool { true }
        #[inline]
        fn is_one(&self) -> bool { self.0.is_one() }
    }

    #[rustfmt::skip]
    impl<I: InnerNumber> Number for NonNegative<I> {
        type Inner = I;
        /// Returns a new *non-negative* `Integer`.
        /// The smallest value saturates to `0`.
        #[inline]
        fn new(value: Self::Inner) -> Self {
            Self(if value < Self::Inner::zero() {
                Self::Inner::zero()
            } else {
                value
            })
        }
        #[inline]
        fn new_max() -> Self { Self::new(Self::Inner::MAX) }
        #[inline]
        fn new_min() -> Self { Self::new(Self::Inner::zero()) }
        #[inline]
        fn can_negative() -> bool { false }
        #[inline]
        fn is_negative(&self) -> bool { false }
        #[inline]
        fn can_positive() -> bool { true }
        #[inline]
        fn is_positive(&self) -> bool { self.0.is_positive() }
        #[inline]
        fn can_zero() -> bool { true }
        #[inline]
        fn is_zero(&self) -> bool { self.0.is_zero() }
        #[inline]
        fn can_one() -> bool { true }
        #[inline]
        fn is_one(&self) -> bool { self.0.is_one() }
    }

    #[rustfmt::skip]
    impl<I: InnerNumber> Number for Positive<I> {
        type Inner = I;
        /// Returns a new *positive* `Integer`.
        /// The smallest value saturates to `1`.
        #[inline]
        fn new(value: Self::Inner) -> Self {
            Self(if value < Self::Inner::one() {
                Self::Inner::one()
            } else {
                value
            })
        }
        #[inline]
        fn new_max() -> Self { Self::new(Self::Inner::MAX) }
        #[inline]
        fn new_min() -> Self { Self::new(Self::Inner::one()) }
        #[inline]
        fn can_negative() -> bool { false }
        #[inline]
        fn is_negative(&self) -> bool { false }
        #[inline]
        fn can_positive() -> bool { true }
        #[inline]
        fn is_positive(&self) -> bool { self.0.is_positive() }
        #[inline]
        fn can_zero() -> bool { false }
        #[inline]
        fn is_zero(&self) -> bool { false }
        #[inline]
        fn can_one() -> bool { true }
        #[inline]
        fn is_one(&self) -> bool { self.0.is_one() }
    }

    #[rustfmt::skip]
    impl<I: InnerNumber> Number for NonPositive<I> {
        type Inner = I;
        /// Returns a new *non-positive* `Integer`.
        /// The largest value Saturates to `0`.
        #[inline]
        fn new(value: Self::Inner) -> Self {
            Self(if value > Self::Inner::zero() {
                Self::Inner::zero()
            } else {
                value
            })
        }
        #[inline]
        fn new_max() -> Self { Self::new(Self::Inner::zero()) }
        #[inline]
        fn new_min() -> Self { Self::new(Self::Inner::MIN) }
        #[inline]
        fn can_negative() -> bool { true }
        #[inline]
        fn is_negative(&self) -> bool { self.0.is_negative() }
        #[inline]
        fn can_positive() -> bool { false }
        #[inline]
        fn is_positive(&self) -> bool { false }
        #[inline]
        fn can_zero() -> bool { true }
        #[inline]
        fn is_zero(&self) -> bool { self.0.is_zero() }
        #[inline]
        fn can_one() -> bool { false }
        #[inline]
        fn is_one(&self) -> bool { false }
    }

    #[rustfmt::skip]
    impl<I: InnerNumber + Neg<Output = I>> Number for Negative<I> {
        type Inner = I;
        /// Returns a new *negative* `Integer`.
        /// The largest value saturates to `-1`.
        #[inline]
        fn new(value: Self::Inner) -> Self {
            Self(if value >= Self::Inner::zero() {
                Self::Inner::one().neg()
            } else {
                value
            })
        }
        #[inline]
        fn new_max() -> Self { Self::new(Self::Inner::one().neg()) }
        #[inline]
        fn new_min() -> Self { Self::new(Self::Inner::MIN) }
        #[inline]
        fn can_negative() -> bool { true }
        #[inline]
        fn is_negative(&self) -> bool { true }
        #[inline]
        fn can_positive() -> bool { false }
        #[inline]
        fn is_positive(&self) -> bool { false }
        #[inline]
        fn can_zero() -> bool { false }
        #[inline]
        fn is_zero(&self) -> bool { false }
        #[inline]
        fn can_one() -> bool { false }
        #[inline]
        fn is_one(&self) -> bool { false }
    }
}

/// implements std traits: Default, From…
mod std_impls {
    use super::{Integer, Negative, NonNegative, NonPositive, Positive};
    use crate::number::{InnerNumber, Number};
    use core::ops::Neg;

    use core::hash::{Hash, Hasher};
    use std::fmt;

    // impl Default

    /// Default: 0.
    impl<I: InnerNumber> Default for Integer<I> {
        #[inline]
        fn default() -> Self {
            Self(I::zero())
        }
    }
    /// Default: 0.
    impl<I: InnerNumber> Default for NonNegative<I> {
        #[inline]
        fn default() -> Self {
            Self(I::zero())
        }
    }
    /// Default: 1.
    impl<I: InnerNumber> Default for Positive<I> {
        #[inline]
        fn default() -> Self {
            Self(I::one())
        }
    }
    /// Default: 0.
    impl<I: InnerNumber> Default for NonPositive<I> {
        #[inline]
        fn default() -> Self {
            Self::new(I::zero())
        }
    }
    /// Default: -1.
    impl<I: InnerNumber + Neg<Output = I>> Default for Negative<I> {
        #[inline]
        fn default() -> Self {
            Self(I::one().neg())
        }
    }

    // impl Copy, clone, Hash, Display

    /// Derives utility traits when the inner type supports them.
    macro_rules! derive_std_traits {
        (all: $($int:ident),+) => {
            $( derive_std_traits!($int); )+
        };
        ($int:ident) => {
            impl<I: InnerNumber + Copy> Copy for $int<I> {}
            impl<I: InnerNumber + Clone> Clone for $int<I> {
                fn clone(&self) -> Self{ Self(self.0.clone()) }
            }
            // This is OK since both PartialEq & Hash are derived from the inner type:
            #[allow(clippy::derive_hash_xor_eq)]
            impl<I: InnerNumber + Hash> Hash for $int<I> {
                fn hash<H: Hasher>(&self, hasher: &mut H) {
                    self.0.hash(hasher);
                }
            }
            impl<I: InnerNumber + fmt::Display> fmt::Display for $int<I> {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "{}", self.0)
                }
            }
        };
    }
    derive_std_traits!(all: Integer, Negative, NonPositive, Positive, NonNegative);
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

        assert_eq![Integer::new(i8::MIN), Integer::<i8>::new_min()];
        assert_eq![Integer::new(i8::MAX), Integer::<i8>::new_max()];

        assert_eq![Negative::new(i8::MIN), Negative::<i8>::new_min()];
        assert_eq![Negative::new(-1), Negative::<i8>::new_max()];

        assert_eq![NonPositive::new(i8::MIN), NonPositive::<i8>::new_min()];
        assert_eq![NonPositive::new(0), NonPositive::<i8>::new_max()];

        assert_eq![Positive::new(1), Positive::<i8>::new_min()];
        assert_eq![Positive::new(i8::MAX), Positive::<i8>::new_max()];
        assert_eq![Positive::new(1), Positive::<u8>::new_min()];
        assert_eq![Positive::new(u8::MAX), Positive::<u8>::new_max()];

        assert_eq![NonNegative::new(0), NonNegative::<i8>::new_min()];
        assert_eq![NonNegative::new(i8::MAX), NonNegative::<i8>::new_max()];
        assert_eq![NonNegative::new(0), NonNegative::<u8>::new_min()];
        assert_eq![NonNegative::new(u8::MAX), NonNegative::<u8>::new_max()];

        //
    }
}
