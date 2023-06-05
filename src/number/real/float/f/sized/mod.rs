// numera::number::real::float::f::sized
//
//!
//
// TOC
//
// - macro
//   - define_float_sized
// - definitions
//   - Float[32|64]

mod core;
pub use self::core::{Float32, Float64};

#[cfg(any(feature = "half", feature = "twofloat"))]
mod external;
#[cfg(feature = "twofloat")]
pub use external::Float128;
#[cfg(feature = "half")]
pub use external::{BFloat16, Float16};
