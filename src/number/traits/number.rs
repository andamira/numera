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
    type Parts;

    /// Forms a new number from its constituent parts.
    ///
    /// # Errors
    /// Returns an error if the `value` does not conform to the invariants
    /// of what's considered a valid state for this type of number.
    #[rustfmt::skip]
    fn from_parts(value: Self::Parts) -> Result<Self> where Self: Sized;

    /// Forms a new number from its constituent parts.
    ///
    /// # Panics
    /// Could panic (specially in debug) if the `value` does not conform to the
    /// invariants of what's considered a valid state for this number.
    ///
    /// # Safety
    /// The invariants inherent to the specific number type must be maintained.
    #[must_use]
    #[cfg(not(feature = "safe"))]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "unsafe")))]
    unsafe fn from_parts_unchecked(value: Self::Parts) -> Self;

    /* auto */

    /// Forms a new number from its converted constituent parts.
    ///
    /// # Errors
    /// Returns an error if the converted `value` does not conform to the
    /// invariants of what's considered a valid state for this number.
    #[inline]
    fn try_from_parts(value: impl Into<Self::Parts>) -> Result<Self>
    where
        Self: Sized,
    {
        Number::from_parts(value.into())
    }
}

/* macros */

/// impl the `Number` trait for values that already has the required trait bounds.
macro_rules! impl_number {
    (many: $($t:ty ),+) => {
        $( impl_number![single: $t]; )+
    };
    (single: $t:ty) => {
        impl Number for $t {
            type Parts = $t;

            #[inline]
            fn from_parts(value: $t) -> Result<Self> { Ok(value) }

            #[inline]
            #[cfg(not(feature = "safe"))]
            unsafe fn from_parts_unchecked(value: Self::Parts) -> Self { value }
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
