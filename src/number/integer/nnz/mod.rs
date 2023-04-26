// numera::number::integer::nnz
//
//! An only *non-negative* integer number ([m][0m]/[o][0o]), from the set $\Z^*$
//! (`>= 0`).
//!
//! $ \Z^* = \lbrace 0, 1, 2, … \rbrace $
//!
//! Sometimes called *Natural number* ([w][1w]) or *counting number*,
//! but those can be confounded with [`PositiveInteger`]s.
//!
//! This type corresponds with the unsigned primitives ([`u8`]…[`u128`]).
//!
//! [0m]: https://mathworld.wolfram.com/NonnegativeInteger.html
//! [0o]: http://oeis.org/A001477
//! [1w]: https://en.wikipedia.org/wiki/Natural_number
//

mod define_sized;
mod impl_from;
mod impl_integer;
mod impl_ops;

pub use define_sized::{
    NonNegativeInteger128, NonNegativeInteger16, NonNegativeInteger32, NonNegativeInteger64,
    NonNegativeInteger8,
};
