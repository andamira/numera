// numera::number::integer::nnz::impl_integer
//
//!
//

use super::*;
use crate::number::{integer::Integer, traits::Number};
use az::CheckedAs;

#[cfg(not(feature = "std"))]
use crate::all::is_prime_brute;
#[cfg(feature = "std")]
use crate::all::is_prime_sieve;

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
            #[inline]
            fn is_prime(&self) -> Option<bool> {
                #[cfg(feature = "std")]
                return Some(is_prime_sieve(self.0.checked_as::<usize>()?));
                #[cfg(not(feature = "std"))]
                return Some(is_prime_brute(self.0.checked_as::<u32>()?));
            }
            #[inline]
            fn gcd(&self, other: &Self) -> Self {
                let (mut a, mut b) = (self.0, other.0);
                while b != 0 {
                    let temp = b;
                    b = a % b;
                    a = temp;
                }
                $t::new(a).unwrap()
            }
            #[inline]
            fn lcm(&self, other: &Self) -> Self {
                $t::new(self.0 * other.0 / self.0.gcd(&other.0)).unwrap()
            }
        }
    };
}

impl_integer![
    many: NonNegativeInteger8,
    NonNegativeInteger16,
    NonNegativeInteger32,
    NonNegativeInteger64,
    NonNegativeInteger128
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nnz_lcm_gcd() {
        let nnz10: NonNegativeInteger32 = 10_u32.into();
        let nnz15: NonNegativeInteger32 = 15_u32.into();

        assert_eq![NonNegativeInteger32::new(30).unwrap(), nnz10.lcm(&nnz15)];
        assert_eq![NonNegativeInteger32::new(5).unwrap(), nnz10.gcd(&nnz15)];
    }
}
