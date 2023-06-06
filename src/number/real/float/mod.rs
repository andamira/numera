// numera::number::real::float
//
//! Floating-point ([m][0m]/[w][0w]) real number representations, subsets of $\R$.
//!
//! [0m]: https://mathworld.wolfram.com/Floating-PointRepresentation.html
//! [0w]: https://en.wikipedia.org/wiki/Floating-point_arithmetic#Floating-point_numbers
//

// mod family;
pub(crate) mod fns;
pub(crate) mod macros;
// mod reals;

// pub mod n0f;
// pub mod nnf;
// pub mod npf;
// pub mod nf;
// pub mod pf;
pub mod f;

pub use all::*;
pub(crate) mod all {
    #[doc(inline)]
    pub use super::f::*;
    // pub use super::{family::*, f::*, reals::Reals};
}
