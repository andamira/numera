// numera::number::integer
//
//! Integer numbers, subsets of $\Z$.
//!
//! An *integer* ([m][0m]/[w][0w]) is a number that can be written
//! without a fractional component.
//!
//! [0m]: https://mathworld.wolfram.com/Integer.html
//! [0w]: https://en.wikipedia.org/wiki/Integer
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
        family::AnyIntegers, n0z::*, nnz::*, npz::*, nz::*, prime::*, pz::*,
        trait_integer::Integer, z::*,
    };

    #[doc(inline)]
    #[cfg(feature = "ibig")]
    pub use super::z::IntegerBig;
}
