// numera::number::integer::z::impl_integer
//
//!
//

use crate::number::{
    integer::{z::*, Integer},
    traits::ConstOne,
};
use devela::az::CheckedAs;

#[cfg(not(feature = "std"))]
use crate::all::is_prime;
#[cfg(feature = "std")]
use crate::all::is_prime_sieve;

macro_rules! impl_integer {
    (many: $($t:ident),+) => {
        $( impl_integer![$t]; )+
    };
    ($t:ident) => {
        /// # Methods for all integers
        impl $t {
            /// Returns `true` if this integer is even.
            #[inline]
            pub const fn is_even(&self) -> bool {
                self.0 & 1 == 0
            }
            /// Returns `true` if this integer is odd.
            #[inline]
            pub const fn is_odd(&self) -> bool {
                !self.is_even()
            }

            /// Returns `true` if this integer is a multiple of the `other`.
            #[inline]
            pub const fn is_multiple_of(&self, other: &Self) -> bool {
                self.0 % other.0 == 0
            }
            /// Returns `true` if this integer is a divisor of the `other`.
            #[inline]
            pub const fn is_divisor_of(&self, other: &Self) -> bool {
                other.is_multiple_of(self)
            }
        }

        /// # Methods for non-negative integers
        impl $t {
            /// Returns `Some(true)` if this integer is prime, `Some(false)` if it's not
            /// prime, or `None` if it can not be determined.
            ///
            /// Returns `None` if this integer can't be represented as a [`usize`],
            /// or as a [`u32`] in `no-std`.
            #[inline]
            pub fn is_prime(&self) -> Option<bool> {
                #[cfg(feature = "std")]
                return Some(is_prime_sieve((self.0).checked_as::<usize>()?));
                #[cfg(not(feature = "std"))]
                return Some(is_prime((self.0).checked_as::<u32>()?));
            }

            /// Returns `true` if `self` and `other` are relative primes,
            /// which means they have only 1 as their only common divisor.
            ///
            /// # Notation
            /// $a \perp b$.
            #[inline]
            pub const fn is_coprime(&self, other: &Self) -> bool {
                self.gcd(other).0 == Self::ONE.0
            }

            /// Calculates the *Greatest Common Divisor* of this integer and `other`.
            #[inline]
            #[must_use]
            pub const fn gcd(&self, other: &Self) -> Self {
                let (mut a, mut b) = (self.0, other.0);
                while b != 0 {
                    let temp = b;
                    b = a % b;
                    a = temp;
                }
                $t(a)
            }

            /// Calculates the *Lowest Common Multiple* of this integer and `other`.
            #[inline]
            #[must_use]
            pub const fn lcm(&self, other: &Self) -> Self {
                $t(self.0 * other.0 / self.gcd(other).0)
            }
        }

        impl Integer for $t {
            #[inline]
            fn integer_is_even(&self) -> bool {
                self.is_even()
            }
            #[inline]
            fn integer_is_multiple_of(&self, other: &Self) -> bool {
                self.is_multiple_of(&other)
            }
            #[inline]
            fn integer_is_prime(&self) -> Option<bool> {
                self.is_prime()
            }
            #[inline]
            fn integer_gcd(&self, other: &Self) -> Option<Self> {
                Some(self.gcd(other))
            }
            #[inline]
            fn integer_lcm(&self, other: &Self) -> Option<Self> {
                Some(self.lcm(other))
            }
        }
    };
}

impl_integer![many: Integer8, Integer16, Integer32, Integer64, Integer128];

#[cfg(test)]
mod tests {
    use crate::all::*;

    #[test]
    fn z_impl_integer() -> NumeraResult<()> {
        /* parity */

        assert_eq![true, Z8::new(5).is_odd()];
        assert_eq![true, Z8::new(-1).is_odd()];
        assert_eq![true, Z8::new(6).is_even()];
        assert_eq![true, Z8::new(0).is_even()];

        /* factors */

        assert_eq![true, Z8::new(10).is_multiple_of(&Z8::new(10))];
        assert_eq![true, Z8::new(10).is_multiple_of(&Z8::new(5))];
        assert_eq![true, Z8::new(10).is_multiple_of(&Z8::new(2))];
        assert_eq![true, Z8::new(10).is_multiple_of(&Z8::new(1))];
        assert_eq![false, Z8::new(10).is_multiple_of(&Z8::new(3))];

        assert_eq![true, Z8::new(1).is_divisor_of(&Z8::new(10))];
        assert_eq![true, Z8::new(2).is_divisor_of(&Z8::new(10))];
        assert_eq![true, Z8::new(5).is_divisor_of(&Z8::new(10))];
        assert_eq![true, Z8::new(10).is_divisor_of(&Z8::new(10))];
        assert_eq![false, Z8::new(3).is_divisor_of(&Z8::new(10))];

        /* primality */

        // absolute primality
        assert_eq![Some(true), Z8::new(5).is_prime()];
        assert_eq![Some(false), Z8::new(6).is_prime()];
        assert_eq![None, Z128::MAX.is_prime()];

        // relative primality
        assert_eq![true, Z8::new(5).is_coprime(&Z8::new(11))];
        assert_eq![false, Z8::new(5).is_coprime(&Z8::new(10))];

        /* LCM, GCD */

        assert_eq![Z32::new(30), Z32::new(10).lcm(&Z32::new(15))];
        assert_eq![Z32::new(5), Z32::new(10).gcd(&Z32::new(15))];

        Ok(())
    }
}
