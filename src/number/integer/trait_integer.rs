// numera::number::integer::trait_integer
//
//!
//
// TOC
//
// - define the `Integer` trait
// - impl for integer primitives

use crate::number::traits::{ConstZero, Number};
use core::num::{
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
    NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
};
use devela::az::CheckedAs;

#[cfg(not(feature = "std"))]
use crate::all::is_prime;
#[cfg(feature = "std")]
use crate::all::is_prime_sieve;

/// Common trait for all integers.
pub trait Integer: Number {
    /// Returns `true` if this integer is even.
    fn integer_is_even(&self) -> bool;
    /// Returns `true` if this integer is odd.
    fn integer_is_odd(&self) -> bool {
        !self.integer_is_even()
    }

    /// Returns `true` if this integer is a multiple of the `other`.
    fn integer_is_multiple_of(&self, other: &Self) -> bool;
    /// Returns `true` if this integer is a divisor of the `other`.
    fn integer_is_divisor_of(&self, other: &Self) -> bool {
        other.integer_is_multiple_of(self)
    }

    /// Returns `Some(true)` if this integer is prime, `Some(false)` if it's not
    /// prime, or `None` if it can not be determined.
    ///
    /// Returns `None` if this integer can't be represented as a [`usize`],
    /// or as a [`u32`] in `no-std`.
    fn integer_is_prime(&self) -> Option<bool>;

    /// Calculates the *Greatest Common Divisor* of this integer and `other`.
    ///
    /// Returns `None` if the operation can't return an integer of the same kind,
    /// e.g. for a `NegativeInteger`.
    #[rustfmt::skip]
    #[must_use]
    fn integer_gcd(&self, other: &Self) -> Option<Self> where Self: Sized;

    /// Calculates the *Lowest Common Multiple* of this integer and `other`.
    ///
    /// Returns `None` if the operation can't return an integer of the same kind,
    /// e.g. for `NegativeInteger`.
    #[rustfmt::skip]
    #[must_use]
    fn integer_lcm(&self, other: &Self) -> Option<Self> where Self: Sized;

    // /// Calculates the Greatest Common Divisor of `self` and `other`.
    // fn gcd_lcm(&self, other: &Self) -> Result<(Self, Self)> where Self: Sized;

    /// Returns the number of digits in base 10, without the sign.
    #[must_use]
    fn integer_digits(&self) -> usize;
}

/// Implements `Integer` for integer primitives.
///
/// # Args
/// - `$t`:
macro_rules! impl_integer {
    (many $($t:ident),+) => { $( impl_integer![$t]; )+ };
    ($t:ident) => {
        impl Integer for $t {
            #[inline]
            fn integer_is_even(&self) -> bool {
                *self & 1 == 0
            }
            #[inline]
            fn integer_is_multiple_of(&self, other: &Self) -> bool {
                *self % *other == 0
            }

            #[inline]
            fn integer_is_prime(&self) -> Option<bool> {
                #[cfg(feature = "std")]
                return Some(is_prime_sieve((*self).checked_as::<usize>()?));
                #[cfg(not(feature = "std"))]
                return Some(is_prime((*self).checked_as::<u32>()?));
            }

            #[inline]
            fn integer_gcd(&self, other: &Self) -> Option<Self> {
                let (mut a, mut b) = (*self, *other);
                while b != Self::ZERO {
                    let temp = b;
                    b = a % b;
                    a = temp;
                }
                Some(a)
            }
            #[inline]
            fn integer_lcm(&self, other: &Self) -> Option<Self> {
                Some(*self * *other / self.integer_gcd(other).unwrap())
            }

            fn integer_digits(&self) -> usize {
                self.checked_ilog10().unwrap_or(0) as usize
            }
        }
    };

    (many_nonzero $($t:ident),+) => { $( impl_integer![nonzero $t]; )+ };
    (nonzero $t:ident) => {
        impl Integer for $t {
            #[inline]
            fn integer_is_even(&self) -> bool {
                self.get() & 1 == 0
            }
            #[inline]
            fn integer_is_multiple_of(&self, other: &Self) -> bool {
                self.get() % other.get() == 0
            }
            #[inline]
            fn integer_is_prime(&self) -> Option<bool> {
                #[cfg(feature = "std")]
                return Some(is_prime_sieve(self.get().checked_as::<usize>()?));
                #[cfg(not(feature = "std"))]
                return Some(is_prime(self.get().checked_as::<u32>()?));
            }
            #[inline]
            fn integer_gcd(&self, other: &Self) -> Option<Self> {
                let (mut a, mut b) = (self.get(), other.get());
                while b != 0 {
                    let temp = b;
                    b = a % b;
                    a = temp;
                }
                Some($t::new(a).unwrap())
            }
            #[inline]
            fn integer_lcm(&self, other: &Self) -> Option<Self> {
                Some($t::new(
                    self.get() * other.get() / self.get().integer_gcd(&other.get()).unwrap()
                ).unwrap())
            }

            fn integer_digits(&self) -> usize {
                self.get().ilog10().try_into().expect("more than usize::MAX digits")
            }
        }
    };
}

impl_integer![many i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize];
impl_integer![many_nonzero
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
    NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize
];
