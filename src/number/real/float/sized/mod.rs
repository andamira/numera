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

#[cfg(feature = "half")]
mod f16;
#[cfg(feature = "half")]
// pub use external::Float16;
pub use f16::{BFloat16, Float16};

#[cfg(feature = "twofloat")]
mod f128;
#[cfg(feature = "twofloat")]
pub use f128::Float128;
