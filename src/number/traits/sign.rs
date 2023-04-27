// numera::number::traits::sign
//
//! The sign properties of numbers.
//!
//! Also implements them for all the supported primitives and external types.
//
// TOC
//
// - definitions
//   - *Sign*
//
//   - Signed
//   - Unsigned
//   - NegSigned
//
// - macros
//   - impl_sign
//
// - impls

use crate::error::NumeraResult;
use core::num::{
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
    NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
};

/* definitions */

/// The sign properties of a number.
///
/// # Relevant traits
/// - [`Signed`].
/// - [`Unsigned`].
/// - [`NegSigned`].
///
/// If a type can represent both positive and negative numbers it must implement
/// the [`Signed`] trait. Otherwise, if it can only represent positive numbers
/// it must implement instead the [`Unsigned`] trait. Otherwise if it can only
/// represent negative numbers it must implement the [`NegSigned`] trait.
///
/// These three traits are mutually exclusive with each other.
pub trait Sign {
    /// Returns `true` if the type can represent positive numbers.
    fn can_positive(&self) -> bool;

    /// Returns `true` if the type can represent negative numbers.
    fn can_negative(&self) -> bool;

    /// Returns `true` if the value is positive ($> 0$).
    fn is_positive(&self) -> bool;

    /// The value is negative ($< 0$).
    fn is_negative(&self) -> bool;
}

/// A number that can represent both positive and negative numbers.
///
/// This trait is mutually exclusive with [`Unsigned`] and [`NegSigned`].
pub trait Signed: Sign {}

/// A number that can *not* represent negative numbers.
///
/// This trait is mutually exclusive with [`Signed`] and [`NegSigned`].
pub trait Unsigned: Sign {}

/// A number that can *not* represent positive numbers.
///
/// This trait is mutually exclusive with [`Unsigned`] and [`Signed`].
pub trait NegSigned: Sign {
    /// The number's inner value representation.
    type Inner;

    /// Returns a new number that contains the negation of the `value`.
    ///
    /// This allows using an unsigned type value to store only negative numbers.
    ///
    /// # Errors
    /// If the provided `value` doesn't maintain the expected invariances for
    /// the concrete type.
    fn new_neg(value: Self::Inner) -> NumeraResult<Self>
    where
        Self: Sized;
}

/* macros */

/// implement `Signed` & `Unsigned` traits for the numeric primitives.
macro_rules! impl_sign {
    // impl `Signed` for signed integer primitives
    (many_signed_prim: $($t:ty),+) => {
        $( impl_sign![signed_prim: $t]; )+
    };
    (signed_prim: $t:ty) => {
        impl Sign for $t {
            fn can_negative(&self) -> bool { true }
            fn can_positive(&self) -> bool { true }
            fn is_negative(&self) -> bool { <$t>::is_negative(*self) }
            fn is_positive(&self) -> bool { <$t>::is_positive(*self) }
        }
        impl Signed for $t {}
    };

    // impl `Signed` for floating-point primitives
    (many_signed_float: $($t:ty, $zero:expr),+) => {
        $( impl_sign![signed_float: $t, $zero]; )+
    };
    (signed_float: $t:ty, $zero:expr) => {
        impl Sign for $t {
            fn can_negative(&self) -> bool { true }
            fn can_positive(&self) -> bool { true }
            fn is_negative(&self) -> bool { self.is_sign_negative() && *self != $zero }
            fn is_positive(&self) -> bool { self.is_sign_positive() && *self != $zero }
        }
        impl Signed for $t {}
    };

    // impl `Signed` for non-zero signed integer primitives
    (many_signed_nonzero: $($t:ty),+) => {
        $( impl_sign![signed_nonzero: $t]; )+
    };
    (signed_nonzero: $t:ty) => {
        impl Sign for $t {
            fn can_negative(&self) -> bool { true }
            fn can_positive(&self) -> bool { true }
            fn is_negative(&self) -> bool { self.get().is_negative() }
            fn is_positive(&self) -> bool { self.get().is_positive() }
        }
        impl Signed for $t {}
    };

    /* Unsigned */

    // impl Unsigned for unsigned integer primitives
    (many_unsigned_prim: $($t:ty),+) => {
        $( impl_sign![unsigned_prim: $t]; )+
    };
    (unsigned_prim: $t:ty) => {
        impl Sign for $t {
            fn can_negative(&self) -> bool { false }
            fn can_positive(&self) -> bool { true }
            fn is_negative(&self) -> bool { false }
            fn is_positive(&self) -> bool { *self != 0 }
        }
        impl Unsigned for $t {}
    };

    // impl Unsigned for unsigned nonzero primitives
    (many_unsigned_nonzero: $($t:ty),+) => {
        $( impl_sign![unsigned_nonzero: $t]; )+
    };
    (unsigned_nonzero: $t:ty) => {
        impl Sign for $t {
            fn can_negative(&self) -> bool { false }
            fn can_positive(&self) -> bool { true }
            fn is_negative(&self) -> bool { false }
            fn is_positive(&self) -> bool { true } // can't be 0
        }
        impl Unsigned for $t {}
    };

}

/* impls */

impl_sign![many_signed_prim: i8, i16, i32, i64, i128, isize];
impl_sign![many_unsigned_prim: u8, u16, u32, u64, u128, usize];
impl_sign![many_signed_float: f32, 0.0, f64, 0.0];

#[rustfmt::skip]
impl_sign![many_signed_nonzero:
    NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize];
#[rustfmt::skip]
impl_sign![many_unsigned_nonzero:
    NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize];

/* impls external*/

#[cfg(feature = "ibig")]
#[rustfmt::skip]
mod impl_ibig {
    use super::{Sign, Signed, Unsigned};
    use ibig::{IBig, UBig};

    impl Sign for IBig {
        fn can_negative(&self) -> bool { true }
        fn can_positive(&self) -> bool { true }
        fn is_negative(&self) -> bool { *self < IBig::from(0u8) }
        fn is_positive(&self) -> bool { *self > IBig::from(0u8) }
    }
    impl Signed for IBig {}

    impl Sign for UBig {
        fn can_negative(&self) -> bool { false }
        fn can_positive(&self) -> bool { true }
        fn is_negative(&self) -> bool { false }
        fn is_positive(&self) -> bool { *self != UBig::from(0u8) }
    }
    impl Unsigned for UBig {}
}

#[cfg(feature = "twofloat")]
mod impl_twofloat {
    use super::{Sign, Signed};
    use twofloat::TwoFloat;
    impl_sign![signed_float: TwoFloat, TwoFloat::from_f64(0.0)];
}

#[cfg(feature = "half")]
mod impl_half {
    use super::{Sign, Signed};
    use half::{bf16, f16};
    impl_sign![many_signed_float: f16, f16::ZERO, bf16, bf16::ZERO];
}
