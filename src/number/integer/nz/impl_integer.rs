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
        impl Integer for $t {
            #[inline(always)]
            fn is_even(&self) -> bool {
                self.0.get().is_even()
            }
            #[inline(always)]
            fn is_multiple_of(&self, other: &Self) -> bool {
                self.0.get().is_multiple_of(&other.0.get())
            }


            /// Returns always `None`, assumming negative numbers can't be prime.
            // https://www.themathdoctors.org/prime-numbers-what-about-negatives/
            // https://math.stackexchange.com/questions/2355731/can-negative-integers-be-prime
            #[inline(always)]
            fn is_prime(&self) -> Option<bool> {
                None
            }
            /// Returns always `None`, since the result should be a positive number.
            #[inline(always)]
            fn gcd(&self, _other: &Self) -> Option<Self> {
                None
            }
            /// Returns always `None`, since the result should be a positive number.
            #[inline(always)]
            fn lcm(&self, _other: &Self) -> Option<Self> {
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
