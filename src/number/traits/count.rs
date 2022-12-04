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

use crate::error::{Error, IntegerError, Result};

/// The countability properties of a number.
///
/// # Relevant traits
/// - [`Countable`].
/// - [`Uncountable`].
///
pub trait Count {
    /// Returns `true` if the number is countable.
    fn is_countable(&self) -> bool;

    /// Returns `false` if the number is countable.
    fn is_uncountable(&self) -> bool {
        !self.is_countable()
    }
}

/// A *countable* number.
pub trait Countable: Count {
    /// Returns the next countable value.
    ///
    /// # Errors
    /// Errors if the operation results in overflow or an invalid value.
    #[rustfmt::skip]
    fn next(&self) -> Result<Self> where Self: Sized;

    /// Returns the previous countable value.
    ///
    /// # Errors
    /// Errors if the operation results in underflow or an invalid value.
    #[rustfmt::skip]
    fn previous(&self) -> Result<Self> where Self: Sized;
}

/// An *uncountable* number.
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
            fn next(&self) -> Result<Self> {
                self.checked_add(1).ok_or(IntegerError::Overflow.into())
            }
            fn previous(&self) -> Result<Self> {
                self.checked_sub(1).ok_or(IntegerError::Underflow.into())
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
            fn next(&self) -> Result<Self> {
                let mut value = self.get().checked_add(1)
                    .ok_or::<Error>(IntegerError::Overflow.into())?;
                if value == 0 {
                    value = value.checked_add(1)
                        .ok_or::<Error>(IntegerError::Overflow.into())?;
                }
                // SAFETY: we just checked the value
                Ok(unsafe { <$t>::new_unchecked(value) })
            }
            fn previous(&self) -> Result<Self> {
                let mut value = self.get().checked_sub(1)
                    .ok_or::<Error>(IntegerError::Underflow.into())?;
                if value == 0 {
                    value = value.checked_sub(1)
                        .ok_or::<Error>(IntegerError::Underflow.into())?;
                }
                // SAFETY: we just checked the value
                Ok(unsafe { <$t>::new_unchecked(value) })
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

#[cfg(feature = "ibig")]
#[rustfmt::skip]
mod impl_ibig {
    use core::ops::{Add, Sub};
    use super::{Count, Countable};
    use ibig::{IBig, UBig};
    use crate::error::{IntegerError, Result};

    impl Count for IBig {
        fn is_countable(&self) -> bool { true }
    }
    impl Countable for IBig {
        fn next(&self) -> Result<Self> { Ok(self.add(1)) }
        fn previous(&self) -> Result<Self> { Ok(self.sub(1)) }
    }

    impl Count for UBig {
        fn is_countable(&self) -> bool { true }
    }
    impl Countable for UBig {
        fn next(&self) -> Result<Self> { Ok(self.add(1)) }
        fn previous(&self) -> Result<Self> {
            if self.bit_len() > 0 { Ok(self.sub(1)) } 
            else { Err(IntegerError::LessThanZero.into()) }
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

        #[cfg(feature = "ibig")]
        assert_impl![countable: ibig::IBig, ibig::UBig];
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
