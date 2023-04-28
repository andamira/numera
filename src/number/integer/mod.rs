// numera::integer
//
//! All kinds of integers, subsets of $\Z$.
//!
//! An *integer* ([w][0w]/[m][0m]) is a number that can be written
//! without a fractional component.
//!
//! For example, $ 21 , 4 , 0 , −2048 $ are integers,
//! while $9.75, \dfrac{1}{2} , \sqrt{2} $ are not.
//!
//! [0w]: https://en.wikipedia.org/wiki/Integer
//! [0m]: https://mathworld.wolfram.com/Integer.html
//

pub(crate) mod macros;

mod family;
pub mod prime;
mod trait_integer;

pub mod n0z;
pub mod nnz;
pub mod npz;
pub mod nz;
pub mod pz;
pub mod z;

/// Abbreviations for all integers.
pub mod abbr {
    #[doc(inline)]
    pub use super::{
        n0z::abbr::*, nnz::abbr::*, npz::abbr::*, nz::abbr::*, pz::abbr::*, z::abbr::*,
    };
}

pub use all::*;
pub(crate) mod all {
    #[doc(inline)]
    pub use super::{
        family::AnyIntegers,
        n0z::{
            NonZeroInteger128, NonZeroInteger16, NonZeroInteger32, NonZeroInteger64,
            NonZeroInteger8, NonZeroIntegers,
        },
        nnz::{
            NonNegativeInteger128, NonNegativeInteger16, NonNegativeInteger32,
            NonNegativeInteger64, NonNegativeInteger8, NonNegativeIntegers,
        },
        npz::{
            NonPositiveInteger128, NonPositiveInteger16, NonPositiveInteger32,
            NonPositiveInteger64, NonPositiveInteger8, NonPositiveIntegers,
        },
        nz::{
            NegativeInteger128, NegativeInteger16, NegativeInteger32, NegativeInteger64,
            NegativeInteger8, NegativeIntegers,
        },
        prime::*,
        pz::{
            PositiveInteger128, PositiveInteger16, PositiveInteger32, PositiveInteger64,
            PositiveInteger8, PositiveIntegers,
        },
        trait_integer::Integer,
        z::{Integer128, Integer16, Integer32, Integer64, Integer8, Integers},
    };

    #[doc(inline)]
    #[cfg(feature = "ibig")]
    pub use super::z::IntegerBig;
}
