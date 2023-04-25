// numera::number::traits::number
//
//! Defines the `Number` trait.
//!
//! Also implements it for all the supported primitives and external types.
//
// TOC
// - definition of `Number`
//
// - macros
//   - impl_number
//
// - impls
//   - primitive integers
//   - external types

use core::num::{
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
    NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
};

use crate::{
    error::NumeraResult as Result,
    number::traits::{Bound, Count, Ident, Sign},
};

/* definitions */

/// Common trait for all numbers.
pub trait Number: Bound + Count + Ident + Sign {
    /// The inner value representation of the number.
    type Inner;

    /// Returns some new number,
    ///
    /// # Errors
    /// Errors if the `value` is not in a valid state for this number subset.
    #[rustfmt::skip]
    fn new(value: Self::Inner) -> Result<Self> where Self: Sized;

    /// Returns a new number.
    ///
    /// # Panics
    /// Panics if the `value` is not in a valid state for this number subset.
    ///
    /// # Safety
    /// The invariants inherent to the specific number type must be maintained.
    unsafe fn new_unchecked(value: Self::Inner) -> Self;
}

/* macros */

/// impl the `Number` trait for values that already has the required trait bounds.
macro_rules! impl_number {
    (many: $($t:ty ),+) => {
        $( impl_number![single: $t]; )+
    };
    (single: $t:ty) => {
        impl Number for $t {
            type Inner = $t;

            #[inline]
            fn new(value: $t) -> Result<Self> { Ok(value) }
            #[inline]
            unsafe fn new_unchecked(value: Self::Inner) -> Self { value }
        }
    };
}

#[rustfmt::skip]
impl_number![many:
    f32, f64,
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
    NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize,
    NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize
];

#[cfg(feature = "twofloat")]
impl_number![single: twofloat::TwoFloat];

#[cfg(feature = "half")]
impl_number![many: half::bf16, half::f16];

#[cfg(feature = "ibig")]
impl_number![many: ibig::IBig, ibig::UBig];
