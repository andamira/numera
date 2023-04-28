// numera::number::integer::n0z
//
//! *Non-zero* integer numbers ([o][o0]), from the set $\Z \setminus 0$
//! (`!= 0`).
//!
//! $ \Z = \lbrace …, -2, -1, 1, 2, … \rbrace $
//!
//! This type corresponds with the *signed* `NonZero` primitives
//! ([`NonZeroI8`][core::num::NonZeroI8]…[`NonZeroI128`][core::num::NonZeroI128]).
//!
//! [o0]: https://oeis.org/wiki/Nonzero_integers
//

mod define_sized;
mod family;
mod impl_from;
mod impl_integer;
mod impl_ops;

pub use define_sized::{
    NonZeroInteger128, NonZeroInteger16, NonZeroInteger32, NonZeroInteger64, NonZeroInteger8,
};
pub use family::NonZeroIntegers;

/// Abbreviations for non-zero integers.
pub mod abbr {
    use super::*;
    use crate::number::macros::define_abbreviations;

    define_abbreviations![many N0z, NonZeroInteger, 8, 16, 32, 64, 128];
}
