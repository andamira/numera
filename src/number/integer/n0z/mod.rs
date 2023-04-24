// numera::number::integer::n0z
//
//! $\Z \setminus 0 \Z = \lbrace …, -2, -1, 1, 2, … \rbrace $
//!
//! [o0]: https://oeis.org/wiki/Nonzero_integers
//

mod define_sized;
mod impl_from;
mod impl_integer;
mod impl_ops;

pub use define_sized::{
    NonZeroInteger128, NonZeroInteger16, NonZeroInteger32, NonZeroInteger64, NonZeroInteger8,
};
