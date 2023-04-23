// numera::number::integer::prime::any
//
//!
//

use super::{Prime16, Prime32, Prime8};
use crate::all::{
    Bound, ConstLowerBounded, ConstUpperBounded, Count, Countable, Ident, LowerBounded, NonNegOne,
    NonOne, NonZero, Number, NumeraResult, Sign, Unsigned, UpperBounded,
};

/// The family of primes.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Primes {
    Prime8(Prime8),
    Prime16(Prime16),
    Prime32(Prime32),
    // Prime64(Prime64),
    // PrimeBig(PrimeBig),
}

/* number impl */

/// This implementation is no-op.
impl Number for Primes {
    type Inner = Self;
    fn new(value: Primes) -> NumeraResult<Self> {
        Ok(value)
    }
    unsafe fn new_unchecked(value: Primes) -> Self {
        value
    }
    fn into_inner(self) -> Self::Inner {
        self
    }
    fn ref_inner(&self) -> &Self::Inner {
        self
    }
}

/// This implementation defers to the actual integer variant.
impl Bound for Primes {
    fn is_lower_bounded(&self) -> bool {
        use Primes::*;
        match self {
            Prime8(p) => p.is_lower_bounded(),
            Prime16(p) => p.is_lower_bounded(),
            Prime32(p) => p.is_lower_bounded(),
        }
    }
    fn is_upper_bounded(&self) -> bool {
        use Primes::*;
        match self {
            Prime8(p) => p.is_upper_bounded(),
            Prime16(p) => p.is_upper_bounded(),
            Prime32(p) => p.is_upper_bounded(),
        }
    }
    fn lower_bound(&self) -> Option<Self> {
        use Primes::*;
        match self {
            Prime8(p) => p.lower_bound().map(|p| p.into()),
            Prime16(p) => p.lower_bound().map(|p| p.into()),
            Prime32(p) => p.lower_bound().map(|p| p.into()),
        }
    }
    fn upper_bound(&self) -> Option<Self> {
        use Primes::*;
        match self {
            Prime8(p) => p.upper_bound().map(|p| p.into()),
            Prime16(p) => p.upper_bound().map(|p| p.into()),
            Prime32(p) => p.upper_bound().map(|p| p.into()),
        }
    }
}

impl LowerBounded for Primes {
    fn new_min() -> Primes {
        Primes::Prime8(Prime8::new_min())
    }
}
impl ConstLowerBounded for Primes {
    const MIN: Self = Primes::Prime8(Prime8::MIN);
}
impl UpperBounded for Primes {
    fn new_max() -> Primes {
        Primes::Prime32(Prime32::new_max())
    }
}
impl ConstUpperBounded for Primes {
    const MAX: Self = Primes::Prime32(Prime32::MAX);
}

impl Count for Primes {
    fn is_countable(&self) -> bool {
        true
    }
}

impl Countable for Primes {
    fn next(&self) -> NumeraResult<Self> {
        use Primes::*;
        match self {
            // Prime8(p) => Prime8(p.),
            Prime8(p) => p.next().map(|p| p.into()),
            Prime16(p) => p.next().map(|p| p.into()),
            #[cfg(feature = "std")]
            Prime32(p) => p.next().map(|p| p.into()),
            // IMPROVE for no-std
            #[cfg(not(feature = "std"))]
            _ => Err(crate::all::NumeraError::NotImplemented),
        }
    }
    fn previous(&self) -> NumeraResult<Self> {
        use Primes::*;
        match self {
            // Prime8(p) => Prime8(p.),
            Prime8(p) => p.previous().map(|p| p.into()),
            Prime16(p) => p.previous().map(|p| p.into()),
            #[cfg(feature = "std")]
            Prime32(p) => p.previous().map(|p| p.into()),
            // IMPROVE for no-std
            #[cfg(not(feature = "std"))]
            _ => Err(crate::all::NumeraError::NotImplemented),
        }
    }
}

impl Ident for Primes {
    fn can_zero(&self) -> bool {
        false
    }
    fn can_one(&self) -> bool {
        false
    }
    fn can_neg_one(&self) -> bool {
        false
    }
    fn is_zero(&self) -> bool {
        false
    }
    fn is_one(&self) -> bool {
        false
    }
    fn is_neg_one(&self) -> bool {
        false
    }
}

impl NonZero for Primes {}
impl NonOne for Primes {}
impl NonNegOne for Primes {}

impl Sign for Primes {
    fn can_positive(&self) -> bool {
        true
    }
    fn can_negative(&self) -> bool {
        false
    }
    fn is_positive(&self) -> bool {
        true
    }
    fn is_negative(&self) -> bool {
        false
    }
}
impl Unsigned for Primes {}
