// numera::number::integer::z
//
//! $\Z$
//

mod define_sized;
mod impl_from;
mod impl_integer;
mod impl_ops;

pub use define_sized::{Integer128, Integer16, Integer32, Integer64, Integer8};

#[cfg(feature = "ibig")]
pub(super) mod define_big;
