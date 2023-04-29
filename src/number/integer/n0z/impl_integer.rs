// numera::number::integer::n0z::impl_integer
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
                return Some($t::from_parts(a).unwrap());

                #[cfg(not(feature = "safe"))]
                // SAFETY: it can't be 0
                return Some(unsafe { $t::from_parts_unchecked(a) });
            }
            #[inline]
            fn lcm(&self, other: &Self) -> Option<Self> {
                #[cfg(feature = "safe")]
                return Some(
                    $t::from_parts(
                        self.0.get() * other.0.get() /
                        self.0.get().gcd(&other.0.get()).unwrap()
                    ).unwrap()
                );

                #[cfg(not(feature = "safe"))]
                return Some(
                    // SAFETY: it can't be 0
                    unsafe {
                        $t::from_parts_unchecked(
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
    many: NonZeroInteger8,
    NonZeroI8,
    NonZeroInteger16,
    NonZeroI16,
    NonZeroInteger32,
    NonZeroI32,
    NonZeroInteger64,
    NonZeroI64,
    NonZeroInteger128,
    NonZeroI128
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n0z_lcm_gcd() {
        let n0z10 = NonZeroInteger32::from_parts(10).unwrap();
        let n0z15 = NonZeroInteger32::from_parts(15).unwrap();

        assert_eq![
            NonZeroInteger32::from_parts(30).unwrap(),
            n0z10.lcm(&n0z15).unwrap()
        ];
        assert_eq![
            NonZeroInteger32::from_parts(5).unwrap(),
            n0z10.gcd(&n0z15).unwrap()
        ];
    }
}
