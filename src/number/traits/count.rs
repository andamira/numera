// numera::number::traits::count
//
//! The countability properties of numbers.
//!
//! Also implements them for all the supported primitives and external types.
//
// TOC
//
// - definitions
//   - *Count*
//   - Countable
//   - Uncountable
//
// - macros
//   - impl_countable
//   - impl_uncountable
//
// - impls
//
// - tests

use core::num::{
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
    NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
};

use crate::error::{IntegerErrors, NumeraErrors, NumeraResult};

/// The countability properties of a number.
///
/// # Relevant traits
/// - [`Countable`].
/// - [`Uncountable`].
///
/// These two traits are mutually exclusive with each other.
pub trait Count {
    /// Returns `true` if the number is countable.
    fn is_countable(&self) -> bool;

    /// Returns `false` if the number is countable.
    fn is_uncountable(&self) -> bool {
        !self.is_countable()
    }
}

/// A number that is *countable*.
///
/// This trait is mutually exclusive with [`Uncountable`].
pub trait Countable: Count {
    /// Returns the next countable value.
    ///
    /// # Errors
    /// Errors if the operation results in overflow or an invalid value.
    #[rustfmt::skip]
    fn next(&self) -> NumeraResult<Self> where Self: Sized;

    /// Returns the previous countable value.
    ///
    /// # Errors
    /// Errors if the operation results in underflow or an invalid value.
    #[rustfmt::skip]
    fn previous(&self) -> NumeraResult<Self> where Self: Sized;
}

/// A number that is *uncountable*.
///
/// This trait is mutually exclusive with [`Countable`].
pub trait Uncountable: Count {}

/* macros */

/// implement `Countable` & `Uncountable` traits for the numeric primitives.
macro_rules! impl_countable {
    // impl `Countable` for integer primitives
    (many_integer: $($t:ty),+) => { $( impl_countable![integer: $t]; )+ };
    (integer: $t:ty) => {
        impl Count for $t {
            fn is_countable(&self) -> bool { true }
        }
        impl Countable for $t {
            fn next(&self) -> NumeraResult<Self> {
                self.checked_add(1).ok_or(IntegerErrors::Overflow.into())
            }
            fn previous(&self) -> NumeraResult<Self> {
                self.checked_sub(1).ok_or(IntegerErrors::Underflow.into())
            }
        }
    };

    // impl `Countable` for non-zero integer primitives
    (many_nonzero: $($t:ty),+) => { $( impl_countable![nonzero: $t]; )+ };
    (nonzero: $t:ty) => {
        impl Count for $t {
            fn is_countable(&self) -> bool { true }
        }
        impl Countable for $t {
            fn next(&self) -> NumeraResult<Self> {
                let mut value = self.get().checked_add(1)
                    .ok_or::<NumeraErrors>(IntegerErrors::Overflow.into())?;
                if value == 0 {
                    value = value.checked_add(1)
                        .ok_or::<NumeraErrors>(IntegerErrors::Overflow.into())?;
                }

                #[cfg(feature = "safe")]
                return Ok(<$t>::new(value).unwrap());

                #[cfg(not(feature = "safe"))]
                // SAFETY: we just checked the value
                return Ok(unsafe { <$t>::new_unchecked(value) });
            }
            fn previous(&self) -> NumeraResult<Self> {
                let mut value = self.get().checked_sub(1)
                    .ok_or::<NumeraErrors>(IntegerErrors::Underflow.into())?;
                if value == 0 {
                    value = value.checked_sub(1)
                        .ok_or::<NumeraErrors>(IntegerErrors::Underflow.into())?;
                }

                #[cfg(feature = "safe")]
                return Ok(<$t>::new(value).unwrap());

                #[cfg(not(feature = "safe"))]
                // SAFETY: we just checked the value
                return Ok(unsafe { <$t>::new_unchecked(value) });
            }
        }
    };
}
macro_rules! impl_uncountable {
    (many_float: $($t:ty),+) => { $( impl_uncountable![float: $t]; )+ };
    (float: $t:ty) => {
        impl Count for $t {
            fn is_countable(&self) -> bool { false }
        }
        impl Uncountable for $t { }
    };
}

/* impls */

impl_uncountable![many_float: f32, f64];

#[rustfmt::skip]
impl_countable![many_integer:
    i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize];

#[rustfmt::skip]
impl_countable![many_nonzero:
    NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize,
    NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize];

/* impls external */

#[cfg(feature = "twofloat")]
impl_uncountable![many_float: twofloat::TwoFloat];

#[cfg(feature = "half")]
impl_uncountable![many_float: half::bf16, half::f16];

#[cfg(feature = "dashu-int")]
#[rustfmt::skip]
mod impl_big {
    use core::ops::{Add, Sub};
    use super::{Count, Countable};
    use dashu_int::{IBig, UBig, ops::BitTest};
    use crate::error::{IntegerErrors, NumeraResult};

    impl Count for IBig {
        fn is_countable(&self) -> bool { true }
    }
    impl Countable for IBig {
        fn next(&self) -> NumeraResult<Self> { Ok(self.add(1)) }
        fn previous(&self) -> NumeraResult<Self> { Ok(self.sub(1)) }
    }

    impl Count for UBig {
        fn is_countable(&self) -> bool { true }
    }
    impl Countable for UBig {
        fn next(&self) -> NumeraResult<Self> { Ok(self.add(1_u8)) }
        fn previous(&self) -> NumeraResult<Self> {
            if self.bit_len() > 0 { Ok(self.sub(1_u8)) }
            else { Err(IntegerErrors::LessThanZero.into()) }
        }
    }
}

/* tests */

// IMPROVE
#[cfg(test)]
mod tests {
    use super::*;
    use static_assertions::*;

    macro_rules! assert_impl {
        (uncountable: $($t:ty),+) => {
            $( assert_impl_all![$t: Uncountable];)+
        };
        (countable: $($t:ty),+) => {
            $( assert_impl_all![$t: Countable];)+
        };
        // BUG:static_assertions
        // (not_uncountable: $($t:ty),+) => {
        //     $( assert_not_impl_all![$t: Uncountable];)+
        // };
        // (not_countable: $($t:ty),+) => {
        //     $( assert_not_impl_all![$t: Countable];)+
        // };
    }

    #[rustfmt::skip]
    #[test]
    fn countable() {
        assert_impl![countable:
            i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize];

        #[cfg(feature = "dashu-int")]
        assert_impl![countable: dashu_int::IBig, dashu_int::UBig];
    }

    #[test]
    fn uncountable() {
        assert_impl![uncountable: f32, f64];
        // BUG:static_assertions
        // assert_not_impl_all![f32: Countable];

        #[cfg(feature = "half")]
        assert_impl![uncountable: half::bf16, half::f16];

        #[cfg(feature = "twofloat")]
        assert_impl![uncountable: twofloat::TwoFloat];
    }
}
