// numera::number::integer::n0z::impl_from
//
//!
//

use crate::number::{
    integer::{
        macros::{from_integer, from_primitive, try_from_integer, try_from_primitive},
        *,
    },
    traits::Number,
};
#[cfg(feature = "try_from")]
use core::num::NonZeroU128;
use core::{
    num::{
        NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroU16, NonZeroU32,
        NonZeroU64, NonZeroU8,
    },
    ops::Neg,
};

/* complementary primitive conversions */

// from smaller NonZeroU
from_primitive![non0 for:NonZeroInteger+16, from:NonZeroU+8];
from_primitive![non0 for:NonZeroInteger+32, from:NonZeroU+8,16];
from_primitive![non0 for:NonZeroInteger+64, from:NonZeroU+8,16,32];
from_primitive![non0 for:NonZeroInteger+128, from:NonZeroU+8,16,32,64];
// try_from bigger or equal sized NonZeroU
try_from_primitive![non0 for:NonZeroInteger+8, from:NonZeroU+8,16,32,64,128];
try_from_primitive![non0 for:NonZeroInteger+16, from:NonZeroU+16,32,64,128];
try_from_primitive![non0 for:NonZeroInteger+32, from:NonZeroU+32,64,128];
try_from_primitive![non0 for:NonZeroInteger+64, from:NonZeroU+64,128];
try_from_primitive![non0 for:NonZeroInteger+128, from:NonZeroU+128];

// from smaller or equal sized NonZeroI
from_primitive![non0 for:NonZeroInteger+8, from:NonZeroI+8];
from_primitive![non0 for:NonZeroInteger+16, from:NonZeroI+8,16];
from_primitive![non0 for:NonZeroInteger+32, from:NonZeroI+8,16,32];
from_primitive![non0 for:NonZeroInteger+64, from:NonZeroI+8,16,32,64];
from_primitive![non0 for:NonZeroInteger+128, from:NonZeroI+8,16,32,64,128];
// try_from bigger NonZeroI
try_from_primitive![non0 for:NonZeroInteger+8, from:NonZeroI+16,32,64,128];
try_from_primitive![non0 for:NonZeroInteger+16, from:NonZeroI+32,64,128];
try_from_primitive![non0 for:NonZeroInteger+32, from:NonZeroI+64,128];
try_from_primitive![non0 for:NonZeroInteger+64, from:NonZeroI+128];

/* remaining fallible primitive conversions */

// try_from u (only the non-zero values)
try_from_primitive![int for:NonZeroInteger+8, from:u+8,16,32,64,128];
try_from_primitive![int for:NonZeroInteger+16, from:u+8,16,32,64,128];
try_from_primitive![int for:NonZeroInteger+32, from:u+8,16,32,64,128];
try_from_primitive![int for:NonZeroInteger+64, from:u+8,16,32,64,128];
try_from_primitive![int for:NonZeroInteger+128, from:u+8,16,32,64,128];

// try_from i (only the non-zero values)
try_from_primitive![int for:NonZeroInteger+8, from:i+8,16,32,64,128];
try_from_primitive![int for:NonZeroInteger+16, from:i+8,16,32,64,128];
try_from_primitive![int for:NonZeroInteger+32, from:i+8,16,32,64,128];
try_from_primitive![int for:NonZeroInteger+64, from:i+8,16,32,64,128];
try_from_primitive![int for:NonZeroInteger+128, from:i+8,16,32,64,128];

/* complementary integer conversions */

// from smaller NonZeroInteger (Self)
from_integer![non0 for:NonZeroInteger+16, from:NonZeroInteger+8];
from_integer![non0 for:NonZeroInteger+32, from:NonZeroInteger+8,16];
from_integer![non0 for:NonZeroInteger+64, from:NonZeroInteger+8,16,32];
from_integer![non0 for:NonZeroInteger+128, from:NonZeroInteger+8,16,32,64];
// try_from bigger NonZeroInteger (Self)
try_from_integer![non0 for:NonZeroInteger+8, from:NonZeroInteger+16,32,64,128];
try_from_integer![non0 for:NonZeroInteger+16, from:NonZeroInteger+32,64,128];
try_from_integer![non0 for:NonZeroInteger+32, from:NonZeroInteger+64,128];
try_from_integer![non0 for:NonZeroInteger+64, from:NonZeroInteger+128];

// from smaller PositiveInteger
from_integer![non0 for:NonZeroInteger+16, from:PositiveInteger+8];
from_integer![non0 for:NonZeroInteger+32, from:PositiveInteger+8,16];
from_integer![non0 for:NonZeroInteger+64, from:PositiveInteger+8,16,32];
from_integer![non0 for:NonZeroInteger+128, from:PositiveInteger+8,16,32,64];
// try_from bigger or equal sized PositiveInteger
try_from_integer![non0 for:NonZeroInteger+8, from:PositiveInteger+8,16,32,64,128];
try_from_integer![non0 for:NonZeroInteger+16, from:PositiveInteger+16,32,64,128];
try_from_integer![non0 for:NonZeroInteger+32, from:PositiveInteger+32,64,128];
try_from_integer![non0 for:NonZeroInteger+64, from:PositiveInteger+64,128];
try_from_integer![non0 for:NonZeroInteger+128, from:PositiveInteger+128];

// from smaller NegativeInteger
from_integer![negnon0_signed for:NonZeroInteger+i+16, from:NegativeInteger+8];
from_integer![negnon0_signed for:NonZeroInteger+i+32, from:NegativeInteger+8,16];
from_integer![negnon0_signed for:NonZeroInteger+i+64, from:NegativeInteger+8,16,32];
from_integer![negnon0_signed for:NonZeroInteger+i+128, from:NegativeInteger+8,16,32,64];
// try_from bigger or equal sized NegativeInteger
try_from_integer![negnon0_signed for:NonZeroInteger+i+8, from:NegativeInteger+8,16,32,64,128];
try_from_integer![negnon0_signed for:NonZeroInteger+i+16, from:NegativeInteger+16,32,64,128];
try_from_integer![negnon0_signed for:NonZeroInteger+i+32, from:NegativeInteger+32,64,128];
try_from_integer![negnon0_signed for:NonZeroInteger+i+64, from:NegativeInteger+64,128];
try_from_integer![negnon0_signed for:NonZeroInteger+i+128, from:NegativeInteger+128];

/* remaining fallible integer conversions */

// try_from Integer (only the non-zero values)
try_from_integer![non0_int for:NonZeroInteger+8, from:Integer+8,16,32,64,128];
try_from_integer![non0_int for:NonZeroInteger+16, from:Integer+8,16,32,64,128];
try_from_integer![non0_int for:NonZeroInteger+32, from:Integer+8,16,32,64,128];
try_from_integer![non0_int for:NonZeroInteger+64, from:Integer+8,16,32,64,128];
try_from_integer![non0_int for:NonZeroInteger+128, from:Integer+8,16,32,64,128];

// try_from NonNegativeInteger (only the non-zero values)
try_from_integer![non0_int for:NonZeroInteger+8, from:NonNegativeInteger+8,16,32,64,128];
try_from_integer![non0_int for:NonZeroInteger+16, from:NonNegativeInteger+8,16,32,64,128];
try_from_integer![non0_int for:NonZeroInteger+32, from:NonNegativeInteger+8,16,32,64,128];
try_from_integer![non0_int for:NonZeroInteger+64, from:NonNegativeInteger+8,16,32,64,128];
try_from_integer![non0_int for:NonZeroInteger+128, from:NonNegativeInteger+8,16,32,64,128];

// try_from NonPositiveInteger (only the non-zero values)
try_from_integer![negnon0_non0 for:NonZeroInteger+8, from:NonPositiveInteger+8,16,32,64,128];
try_from_integer![negnon0_non0 for:NonZeroInteger+16, from:NonPositiveInteger+8,16,32,64,128];
try_from_integer![negnon0_non0 for:NonZeroInteger+32, from:NonPositiveInteger+8,16,32,64,128];
try_from_integer![negnon0_non0 for:NonZeroInteger+64, from:NonPositiveInteger+8,16,32,64,128];
try_from_integer![negnon0_non0 for:NonZeroInteger+128, from:NonPositiveInteger+8,16,32,64,128];

#[cfg(test)]
mod tests {
    use crate::all::*;
    use core::num::{NonZeroI8, NonZeroU8};

    #[test]
    fn n0z_from() -> NumeraResult<()> {
        /* complementary primitive conversions */

        // from smaller NonZeroU
        assert_eq![N0z16::new(100)?, NonZeroU8::new(100).unwrap().into()];

        // from smaller or equal sized NonZeroI
        assert_eq![N0z8::new(100)?, NonZeroI8::new(100).unwrap().into()];
        assert_eq![N0z16::new(100)?, NonZeroI8::new(100).unwrap().into()];

        /* complementary Integer conversions */

        // from smaller NonZeroInteger (Self)
        assert_eq![N0z16::new(100)?, N0z8::new(100)?.into()];

        // from smaller PositiveInteger
        assert_eq![N0z16::new(100)?, Pz8::new(100)?.into()];

        // from smaller NegativeInteger
        assert_eq![N0z16::new(-100)?, Nz8::new_neg(100)?.into()];

        Ok(())
    }

    #[test]
    #[cfg(feature = "try_from")]
    fn n0z_try_from() -> NumeraResult<()> {
        use core::num::{NonZeroI16, NonZeroU16};

        /* complementary primitive conversions */

        // try_from bigger or equal sized NonZeroU
        assert_eq![N0z8::new(100)?, NonZeroU8::new(100).unwrap().try_into()?];
        assert_eq![N0z8::new(100)?, NonZeroU16::new(100).unwrap().try_into()?];
        assert![TryInto::<N0z8>::try_into(NonZeroU16::new(200).unwrap()).is_err()];

        // try_from bigger NonZeroI
        assert_eq![N0z8::new(100)?, NonZeroI16::new(100).unwrap().try_into()?];
        assert![TryInto::<N0z8>::try_into(NonZeroI16::new(200).unwrap()).is_err()];

        /* remaining fallible primitive conversions */

        // try_from u (only the non-zero values)
        assert_eq![N0z8::new(100)?, 100_u8.try_into()?];
        assert_eq![N0z8::new(100)?, 100_u16.try_into()?];
        assert_eq![N0z16::new(200)?, 200_u8.try_into()?];
        assert![TryInto::<N0z8>::try_into(0_u8).is_err()];
        assert![TryInto::<N0z8>::try_into(200_u16).is_err()];

        // try_from i (only the non-zero values)
        assert_eq![N0z8::new(100)?, 100_i8.try_into()?];
        assert_eq![N0z8::new(-100)?, (-100_i8).try_into()?];
        assert_eq![N0z8::new(100)?, 100_i16.try_into()?];
        assert_eq![N0z16::new(100)?, 100_i8.try_into()?];
        assert![TryInto::<N0z8>::try_into(0_i16).is_err()];
        assert![TryInto::<N0z8>::try_into(200_i16).is_err()];
        assert![TryInto::<N0z8>::try_into(-200_i16).is_err()];

        /* complementary Integer conversions */

        // try_from bigger NonZeroInteger
        assert_eq![N0z8::new(100)?, N0z16::new(100)?.try_into()?];
        assert_eq![N0z8::new(-100)?, N0z16::new(-100)?.try_into()?];
        assert![TryInto::<N0z8>::try_into(N0z16::new(200)?).is_err()];
        assert![TryInto::<N0z8>::try_into(N0z16::new(-200)?).is_err()];

        // from bigger or equal sized PositiveInteger
        assert_eq![N0z8::new(100)?, Pz16::new(100)?.try_into()?];
        assert_eq![N0z8::new(100)?, Pz8::new(100)?.try_into()?];
        assert![TryInto::<N0z8>::try_into(Pz16::new(200)?).is_err()];

        // from bigger or equal sized NegativeInteger
        assert_eq![N0z8::new(-100)?, Nz16::new_neg(100)?.try_into()?];
        assert_eq![N0z8::new(-100)?, Nz8::new_neg(100)?.try_into()?];
        assert![TryInto::<N0z8>::try_into(Nz16::new_neg(200)?).is_err()];

        /* remaining fallible integer conversions */

        // try_from Integer
        assert_eq![N0z8::new(100)?, Z8::new(100).try_into()?];
        assert_eq![N0z8::new(-100)?, Z8::new(-100).try_into()?];
        assert_eq![N0z8::new(100)?, Z16::new(100).try_into()?];
        assert_eq![N0z16::new(100)?, Z8::new(100).try_into()?];
        assert![TryInto::<N0z8>::try_into(Z16::new(200)).is_err()];

        // try_from NonNegativeInteger
        assert_eq![N0z8::new(100)?, Nnz8::new(100).try_into()?];
        assert_eq![N0z8::new(100)?, Nnz16::new(100).try_into()?];
        assert_eq![N0z16::new(100)?, Nnz8::new(100).try_into()?];
        assert![TryInto::<N0z8>::try_into(Nnz16::new(200)).is_err()];

        // try_from NonPositiveInteger
        assert_eq![N0z8::new(-100)?, Npz8::new_neg(100).try_into()?];
        assert_eq![N0z8::new(-100)?, Npz16::new_neg(100).try_into()?];
        assert_eq![N0z16::new(-100)?, Npz8::new_neg(100).try_into()?];
        assert![TryInto::<N0z8>::try_into(Npz16::new_neg(200)).is_err()];

        Ok(())
    }
}
