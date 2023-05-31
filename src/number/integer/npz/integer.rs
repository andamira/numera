// numera::number::integer::npz::integer
//
//!
//

use super::*;
use crate::number::integer::Integers;

macro_rules! impl_integer {
    (many: $($t:ident),+) => {
        $( impl_integer![$t]; )+
    };
    ($t:ident) => {
        /// # Methods for all integers
        impl $t {
            /// Returns `true` if this integer is even.
            #[inline]
            #[must_use]
            pub const fn is_even(&self) -> bool {
                self.0 & 1 == 0
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
                self.0 % other.0 == 0
            }
            /// Returns `true` if this integer is a divisor of the `other`.
            #[inline]
            #[must_use]
            pub const fn is_divisor_of(&self, other: &Self) -> bool {
                other.is_multiple_of(self)
            }

            /// Returns the number of digits in base 10.
            #[inline]
            #[must_use]
            pub const fn digits(&self) -> usize {
                if let Some(n) = self.0.checked_ilog10() {
                     n as usize + 1
                } else {
                    1
                }
            }
        }

        impl Integers for $t {
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
            fn integer_digits(&self) -> usize {
                self.digits()
            }
        }
    };
}

impl_integer![
    many: NonPositiveInteger8,
    NonPositiveInteger16,
    NonPositiveInteger32,
    NonPositiveInteger64,
    NonPositiveInteger128
];
