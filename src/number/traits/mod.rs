// numera::number::traits
//
//! All general number related traits.
//

pub mod bound;
pub mod count;
pub mod ident;
pub mod number;
pub mod sign;

pub use all::*;
pub(crate) mod all {
    #[doc(inline)]
    pub use super::{
        bound::{
            Bound, Bounded, ConstBounded, ConstLowerBounded, ConstUpperBounded, LowerBounded,
            NonBounded, NonLowerBounded, NonUpperBounded, UpperBounded,
        },
        count::{Count, Countable, Uncountable},
        ident::{
            ConstNegOne, ConstOne, ConstZero, Ident, NegOne, NonNegOne, NonOne, NonZero, One, Zero,
        },
        number::Number,
        sign::{NegSigned, Sign, Signed, Unsigned},
    };
}
