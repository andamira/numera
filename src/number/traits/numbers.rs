// numera::number::traits::numbers
//
//! Defines the `Numbers` trait.
//!
//! Also implements it for all the supported primitives and external types.
//
// TOC
// - definition of `Numbers`
//
// - macros
//   - impl_numbers
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
pub trait Numbers: Bound + Count + Ident + Sign {
    /// The inner primitive representation of the number.
    ///
    /// May be the same as `InnermostRepr`.
    ///
    /// For example a `NonNegativeInteger8` would have `InnerRepr = NonZeroU8`,
    /// and `InnermostRepr = u8`, while an `Integer8` and an `i8` would both
    /// have an `i8` in both cases.
    type InnerRepr;

    /// The innermost primitive representation of the number.
    ///
    /// May be the same as `InnerRepr`.
    ///
    /// For example a `NonNegativeInteger8` would have `InnerRepr = NonZeroU8`,
    /// and `InnermostRepr = u8`, while an `Integer8` and an `i8` would both
    /// have an `i8` in both cases.
    type InnermostRepr;

    /// Forms a new number from its inner representation.
    ///
    /// # Errors
    /// Returns an error if the `value` does not conform to the invariants
    /// of what's considered a valid state for this type of number.
    ///
    /// For example a `NonNegativeInteger8` would have `InnerRepr = NonZeroU8`,
    /// and `InnermostRepr = u8`, while an `Integer8` would have i8 in both cases.
    #[rustfmt::skip]
    fn from_inner_repr(value: Self::InnerRepr) -> Result<Self> where Self: Sized;

    /// Forms a new number from its inner representation.
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
    unsafe fn from_inner_repr_unchecked(value: Self::InnerRepr) -> Self;

    /// Forms a new number from its innermost representation.
    ///
    /// # Errors
    /// Returns an error if the `value` does not conform to the invariants
    /// of what's considered a valid state for this type of number.
    ///
    /// For example a `NonNegativeInteger8` would have `InnerRepr = NonZeroU8`,
    /// and `InnermostRepr = u8`, while an `Integer8` would have i8 in both cases.
    #[rustfmt::skip]
    fn from_innermost_repr(value: Self::InnermostRepr) -> Result<Self> where Self: Sized;

    /// Forms a new number from its innermost representation.
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
    unsafe fn from_innermost_repr_unchecked(value: Self::InnermostRepr) -> Self;

    /// Deconstructs the number to its inner representation.
    fn into_inner_repr(self) -> Self::InnerRepr;

    /// Deconstructs the number to its innermost representation.
    fn into_innermost_repr(self) -> Self::InnermostRepr;

    /* auto */

    /// Forms a new number from its converted inner representation.
    ///
    /// # Errors
    /// Returns an error if the converted `value` does not conform to the
    /// invariants of what's considered a valid state for this number.
    #[inline]
    fn try_from_inner_repr(value: impl Into<Self::InnerRepr>) -> Result<Self>
    where
        Self: Sized,
    {
        Numbers::from_inner_repr(value.into())
    }
}

/* macros */

/// impl the `Numbers` trait for values that already has the required trait bounds.
macro_rules! impl_numbers {
    (many: $($t:ty ),+) => {
        $( impl_numbers![single: $t]; )+
    };
    (single: $t:ty) => {
        impl Numbers for $t {
            type InnerRepr = $t;
            type InnermostRepr = $t;

            #[inline]
            fn from_inner_repr(value: Self::InnerRepr) -> Result<Self> { Ok(value) }
            #[inline]
            #[cfg(not(feature = "safe"))]
            unsafe fn from_inner_repr_unchecked(value: Self::InnerRepr) -> Self { value }

            #[inline]
            fn from_innermost_repr(value: Self::InnermostRepr) -> Result<Self> { Ok(value) }
            #[inline]
            #[cfg(not(feature = "safe"))]
            unsafe fn from_innermost_repr_unchecked(value: Self::InnermostRepr) -> Self { value }

            #[inline]
            fn into_inner_repr(self) -> Self::InnerRepr { self }
            #[inline]
            fn into_innermost_repr(self) -> Self::InnermostRepr { self }
        }
    };
}

#[rustfmt::skip]
impl_numbers![many:
    f32, f64,
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
    NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize,
    NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize
];

#[cfg(feature = "twofloat")]
impl_numbers![single: twofloat::TwoFloat];

#[cfg(feature = "half")]
impl_numbers![many: half::bf16, half::f16];

#[cfg(feature = "dashu-int")]
impl_numbers![many: dashu_int::IBig, dashu_int::UBig];
