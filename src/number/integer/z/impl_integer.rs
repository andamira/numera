// numera::integer::z::impl_integer
//
//!
//

use super::{Integer128, Integer16, Integer32, Integer64, Integer8};
use crate::number::{integer::Integer, traits::Number};
use az::CheckedAs;
use primal_sieve::Sieve;

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
                let u = self.0.checked_as::<usize>()?;
                Some(Sieve::new(u).is_prime(u))
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

impl_integer![many: Integer8, Integer16, Integer32, Integer64, Integer128];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lcm_gcd() {
        let z10: Integer32 = 10.into();
        let z15: Integer32 = 15.into();

        assert_eq![Integer32::new(30).unwrap(), z10.lcm(&z15)];
        assert_eq![Integer32::new(5).unwrap(), z10.gcd(&z15)];
    }
}
