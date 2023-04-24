// numera::integer
//
//! All integer number types.
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

mod any;
pub mod prime;
mod trait_integer;
mod z;

mod n0z;

// mod nnz;
// mod nz;
// mod npz;
// mod pz;

pub use all::*;
pub(crate) mod all {
    #[doc(inline)]
    pub use super::{
        any::Integers,
        prime::*,
        trait_integer::Integer,
        z::{Integer128, Integer16, Integer32, Integer64, Integer8},
        n0z::{NonZeroInteger128, NonZeroInteger16, NonZeroInteger32, NonZeroInteger64, NonZeroInteger8},
    };

    #[cfg(feature = "ibig")]
    pub use super::z::define_big::IntegerBig;
}
