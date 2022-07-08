// numera::::traits
//
//!
//

mod bounded;
mod continuity;
mod identities;
mod number;
mod ops;
mod sign;

pub use bounded::{
    Bounded, ConstBounded, ConstLowerBounded, ConstUpperBounded, LowerBounded, UpperBounded,
};
pub use continuity::{Continuous, Discrete};
pub use identities::{ConstNegOne, ConstOne, ConstZero, NegOne, NonZero, One, Zero};
pub use number::{Number, NumberAble};
pub use ops::{CheckedAdd, CheckedDiv, CheckedMul, CheckedNeg, CheckedRem, CheckedSub};
pub use sign::Sign;
