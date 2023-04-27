// numera::number::integer::npz
//
//! An only *non-positive* integer number ([m][0m]), from the set $\Z^- \cup {0}$
//! (`> 0`).
//!
//! $ {0} \cup Z^- = \lbrace …, -2, -1, 0 \rbrace $
//!
//! [0m]: https://mathworld.wolfram.com/NonpositiveInteger.html
//

mod define_sized;
mod family;
mod impl_from;
mod impl_integer;
mod impl_ops;

pub use define_sized::{
    NonPositiveInteger128, NonPositiveInteger16, NonPositiveInteger32, NonPositiveInteger64,
    NonPositiveInteger8,
};
pub use family::NonPositiveIntegers;
