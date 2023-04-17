// numera::number::traits::bounds
//
//! The bound properties of numbers.
//!
//! Also implements them for all the supported primitives and external types.
//
// TOC
//
// - definitions
//   - *Bound*
//
//   - Bounded
//   - LowerBounded
//   - UpperBounded
//
//   - ConstBounded
//   - ConstLowerBounded
//   - ConstUpperBounded
//
//   - NonBounded
//   - NonLowerBounded
//   - NonUpperBounded
//
// - macros
//   - impl_bounded
//
// - impls

use core::num::{
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
    NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
};

/* definitions */

/// The bound properties of a number.
///
/// # Relevant traits
/// - [`LowerBounded`], [`UpperBounded`], [`Bounded`].
/// - [`ConstLowerBounded`], [`ConstUpperBounded`], [`ConstBounded`].
/// - [`NonLowerBounded`], [`NonUpperBounded`], [`NonBounded`].
pub trait Bound: PartialEq {
    /// Returns true if the number is lower bounded.
    fn is_lower_bounded(&self) -> bool;

    /// Returns true if the number is upper bounded.
    fn is_upper_bounded(&self) -> bool;

    /// Returns the lower bound, if any.
    fn lower_bound(&self) -> Option<Self>
    where
        Self: Sized;

    /// Returns the upper bound, if any.
    fn upper_bound(&self) -> Option<Self>
    where
        Self: Sized;
}

/// Both lower and upper bounds. Automatic trait
pub trait Bounded: LowerBounded + UpperBounded {}

/// A lower bound.
pub trait LowerBounded: Bound {
    /// The smallest value that can be represented with this type.
    fn new_min() -> Self;
}

/// An upper bound.
pub trait UpperBounded: Bound {
    /// The largest value that can be represented with this type.
    fn new_max() -> Self;
}

/// Both *const* lower and upper bounds. Automatic trait.
pub trait ConstBounded: ConstLowerBounded + ConstUpperBounded {}

/// A *const* lower bound.
pub trait ConstLowerBounded: Bound {
    /// The smallest value that can be represented with this type.
    const MIN: Self;
}

/// A *const* upper bound.
pub trait ConstUpperBounded: Bound {
    /// The smallest value that can be represented with this type.
    const MAX: Self;
}

/// Is *not* lower bounded.
pub trait NonLowerBounded: Bound {}

/// Is *not* upper bounded.
pub trait NonUpperBounded: Bound {}

/// Is *not* either lower or upper bounded.
pub trait NonBounded: NonLowerBounded + NonUpperBounded {}

/* macros */

/// implement both *const* & *non-const*, lower and/or upper `Bounded` traits.
macro_rules! impl_bounded {
    (many_both: $($t:ty),+) => {
        $( impl_bounded![both: $t]; )+
    };
    (both: $t:ty) => {
        impl Bound for $t {
            fn is_lower_bounded(&self) -> bool { true }
            fn is_upper_bounded(&self) -> bool { true }
            fn lower_bound(&self) -> Option<Self> where Self: Sized { Some(<$t>::MIN) }
            fn upper_bound(&self) -> Option<Self> where Self: Sized { Some(<$t>::MAX) }
        }
        impl ConstLowerBounded for $t { const MIN: Self = <$t>::MIN; }
        impl LowerBounded for $t { fn new_min() -> Self { <$t>::MIN }}
        impl ConstUpperBounded for $t { const MAX: Self = <$t>::MAX; }
        impl UpperBounded for $t { fn new_max() -> Self { <$t>::MAX }}
    };
}

/// implement both *const* & *non-const*, lower and/or upper `Bounded` traits
/// for non-zero primitives.
macro_rules! impl_bounded_nonzero {
    (many_both: $($t:ty, $lb:expr, $ub:expr),+) => {
        $( impl_bounded_nonzero![both: $t, $lb, $ub]; )+
    };
    (both: $t:ty, $lb:expr, $ub:expr) => {
        impl Bound for $t {
            fn is_lower_bounded(&self) -> bool { true }
            fn is_upper_bounded(&self) -> bool { true }
            fn lower_bound(&self) -> Option<Self> { Some(<$t as ConstLowerBounded>::MIN) }
            fn upper_bound(&self) -> Option<Self> { Some(<$t as ConstUpperBounded>::MAX) }
        }
        // SAFETY: we use a known valid constant
        impl ConstLowerBounded for $t { const MIN: Self = unsafe { <$t>::new_unchecked($lb) }; }
        impl LowerBounded for $t { fn new_min() -> Self { unsafe { <$t>::new_unchecked($lb) } } }
        impl ConstUpperBounded for $t { const MAX: Self = unsafe { <$t>::new_unchecked($ub) }; }
        impl UpperBounded for $t { fn new_max() -> Self { unsafe { <$t>::new_unchecked($ub) } } }
    };
}

/// implement just the *non-const* `LowerBounded` or `UpperBounded` traits.
//
// Used for `ibig::UBig`.
#[rustfmt::skip]
#[allow(unused_macros)]
macro_rules! impl_nonconst_bounded {
    // impl only non-const lower bound `$b`
    (only_lower: $t:ty, $b:expr) => {
        impl Bound for $t {
            fn is_lower_bounded(&self) -> bool { true }
            fn is_upper_bounded(&self) -> bool { false }
            fn lower_bound(&self) -> Option<Self> { Some($b) }
            fn upper_bound(&self) -> Option<Self> { None }
        }
        impl LowerBounded for $t { fn new_min() -> Self { $b } }
        impl NonUpperBounded for UBig {}
    };

    // impl only non-const upper bound `$b`
    (only_upper: $t:ty, $b:expr) => {
        impl Bound for $t {
            fn is_lower_bounded(&self) -> bool { false }
            fn is_upper_bounded(&self) -> bool { true }
            fn lower_bound(&self) -> Option<Self> { None }
            fn upper_bound(&self) -> Option<Self> { Some($b) }
        }
        impl UpperBounded for $t { fn new_max() -> Self { $b } }
        impl NonLowerBounded for UBig {}
    };
}

/// implement `NonBounded`.
//
// Used for `ibig::IBig`.
#[allow(unused_macros)]
macro_rules! impl_nonbounded {
    ($t:ty) => {
        impl Bound for $t {
            fn is_lower_bounded(&self) -> bool {
                false
            }
            fn is_upper_bounded(&self) -> bool {
                false
            }
            fn lower_bound(&self) -> Option<Self> {
                None
            }
            fn upper_bound(&self) -> Option<Self> {
                None
            }
        }
        impl NonUpperBounded for $t {}
        impl NonLowerBounded for $t {}
    };
}

/* impls */

/// auto-impl `Bounded`.
impl<T: LowerBounded + UpperBounded> Bounded for T {}

/// auto-impl `ConstBounded`.
impl<T: ConstLowerBounded + ConstUpperBounded> ConstBounded for T {}

/// auto-impl `NonBounded`.
impl<T: NonLowerBounded + NonUpperBounded> NonBounded for T {}

#[rustfmt::skip]
impl_bounded![many_both:
    f32, f64, i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize];

#[rustfmt::skip]
impl_bounded_nonzero![many_both:
    NonZeroU8, 1, u8::MAX, NonZeroU16, 1, u16::MAX, NonZeroU32, 1, u32::MAX,
    NonZeroU64, 1, u64::MAX, NonZeroU128, 1, u128::MAX, NonZeroUsize, 1, usize::MAX,
    NonZeroI8, i8::MIN, i8::MAX, NonZeroI16, i16::MIN, i16::MAX,
    NonZeroI32, i32::MIN, i32::MAX, NonZeroI64, i64::MIN, i64::MAX,
    NonZeroI128, i128::MIN, i128::MAX, NonZeroIsize, isize::MIN, isize::MAX
];

/* impls external */

#[cfg(feature = "twofloat")]
impl_bounded![both: twofloat::TwoFloat];

#[cfg(feature = "half")]
impl_bounded![many_both: half::bf16, half::f16];

#[cfg(feature = "ibig")]
mod impl_ibig {
    use super::*;
    use ibig::{IBig, UBig};

    impl_nonconst_bounded![only_lower: UBig, UBig::from(0u8)];

    impl_nonbounded![IBig];
}