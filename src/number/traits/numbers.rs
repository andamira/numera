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
    error::{NumeraError, NumeraResult},
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

    /// Forms a new number from its given `inner` representation.
    ///
    /// # Errors
    /// Returns an error if `inner` does not conform to the invariants
    /// of what's considered a valid state for this type of number.
    ///
    /// For example a `NonNegativeInteger8` would have `InnerRepr = NonZeroU8`,
    /// and `InnermostRepr = u8`, while an `Integer8` would have i8 in both cases.
    #[rustfmt::skip]
    fn from_inner_repr(inner: Self::InnerRepr) -> NumeraResult<Self> where Self: Sized;

    /// Forms a new number from its given `inner` representation.
    ///
    /// # Panics
    /// Could panic (specially in debug) if `inner` does not conform to the
    /// invariants of what's considered a valid state for this number.
    ///
    /// # Safety
    /// The invariants inherent to the specific number type must be maintained.
    #[must_use]
    #[cfg(not(feature = "safe"))]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "not(safe)")))]
    unsafe fn from_inner_repr_unchecked(inner: Self::InnerRepr) -> Self;

    /// Forms a new number from its `innermost` representation.
    ///
    /// # Errors
    /// Returns an error if `innermost` does not conform to the invariants
    /// of what's considered a valid state for this type of number.
    ///
    /// For example a `NonNegativeInteger8` would have `InnerRepr = NonZeroU8`,
    /// and `InnermostRepr = u8`, while an `Integer8` would have i8 in both cases.
    #[rustfmt::skip]
    fn from_innermost_repr(innermost: Self::InnermostRepr) -> NumeraResult<Self> where Self: Sized;

    /// Forms a new number from its `innermost` representation.
    ///
    /// # Panics
    /// Could panic (specially in debug) if `innermost` does not conform to the
    /// invariants of what's considered a valid state for this number.
    ///
    /// # Safety
    /// The invariants inherent to the specific number type must be maintained.
    #[must_use]
    #[cfg(not(feature = "safe"))]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "not(safe)")))]
    unsafe fn from_innermost_repr_unchecked(innermost: Self::InnermostRepr) -> Self;

    /// Deconstructs the number to its inner representation.
    fn into_inner_repr(self) -> Self::InnerRepr;

    /// Deconstructs the number to its innermost representation.
    fn into_innermost_repr(self) -> Self::InnermostRepr;

    /* auto */

    /// Forms a new number from its converted given `inner` representation.
    ///
    /// # Errors
    /// Returns an error if the converted `inner` does not conform to the
    /// invariants of what's considered a valid state for this number.
    #[inline]
    fn try_from_inner_repr(inner: impl Into<Self::InnerRepr>) -> NumeraResult<Self>
    where
        Self: Sized,
    {
        Numbers::from_inner_repr(inner.into())
    }
}

/* macros */

/// impl the `Numbers` trait for primitives.
macro_rules! impl_numbers {
    // $t: outer type == inner type
    ($($t:ty ),+) => { $( impl_numbers![@$t]; )+ };
    (@$t:ty) => {
        impl Numbers for $t {
            type InnerRepr = $t;
            type InnermostRepr = $t;

            #[inline]
            fn from_inner_repr(value: Self::InnerRepr) -> NumeraResult<Self> { Ok(value) }
            #[inline]
            #[cfg(not(feature = "safe"))]
            unsafe fn from_inner_repr_unchecked(value: Self::InnerRepr) -> Self { value }

            #[inline]
            fn from_innermost_repr(value: Self::InnermostRepr) -> NumeraResult<Self> { Ok(value) }
            #[inline]
            #[cfg(not(feature = "safe"))]
            unsafe fn from_innermost_repr_unchecked(value: Self::InnermostRepr) -> Self { value }

            #[inline]
            fn into_inner_repr(self) -> Self::InnerRepr { self }
            #[inline]
            fn into_innermost_repr(self) -> Self::InnermostRepr { self }
        }
    };

    // $t: outer type
    // $i: inner repr type
    (non0 $($t:ident + $i:ident ),+) => { $( impl_numbers![@non0 $t+$i]; )+ };
    (@non0 $t:ident + $i:ident) => {
        impl Numbers for $t {
            type InnerRepr = $i;
            type InnermostRepr = $i;

            #[inline]
            fn from_inner_repr(value: Self::InnerRepr) -> NumeraResult<Self> {
                $t::new(value).ok_or(NumeraError::Conversion)
            }
            #[inline]
            #[cfg(not(feature = "safe"))]
            unsafe fn from_inner_repr_unchecked(value: Self::InnerRepr) -> Self {
                $t::new_unchecked(value)
            }

            #[inline]
            fn from_innermost_repr(value: Self::InnermostRepr) -> NumeraResult<Self> {
                $t::new(value).ok_or(NumeraError::Conversion)
            }
            #[inline]
            #[cfg(not(feature = "safe"))]
            unsafe fn from_innermost_repr_unchecked(value: Self::InnermostRepr) -> Self {
                $t::new_unchecked(value)
            }

            #[inline]
            fn into_inner_repr(self) -> Self::InnerRepr { self.get() }
            #[inline]
            fn into_innermost_repr(self) -> Self::InnermostRepr { self.get() }
        }
    };
}

#[rustfmt::skip]
impl_numbers![f32, f64, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize];
#[rustfmt::skip]
impl_numbers![non0
    NonZeroI8+i8, NonZeroI16+i16, NonZeroI32+i32, NonZeroI64+i64,
    NonZeroI128+i128, NonZeroIsize+isize,
    NonZeroU8+u8, NonZeroU16+u16, NonZeroU32+u32, NonZeroU64+u64,
    NonZeroU128+u128, NonZeroUsize+usize
];

#[cfg(feature = "twofloat")]
impl_numbers![@twofloat::TwoFloat];

#[cfg(feature = "half")]
impl_numbers![half::bf16, half::f16];

#[cfg(feature = "dashu-int")]
impl_numbers![dashu_int::IBig, dashu_int::UBig];
