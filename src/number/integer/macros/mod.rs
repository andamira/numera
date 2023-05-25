// numera::number::integer::macros
//
//!
//

mod from;

#[cfg_attr(feature = "nightly", doc(cfg(feature = "try_from")))]
mod try_from;

pub(crate) use {from::*, try_from::*};
