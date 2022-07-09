// numera::traits
//
//! Numeric traits.
//!
//! All traits are re-exported from the root of this module,
//! and from thematic submodules.
//

mod define;

// public root re-exports

pub use crate::rational::Fraction;
pub use define::*;

// public sub-modules re-exports

/// Lower and upper bounds.
pub mod bounded {
    pub use super::define::bounded::{
        Bounded, ConstBounded, ConstLowerBounded, ConstUpperBounded, LowerBounded, UpperBounded,
    };
}

/// Related with `0`, `1` and `-1`.
pub mod identities {
    pub use super::define::identities::{
        ConstNegOne, ConstOne, ConstZero, NegOne, NonZero, One, Zero,
    };
}

/// Arithmetic operators.
pub mod ops {
    #[doc(inline)]
    pub use checked::*;

    /// Operators that returns `None` instead of panicking.
    pub mod checked {
        pub use crate::traits::define::ops::{
            CheckedAdd, CheckedDiv, CheckedMul, CheckedNeg, CheckedRem, CheckedSub,
        };
    }
}

/// General number traits.
pub mod number {
    pub use super::define::continuity::{Continuous, Discrete};
    pub use super::define::number::{Number, NumberAble};
    pub use super::define::sign::Sign;
}
