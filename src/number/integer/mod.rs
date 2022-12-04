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
//

mod impl_from;
pub(crate) use impl_from::impl_from;

mod trait_integer;
pub use trait_integer::Integer;

mod any;
pub use any::Integers;

// mod prime;

mod z;
#[cfg(feature = "ibig")]
pub use z::define_big::IntegerBig;
pub use z::{Integer128, Integer16, Integer32, Integer64, Integer8};

// mod n0z;
// pub use n0z::NonZeroInteger8;

// mod nnz;
// mod nz;
// mod npz;
// mod pz;
