// numera::number::integer::n0z::impl_from
//
//!
//
// TOC
// - complementary primitive conversions
// - complementary integer conversions
// - fallible primitive conversions TODO
// - fallible integer conversions TODO

use crate::number::{
    integer::{
        abbr::*,
        macros::{
            impl_from_integer, impl_from_primitive, impl_try_from_integer, impl_try_from_primitive,
        },
    },
    traits::Number,
};
use core::{
    num::{
        NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroU128, NonZeroU16,
        NonZeroU32, NonZeroU64, NonZeroU8,
    },
    ops::Neg,
};

/* complementary primitive conversions */

// from smaller NonZeroU
impl_from_primitive![nonzero for: N0z + 16, from: NonZeroU + 8];
impl_from_primitive![nonzero for: N0z + 32, from: NonZeroU + 8, 16];
impl_from_primitive![nonzero for: N0z + 64, from: NonZeroU + 8, 16, 32];
impl_from_primitive![nonzero for: N0z + 128, from: NonZeroU + 8, 16, 32, 64];
// from bigger or equal sized NonZeroU
impl_try_from_primitive![nonzero for: N0z + 8, from: NonZeroU + 8, 16, 32, 64, 128];
impl_try_from_primitive![nonzero for: N0z + 16, from: NonZeroU + 16, 32, 64, 128];
impl_try_from_primitive![nonzero for: N0z + 32, from: NonZeroU + 32, 64, 128];
impl_try_from_primitive![nonzero for: N0z + 64, from: NonZeroU + 64, 128];
impl_try_from_primitive![nonzero for: N0z + 128, from: NonZeroU + 128];

// from smaller or equal sized NonZeroI
impl_from_primitive![nonzero for: N0z + 8, from: NonZeroI + 8];
impl_from_primitive![nonzero for: N0z + 16, from: NonZeroI + 8, 16];
impl_from_primitive![nonzero for: N0z + 32, from: NonZeroI + 8, 16, 32];
impl_from_primitive![nonzero for: N0z + 64, from: NonZeroI + 8, 16, 32, 64];
impl_from_primitive![nonzero for: N0z + 128, from: NonZeroI + 8, 16, 32, 64, 128];
// from bigger
impl_try_from_primitive![nonzero for: N0z + 8, from: NonZeroI + 16, 32, 64, 128];
impl_try_from_primitive![nonzero for: N0z + 16, from: NonZeroI + 32, 64, 128];
impl_try_from_primitive![nonzero for: N0z + 32, from: NonZeroI + 64, 128];
impl_try_from_primitive![nonzero for: N0z + 64, from: NonZeroI + 128];

/* complementary integer conversions */

// from smaller sized NonZeroInteger (Self)
impl_from_integer![nonzero for: N0z + i + 16, from: N0z + 8];
impl_from_integer![nonzero for: N0z + i + 32, from: N0z + 8, 16];
impl_from_integer![nonzero for: N0z + i + 64, from: N0z + 8, 16, 32];
impl_from_integer![nonzero for: N0z + i + 128, from: N0z + 8, 16, 32, 64];
// from bigger sized NonZeroInteger (Self)
impl_try_from_integer![nonzero for: N0z + i + 8, from: N0z + 16, 32, 64, 128];
impl_try_from_integer![nonzero for: N0z + i + 16, from: N0z + 32, 64, 128];
impl_try_from_integer![nonzero for: N0z + i + 32, from: N0z + 64, 128];
impl_try_from_integer![nonzero for: N0z + i + 64, from: N0z + 128];

// from smaller sized PositiveInteger
impl_from_integer![nonzero for: N0z + i + 16, from: Pz + 8];
impl_from_integer![nonzero for: N0z + i + 32, from: Pz + 8, 16];
impl_from_integer![nonzero for: N0z + i + 64, from: Pz + 8, 16, 32];
impl_from_integer![nonzero for: N0z + i + 128, from: Pz + 8, 16, 32, 64];
// from bigger or equal sized PositiveInteger
impl_try_from_integer![nonzero for: N0z + i + 8, from: Pz + 8, 16, 32, 64, 128];
impl_try_from_integer![nonzero for: N0z + i + 16, from: Pz + 16, 32, 64, 128];
impl_try_from_integer![nonzero for: N0z + i + 32, from: Pz + 32, 64, 128];
impl_try_from_integer![nonzero for: N0z + i + 64, from: Pz + 64, 128];
impl_try_from_integer![nonzero for: N0z + i + 128, from: Pz + 128];

// from smaller sized NegativeInteger
impl_from_integer![nonzero_neg for: N0z + i + 16, from: Nz + 8];
impl_from_integer![nonzero_neg for: N0z + i + 32, from: Nz + 8, 16];
impl_from_integer![nonzero_neg for: N0z + i + 64, from: Nz + 8, 16, 32];
impl_from_integer![nonzero_neg for: N0z + i + 128, from: Nz + 8, 16, 32, 64];
// from bigger or equal sized NegativeInteger
impl_try_from_integer![nonzero_neg for: N0z + i + 8, from: Nz + 8, 16, 32, 64, 128];
impl_try_from_integer![nonzero_neg for: N0z + i + 16, from: Nz + 16, 32, 64, 128];
impl_try_from_integer![nonzero_neg for: N0z + i + 32, from: Nz + 32, 64, 128];
impl_try_from_integer![nonzero_neg for: N0z + i + 64, from: Nz + 64, 128];
impl_try_from_integer![nonzero_neg for: N0z + i + 128, from: Nz + 128];

/* fallible primitive conversions */

// try_from u
impl_try_from_primitive![int for: N0z + 8, from: u + 8, 16, 32, 64, 128];
// impl_try_from_primitive![int for: N0z + 16, from: u + 8, 16, 32, 64, 128];// FIX Infallible
// impl_try_from_primitive![int for: N0z + 32, from: u + 8, 16, 32, 64, 128];
// impl_try_from_primitive![int for: N0z + 64, from: u + 8, 16, 32, 64, 128];
// impl_try_from_primitive![int for: N0z + 128, from: u + 8, 16, 32, 64, 128];

// try_from i

/* fallible integer conversions */

// try_from Integer
// impl_try_from_integer![int for: N0z + i + 8, from: Z + 16]; // FIX EXAMPLE :S
//
// impl_try_from_integer![int for: N0z + i + 8, from: Z + 8, 16, 32, 64, 128];
// impl_try_from_integer![int for: N0z + i + 16, from: Z + 8, 16, 32, 64, 128];
// impl_try_from_integer![int for: N0z + i + 32, from: Z + 8, 16, 32, 64, 128];
// impl_try_from_integer![int for: N0z + i + 64, from: Z + 8, 16, 32, 64, 128];
// impl_try_from_integer![int for: N0z + i + 128, from: Z + 8, 16, 32, 64, 128];

// try_from NonNegativeInteger

// try_from NonPositiveInteger

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
