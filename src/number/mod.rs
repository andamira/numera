// numera::number
//
//! All kinds of numbers.
//!
//! A *number* is a mathematical object used to count, measure, and label
//! ([w][1w]/[m][1m]).
//!
//! [1w]: https://en.wikipedia.org/wiki/Number
//! [1m]: https://mathworld.wolfram.com/Number.html
//

pub(crate) mod macros;

pub mod integer;
pub mod rational;
pub mod real;
pub mod traits;

mod family;
mod no;

#[doc(inline)]
pub use all::{
    AnyInteger, AnyNumber, AnyRational, Bound, Count, Ident, Integer, Integer32, Integer64,
    NoNumber, Number, Numbers, Prime, Prime16, Prime32, Rational, Rational32, Rational64, Sign,
};
pub(crate) mod all {
    pub(crate) use super::macros::*;
    #[doc(inline)]
    pub use super::{
        family::*, integer::all::*, no::NoNumber, rational::all::*, real::all::*, traits::all::*,
    };
}
