//! An integer [] is a number that can be written without a fractional component.
//!
//! For example, 21, 4, 0, and −2048 are integers, while 9.75, 5+1/2, and √2 are not.
//!
//! [0w]: https://en.wikipedia.org/wiki/Integer

use core::ops::Neg;
use num_integer::Integer as NumInt;

/// An `Integer` number ([w][w1]/[m][m1]),
/// from the set $\Z \lbrace …, -2, -1, 0, 1, 2, … \rbrace $.
///
/// [w1]: https://en.wikipedia.org/wiki/Integer
/// [m1]: https://mathworld.wolfram.com/Integer.html
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Integer<I>(I)
where
    // NumInt already includes:
    // Sized + PartialEq + Eq + PartialOrd + Ord + Zero + One + NumOps + NumCast
    I: NumInt + Neg;

impl<I: NumInt + Neg> Integer<I> {
    /// Returns a new `Integer` number.
    pub fn new(value: I) -> Self {
        Self(value)
    }

    /// Returns a new `Integer` number,
    /// where a `value` of `0` is converted to `1`.
    pub fn new_0to1(value: I) -> Self {
        let value = if value == I::zero() { I::one() } else { value };
        Self(value)
    }

    /// Returns a new `Integer`,
    /// but only if `value` $!= 0$.
    pub fn new_nonzero(value: I) -> Option<Self> {
        if value == I::zero() {
            None
        } else {
            Some(Self(value))
        }
    }

    /// If the inner value is `0` then it is changed to `1`, and returns `true`.
    ///
    /// Returns `false` otherwise.
    pub fn let0be1(&mut self) -> bool {
        if self.0.is_zero() {
            self.0 = I::one();
            true
        } else {
            false
        }
    }

    /// If the inner value is `1` then it is changed to `0`, and returns `true`.
    ///
    /// Returns `false` otherwise.
    pub fn let1be0(&mut self) -> bool {
        if self.0.is_zero() {
            self.0 = I::one();
            true
        } else {
            false
        }
    }
}

impl<I: NumInt + Neg + Clone> Integer<I> {
    /// Returns a clone of the underlying data type.
    pub fn get(&self) -> I {
        self.0.clone()
    }
}

mod traits_implementations {
    use super::{Integer, Neg, NumInt};
    use core::hash::{Hash, Hasher};
    use num_traits::{One, Zero};
    use std::fmt;

    /// Implements `Copy` iff the inner type implements it.
    impl<I: NumInt + Neg + Copy> Copy for Integer<I> {}

    /// Implements `Clone` iff the inner type implements it.
    impl<I: NumInt + Neg + Clone> Clone for Integer<I> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }

    /// Implements `Hash` iff the inner type implements it.
    #[allow(clippy::derive_hash_xor_eq)]
    // This is OK since both PartialEq & Hash are derived from the inner type:
    impl<I: NumInt + Neg + Hash> Hash for Integer<I> {
        fn hash<H: Hasher>(&self, hasher: &mut H) {
            self.0.hash(hasher);
        }
    }

    /// Implements `PartialEq` against the inner type.
    ///
    /// This allows, for example, to compare containers of both types.
    impl<I: NumInt + Neg> PartialEq<I> for Integer<I> {
        fn eq(&self, other: &I) -> bool {
            self.0 == *other
        }
    }

    /// Implements `Display` iff the inner type implements it.
    impl<I: NumInt + Neg + fmt::Display> fmt::Display for Integer<I> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    // Implements the aritmetic operators iff the inner type implements `Clone`.
    crate::impl_ops![I, Integer, Integer<I>, Self, NumInt + Neg + Clone];

    /// Implements the `Zero` identity.
    impl<I: NumInt + Neg + Clone> Zero for Integer<I> {
        fn zero() -> Self {
            Self(I::zero())
        }
        fn is_zero(&self) -> bool {
            self.0.is_zero()
        }
    }

    /// Implements the `One` identity.
    impl<I: NumInt + Neg + Clone> One for Integer<I> {
        fn one() -> Self {
            Self(I::one())
        }
        fn is_one(&self) -> bool {
            self.0.is_one()
        }
    }

    /// Implements `From` from the inner type.
    impl<I: NumInt + Neg> From<I> for Integer<I> {
        fn from(value: I) -> Self {
            Self(value)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Integer;

    #[test]
    /// Testing that the type implements a set of traits.
    fn check_traits() {
        // Copy
        fn is_copy<T: Copy>() {}
        is_copy::<Integer<i32>>();

        // PartialEq
        assert_eq![Integer::new(-1), -1];
    }

    #[test]
    fn constructors() {
        let i0 = Integer::new(0);
        let i1 = Integer::new(1);
        let i0to1 = Integer::new_0to1(0);
        assert_ne![i0, i0to1];
        assert_eq![i1, i0to1];

        assert_eq![None, Integer::new_nonzero(0)];
        assert_eq![Some(Integer::new(3)), Integer::new_nonzero(3)];
    }

    #[test]
    fn conversions() {
        assert_eq![1, Integer::new(1).get()];
        assert_eq![1_i32, Integer::new(1).get()];
        assert_eq![1_i64, Integer::new(1).get()];
        assert_eq![1, Integer::new(1_i16).get()];
    }

    #[test]
    fn ops() {
        let i1 = Integer::new(-10);
        let i2 = Integer::new(2);
        let results = [i1 + i2, i1 - i2, i1 * i2, i1 / i2, i1 % i2];
        assert_eq![results, [-8, -12, -20, -5, 0]];
    }
}
