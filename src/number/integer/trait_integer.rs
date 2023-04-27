// numera::number::integer::impl_integer
//
//!
//
// TOC
//
// - define the `Integer` trait
// - impl for integer primitives

use crate::number::traits::{ConstZero, Number};
use az::CheckedAs;
use core::num::{
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
    NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
};

#[cfg(not(feature = "std"))]
use crate::all::is_prime_brute;
#[cfg(feature = "std")]
use crate::all::is_prime_sieve;

/// Common functions for all integers.
pub trait Integer: Number {
    /// Returns `true` if the number is even.
    fn is_even(&self) -> bool;
    /// Returns `true` if the number is odd.
    #[rustfmt::skip]
    fn is_odd(&self) -> bool { !self.is_even() }

    /// Returns `true` if the number is a multiple of `other`.
    fn is_multiple_of(&self, other: &Self) -> bool;
    #[rustfmt::skip]
    fn is_divisor_of(&self, other: &Self) -> bool { other.is_multiple_of(self) }

    /// Returns `Some(true)` if the number is prime, `Some(false)` if not prime,
    /// or `None` if it can not be determined.
    ///
    /// Returns `None` if the number can't be represented as a [`usize`],
    /// or as a [`u32`] in `no-std`.
    fn is_prime(&self) -> Option<bool>;

    /// Calculates the Greatest Common Divisor of `self` and `other`.
    ///
    /// Returns `None` if the operation can't return a number of the same kind,
    /// e.g. for `Negative` numbers.
    // THINK IMPROVE
    #[must_use]
    fn gcd(&self, other: &Self) -> Option<Self>
    where
        Self: Sized;

    /// Calculates the Lowest Common Multiple of `self` and `other`.
    ///
    /// Returns `None` if the operation can't return a number of the same kind,
    /// e.g. for `Negative` numbers.
    // THINK IMPROVE
    #[must_use]
    fn lcm(&self, other: &Self) -> Option<Self>
    where
        Self: Sized;

    // /// Calculates the Greatest Common Divisor of `self` and `other`.
    // fn gcd_lcm(&self, other: &Self) -> Result<(Self, Self)> where Self: Sized;
}

/// Implements `Integer` for integer primitives.
///
/// # Args
/// - `$t`:
macro_rules! impl_integer {
    (many $($t:ident),+) => { $( impl_integer![$t]; )+ };
    ($t:ident) => {
        impl Integer for $t {
            #[inline(always)]
            fn is_even(&self) -> bool {
                *self & 1 == 0
            }
            #[inline(always)]
            fn is_multiple_of(&self, other: &Self) -> bool {
                *self % *other == 0
            }

            #[inline]
            fn is_prime(&self) -> Option<bool> {
                #[cfg(feature = "std")]
                return Some(is_prime_sieve((*self).checked_as::<usize>()?));
                #[cfg(not(feature = "std"))]
                return Some(is_prime_brute((*self).checked_as::<u32>()?));
            }

            #[inline]
            fn gcd(&self, other: &Self) -> Option<Self> {
                let (mut a, mut b) = (*self, *other);
                while b != Self::ZERO {
                    let temp = b;
                    b = a % b;
                    a = temp;
                }
                Some(a)
            }
            #[inline]
            fn lcm(&self, other: &Self) -> Option<Self> {
                Some(*self * *other / self.gcd(other).unwrap())
            }
        }
    };

    (many_nonzero $($t:ident),+) => { $( impl_integer![nonzero $t]; )+ };
    (nonzero $t:ident) => {
        impl Integer for $t {
            #[inline(always)]
            fn is_even(&self) -> bool {
                self.get() & 1 == 0
            }
            #[inline(always)]
            fn is_multiple_of(&self, other: &Self) -> bool {
                self.get() % other.get() == 0
            }
            #[inline]
            fn is_prime(&self) -> Option<bool> {
                #[cfg(feature = "std")]
                return Some(is_prime_sieve(self.get().checked_as::<usize>()?));
                #[cfg(not(feature = "std"))]
                return Some(is_prime_brute(self.get().checked_as::<u32>()?));
            }
            #[inline]
            fn gcd(&self, other: &Self) -> Option<Self> {
                let (mut a, mut b) = (self.get(), other.get());
                while b != 0 {
                    let temp = b;
                    b = a % b;
                    a = temp;
                }
                Some($t::new(a).unwrap())
            }
            #[inline]
            fn lcm(&self, other: &Self) -> Option<Self> {
                Some($t::new(
                    self.get() * other.get() / self.get().gcd(&other.get()).unwrap()
                ).unwrap())
            }
        }
    };
}

impl_integer![many i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize];
impl_integer![many_nonzero
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
    NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize
];
