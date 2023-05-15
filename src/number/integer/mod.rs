// numera::number::integer
//
//! Integer numbers, subsets of $\Z$.
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
        family::AnyIntegers, n0z::*, nnz::*, npz::*, nz::*, prime::*, pz::*,
        trait_integer::Integer, z::*,
    };

    #[doc(inline)]
    #[cfg(feature = "ibig")]
    pub use super::z::IntegerBig;
}
