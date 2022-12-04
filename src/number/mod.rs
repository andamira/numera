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
pub use any::{AnyNumbers, Numbers};

mod no;
pub use no::NoNumber;

pub mod traits;
#[doc(inline)]
pub use traits::Number;

pub mod integer;

// pub mod complex;
// pub mod rational;
// pub mod real;
