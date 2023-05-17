// numera::number::real::float
//
//! Floating-point ([m][0w]/[w][0m]) real number representations, subsets of $\R$.
//!
//! [0m]: https://mathworld.wolfram.com/Floating-PointRepresentation.html
//! [0w]: https://en.wikipedia.org/wiki/Floating-point_arithmetic#Floating-point_numbers
//

mod define_sized;
// mod family;

pub use define_sized::{Float32, Float64};
