// numera::number::real::float::f
//
//! Floating-point numbers ([m][0m]/[w][0w]), from the set $\R$.
//!
//! $\R = \lbrace …, -1, …, -0.5, …, 0, …, 0.5, …, 1, … \rbrace$
//!
//! [0m]: https://mathworld.wolfram.com/Floating-PointRepresentation.html
//! [0w]: https://en.wikipedia.org/wiki/Floating-point_arithmetic#Floating-point_numbers
//

mod convert;
// mod family;
// mod ops;
// mod real;
mod sized;

// pub use family::Float;
pub use sized::{Float32, Float64};

#[cfg(feature = "twofloat")]
pub use sized::Float128;
#[cfg(feature = "half")]
pub use sized::{BFloat16, Float16};

use crate::number::macros::define_abbreviations;

// define_abbreviations![F, Float, 32, 64]; // TODO: needs family
define_abbreviations![sized F, Float, 32];
define_abbreviations![sized F, Float, 64];

#[cfg(feature = "half")]
define_abbreviations![sized F, Float, 16];
#[cfg(feature = "half")]
define_abbreviations![sized Bf, BFloat, 16];
#[cfg(feature = "twofloat")]
define_abbreviations![sized F, Float, 128];
