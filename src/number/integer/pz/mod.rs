// numera::number::integer::pz
//
//! An only *positive* integer number ([m][0m]), from the set $\Z^+$ (`> 0`).
//!
//! $ \Z^+ = \lbrace 1, 2, … \rbrace $
//!
//! Doesn't include 0.
//!
//! Sometimes called *Natural number* ([w][1w]) or *counting number*,
//! but in that case it can be confounded with [`NonNegativeInteger`].
//!
//! [0m]: https://mathworld.wolfram.com/PositiveInteger.html
//! [1w]: https://en.wikipedia.org/wiki/Natural_number
//
mod define_sized;
mod family;
mod impl_from;
mod impl_integer;
mod impl_ops;

pub use define_sized::{
    PositiveInteger128, PositiveInteger16, PositiveInteger32, PositiveInteger64, PositiveInteger8,
};
pub use family::PositiveIntegers;
