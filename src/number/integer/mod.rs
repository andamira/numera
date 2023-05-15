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

pub use all::*;
pub(crate) mod all {
    #[doc(inline)]
    pub use super::{
        family::AnyIntegers,
        n0z::{
            N0z128, N0z16, N0z32, N0z64, N0z8, NonZeroInteger128, NonZeroInteger16,
            NonZeroInteger32, NonZeroInteger64, NonZeroInteger8, NonZeroIntegers,
        },
        nnz::{
            Nnz128, Nnz16, Nnz32, Nnz64, Nnz8, NonNegativeInteger128, NonNegativeInteger16,
            NonNegativeInteger32, NonNegativeInteger64, NonNegativeInteger8, NonNegativeIntegers,
        },
        npz::{
            NonPositiveInteger128, NonPositiveInteger16, NonPositiveInteger32,
            NonPositiveInteger64, NonPositiveInteger8, NonPositiveIntegers, Npz128, Npz16, Npz32,
            Npz64, Npz8,
        },
        nz::{
            NegativeInteger128, NegativeInteger16, NegativeInteger32, NegativeInteger64,
            NegativeInteger8, NegativeIntegers, Nz128, Nz16, Nz32, Nz64, Nz8,
        },
        prime::*,
        pz::{
            PositiveInteger128, PositiveInteger16, PositiveInteger32, PositiveInteger64,
            PositiveInteger8, PositiveIntegers, Pz128, Pz16, Pz32, Pz64, Pz8,
        },
        trait_integer::Integer,
        z::{
            Integer128, Integer16, Integer32, Integer64, Integer8, Integers, Z128, Z16, Z32, Z64,
            Z8,
        },
    };

    #[doc(inline)]
    #[cfg(feature = "ibig")]
    pub use super::z::IntegerBig;
}
