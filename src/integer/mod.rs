// numera::integer
//
//! Integer number types.
//!
//! An *integer* ([w][0w]/[m][0m]) is a number that can be written
//! without a fractional component.
//!
//! For example, $ 21 , 4 , 0 , −2048 $ are integers,
//! while $9.75, \dfrac{1}{2} , \sqrt{2} $ are not.
//!
//! [0w]: https://en.wikipedia.org/wiki/Integer
//! [0m]: https://mathworld.wolfram.com/Integer.html

use crate::{
    macros::define_integer_prim,
    traits::{NonZero, Number, Signed},
};

mod z; // Integer

// mod n0z; // NonZeroInteger
// mod nnz; // NonNegativeInteger
// mod nz; // NegativeInteger
// mod npz; // NonPositiveInteger
// mod pz; // PositiveInteger

pub use z::*;

/// Acronyms for integer types.
pub mod a {
    use super::*;
    use crate::macros::define_acronyms;

    define_acronyms![Z, Integer, 8, 16, 32, 64, 128];
    // define_acronyms![N0z, NonZeroInteger, 8, 16, 32, 64, 128];
    // define_acronyms![Nz, NegativeInteger, 8, 16, 32, 64, 128];
    // define_acronyms![NnZ, NonNegativeInteger, 8, 16, 32, 64, 128];
    // define_acronyms![Npz, NonPositiveInteger, 8, 16, 32, 64, 128];
    // define_acronyms![Pz, PositiveInteger, 8, 16, 32, 64, 128];
}

// /// A generic *non-zero* integer number (`!= 0`), from the set $\Z \setminus 0$.
// pub struct NonZeroInteger<N: Number + Signed + NonZero>(N);
// #[rustfmt::skip]
// define_integer_prim![nonzero value, NonZeroInteger, i, // assert![value != 0];
//     "*non-zero* integer number", "(`!= 0`), from the set $\\Z \\setminus 0$.",
//     "",
//     "", MIN, MAX,
//     ("An", 8), ("A", 16), ("A", 32), ("A", 64), ("A", 128)];
//
// /// A generic *positive* integer number (`> 0`), from the set $\Z^+$.
// pub struct PositiveInteger<N: Number + Unsigned + NonZero>(N);
// #[rustfmt::skip]
// define_integer_prim![nonzero value, PositiveInteger, u, // assert![value > 0];
//     "*positive* integer number", "(`> 0`), from the set $\\Z^+$.",
//     "",
//     "", MIN, MAX,
//     ("An", 8), ("A", 16), ("A", 32), ("A", 64), ("A", 128)];
//
// /// A generic *negative* integer number (`< 0`), from the set $\Z^-$.
// pub struct NegativeInteger<N: Number + NegSigned + NonZero>(N);
// #[rustfmt::skip]
// define_integer_prim![nonzero value, NegativeInteger, u, // assert![value != 0];
//     "*negative* integer number", "(`< 0`), from the set $\\Z^-$.",
//     "**Note:** The unsigned `value` will be reinterpreted as having a *negative* sign.",
//     "-", MAX, MIN,
//     ("An", 8), ("A", 16), ("A", 32), ("A", 64), ("A", 128)];
//
// /// A generic *non-negative* integer number (`>= 0`), from the set $\Z^*$.
// pub struct NonNegativeInteger<N: Number + Unsigned>(N);
// #[rustfmt::skip]
// define_integer_prim![NonNegativeInteger, u,
//     "*non-negative* integer number", "(`>= 0`), from the set $\\Z^*$.",
//     "",
//     "", MIN, MAX,
//     ("An", 8), ("A", 16), ("A", 32), ("A", 64), ("A", 128)];
//
// /// A generic *non-positive* integer number (`<= 0`), from the set $\Z^- \cup {0}$.
// pub struct NonPositiveInteger<N: Number + NegSigned>(N);
// #[rustfmt::skip]
// define_integer_prim![NonPositiveInteger, u,
//     "*non-positive* integer number", "(`<= 0`), from the set $\\Z^- \\cup {0}$.",
//     "**Note**: The unsigned `value` will be reinterpreted as having a *negative* sign.",
//     "-", MAX, MIN,
//     ("An", 8), ("A", 16), ("A", 32), ("A", 64), ("A", 128)];
