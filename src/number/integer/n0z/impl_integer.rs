// numera::number::integer::n0z::impl_integer
//
//!
//

use super::{
    NonZeroInteger128, NonZeroInteger16, NonZeroInteger32, NonZeroInteger64, NonZeroInteger8,
};
use crate::number::{integer::Integer, traits::Number};
use az::CheckedAs;
use core::num::{NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8};

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
            #[inline(always)]
            fn is_even(&self) -> bool {
                self.0.get().is_even()
            }
            #[inline(always)]
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
            fn gcd(&self, other: &Self) -> Self {
                let (mut a, mut b) = (self.0.get(), other.0.get());
                while b != 0 {
                    let temp = b;
                    b = a % b;
                    a = temp;
                }
                // SAFETY: it can't be 0
                unsafe { $t::new_unchecked($inner::new_unchecked(a)) }
            }
            #[inline]
            fn lcm(&self, other: &Self) -> Self {
                // SAFETY: it can't be 0
                unsafe {
                    $t::new_unchecked(
                        $inner::new_unchecked(
                            self.0.get() * other.0.get() /
                            self.0.get().gcd(&other.0.get())
                        )
                    )
                }

                // TODO safe
                // $t::new(
                //     $inner::new(self.0.get() * other.0.get() /
                //     self.0.get().gcd(&other.0.get())).unwrap()
                // ).unwrap()
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
    fn lcm_gcd() {
        let z10: Integer32 = 10.into();
        let z15: Integer32 = 15.into();

        assert_eq![Integer32::new(30).unwrap(), z10.lcm(&z15)];
        assert_eq![Integer32::new(5).unwrap(), z10.gcd(&z15)];
    }
}
