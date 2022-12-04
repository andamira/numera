// numera::number::traits
//
//! General number related traits.
//

pub mod bound;
pub mod count;
pub mod ident;
pub mod number;
pub mod sign;

#[doc(inline)]
pub use bound::{
    Bound, Bounded, ConstBounded, ConstLowerBounded, ConstUpperBounded, LowerBounded, NonBounded,
    NonLowerBounded, NonUpperBounded, UpperBounded,
};
#[doc(inline)]
pub use count::{Count, Countable, Uncountable};
#[doc(inline)]
pub use ident::{
    ConstNegOne, ConstOne, ConstZero, Ident, NegOne, NonNegOne, NonOne, NonZero, One, Zero,
};
#[doc(inline)]
pub use number::Number;
#[doc(inline)]
pub use sign::{NegSigned, Sign, Signed, Unsigned};
