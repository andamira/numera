// numera::number::integer::n0z::integer
//
//!
//

use crate::number::{
    integer::{n0z::*, Integer},
    traits::ConstOne,
};
use devela::az::CheckedAs;

#[cfg(not(feature = "std"))]
use crate::all::is_prime;
#[cfg(feature = "std")]
use crate::all::is_prime_sieve;

macro_rules! impl_nonzero_integer {
    // $t:
    // $inner:
    (many: $($t:ident, $inner:ident),+) => {
        $( impl_nonzero_integer![$t, $inner]; )+
    };
    ($t:ident, $inner:ident) => {
        /// # Methods for all integers
        impl $t {
            /// Returns `true` if this integer is even.
            #[inline]
            #[must_use]
            pub const fn is_even(&self) -> bool {
                self.0.get() & 1 == 0
            }
            /// Returns `true` if this integer is odd.
            #[inline]
            #[must_use]
            pub const fn is_odd(&self) -> bool {
                !self.is_even()
            }

            /// Returns `true` if this integer is a multiple of the `other`.
            #[inline]
            #[must_use]
            pub const fn is_multiple_of(&self, other: &Self) -> bool {
                self.0.get() % other.0.get() == 0
            }
            /// Returns `true` if this integer is a divisor of the `other`.
            #[inline]
            #[must_use]
            pub const fn is_divisor_of(&self, other: &Self) -> bool {
                other.is_multiple_of(self)
            }

            /// Returns `true` if `self` and `other` are relative primes,
            /// which means they have only 1 as their only common divisor.
            ///
            /// # Notation
            /// $a \perp b$.
            #[inline]
            #[must_use]
            pub const fn is_coprime(&self, other: &Self) -> bool {
                self.gcd(other).0.get() == Self::ONE.0.get()
            }

            /// Returns the number of digits in base 10.
            #[inline]
            #[must_use]
            pub const fn digits(&self) -> usize {
                if let Some(n) = self.0.get().checked_ilog10() {
                     n as usize + 1
                } else {
                    1
                }
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
                return Some(is_prime_sieve((self.0.get()).checked_as::<usize>()?));
                #[cfg(not(feature = "std"))]
                return Some(is_prime((self.0.get()).checked_as::<u32>()?));
            }

            /// Calculates the *Greatest Common Divisor* of this integer and `other`.
            #[inline]
            #[must_use]
            pub const fn gcd(&self, other: &Self) -> Self {
                let (mut a, mut b) = (self.0.get(), other.0.get());
                while b != 0 {
                    let temp = b;
                    b = a % b;
                    a = temp;
                }
                #[cfg(feature = "safe")]
                if let Ok(n0z) = $t::new(a) {
                    n0z
                } else {
                    unreachable![]
                }

                #[cfg(not(feature = "safe"))]
                // SAFETY: when self & other are not 0, the result can't be 0
                return unsafe { $t::new_unchecked(a) };
            }

            /// Calculates the *Lowest Common Multiple* of this integer and `other`.
            #[inline]
            #[must_use]
            pub const fn lcm(&self, other: &Self) -> Self {
                let lcm = self.0.get() * other.0.get() / self.gcd(other).0.get();

                #[cfg(feature = "safe")]
                if let Ok(n0z) = $t::new(lcm) {
                    n0z
                } else {
                    unreachable![]
                }
                #[cfg(not(feature = "safe"))]
                // SAFETY: when self & other are not 0, the result can't be 0
                return unsafe { $t::new_unchecked(lcm) };
            }
        }
        impl Integer for $t {
            #[inline]
            fn integer_is_even(&self) -> bool {
                self.is_even()
            }
            #[inline]
            fn integer_is_multiple_of(&self, other: &Self) -> bool {
                self.is_multiple_of(other)
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
            #[inline]
            fn integer_digits(&self) -> usize {
                self.digits()
            }
        }
    };
}

impl_nonzero_integer![
    many: NonZeroInteger8,
    NonZeroI8,
    NonZeroInteger16,
    NonZeroI16,
    NonZeroInteger32,
    NonZeroI32,
    NonZeroInteger64,
    NonZeroI64,
    NonZeroInteger128,
    NonZeroI128
];

#[cfg(test)]
mod tests {
    use crate::error::NumeraResult;
    use crate::number::integer::n0z::*;

    #[test]
    fn n0z_lcm_gcd() -> NumeraResult<()> {
        let n0z10 = N0z32::new(10)?;
        let n0z15 = N0z32::new(15)?;

        assert_eq![N0z32::new(30)?, n0z10.lcm(&n0z15)];
        assert_eq![N0z32::new(5)?, n0z10.gcd(&n0z15)];

        Ok(())
    }
}
