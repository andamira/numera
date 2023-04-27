// numera::number::integer::z::impl_integer
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
            #[inline]
            fn is_even(&self) -> bool {
                self.0.is_even()
            }
            #[inline]
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
            fn gcd(&self, other: &Self) -> Option<Self> {
                let (mut a, mut b) = (self.0, other.0);
                while b != 0 {
                    let temp = b;
                    b = a % b;
                    a = temp;
                }
                Some($t::new(a).unwrap())
            }
            #[inline]
            fn lcm(&self, other: &Self) -> Option<Self> {
                Some($t::new(self.0 * other.0 / self.0.gcd(&other.0).unwrap()).unwrap())
            }
        }
    };
}

impl_integer![many: Integer8, Integer16, Integer32, Integer64, Integer128];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn z_lcm_gcd() {
        let z10: Integer32 = 10.into();
        let z15: Integer32 = 15.into();

        assert_eq![Integer32::new(30).unwrap(), z10.lcm(&z15).unwrap()];
        assert_eq![Integer32::new(5).unwrap(), z10.gcd(&z15).unwrap()];
    }
}
