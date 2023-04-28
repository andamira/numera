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

mod family;
mod no;

pub mod integer;
pub mod traits;

#[doc(inline)]
pub use all::{
    AnyIntegers, AnyNumbers, Bound, Count, Ident, Integer, Integer32, Integer64, Number, Numbers,
    Prime16, Prime32, Sign,
};
pub(crate) mod all {
    #[doc(inline)]
    pub use super::{
        family::{AnyNumbers, Numbers},
        integer::all::*,
        traits::all::*,
    };
}
