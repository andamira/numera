// numera::integer::z
//
//! $\Z$
//

mod define_sized;
pub use define_sized::{Integer128, Integer16, Integer32, Integer64, Integer8};

#[cfg(feature = "ibig")]
pub(super) mod define_big;

// mod impl_ops;

mod impl_integer;

mod impl_from;
