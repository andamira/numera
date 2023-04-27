// numera::number::integer::pz::impl_integer
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

macro_rules! impl_nonzero_integer {
    // $t:
    // $inner:
    (many: $($t:ident, $inner:ident),+) => {
        $( impl_nonzero_integer![$t, $inner]; )+
    };
    ($t:ident, $inner:ident) => {
        impl Integer for $t {
            #[inline]
            fn is_even(&self) -> bool {
                self.0.get().is_even()
            }
            #[inline]
            fn is_multiple_of(&self, other: &Self) -> bool {
                self.0.get().is_multiple_of(&other.0.get())
            }
            #[inline]
            fn is_prime(&self) -> Option<bool> {
                #[cfg(feature = "std")]
                return Some(is_prime_sieve(self.0.get().checked_as::<usize>()?));
                #[cfg(not(feature = "std"))]
                return Some(is_prime_brute(self.0.get().checked_as::<u32>()?));
            }
            #[inline]
            fn gcd(&self, other: &Self) -> Option<Self> {
                let (mut a, mut b) = (self.0.get(), other.0.get());
                while b != 0 {
                    let temp = b;
                    b = a % b;
                    a = temp;
                }
                #[cfg(feature = "safe")]
                return Some($t::new(a).unwrap());

                #[cfg(not(feature = "safe"))]
                // SAFETY: it can't be 0
                return Some(unsafe { $t::new_unchecked(a) });
            }
            #[inline]
            fn lcm(&self, other: &Self) -> Option<Self> {
                #[cfg(feature = "safe")]
                return Some(
                    $t::new(
                        self.0.get() * other.0.get() /
                        self.0.get().gcd(&other.0.get()).unwrap()
                    ).unwrap()
                );

                #[cfg(not(feature = "safe"))]
                return Some(
                    // SAFETY: it can't be 0
                    unsafe {
                        $t::new_unchecked(
                            self.0.get() * other.0.get() /
                            self.0.get().gcd(&other.0.get()).unwrap()
                        )
                    }
                );
            }
        }
    };
}

impl_nonzero_integer![
    many: PositiveInteger8,
    NonZeroU8,
    PositiveInteger16,
    NonZeroU16,
    PositiveInteger32,
    NonZeroU32,
    PositiveInteger64,
    NonZeroU64,
    PositiveInteger128,
    NonZeroU128
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pz_lcm_gcd() {
        let pz10 = PositiveInteger32::new(10).unwrap();
        let pz15 = PositiveInteger32::new(15).unwrap();

        assert_eq![
            PositiveInteger32::new(30).unwrap(),
            pz10.lcm(&pz15).unwrap()
        ];
        assert_eq![PositiveInteger32::new(5).unwrap(), pz10.gcd(&pz15).unwrap()];
    }
}
