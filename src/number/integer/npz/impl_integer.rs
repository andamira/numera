// numera::number::integer::npz::impl_integer
//
//!
//

use super::*;
use crate::number::integer::Integer;

macro_rules! impl_integer {
    (many: $($t:ident),+) => {
        $( impl_integer![$t]; )+
    };
    ($t:ident) => {
        impl Integer for $t {
            #[inline]
            fn is_even(&self) -> bool {
                self.0.is_even()
            }
            #[inline]
            fn is_multiple_of(&self, other: &Self) -> bool {
                self.0.is_multiple_of(&other.0)
            }

            /// Returns always `None`, assumming negative numbers can't be prime.
            // https://www.themathdoctors.org/prime-numbers-what-about-negatives/
            // https://math.stackexchange.com/questions/2355731/can-negative-integers-be-prime
            #[inline]
            fn is_prime(&self) -> Option<bool> {
                None
            }
            /// Returns always `None`, since the result should be a positive number.
            #[inline]
            fn gcd(&self, _other: &Self) -> Option<Self> {
                None
            }
            /// Returns always `None`, since the result should be a positive number.
            #[inline]
            fn lcm(&self, _other: &Self) -> Option<Self> {
                None
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
