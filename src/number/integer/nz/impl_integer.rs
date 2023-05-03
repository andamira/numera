// numera::number::integer::nz::impl_integer
//
//!
//

use super::*;
use crate::number::integer::Integer;

macro_rules! impl_nonzero_integer {
    // $t:
    // $inner:
    (many: $($t:ident, $inner:ident),+) => {
        $( impl_nonzero_integer![$t, $inner]; )+
    };
    ($t:ident, $inner:ident) => {
        /// # Methods for all integers.
        impl $t {
            /// Returns `true` if this integer is even.
            #[inline]
            pub const fn is_even(&self) -> bool {
                self.0.get() & 1 == 0
            }
            /// Returns `true` if this integer is odd.
            #[inline]
            pub const fn is_odd(&self) -> bool {
                !self.is_even()
            }

            /// Returns `true` if this integer is a multiple of the `other`.
            #[inline]
            pub const fn is_multiple_of(&self, other: &Self) -> bool {
                self.0.get() % other.0.get() == 0
            }
            /// Returns `true` if this integer is a divisor of the `other`.
            #[inline]
            pub const fn is_divisor_of(&self, other: &Self) -> bool {
                other.is_multiple_of(self)
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
        }
    };
}

impl_nonzero_integer![
    many: NegativeInteger8,
    NonZeroU8,
    NegativeInteger16,
    NonZeroU16,
    NegativeInteger32,
    NonZeroU32,
    NegativeInteger64,
    NonZeroU64,
    NegativeInteger128,
    NonZeroU128
];
