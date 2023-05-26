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
mod fns;
mod integer_trait;
pub mod prime;

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
        family::AnyIntegers, integer_trait::Integer, n0z::*, nnz::*, npz::*, nz::*, prime::*,
        pz::*, z::*,
    };

    #[doc(inline)]
    #[cfg(feature = "ibig")]
    pub use super::{
        fns::{bit_len, bit_len_next_power},
        z::IntegerBig,
    };
}
