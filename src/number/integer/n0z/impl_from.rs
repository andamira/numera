// numera::number::integer::n0z::impl_from
//
//!
//

use crate::number::{
    integer::{
        abbr::*,
        macros::{impl_from_integer, impl_from_primitive},
    },
    traits::Number,
};
use core::{
    num::{
        NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroU16, NonZeroU32,
        NonZeroU64, NonZeroU8,
    },
    ops::Neg,
};

/* infallibe conversions */

// from smaller NonZeroU
impl_from_primitive![many_nonzero for: N0z + 16, from: NonZeroU + 8];
impl_from_primitive![many_nonzero for: N0z + 32, from: NonZeroU + 8, 16];
impl_from_primitive![many_nonzero for: N0z + 64, from: NonZeroU + 8, 16, 32];
impl_from_primitive![many_nonzero for: N0z + 128, from: NonZeroU + 8, 16, 32, 64];
// from smaller or equal sized NonZeroI
impl_from_primitive![many_nonzero for: N0z + 8, from: NonZeroI + 8];
impl_from_primitive![many_nonzero for: N0z + 16, from: NonZeroI + 8, 16];
impl_from_primitive![many_nonzero for: N0z + 32, from: NonZeroI + 8, 16, 32];
impl_from_primitive![many_nonzero for: N0z + 64, from: NonZeroI + 8, 16, 32, 64];
impl_from_primitive![many_nonzero for: N0z + 128, from: NonZeroI + 8, 16, 32, 64, 128];

// from smaller sized PositiveInteger
impl_from_integer![many_nonzero for: N0z + i + 16, from: Pz + 8];
impl_from_integer![many_nonzero for: N0z + i + 32, from: Pz + 8, 16];
impl_from_integer![many_nonzero for: N0z + i + 64, from: Pz + 8, 16, 32];
impl_from_integer![many_nonzero for: N0z + i + 128, from: Pz + 8, 16, 32, 64];

// from smaller sized NegativeInteger
impl_from_integer![many_nonzero_neg for: N0z + i + 16, from: Nz + 8];
impl_from_integer![many_nonzero_neg for: N0z + i + 32, from: Nz + 8, 16];
impl_from_integer![many_nonzero_neg for: N0z + i + 64, from: Nz + 8, 16, 32];
impl_from_integer![many_nonzero_neg for: N0z + i + 128, from: Nz + 8, 16, 32, 64];

#[cfg(test)]
mod tests {
    // use super::*;
    // use crate::all::NumeraResult;
    //
    // #[test]
    // fn n0z_from() -> NumeraResult<()> { // TODO
    //     Ok(())
    // }
}
