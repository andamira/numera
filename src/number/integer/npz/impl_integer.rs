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
            #[inline(always)]
            fn is_even(&self) -> bool {
                self.0.is_even()
            }
            #[inline(always)]
            fn is_multiple_of(&self, other: &Self) -> bool {
                self.0.is_multiple_of(&other.0)
            }

            /// Returns always `None`, assumming negative numbers can't be prime.
            // https://www.themathdoctors.org/prime-numbers-what-about-negatives/
            // https://math.stackexchange.com/questions/2355731/can-negative-integers-be-prime
            #[inline]
            fn is_prime(&self) -> Option<bool> {
                Some(false)
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

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn npz_lcm_gcd() {
//         // let nnz10: NonPositiveInteger32 = 10_u32.into();
//         // let nnz15: NonPositiveInteger32 = 15_u32.into();
//
//         // assert_eq![NonPositiveInteger32::new(30).unwrap(), nnz10.lcm(&nnz15).unwrap()];
//         // assert_eq![NonPositiveInteger32::new(5).unwrap(), nnz10.gcd(&nnz15).unwrap()];
//     }
// }
