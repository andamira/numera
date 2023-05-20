// numera::number::real::float
//
//! Floating-point ([m][0m]/[w][0w]) real number representations, subsets of $\R$.
//!
//! [0m]: https://mathworld.wolfram.com/Floating-PointRepresentation.html
//! [0w]: https://en.wikipedia.org/wiki/Floating-point_arithmetic#Floating-point_numbers
//

mod define_sized;
// mod family;
pub(crate) mod fns;

pub use define_sized::{Float32, Float64};

use crate::number::macros::define_abbreviations;
define_abbreviations![many F, Float, 32, 64];
