// numera::number::integer::z::impl_from
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
}; // NonZeroU128,

/* infallible conversions */

// from smaller u
impl_from_primitive![many for: Z + 16, from: u + 8];
impl_from_primitive![many for: Z + 32, from: u + 8, 16];
impl_from_primitive![many for: Z + 64, from: u + 8, 16, 32];
impl_from_primitive![many for: Z + 128, from: u + 8, 16, 32, 64];
// from smaller or equal sized i
impl_from_primitive![many for: Z + 8, from: i + 8];
impl_from_primitive![many for: Z + 16, from: i + 8, 16];
impl_from_primitive![many for: Z + 32, from: i + 8, 16, 32];
impl_from_primitive![many for: Z + 64, from: i + 8, 16, 32, 64];
impl_from_primitive![many for: Z + 128, from: i + 8, 16, 32, 64, 128];

// from smaller NonZeroU
impl_from_primitive![many_nonzero for: Z + 16, from: NonZeroU + 8];
impl_from_primitive![many_nonzero for: Z + 32, from: NonZeroU + 8, 16];
impl_from_primitive![many_nonzero for: Z + 64, from: NonZeroU + 8, 16, 32];
impl_from_primitive![many_nonzero for: Z + 128, from: NonZeroU + 8, 16, 32, 64];
// from smaller or equal sized NonZeroI
impl_from_primitive![many_nonzero for: Z + 8, from: NonZeroI + 8];
impl_from_primitive![many_nonzero for: Z + 16, from: NonZeroI + 8, 16];
impl_from_primitive![many_nonzero for: Z + 32, from: NonZeroI + 8, 16, 32];
impl_from_primitive![many_nonzero for: Z + 64, from: NonZeroI + 8, 16, 32, 64];
impl_from_primitive![many_nonzero for: Z + 128, from: NonZeroI + 8, 16, 32, 64, 128];

// from smaller sized Integer (Self)
impl_from_integer![many_int for: Z + i + 16, from: Z + 8];
impl_from_integer![many_int for: Z + i + 32, from: Z + 8, 16];
impl_from_integer![many_int for: Z + i + 64, from: Z + 8, 16, 32];
impl_from_integer![many_int for: Z + i + 128, from: Z + 8, 16, 32, 64];

// from smaller or equal sized NonZeroInteger
impl_from_integer![many_nonzero for: Z + i + 8, from: N0z + 8];
impl_from_integer![many_nonzero for: Z + i + 16, from: N0z + 8, 16];
impl_from_integer![many_nonzero for: Z + i + 32, from: N0z + 8, 16, 32];
impl_from_integer![many_nonzero for: Z + i + 64, from: N0z + 8, 16, 32, 64];
impl_from_integer![many_nonzero for: Z + i + 128, from: N0z + 8, 16, 32, 64, 128];

// from smaller sized NonNegativeInteger
impl_from_integer![many_int for: Z + i + 16, from: Nnz + 8];
impl_from_integer![many_int for: Z + i + 32, from: Nnz + 8, 16];
impl_from_integer![many_int for: Z + i + 64, from: Nnz + 8, 16, 32];
impl_from_integer![many_int for: Z + i + 128, from: Nnz + 8, 16, 32, 64];

// from smaller sized PositiveInteger
impl_from_integer![many_nonzero for: Z + i + 16, from: Pz + 8];
impl_from_integer![many_nonzero for: Z + i + 32, from: Pz + 8, 16];
impl_from_integer![many_nonzero for: Z + i + 64, from: Pz + 8, 16, 32];
impl_from_integer![many_nonzero for: Z + i + 128, from: Pz + 8, 16, 32, 64];

// from smaller sized NonPositiveInteger
impl_from_integer![many_int_neg for: Z + i + 16, from: Npz + 8];
impl_from_integer![many_int_neg for: Z + i + 32, from: Npz + 8, 16];
impl_from_integer![many_int_neg for: Z + i + 64, from: Npz + 8, 16, 32];
impl_from_integer![many_int_neg for: Z + i + 128, from: Npz + 8, 16, 32, 64];

// from smaller sized NegativeInteger
impl_from_integer![many_nonzero_neg for: Z + i + 16, from: Nz + 8];
impl_from_integer![many_nonzero_neg for: Z + i + 32, from: Nz + 8, 16];
impl_from_integer![many_nonzero_neg for: Z + i + 64, from: Nz + 8, 16, 32];
impl_from_integer![many_nonzero_neg for: Z + i + 128, from: Nz + 8, 16, 32, 64];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::all::NumeraResult;

    #[test]
    fn z_from() -> NumeraResult<()> {
        let _5 = Z8::new(5)?;

        // 3 ways:
        assert_eq![<i8 as Into<Z8>>::into(5), _5];
        assert_eq![Into::<Z8>::into(5), _5];
        assert_eq![_5, 5.into()];

        // from u,i
        assert_eq![Z16::new(100)?, 100_u8.into()];
        assert_eq![Z16::new(100)?, 100_i16.into()];

        // from smaller Z
        assert_eq![Z16::new(100)?, Z8::new(100)?.into()];
        assert_eq![Z32::new(100)?, Z8::new(100)?.into()];
        assert_eq![Z32::new(100)?, Z16::new(100)?.into()];
        // ...
        assert_eq![Z128::new(100)?, Z8::new(100)?.into()];
        assert_eq![Z128::new(100)?, Z64::new(100)?.into()];

        // from smaller or equal sized N0z
        assert_eq![Z16::new(100)?, N0z8::new(100)?.into()];
        assert_eq![Z16::new(100)?, N0z16::new(100)?.into()];
        assert_eq![Z128::new(100)?, N0z128::new(100)?.into()];

        // from smaller Nnz
        assert_eq![Z16::new(100)?, Nnz8::new(100)?.into()];
        assert_eq![Z128::new(100)?, Nnz64::new(100)?.into()];

        // from smaller Npz
        assert_eq![Z16::new(-100)?, Npz8::new_neg(100)?.into()];
        assert_eq![Z128::new(-100)?, Npz64::new_neg(100)?.into()];

        Ok(())
    }
}
