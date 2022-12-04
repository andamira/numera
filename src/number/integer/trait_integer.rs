// numera::number::integer::impl_integer
//
//!
//
// TOC
//
// - define the `Integer` trait
// - impl for integer primitives

use crate::number::traits::Number;

use core::num::{
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
    NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
};

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

    // /// Calculates the Greatest Common Divisor of `self` and `other`.
    // #[must_use]
    // fn gcd(&self, other: &Self) -> Result<Self>;

    // /// Calculates the Lowest Common Multiple of `self` and `other`.
    // #[must_use]
    // fn lcm(&self, other: &Self) -> Result<Self>;

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
            #[inline]
            fn is_even(&self) -> bool {
                *self & 1 == 0
            }
            #[inline]
            fn is_multiple_of(&self, other: &Self) -> bool {
                *self % *other == 0
            }

        }
    };

    (many_nonzero $($t:ident),+) => { $( impl_integer![nonzero $t]; )+ };
    (nonzero $t:ident) => {
        impl Integer for $t {
            #[inline]
            fn is_even(&self) -> bool {
                self.get() & 1 == 0
            }
            #[inline]
            fn is_multiple_of(&self, other: &Self) -> bool {
                self.get() % other.get() == 0
            }

        }
    };
}

impl_integer![many i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize];
impl_integer![many_nonzero
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
    NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize
];
