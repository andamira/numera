// numera::number::traits
//
//!
//

mod bounded;
mod identities;
mod number;
mod sign;

pub use bounded::{
    Bounded, ConstBounded, ConstLowerBounded, ConstUpperBounded, LowerBounded, UpperBounded,
};
pub use identities::{ConstNegOne, ConstOne, ConstZero, NegOne, NonZero, One, Zero};
pub use number::{InnerNumber, Number};
pub use sign::Sign;
