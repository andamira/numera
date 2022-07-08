// numera::number::integer::impl_std
//
//! implements std/core utility traits.
//

use super::{Integer, Negative, NonNegative, NonPositive, Positive};
use crate::number::traits::{InnerNumber, Number};
use core::ops::Neg;

use core::hash::{Hash, Hasher};

#[cfg(feature = "std")]
use std::fmt;

// impl Default

/// Default: 0.
impl<I: InnerNumber> Default for Integer<I> {
    #[inline]
    fn default() -> Self {
        Self(I::new_zero())
    }
}
/// Default: 0.
impl<I: InnerNumber> Default for NonNegative<I> {
    #[inline]
    fn default() -> Self {
        Self(I::new_zero())
    }
}
/// Default: 1.
impl<I: InnerNumber> Default for Positive<I> {
    #[inline]
    fn default() -> Self {
        Self(I::new_one())
    }
}
/// Default: 0.
impl<I: InnerNumber> Default for NonPositive<I> {
    #[inline]
    fn default() -> Self {
        Self::new(I::new_zero())
    }
}
/// Default: -1.
impl<I: InnerNumber + Neg<Output = I>> Default for Negative<I> {
    #[inline]
    fn default() -> Self {
        Self(I::new_one().neg())
    }
}

// impl Copy, clone, Hash, Display

/// Derives utility traits when the inner type supports them.
macro_rules! derive_std_traits {
    (all: $($int:ident),+) => {
        $( derive_std_traits!($int); )+
    };
    ($int:ident) => {
        impl<I: InnerNumber + Copy> Copy for $int<I> {}
        impl<I: InnerNumber + Clone> Clone for $int<I> {
            fn clone(&self) -> Self{ Self(self.0.clone()) }
        }
        // This is OK since both PartialEq & Hash are derived from the inner type:
        #[allow(clippy::derive_hash_xor_eq)]
        impl<I: InnerNumber + Hash> Hash for $int<I> {
            fn hash<H: Hasher>(&self, hasher: &mut H) {
                self.0.hash(hasher);
            }
        }
        #[cfg(feature = "std")]
        impl<I: InnerNumber + fmt::Display> fmt::Display for $int<I> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
}

derive_std_traits!(all: Integer, Negative, NonPositive, Positive, NonNegative);
