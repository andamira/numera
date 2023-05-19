// numera::number::integer::nz::impl_integer
//
//!
//

use crate::number::integer::*;
use devela::paste;

macro_rules! impl_nonzero_integer {
    // $t: base name type
    // $b: bitsize
    // $inner:
    (many: $($t:ident + $b:literal + $inner:ident),+) => {
        $( impl_nonzero_integer![$t + $b + $inner]; )+
    };
    ($t:ident + $b:literal + $inner:ident) => { paste! {
        /// # Methods for all integers
        impl [<$t$b>] {
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

            // /// Returns `true` if `self` and `other` are relative primes,
            // /// which means they have only 1 as their only common divisor.
            // ///
            // /// # Notation
            // /// $a \perp b$.
            // #[inline]
            // #[must_use]
            // pub const fn is_coprime(&self, other: &Self) -> bool {
            //     self.gcd(other).0 == Self::ONE.0 // FIX
            // }

            /// Returns the number of digits in base 10.
            #[inline]
            #[must_use]
            pub const fn digits(&self) -> u32 {
                self.0.ilog10() + 1
            }
        }

        /// # Methods that are open to the result being a different kind of integer.
        impl [<$t$b>] {
            // TODO: NEED open_neg
            //
            // /// Calculates the *Greatest Common Divisor* of this integer and `other`.
            // #[inline]
            // #[must_use]
            // pub const fn open_gcd(&self, other: &Self) -> [<PositiveInteger$b>] {
            //     let pself = [<PositiveInteger$b
            //     let (mut a, mut b) = (self.0, other.0);
            //     while b != 0 {
            //         let temp = b;
            //         b = a % b;
            //         a = temp;
            //     }
            //     Self(a)
            // }
        }

        impl Integer for [<$t$b>] {
            #[inline]
            fn integer_is_even(&self) -> bool {
                self.is_even()
            }
            #[inline]
            fn integer_is_multiple_of(&self, other: &Self) -> bool {
                self.is_multiple_of(other)
            }


            /// Returns always `None`, since negative numbers can't be prime.
            #[inline]
            fn integer_is_prime(&self) -> Option<bool> {
                None
            }
            /// Returns always `None`, since the result must be a positive number.
            #[inline]
            fn integer_gcd(&self, _other: &Self) -> Option<Self> {
                None
            }
            /// Returns always `None`, since the result must be a non-negative number.
            #[inline]
            fn integer_lcm(&self, _other: &Self) -> Option<Self> {
                None
            }

            #[inline]
            fn integer_digits(&self) -> u32 {
                self.digits()
            }
        }
    }};
}

impl_nonzero_integer![
    many: NegativeInteger + 8 + NonZeroU8,
    NegativeInteger + 16 + NonZeroU16,
    NegativeInteger + 32 + NonZeroU32,
    NegativeInteger + 64 + NonZeroU64,
    NegativeInteger + 128 + NonZeroU128
];
