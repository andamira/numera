// numera::number::integer::impl_std
//
//! implements std/core utility traits.
//

use super::{Integer, NegativeInteger, NonNegativeInteger, NonPositiveInteger, PositiveInteger};
use crate::number::traits::{NegOne, Number, NumberAble, One, Zero};

use core::hash::{Hash, Hasher};

#[cfg(feature = "std")]
use std::fmt;

// impl Default

// NOTE: NonZeroInteger doesn't implement Default.

/// Default: 0.
impl<I: NumberAble + Zero> Default for Integer<I> {
    #[inline]
    fn default() -> Self {
        Self(I::new_zero())
    }
}

/// Default: 0.
impl<I: NumberAble + Zero> Default for NonNegativeInteger<I> {
    #[inline]
    fn default() -> Self {
        Self(I::new_zero())
    }
}
/// Default: 1.
impl<I: NumberAble + One> Default for PositiveInteger<I> {
    #[inline]
    fn default() -> Self {
        Self(I::new_one())
    }
}
/// Default: 0.
impl<I: NumberAble + Zero + NegOne> Default for NonPositiveInteger<I> {
    #[inline]
    fn default() -> Self {
        Self::new(I::new_zero())
    }
}
/// Default: -1.
impl<I: NumberAble + NegOne> Default for NegativeInteger<I> {
    #[inline]
    fn default() -> Self {
        Self(I::new_neg_one())
    }
}

// impl Copy, clone, Hash, Display

/// Derives utility traits when the inner type supports them.
macro_rules! derive_std_traits {
    (all: $($int:ident),+) => {
        $( derive_std_traits!($int); )+
    };
    ($int:ident) => {
        impl<I: NumberAble + Copy> Copy for $int<I> {}
        impl<I: NumberAble + Clone> Clone for $int<I> {
            fn clone(&self) -> Self{ Self(self.0.clone()) }
        }
        // This is OK since both PartialEq & Hash are derived from the inner type:
        #[allow(clippy::derive_hash_xor_eq)]
        impl<I: NumberAble + Hash> Hash for $int<I> {
            fn hash<H: Hasher>(&self, hasher: &mut H) {
                self.0.hash(hasher);
            }
        }
        #[cfg(feature = "std")]
        impl<I: NumberAble + fmt::Display> fmt::Display for $int<I> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
}

#[rustfmt::skip]
derive_std_traits![all:
    Integer, NegativeInteger, NonPositiveInteger, PositiveInteger, NonNegativeInteger
];
