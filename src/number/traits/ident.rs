// numera::number::traits::ident
//
//! The identity properties of numbers.
//!
//! Also implements them for primitives and supported external types.
//
// TOC
//
// - definitions
//   - *Ident*
//
//   - Zero
//   - NonZero
//   - ConstZero
//
//   - One
//   - NonOne
//   - ConstOne
//
//   - NegOne
//   - NonNegOne
//   - ConstNegOne
//
// - macros
//   - impl_ident
//
// - impls
//   - ints
//   - floats
//   - nonzero
//   - external: ibig, twofloat, half

use core::num::{
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
    NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
};

/* definitions */

/// The identity properties of a number.
///
/// # Relevant traits
/// - [`Zero`], [`NonZero`], [`ConstZero`].
/// - [`One`], [`NonOne`], [`ConstOne`].
/// - [`NegOne`], [`NonNegOne`], [`ConstNegOne`].
///
pub trait Ident {
    /// Returns `true` if the number can represent `0`,
    /// the additive identity.
    ///
    /// If `true` the type should also implement the [`Zero`] trait,
    /// otherwise the type should implement the [`NonZero`] trait.
    fn can_zero(&self) -> bool;

    /// Returns `true` if the number can represent `1`,
    /// the multiplicative identity.
    ///
    /// If `true` the type should also implement the [`One`] trait,
    /// otherwise the type should implement the [`NonOne`] trait.
    fn can_one(&self) -> bool;

    /// Returns `true` if the number can represent `-1`,
    /// the additive inverse of the multiplicative identity.
    ///
    /// If `true` the type should also implement the [`NegOne`] trait,
    /// otherwise the type should implement the [`NonNegOne`] trait.
    fn can_neg_one(&self) -> bool;

    //

    /// Returns `true` if the current value is `0`.
    /// the additive identity.
    fn is_zero(&self) -> bool;

    /// Returns `true` if the current value is `1`,
    /// the multiplicative identity.
    fn is_one(&self) -> bool;

    /// Returns `true` if the current value is `-1`,
    /// the additive inverse of the multiplicative identity.
    fn is_neg_one(&self) -> bool;
}

// 0

/// A number that can represent `0`, the additive identity.
///
/// See also: [`ConstZero`].
///
/// This trait is mutually exclusive with [`NonZero`].
pub trait Zero: Ident {
    /// Returns a new additive identity, `0`.
    fn new_zero() -> Self;

    /// Sets this number to `0`.
    #[rustfmt::skip]
    #[inline]
    fn set_zero(&mut self) where Self: Sized { *self = Self::new_zero(); }
}

/// A number that can *not* represent `0`, the additive identity.
///
/// This trait is mutually exclusive with [`Zero`] and [`ConstZero`].
pub trait NonZero: Ident {}

/// A number that supports a *const* value of `0`, the additive identity.
///
/// See also: [`Zero`].
///
/// This trait is mutually exclusive with [`NonZero`].
pub trait ConstZero: Ident {
    /// The additive identity, `0`.
    const ZERO: Self;
}

// 1

/// A number that can represent `1`, the multiplicative identity.
///
/// See also: [`ConstOne`].
///
/// This trait is mutually exclusive with [`NonOne`].
pub trait One: Ident {
    /// Returns a new multiplicative identity, `1`.
    fn new_one() -> Self;

    /// Sets this number to `1`.
    #[rustfmt::skip]
    #[inline]
    fn set_one(&mut self) where Self: Sized { *self = Self::new_one(); }
}

/// A number that can *not* represent `1`, the multiplicative identity.
///
/// This trait is mutually exclusive with [`One`] and [`ConstOne`].
pub trait NonOne: Ident {}

/// A number that supports a *const* value of `1`, the multiplicative identity.
///
/// See also: [`One`].
///
/// This trait is mutually exclusive with [`NonOne`].
pub trait ConstOne: Ident {
    /// The multiplicative identity, `1`.
    const ONE: Self;
}

// -1

/// A number that can represent `-1`,
/// the additive inverse of the multiplicative identity.
///
/// See also: [`ConstNegOne`].
///
/// This trait is mutually exclusive with [`NonNegOne`].
pub trait NegOne: Ident {
    /// Returns a new additive inverse of the multiplicative identity, `-1`.
    fn new_neg_one() -> Self;

    /// Sets this number to `-1`.
    #[rustfmt::skip]
    fn set_neg_one(&mut self) where Self: Sized { *self = Self::new_neg_one(); }
}

/// A number that can *not* represent `-1`,
/// the additive inverse of the multiplicative identity.
///
/// This trait is mutually exclusive with [`NegOne`] and [`ConstNegOne`].
pub trait NonNegOne: Ident {}

/// A number that supports a *const* value of `-1`,
/// the additive inverse of the multiplicative identity.
///
/// See also: [`NegOne`].
///
/// This trait is mutually exclusive with [`NonNegOne`].
pub trait ConstNegOne: Ident {
    /// The additive inverse of the multiplicative identity, `-1`.
    const NEG_ONE: Self;
}

/* macros*/

/// impl the identity traits for integer, float & nonzero primitives.
//
// TOC
// - [many_]float              : [Const][Zero|One|NegOne]
// - [many_]signed_int         : [Const][Zero|One|NegOne]
// - [many_]unsigned_int       : [Const][Zero|One|NonNegOne]
// - [many_]signed_nonzero     : [Const][NonZero|One|NegOne]
// - [many_]unsigned_nonzero   : [Const][NonZero|One|NonNegOne]
// - [many_]signed_nonconst    : [Zero|One|NegOne]
// - [many_]unsigned_nonconst  : [Zero|One|NonNegOne]
macro_rules! impl_ident {
    // impl [Const][Zero|One|NegOne] for floating-point primitives.
    (many_float: $($t:ty, $zero:expr, $one:expr, $neg_one:expr),+) => {
        $( impl_ident![float: $t, $zero, $one, $neg_one]; )+
    };
    (float: $t:ty, $zero:expr, $one:expr, $neg_one:expr) => {
        impl Ident for $t {
            #[inline]
            fn can_zero(&self) -> bool { true }
            #[inline]
            fn can_one(&self) -> bool { true }
            #[inline]
            fn can_neg_one(&self) -> bool { true }

            #[inline]
            fn is_zero(&self) -> bool {
                #[cfg(feature = "std")]
                return (*self).abs() <= <$t>::EPSILON;
                #[cfg(not(feature = "std"))]
                if self.is_sign_positive() {
                    *self <= <$t>::EPSILON
                } else {
                    *self >= <$t>::EPSILON
                }
            }

            #[inline]
            fn is_one(&self) -> bool {
                #[cfg(feature = "std")]
                return (*self - 1.0).abs() <= <$t>::EPSILON;

                #[cfg(not(feature = "std"))]
                if self.is_sign_positive() {
                    *self -1. <= <$t>::EPSILON
                } else {
                    *self -1. >= <$t>::EPSILON
                }
            }

            #[inline]
            fn is_neg_one(&self) -> bool {
                #[cfg(feature = "std")]
                return (*self + 1.0).abs() <= <$t>::EPSILON;

                #[cfg(not(feature = "std"))]
                if self.is_sign_positive() {
                    *self +1. <= <$t>::EPSILON
                } else {
                    *self +1. >= <$t>::EPSILON
                }
            }
        }

        impl ConstZero for $t { const ZERO: Self = $zero; }
        impl Zero for $t { fn new_zero() -> Self { $zero } }

        impl ConstOne for $t { const ONE: Self = $one; }
        impl One for $t { fn new_one() -> Self { $one } }

        impl ConstNegOne for $t { const NEG_ONE: Self = $neg_one; }
        impl NegOne for $t { fn new_neg_one() -> Self { $neg_one } }
    };

    // impl [Const][Zero|One|NegOne] for signed integer primitives.
    (many_signed_int: $($t:ty, $zero:expr, $one:expr, $neg_one:expr),+) => {
        $( impl_ident![signed_int: $t, $zero, $one, $neg_one]; )+
    };
    (signed_int: $t:ty, $zero:expr, $one:expr, $neg_one:expr) => {
        impl Ident for $t {
            #[inline]
            fn can_zero(&self) -> bool { true }
            #[inline]
            fn can_one(&self) -> bool { true }
            #[inline]
            fn can_neg_one(&self) -> bool { true }

            #[inline]
            fn is_zero(&self) -> bool { *self == $zero }
            #[inline]
            fn is_one(&self) -> bool { *self == $one }
            #[inline]
            fn is_neg_one(&self) -> bool { *self == $neg_one }
        }
        impl ConstZero for $t { const ZERO: Self = $zero; }
        impl ConstOne for $t { const ONE: Self = $one; }
        impl ConstNegOne for $t { const NEG_ONE: Self = $neg_one; }
        impl Zero for $t {
            #[inline]
            fn new_zero() -> Self { $zero }
        }
        impl One for $t {
            #[inline]
            fn new_one() -> Self { $one }
        }
        impl NegOne for $t {
            #[inline]
            fn new_neg_one() -> Self { $neg_one }
        }
    };

    // impl [Const][Zero|One|NonNegOne] for unsigned integer primitives.
    (many_unsigned_int: $($t:ty, $zero:expr, $one:expr),+) => {
        $( impl_ident![unsigned_int: $t, $zero, $one]; )+
    };
    (unsigned_int: $t:ty, $zero:expr, $one:expr) => {
        impl Ident for $t {
            #[inline]
            fn can_zero(&self) -> bool { true }
            #[inline]
            fn can_one(&self) -> bool { true }
            #[inline]
            fn can_neg_one(&self) -> bool { false }

            #[inline]
            fn is_zero(&self) -> bool { *self == $zero }
            #[inline]
            fn is_one(&self) -> bool { *self == $one }
            #[inline]
            fn is_neg_one(&self) -> bool { false }
        }

        impl ConstZero for $t { const ZERO: Self = $zero; }
        impl Zero for $t {
            #[inline]
            fn new_zero() -> Self { $zero }
        }

        impl ConstOne for $t { const ONE: Self = $one; }
        impl One for $t {
            #[inline]
            fn new_one() -> Self { $one }
        }

        impl NonNegOne for $t {}
    };

    // impl [Const][NonZero|One|NegOne] for signed nonzero primitives.
    (many_signed_nonzero: $($t:ty, $one:expr, $neg_one:expr),+) => {
        $( impl_ident![signed_nonzero: $t, $one, $neg_one]; )+
    };
    (signed_nonzero: $t:ty, $one:expr, $neg_one:expr) => {
        impl Ident for $t {
            fn can_zero(&self) -> bool { false }
            fn can_one(&self) -> bool { true }
            fn can_neg_one(&self) -> bool { true }

            fn is_zero(&self) -> bool { false }
            fn is_one(&self) -> bool { self.get() == $one }
            fn is_neg_one(&self) -> bool { self.get() == $neg_one }
        }

        impl NonZero for $t {}

        impl ConstOne for $t {
            #[cfg(feature = "safe")]
            const ONE: Self = if let Some(n) = <$t>::new($one)
                { n } else { unreachable!() };

            #[cfg(not(feature = "safe"))]
            // SAFETY: constant value
            const ONE: Self = unsafe { <$t>::new_unchecked($one) };
        }
        impl One for $t {
            #[cfg(feature = "safe")]
            fn new_one() -> Self { <$t>::new($one).unwrap() }

            #[cfg(not(feature = "safe"))]
            // SAFETY: constant value
            fn new_one() -> Self { unsafe { <$t>::new_unchecked($one) } }
        }

        impl ConstNegOne for $t {
            #[cfg(feature = "safe")]
            const NEG_ONE: Self = if let Some(n) = <$t>::new($neg_one)
                { n } else { unreachable!() };

            #[cfg(not(feature = "safe"))]
            // SAFETY: constant value
            const NEG_ONE: Self = unsafe { <$t>::new_unchecked($neg_one) };
        }
        impl NegOne for $t {
            #[inline]
            #[cfg(feature = "safe")]
            fn new_neg_one() -> Self { <$t>::new($neg_one).unwrap() }

            #[inline]
            #[cfg(not(feature = "safe"))]
            // SAFETY: constant value
            fn new_neg_one() -> Self { unsafe { <$t>::new_unchecked($neg_one) } }
        }
    };

    // impl [Const][NonZero|One|NonNegOne] for unsigned nonzero primitives.
    (many_unsigned_nonzero: $($t:ty, $one:expr),+) => {
        $( impl_ident![unsigned_nonzero: $t, $one]; )+
    };
    (unsigned_nonzero: $t:ty, $one:expr) => {
        impl Ident for $t {
            #[inline]
            fn can_zero(&self) -> bool { false }
            #[inline]
            fn can_one(&self) -> bool { true }
            #[inline]
            fn can_neg_one(&self) -> bool { false }

            #[inline]
            fn is_zero(&self) -> bool { false }
            #[inline]
            fn is_one(&self) -> bool { self.get() == $one }
            #[inline]
            fn is_neg_one(&self) -> bool { false }
        }

        impl NonZero for $t {}

        impl ConstOne for $t {
            #[cfg(feature = "safe")]
            const ONE: Self = if let Some(n) = <$t>::new($one)
                { n } else { unreachable!() };

            #[cfg(not(feature = "safe"))]
            // SAFETY: constant value
            const ONE: Self = unsafe { <$t>::new_unchecked($one) };
        }
        impl One for $t {
            #[inline]
            #[cfg(feature = "safe")]
            fn new_one() -> Self { <$t>::new($one).unwrap() }

            #[inline]
            #[cfg(not(feature = "safe"))]
            // SAFETY: constant value
            fn new_one() -> Self { unsafe { <$t>::new_unchecked($one) } }
        }

        impl NonNegOne for $t {}
    };

    // impl non-const [NonZero|One|NegOne].
    (many_signed_nonconst: $($t:ty, $zero:expr, $one:expr, $neg_one:expr),+) => {
        $( impl_ident![signed_nonconst: $t, $zero, $one, $neg_one]; )+
    };
    (signed_nonconst: $t:ty, $zero:expr, $one:expr, $neg_one:expr) => {
        impl Ident for $t {
            #[inline]
            fn can_zero(&self) -> bool { true }
            #[inline]
            fn can_one(&self) -> bool { true }
            #[inline]
            fn can_neg_one(&self) -> bool { true }

            #[inline]
            fn is_zero(&self) -> bool { *self == $zero }
            #[inline]
            fn is_one(&self) -> bool { *self == $one }
            #[inline]
            fn is_neg_one(&self) -> bool { *self == $neg_one }
        }

        impl Zero for $t {
            #[inline]
            fn new_zero() -> Self { $zero }
        }

        impl One for $t {
            #[inline]
            fn new_one() -> Self { $one }
        }

        impl NegOne for $t {
            #[inline]
            fn new_neg_one() -> Self { $neg_one }
        }
    };

    // impl non-const [NonZero|One].
    (many_unsigned_nonconst: $($t:ty, $zero:expr, $one:expr),+) => {
        $( impl_ident![unsigned_nonconst: $t, $zero, $one]; )+
    };
    (unsigned_nonconst: $t:ty, $zero:expr, $one:expr) => {
        impl Ident for $t {
            #[inline]
            fn can_zero(&self) -> bool { true }
            #[inline]
            fn can_one(&self) -> bool { true }
            #[inline]
            fn can_neg_one(&self) -> bool { false }

            #[inline]
            fn is_zero(&self) -> bool { *self == $zero }
            #[inline]
            fn is_one(&self) -> bool { *self == $one }
            #[inline]
            fn is_neg_one(&self) -> bool { false }
        }

        impl Zero for $t { fn new_zero() -> Self { $zero } }

        impl One for $t { fn new_one() -> Self { $one } }

        impl NonNegOne for $t {}
    };
}

/* impls */

// 0, 1, -1
#[rustfmt::skip]
impl_ident![many_signed_int:
    i8, 0, 1, -1, i16, 0, 1, -1, i32, 0, 1, -1, i64, 0, 1, -1, i128, 0, 1, -1, isize, 0, 1, -1];

// 0, 1
#[rustfmt::skip]
impl_ident![many_unsigned_int:
    u8, 0, 1, u16, 0, 1, u32, 0, 1, u64, 0, 1, u128, 0, 1, usize, 0, 1];

// 0, 1, -1
#[rustfmt::skip]
impl_ident![many_float: f32, 0.0, 1.0, -1.0, f64, 0.0, 1.0, -1.0];

// !0, 1, -1
#[rustfmt::skip]
impl_ident![many_signed_nonzero:
    NonZeroI8, 1, -1, NonZeroI16, 1, -1, NonZeroI32, 1, -1,
    NonZeroI64, 1, -1, NonZeroI128, 1, -1, NonZeroIsize, 1, -1];

// !0, 1
#[rustfmt::skip]
impl_ident![many_unsigned_nonzero:
    NonZeroU8, 1, NonZeroU16, 1, NonZeroU32, 1, NonZeroU64, 1, NonZeroU128, 1, NonZeroUsize, 1];

/* impls for external types */

#[rustfmt::skip]
#[cfg(feature = "ibig")]
mod impl_ibig {
    use super::*;
    use ibig::{IBig, UBig};

    impl_ident![signed_nonconst: IBig, IBig::from(0_i8), IBig::from(1_i8), IBig::from(-1_i8)];
    impl_ident![unsigned_nonconst: UBig, UBig::from(0_u8), UBig::from(1_u8)];
}

#[rustfmt::skip]
#[cfg(feature = "twofloat")]
mod impl_twofloat {
    use super::*;
    use twofloat::TwoFloat;

    impl Ident for TwoFloat {
        fn can_zero(&self) -> bool { true }
        fn can_one(&self) -> bool { true }
        fn can_neg_one(&self) -> bool { true }

        fn is_zero(&self) -> bool { self == &Self::new_zero() }
        fn is_one(&self) -> bool { self == &Self::new_one() }
        fn is_neg_one(&self) -> bool { self == &Self::new_neg_one() }
    }

    impl ConstZero for TwoFloat { const ZERO: Self = Self::from_f64(0.0); }
    impl ConstOne for TwoFloat { const ONE: Self = Self::from_f64(1.0); }
    impl ConstNegOne for TwoFloat { const NEG_ONE: Self = Self::from_f64(-1.0); }

    impl Zero for TwoFloat {
        fn new_zero() -> Self { TwoFloat::from(0.0) }
    }
    impl One for TwoFloat {
        fn new_one() -> Self { TwoFloat::from(1.0) }
    }
    impl NegOne for TwoFloat {
        fn new_neg_one() -> Self { TwoFloat::from(-1.0) }
    }
}

#[rustfmt::skip]
#[cfg(feature = "half")]
mod impl_half {
    use super::*;
    use half::{bf16, f16};

    macro_rules! impl_const_onezero {
        ($($t:ty),+) => {
            $(
            impl Ident for $t {
                fn can_zero(&self) -> bool { true }
                fn can_one(&self) -> bool { true }
                fn can_neg_one(&self) -> bool { true }

                fn is_zero(&self) -> bool { self != &Self::new_zero() }
                fn is_one(&self) -> bool { self != &Self::new_one() }
                fn is_neg_one(&self) -> bool { self != &Self::new_neg_one() }
            }

            impl ConstZero for $t { const ZERO: Self = Self::from_f32_const(0.0); }
            impl ConstOne for $t { const ONE: Self = Self::from_f32_const(1.0); }
            impl ConstNegOne for $t { const NEG_ONE: Self = Self::from_f32_const(-1.0); }

            impl Zero for $t {
                fn new_zero() -> Self { <$t>::from_f32_const(0.0) }
            }
            impl One for $t {
                fn new_one() -> Self { <$t>::from_f32_const(1.0) }
            }
            impl NegOne for $t {
                fn new_neg_one() -> Self { <$t>::from_f32_const(-1.0) }
            }
            )+
        };
    }
    impl_const_onezero![bf16, f16];
}

/// Tests
// TODO: CHECK
#[cfg(test)]
mod tests {
    use super::*;
    use static_assertions::assert_impl_all;

    /// Checks the `[Const][One|Zero]` traits for primitives.
    #[test]
    fn onezero_primitives() {
        macro_rules! assert_impl_onezero {
            (both: $($t:ty),+) => {
                assert_impl_onezero![@const: $($t),+];
                assert_impl_onezero![@nonconst: $($t),+];
            };
            (@const: $($t:ty),+) => {
                $( assert_impl_all![$t: ConstOne, ConstZero];)+
            };
            (@nonconst: $($t:ty),+) => {
                $( assert_impl_all![$t: One, Zero];)+
            };
        }
        assert_impl_onezero![both: i8, i16, i32, i64, i128, isize];
        assert_impl_onezero![both: u8, u16, u32, u64, u128, usize];
        assert_impl_onezero![both: f32, f64];

        #[cfg(feature = "twofloat")]
        assert_impl_onezero![@nonconst: twofloat::TwoFloat];

        #[cfg(feature = "half")]
        assert_impl_onezero![both: half::f16, half::bf16];
    }

    /// Checks the `[Const][NegOne]` traits for primitives.
    #[test]
    fn neg1_primitives() {
        macro_rules! assert_impl_neg1 {
            (both: $($t:ty),+) => {
                assert_impl_neg1![@const: $($t),+];
                assert_impl_neg1![@nonconst: $($t),+];
            };
            (@const: $($t:ty),+) => {
                $( assert_impl_all![$t: ConstNegOne];)+
            };
            (@nonconst: $($t:ty),+) => {
                $( assert_impl_all![$t: NegOne];)+
            };
        }
        assert_impl_neg1![both: i8, i16, i32, i64, i128, isize];
        assert_impl_neg1![both: f32, f64];

        #[cfg(feature = "twofloat")]
        assert_impl_neg1![@nonconst: twofloat::TwoFloat];

        #[cfg(feature = "half")]
        assert_impl_neg1![@nonconst: half::f16, half::bf16];
        // WIP assert_impl_neg1![both: half::f16, half::bf16];
    }
}
