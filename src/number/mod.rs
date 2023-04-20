// numera::number
//
//! Numbers.
//!
//! A *number* is a mathematical object used to count, measure, and label
//! ([w][1w]/[m][1m]).
//!
//! [1w]: https://en.wikipedia.org/wiki/Number
//! [1m]: https://mathworld.wolfram.com/Number.html
//

mod any;
mod no;

pub mod integer;
pub mod traits;

pub use all::{
    Bound, Count, Ident, Integer, Integer128, Integer16, Integer32, Integer64, Integer8, Integers,
    Number, Sign,
};
pub(crate) mod all {
    #[doc(inline)]
    pub use super::{
        any::{AnyNumbers, Numbers},
        integer::all::*,
        traits::*,
    };
}
