// numera::integer::impl_util
//
//! implements core/std utility traits.
//

use super::{Integer, NegativeInteger, NonNegativeInteger, NonPositiveInteger, PositiveInteger};
use crate::traits::{NegOne, Number, One, Zero};

use core::hash::{Hash, Hasher};

#[cfg(feature = "std")]
use std::fmt;

// impl Default

// NOTE: NonZeroInteger doesn't implement Default.

/// Default: 0.
impl<I: Number + Zero> Default for Integer<I> {
    #[inline]
    fn default() -> Self {
        Self(I::new_zero())
    }
}

/// Default: 0.
impl<I: Number + Zero> Default for NonNegativeInteger<I> {
    #[inline]
    fn default() -> Self {
        Self(I::new_zero())
    }
}
/// Default: 1.
impl<I: Number + One> Default for PositiveInteger<I> {
    #[inline]
    fn default() -> Self {
        Self(I::new_one())
    }
}
/// Default: 0.
impl<I: Number + Zero + NegOne> Default for NonPositiveInteger<I> {
    #[inline]
    fn default() -> Self {
        Self(I::new_zero())
    }
}
/// Default: -1.
impl<I: Number + NegOne> Default for NegativeInteger<I> {
    #[inline]
    fn default() -> Self {
        Self(I::new_neg_one())
    }
}

// impl Copy, Hash, Display

/// Derives utility traits when the inner type supports them.
macro_rules! derive_std_traits {
    (all: $($int:ident),+) => {
        $( derive_std_traits!($int); )+
    };
    ($int:ident) => {
        impl<I: Number + Copy> Copy for $int<I> {}
        // This is OK since both PartialEq & Hash are derived from the inner type:
        #[allow(clippy::derive_hash_xor_eq)]
        impl<I: Number + Hash> Hash for $int<I> {
            fn hash<H: Hasher>(&self, hasher: &mut H) {
                self.0.hash(hasher);
            }
        }
        #[cfg(feature = "std")]
        impl<I: Number + fmt::Display> fmt::Display for $int<I> {
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
