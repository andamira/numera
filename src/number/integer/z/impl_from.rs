// numera::number::integer::z::impl_from
//
//!
//
// TOC
// - complementary primitive conversions
// - complementary integer conversions

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

// from smaller u
impl_from_primitive![int for: Z + 16, from: u + 8];
impl_from_primitive![int for: Z + 32, from: u + 8, 16];
impl_from_primitive![int for: Z + 64, from: u + 8, 16, 32];
impl_from_primitive![int for: Z + 128, from: u + 8, 16, 32, 64];
// try_from bigger or equal sized u
impl_try_from_primitive![int for: Z + 8, from: u + 8, 16, 32, 64, 128];
impl_try_from_primitive![int for: Z + 16, from: u + 16, 32, 64, 128];
impl_try_from_primitive![int for: Z + 32, from: u + 32, 64, 128];
impl_try_from_primitive![int for: Z + 64, from: u + 64, 128];
impl_try_from_primitive![int for: Z + 128, from: u + 128];

// from smaller or equal sized i
impl_from_primitive![int for: Z + 8, from: i + 8];
impl_from_primitive![int for: Z + 16, from: i + 8, 16];
impl_from_primitive![int for: Z + 32, from: i + 8, 16, 32];
impl_from_primitive![int for: Z + 64, from: i + 8, 16, 32, 64];
impl_from_primitive![int for: Z + 128, from: i + 8, 16, 32, 64, 128];
// try_from bigger i
impl_try_from_primitive![int for: Z + 8, from: i + 16, 32, 64, 128];
impl_try_from_primitive![int for: Z + 16, from: i + 32, 64, 128];
impl_try_from_primitive![int for: Z + 32, from: i + 64, 128];
impl_try_from_primitive![int for: Z + 64, from: i + 128];

// from smaller NonZeroU
impl_from_primitive![nonzero for: Z + 16, from: NonZeroU + 8];
impl_from_primitive![nonzero for: Z + 32, from: NonZeroU + 8, 16];
impl_from_primitive![nonzero for: Z + 64, from: NonZeroU + 8, 16, 32];
impl_from_primitive![nonzero for: Z + 128, from: NonZeroU + 8, 16, 32, 64];
// try_from bigger or equal sized NonZeroU
impl_try_from_primitive![nonzero for: Z + 8, from: NonZeroU + 8, 16, 32, 64, 128];
impl_try_from_primitive![nonzero for: Z + 16, from: NonZeroU + 16, 32, 64, 128];
impl_try_from_primitive![nonzero for: Z + 32, from: NonZeroU + 32, 64, 128];
impl_try_from_primitive![nonzero for: Z + 64, from: NonZeroU + 64, 128];
impl_try_from_primitive![nonzero for: Z + 128, from: NonZeroU + 128];

// from smaller or equal sized NonZeroI
impl_from_primitive![nonzero for: Z + 8, from: NonZeroI + 8];
impl_from_primitive![nonzero for: Z + 16, from: NonZeroI + 8, 16];
impl_from_primitive![nonzero for: Z + 32, from: NonZeroI + 8, 16, 32];
impl_from_primitive![nonzero for: Z + 64, from: NonZeroI + 8, 16, 32, 64];
impl_from_primitive![nonzero for: Z + 128, from: NonZeroI + 8, 16, 32, 64, 128];
// try_from bigger NonZeroI
impl_try_from_primitive![nonzero for: Z + 8, from: NonZeroI + 16, 32, 64, 128];
impl_try_from_primitive![nonzero for: Z + 16, from: NonZeroI + 32, 64, 128];
impl_try_from_primitive![nonzero for: Z + 32, from: NonZeroI + 64, 128];
impl_try_from_primitive![nonzero for: Z + 64, from: NonZeroI + 128];

/* complementary Integer conversions */

// from smaller or equal sized Integer (Self)
impl_from_integer![int for: Z + i + 16, from: Z + 8];
impl_from_integer![int for: Z + i + 32, from: Z + 8, 16];
impl_from_integer![int for: Z + i + 64, from: Z + 8, 16, 32];
impl_from_integer![int for: Z + i + 128, from: Z + 8, 16, 32, 64];
// try_from bigger Integer (Self)
impl_try_from_integer![int for: Z + i + 8, from: Z + 16, 32, 64, 128];
impl_try_from_integer![int for: Z + i + 16, from: Z + 32, 64, 128];
impl_try_from_integer![int for: Z + i + 32, from: Z + 64, 128];
impl_try_from_integer![int for: Z + i + 64, from: Z + 128];

// from smaller or equal sized NonZeroInteger
impl_from_integer![nonzero for: Z + i + 8, from: N0z + 8];
impl_from_integer![nonzero for: Z + i + 16, from: N0z + 8, 16];
impl_from_integer![nonzero for: Z + i + 32, from: N0z + 8, 16, 32];
impl_from_integer![nonzero for: Z + i + 64, from: N0z + 8, 16, 32, 64];
impl_from_integer![nonzero for: Z + i + 128, from: N0z + 8, 16, 32, 64, 128];
// try_from bigger NonZeroInteger
impl_try_from_integer![nonzero for: Z + i + 8, from: N0z + 16, 32, 64, 128];
impl_try_from_integer![nonzero for: Z + i + 16, from: N0z + 32, 64, 128];
impl_try_from_integer![nonzero for: Z + i + 32, from: N0z + 64, 128];
impl_try_from_integer![nonzero for: Z + i + 64, from: N0z + 128];

// from smaller NonNegativeInteger
impl_from_integer![int for: Z + i + 16, from: Nnz + 8];
impl_from_integer![int for: Z + i + 32, from: Nnz + 8, 16];
impl_from_integer![int for: Z + i + 64, from: Nnz + 8, 16, 32];
impl_from_integer![int for: Z + i + 128, from: Nnz + 8, 16, 32, 64];
// from bigger or equal sized NonNegativeInteger
impl_try_from_integer![int for: Z + i + 8, from: Nnz + 8, 16, 32, 64, 128];
impl_try_from_integer![int for: Z + i + 16, from: Nnz + 16, 32, 64, 128];
impl_try_from_integer![int for: Z + i + 32, from: Nnz + 32, 64, 128];
impl_try_from_integer![int for: Z + i + 64, from: Nnz + 64, 128];
impl_try_from_integer![int for: Z + i + 128, from: Nnz + 128];

// from smaller PositiveInteger
impl_from_integer![nonzero for: Z + i + 16, from: Pz + 8];
impl_from_integer![nonzero for: Z + i + 32, from: Pz + 8, 16];
impl_from_integer![nonzero for: Z + i + 64, from: Pz + 8, 16, 32];
impl_from_integer![nonzero for: Z + i + 128, from: Pz + 8, 16, 32, 64];
// from bigger or equal sized PositiveInteger
impl_try_from_integer![nonzero for: Z + i + 8, from: Pz + 8, 16, 32, 64, 128];
impl_try_from_integer![nonzero for: Z + i + 16, from: Pz + 16, 32, 64, 128];
impl_try_from_integer![nonzero for: Z + i + 32, from: Pz + 32, 64, 128];
impl_try_from_integer![nonzero for: Z + i + 64, from: Pz + 64, 128];
impl_try_from_integer![nonzero for: Z + i + 128, from: Pz + 128];

// from smaller NonPositiveInteger
impl_from_integer![int_neg for: Z + i + 16, from: Npz + 8];
impl_from_integer![int_neg for: Z + i + 32, from: Npz + 8, 16];
impl_from_integer![int_neg for: Z + i + 64, from: Npz + 8, 16, 32];
impl_from_integer![int_neg for: Z + i + 128, from: Npz + 8, 16, 32, 64];
// from bigger or equal sized NonPositiveInteger
impl_try_from_integer![int_neg for: Z + i + 8, from: Npz + 8, 16, 32, 64, 128];
impl_try_from_integer![int_neg for: Z + i + 16, from: Npz + 16, 32, 64, 128];
impl_try_from_integer![int_neg for: Z + i + 32, from: Npz + 32, 64, 128];
impl_try_from_integer![int_neg for: Z + i + 64, from: Npz + 64, 128];
impl_try_from_integer![int_neg for: Z + i + 128, from: Npz + 128];

// from smaller NegativeInteger
impl_from_integer![nonzero_neg for: Z + i + 16, from: Nz + 8];
impl_from_integer![nonzero_neg for: Z + i + 32, from: Nz + 8, 16];
impl_from_integer![nonzero_neg for: Z + i + 64, from: Nz + 8, 16, 32];
impl_from_integer![nonzero_neg for: Z + i + 128, from: Nz + 8, 16, 32, 64];
// from bigger or equal sized NegativeInteger
impl_try_from_integer![nonzero_neg for: Z + i + 8, from: Nz + 8, 16, 32, 64, 128];
impl_try_from_integer![nonzero_neg for: Z + i + 16, from: Nz + 16, 32, 64, 128];
impl_try_from_integer![nonzero_neg for: Z + i + 32, from: Nz + 32, 64, 128];
impl_try_from_integer![nonzero_neg for: Z + i + 64, from: Nz + 64, 128];
impl_try_from_integer![nonzero_neg for: Z + i + 128, from: Nz + 128];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::all::{NegSigned, NumeraResult};

    #[test]
    fn z_from() -> NumeraResult<()> {
        let _5 = Z8::from_parts(5)?;

        // 3 ways:
        assert_eq![<i8 as Into<Z8>>::into(5), _5];
        assert_eq![Into::<Z8>::into(5), _5];
        assert_eq![_5, 5.into()];

        // from u,i
        assert_eq![Z16::from_parts(100)?, 100_u8.into()];
        assert_eq![Z16::from_parts(100)?, 100_i16.into()];

        // from smaller Z
        assert_eq![Z16::from_parts(100)?, Z8::from_parts(100)?.into()];
        assert_eq![Z32::from_parts(100)?, Z8::from_parts(100)?.into()];
        assert_eq![Z32::from_parts(100)?, Z16::from_parts(100)?.into()];
        // ...
        assert_eq![Z128::from_parts(100)?, Z8::from_parts(100)?.into()];
        assert_eq![Z128::from_parts(100)?, Z64::from_parts(100)?.into()];

        // from smaller or equal sized N0z
        assert_eq![Z16::from_parts(100)?, N0z8::from_parts(100)?.into()];
        assert_eq![Z16::from_parts(100)?, N0z16::from_parts(100)?.into()];
        assert_eq![Z128::from_parts(100)?, N0z128::from_parts(100)?.into()];

        // from smaller Nnz
        assert_eq![Z16::from_parts(100)?, Nnz8::from_parts(100)?.into()];
        assert_eq![Z128::from_parts(100)?, Nnz64::from_parts(100)?.into()];

        // from smaller Npz
        assert_eq![Z16::from_parts(-100)?, Npz8::new_neg(100)?.into()];
        assert_eq![Z128::from_parts(-100)?, Npz64::new_neg(100)?.into()];

        Ok(())
    }
}
