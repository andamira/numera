// numera::integer::impl_util
//
//! implements core/std utility traits.
//

use super::{
    Integer, NegativeInteger, NonNegativeInteger, NonPositiveInteger, NonZeroInteger,
    PositiveInteger,
};
use crate::traits::{NegOne, Number, One, Signed, Zero};

use core::hash::{Hash, Hasher};

#[cfg(feature = "std")]
use std::fmt;

// impl Default

// NOTE: NonZeroInteger doesn't implement Default.

/// Default: 0.
impl<N: Number + Signed + Zero> Default for Integer<N> {
    #[inline]
    fn default() -> Self {
        Self(N::new_zero())
    }
}

/// Default: 0.
impl<N: Number + Zero> Default for NonNegativeInteger<N> {
    #[inline]
    fn default() -> Self {
        Self(N::new_zero())
    }
}
/// Default: 1.
impl<N: Number + One> Default for PositiveInteger<N> {
    #[inline]
    fn default() -> Self {
        Self(N::new_one())
    }
}
/// Default: 0.
// MAYBE:supertrait
impl<N: Number + Signed + Zero + NegOne> Default for NonPositiveInteger<N> {
    #[inline]
    fn default() -> Self {
        Self(N::new_zero())
    }
}
/// Default: -1.
// MAYBE:supertrait
impl<N: Number + Signed + NegOne> Default for NegativeInteger<N> {
    #[inline]
    fn default() -> Self {
        Self(N::new_neg_one())
    }
}

// impl Copy, Hash, Display

/// Derives copy.
macro_rules! derive_std {
    (copy: $int:ident, $($bounds:tt)+) => {
        impl<N: $($bounds)+> Copy for $int<N> {}
    };
    (hash: $int:ident, $($bounds:tt)+) => {
        // This is OK since both PartialEq & Hash are derived from the inner type:
        #[allow(clippy::derive_hash_xor_eq)]
        impl<N: $($bounds)+> Hash for $int<N> {
            fn hash<H: Hasher>(&self, hasher: &mut H) {
                self.0.hash(hasher);
            }
        }
    };
    (display: $int:ident, $($bounds:tt)+) => {
        #[cfg(feature = "std")]
        impl<N: $($bounds)+> fmt::Display for $int<N> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
}

derive_std![copy: Integer, Number + Signed + Copy];
derive_std![hash: Integer, Number + Signed + Hash];
derive_std![display: Integer, Number + Signed + fmt::Display];

derive_std![copy: NonZeroInteger, Number + Signed + Copy];
derive_std![hash: NonZeroInteger, Number + Signed + Hash];
derive_std![display: NonZeroInteger, Number + Signed + fmt::Display];

derive_std![copy: NegativeInteger, Number + Signed + Copy];
derive_std![hash: NegativeInteger, Number + Signed + Hash];
derive_std![display: NegativeInteger, Number + Signed + fmt::Display];

derive_std![copy: NonPositiveInteger, Number + Signed + Copy];
derive_std![hash: NonPositiveInteger, Number + Signed + Hash];
derive_std![display: NonPositiveInteger, Number + Signed + fmt::Display];

derive_std![copy: PositiveInteger, Number + Copy];
derive_std![hash: PositiveInteger, Number + Hash];
derive_std![display: PositiveInteger, Number + fmt::Display];

derive_std![copy: NonNegativeInteger, Number + Copy];
derive_std![hash: NonNegativeInteger, Number + Hash];
derive_std![display: NonNegativeInteger, Number + fmt::Display];
