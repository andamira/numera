// numera::traits
//
//! Numeric traits.
//!
//! All traits are re-exported both from here and the thematic submodules.
//

mod define_bounds;
mod define_continuity;
mod define_identities;
mod define_number;
mod define_ops;

// re-export all here

#[doc(inline)]
pub use self::{bounds::*, identities::*, number::*, ops::*};
pub use crate::rational::Fraction;
pub use define_number::Number;

// & grouped thematically:

/// Lower and upper bounds.
pub mod bounds {
    pub use super::define_bounds::{
        Bounded, ConstBounded, ConstLowerBounded, ConstUpperBounded, LowerBounded, UpperBounded,
    };
}

/// Related with `0`, `1` and `-1`.
pub mod identities {
    pub use super::define_identities::{
        ConstNegOne, ConstOne, ConstZero, NegOne, NonZero, One, Zero,
    };
}

/// Arithmetic operators.
pub mod ops {
    #[doc(inline)]
    pub use checked::*;

    /// Operators that returns `None` instead of panicking.
    pub mod checked {
        pub use crate::traits::define_ops::{
            CheckedAdd, CheckedDiv, CheckedMul, CheckedNeg, CheckedRem, CheckedSub,
        };
    }
}

/// General number traits.
pub mod number {
    pub use super::define_continuity::{Continuous, Discrete};
    pub use super::define_number::Number;
}
