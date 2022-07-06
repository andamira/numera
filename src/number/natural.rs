//!

use num_integer::Integer as NumInt;
use num_traits::Unsigned;

/// A `Natural` number ([w][w1]/[m][m1]), from the set
/// $\N \lbrace 0, 1, 2, 3 … \rbrace $ of non-negative integers.
///
/// Alternative notations:
/// $ \N _0 = \N ^0 = \Z^+_0 = \Z _{\ge 0} = \lbrace 0 \rbrace \cup \Z^+ $
///
/// # About 0
/// There's no unanimous consensus on whether the natural numbers include 0 or
/// not. This library is opinionated in this regard, for the sake of clarity
/// and simplicity, to include 0 in the set of $\N$. This have the following
/// advantages:
/// - Simpler implementation. No need to distinguish between *naturals* and
///   *whole* numbers in the code, (also whole numbers don't have a dedicated
///   mathematical symbol, and don't have a dedicated word in many languages).
/// - There are methods to manage unwated zeroes in `Natural` and `Integer`.
/// - Agrees with the *[ISO 80000-2]* standard.
///
/// [w1]: https://en.wikipedia.org/wiki/Natural_number
/// [m1]: https://mathworld.wolfram.com/NaturalNumber.html
/// [ISO 80000-2]: https://en.wikipedia.org/wiki/ISO/IEC_80000#Part_2:_Mathematics
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Natural<N>(N)
where
    // NumInt already includes:
    // Sized + PartialEq + Eq + PartialOrd + Ord + Zero + One + NumOps
    N: NumInt + Unsigned;

impl<N: NumInt + Unsigned> Natural<N> {
    /// Returns a new `Natural` number.
    pub fn new(value: N) -> Self {
        let n = if value == N::zero() { N::one() } else { value };
        Self(n)
    }

    /// Returns a new `Natural` number,
    /// where a `value` of `0` is converted to `1`.
    pub fn new_0to1(value: N) -> Self {
        let value = if value == N::zero() { N::one() } else { value };
        Self(value)
    }

    /// Returns a new `Natural` number,
    /// but only if `value` $ != 0$.
    pub fn new_nonzero(value: N) -> Option<Self> {
        if value == N::zero() {
            None
        } else {
            Some(Self(value))
        }
    }

    /// If the inner value is `0` then it is changed to `1` and returns `true`.
    ///
    /// Returns `false` otherwise.
    pub fn if0_set1(&mut self) -> bool {
        if self.0.is_zero() {
            self.0 = N::one();
            true
        } else {
            false
        }
    }

    /// If the inner value is `1` then it is changed to `0` and returns `true`.
    ///
    /// Returns `false` otherwise.
    pub fn if1_set0(&mut self) -> bool {
        if self.0.is_zero() {
            self.0 = N::one();
            true
        } else {
            false
        }
    }

    /// If the inner value is `x` then it is changed to `y` and returns `true`.
    ///
    /// Returns `false` otherwise.
    pub fn ifx_sety(&mut self, x: N, y: N) -> bool {
        if self.0 == x {
            self.0 = y;
            true
        } else {
            false
        }
    }
}

impl<N: NumInt + Unsigned + Clone> Natural<N> {
    /// Returns a clone of the underlying data type.
    pub fn get(&self) -> N {
        self.0.clone()
    }
}

mod traits_implementations {
    use super::{Natural, NumInt, Unsigned};
    use core::hash::{Hash, Hasher};
    use num_traits::{One, Zero};
    use std::fmt;

    /// Implements `Copy` iff the inner type implements it.
    impl<N: NumInt + Unsigned + Copy> Copy for Natural<N> {}

    /// Implements `Clone` iff the inner type implements it.
    impl<N: NumInt + Unsigned + Clone> Clone for Natural<N> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }

    /// Implements `Hash` iff the inner type implements it.
    #[allow(clippy::derive_hash_xor_eq)]
    // This is OK since both PartialEq & Hash are derived from the inner type:
    impl<N: NumInt + Unsigned + Hash> Hash for Natural<N> {
        fn hash<H: Hasher>(&self, hasher: &mut H) {
            self.0.hash(hasher);
        }
    }

    /// Implements `PartialEq` against the inner type.
    ///
    /// This allows, for example, to compare containers of both types.
    impl<N: NumInt + Unsigned> PartialEq<N> for Natural<N> {
        fn eq(&self, other: &N) -> bool {
            self.0 == *other
        }
    }

    /// Implements `Display` iff the inner type implements it.
    impl<N: NumInt + Unsigned + fmt::Display> fmt::Display for Natural<N> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    crate::impl_ops![N, Natural, Natural<N>, Self, NumInt + Unsigned + Clone];

    /// Implements the `Zero` identity.
    impl<N: NumInt + Unsigned + Clone> Zero for Natural<N> {
        fn zero() -> Self {
            Self(N::zero())
        }
        fn is_zero(&self) -> bool {
            self.0.is_zero()
        }
    }

    /// Implements the `One` identity.
    impl<N: NumInt + Unsigned + Clone> One for Natural<N> {
        fn one() -> Self {
            Self(N::one())
        }
        fn is_one(&self) -> bool {
            self.0.is_one()
        }
    }

    /// Implements `From` from the inner type.
    impl<N: NumInt + Unsigned> From<N> for Natural<N> {
        fn from(value: N) -> Self {
            Self(value)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Testing that the type implements a set of traits.
    fn check_traits() {
        // Copy
        fn is_copy<T: Copy>() {}
        is_copy::<Natural<u32>>();

        // PartialEq
        assert_eq![Natural::new(32_u8), 32];
    }

    #[test]
    fn constructors() {
        let n0 = Natural::new(0_u8);
        let n1 = Natural::new(1_u8);
        assert_eq![n0, n1];

        assert_eq![None, Natural::new_nonzero(0_u32)];
        assert_eq![Some(Natural::new(3_u32)), Natural::new_nonzero(3)];
    }

    #[test]
    fn conversions() {
        assert_eq![1_u32, Natural::new(1).get()];
        assert_eq![1_u32, Natural::new(1).get()];
        assert_eq![1_u64, Natural::new(1).get()];
        assert_eq![1, Natural::new(1_u16).get()];
    }
}
