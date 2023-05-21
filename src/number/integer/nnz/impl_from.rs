// numera::number::integer::nnz::impl_from
//
//!
//

#[cfg(feature = "try_from")]
use crate::number::traits::{ConstZero, Ident};
use crate::number::{
    integer::{
        macros::{
            from_integer, from_primitive, try_from_any, try_from_integer, try_from_primitive,
        },
        *,
    },
    traits::Number,
};
#[cfg(feature = "try_from")]
use core::num::{NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8};
use core::num::{NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8};

/* complementary primitive conversions */

// from smaller or equal sized u
from_primitive![int for:NonNegativeInteger+8, from:u+8];
from_primitive![int for:NonNegativeInteger+16, from:u+8,16];
from_primitive![int for:NonNegativeInteger+32, from:u+8,16,32];
from_primitive![int for:NonNegativeInteger+64, from:u+8,16,32,64];
from_primitive![int for:NonNegativeInteger+128, from:u+8,16,32,64,128];
// try_from bigger u
try_from_primitive![int for:NonNegativeInteger+8, from:u+16,32,64,128];
try_from_primitive![int for:NonNegativeInteger+16, from:u+32,64,128];
try_from_primitive![int for:NonNegativeInteger+32, from:u+64,128];
try_from_primitive![int for:NonNegativeInteger+64, from:u+128];

// from smaller or equal sized NonZeroU
from_primitive![non0 for:NonNegativeInteger+8, from:NonZeroU+8];
from_primitive![non0 for:NonNegativeInteger+16, from:NonZeroU+8,16];
from_primitive![non0 for:NonNegativeInteger+32, from:NonZeroU+8,16,32];
from_primitive![non0 for:NonNegativeInteger+64, from:NonZeroU+8,16,32,64];
from_primitive![non0 for:NonNegativeInteger+128, from:NonZeroU+8,16,32,64,128];
// try_from bigger NonZeroU
try_from_primitive![non0 for:NonNegativeInteger+8, from:NonZeroU+16,32,64,128];
try_from_primitive![non0 for:NonNegativeInteger+16, from:NonZeroU+32,64,128];
try_from_primitive![non0 for:NonNegativeInteger+32, from:NonZeroU+64,128];
try_from_primitive![non0 for:NonNegativeInteger+64, from:NonZeroU+128];

/* remaining fallible primitive conversions */

// try_from i (only the non-negative values)
try_from_primitive![int for:NonNegativeInteger+8, from:i+8,16,32,64,128];
try_from_primitive![int for:NonNegativeInteger+16, from:i+8,16,32,64,128];
try_from_primitive![int for:NonNegativeInteger+32, from:i+8,16,32,64,128];
try_from_primitive![int for:NonNegativeInteger+64, from:i+8,16,32,64,128];
try_from_primitive![int for:NonNegativeInteger+128, from:i+8,16,32,64,128];

// try_from NonZeroI (only the non-negative values)
try_from_primitive![non0 for:NonNegativeInteger+8, from:NonZeroI+8,16,32,64,128];
try_from_primitive![non0 for:NonNegativeInteger+16, from:NonZeroI+8,16,32,64,128];
try_from_primitive![non0 for:NonNegativeInteger+32, from:NonZeroI+8,16,32,64,128];
try_from_primitive![non0 for:NonNegativeInteger+64, from:NonZeroI+8,16,32,64,128];
try_from_primitive![non0 for:NonNegativeInteger+128, from:NonZeroI+8,16,32,64,128];

/* complementary Integer conversions */

// from smaller NonNegativeInteger (Self)
from_integer![int for:NonNegativeInteger+16, from:NonNegativeInteger+8];
from_integer![int for:NonNegativeInteger+32, from:NonNegativeInteger+8,16];
from_integer![int for:NonNegativeInteger+64, from:NonNegativeInteger+8,16,32];
from_integer![int for:NonNegativeInteger+128, from:NonNegativeInteger+8,16,32, 64];
// try_from bigger NonNegativeInteger (Self)
try_from_integer![int for:NonNegativeInteger+8, from:NonNegativeInteger+16,32,64,128];
try_from_integer![int for:NonNegativeInteger+16, from:NonNegativeInteger+32,64,128];
try_from_integer![int for:NonNegativeInteger+32, from:NonNegativeInteger+64,128];
try_from_integer![int for:NonNegativeInteger+64, from:NonNegativeInteger+128];

// from smaller or equal sized PositiveInteger
from_integer![non0 for:NonNegativeInteger+8, from:PositiveInteger+8];
from_integer![non0 for:NonNegativeInteger+16, from:PositiveInteger+8,16];
from_integer![non0 for:NonNegativeInteger+32, from:PositiveInteger+8,16,32];
from_integer![non0 for:NonNegativeInteger+64, from:PositiveInteger+8,16,32,64];
from_integer![non0 for:NonNegativeInteger+128, from:PositiveInteger+8,16,32,64,128];
// try_from bigger PositiveInteger
try_from_integer![non0 for:NonNegativeInteger+8, from:PositiveInteger+16,32,64,128];
try_from_integer![non0 for:NonNegativeInteger+16, from:PositiveInteger+32,64,128];
try_from_integer![non0 for:NonNegativeInteger+32, from:PositiveInteger+64,128];
try_from_integer![non0 for:NonNegativeInteger+64, from:PositiveInteger+128];

// from smaller or equal sized Prime
from_integer![int for:NonNegativeInteger+8, from:Prime+8];
from_integer![int for:NonNegativeInteger+16, from:Prime+8,16];
from_integer![int for:NonNegativeInteger+32, from:Prime+8,16,32];
from_integer![int for:NonNegativeInteger+64, from:Prime+8,16,32,64];
from_integer![int for:NonNegativeInteger+128, from:Prime+8,16,32,64,128];
// try_from bigger Prime
try_from_integer![int for:NonNegativeInteger+8, from:Prime+16,32,64,128];
try_from_integer![int for:NonNegativeInteger+16, from:Prime+32,64,128];
try_from_integer![int for:NonNegativeInteger+32, from:Prime+64,128];
try_from_integer![int for:NonNegativeInteger+64, from:Prime+128];

/* remaining fallible integer conversions */

// try_from Integer (only the non-negative values)
try_from_integer![int for:NonNegativeInteger+8, from:Integer+8,16,32,64,128];
try_from_integer![int for:NonNegativeInteger+16, from:Integer+8,16,32,64,128];
try_from_integer![int for:NonNegativeInteger+32, from:Integer+8,16,32,64,128];
try_from_integer![int for:NonNegativeInteger+64, from:Integer+8,16,32,64,128];
try_from_integer![int for:NonNegativeInteger+128, from:Integer+8,16,32,64,128];

// try_from NonZeroInteger (only the positive values)
try_from_integer![non0 for:NonNegativeInteger+8, from:NonZeroInteger+8,16,32,64,128];
try_from_integer![non0 for:NonNegativeInteger+16, from:NonZeroInteger+8,16,32,64,128];
try_from_integer![non0 for:NonNegativeInteger+32, from:NonZeroInteger+8,16,32,64,128];
try_from_integer![non0 for:NonNegativeInteger+64, from:NonZeroInteger+8,16,32,64,128];
try_from_integer![non0 for:NonNegativeInteger+128, from:NonZeroInteger+8,16,32,64,128];

// try_from NonPositiveInteger (only the 0)
try_from_any![zero for:NonNegativeInteger+8, from:NonPositiveInteger+8,16,32,64,128];
try_from_any![zero for:NonNegativeInteger+16, from:NonPositiveInteger+8,16,32,64,128];
try_from_any![zero for:NonNegativeInteger+32, from:NonPositiveInteger+8,16,32,64,128];
try_from_any![zero for:NonNegativeInteger+64, from:NonPositiveInteger+8,16,32,64,128];
try_from_any![zero for:NonNegativeInteger+128, from:NonPositiveInteger+8,16,32,64,128];

/* impossible Integer conversions */

// try_from NegativeInteger (no valid values)
try_from_any![error for: NonNegativeInteger+8, from: NegativeInteger+8,16,32,64,128];
try_from_any![error for: NonNegativeInteger+16, from: NegativeInteger+8,16,32,64,128];
try_from_any![error for: NonNegativeInteger+32, from: NegativeInteger+8,16,32,64,128];
try_from_any![error for: NonNegativeInteger+64, from: NegativeInteger+8,16,32,64,128];
try_from_any![error for: NonNegativeInteger+128, from: NegativeInteger+8,16,32,64,128];

#[cfg(test)]
mod tests {
    use crate::all::*;
    use core::num::NonZeroU8;

    #[test]
    fn nnz_from() -> NumeraResult<()> {
        /* complementary primitive conversions */

        // from smaller or equal sized u
        assert_eq![Nnz8::new(200), 200_u8.into()];
        assert_eq![Nnz16::new(200), 200_u8.into()];

        // from smaller or equal sized NonZeroU
        assert_eq![Nnz8::new(200), NonZeroU8::new(200).unwrap().into()];
        assert_eq![Nnz16::new(200), NonZeroU8::new(200).unwrap().into()];

        /* complementary Integer conversions */

        // from smaller NonNegativeInteger (Self)
        assert_eq![Nnz16::new(200), Nnz8::new(200).into()];

        // from smaller or equal sized PositiveInteger
        assert_eq![Nnz16::new(200), Pz8::new(200)?.into()];
        assert_eq![Nnz8::new(200), Pz8::new(200)?.into()];

        // from smaller or equal sized PRime
        assert_eq![Nnz16::new(251), P8::new(251)?.into()];
        assert_eq![Nnz8::new(251), P8::new(251)?.into()];

        Ok(())
    }

    #[test]
    #[cfg(feature = "try_from")]
    fn nnz_try_from() -> NumeraResult<()> {
        use core::num::{NonZeroI16, NonZeroI8, NonZeroU16};

        /* complementary primitive conversions */

        // try_from bigger u
        assert_eq![Nnz8::new(200), 200_u16.try_into()?];
        assert![TryInto::<Nnz8>::try_into(500u16).is_err()];

        // try_from bigger NonZeroU
        assert_eq![Nnz8::new(200), NonZeroU16::new(200).unwrap().try_into()?];
        assert![TryInto::<Nnz8>::try_into(NonZeroU16::new(500).unwrap()).is_err()];

        /* remaining fallible primitive conversions */

        // try_from i (only the non-negative values)
        assert_eq![Nnz8::new(0), 0_i8.try_into()?];
        assert_eq![Nnz8::new(100), 100_i8.try_into()?];
        assert_eq![Nnz8::new(200), 200_i16.try_into()?];
        assert_eq![Nnz16::new(100), 100_i8.try_into()?];
        assert![TryInto::<Nnz8>::try_into(-100_i8).is_err()];
        assert![TryInto::<Nnz8>::try_into(500_i16).is_err()];

        // try_from NonZeroI (only the non-negative values)
        assert_eq![Nnz8::new(100), NonZeroI8::new(100).unwrap().try_into()?];
        assert_eq![Nnz8::new(200), NonZeroI16::new(200).unwrap().try_into()?];
        assert_eq![Nnz16::new(100), NonZeroI8::new(100).unwrap().try_into()?];
        assert![TryInto::<Nnz8>::try_into(NonZeroI8::new(-100).unwrap()).is_err()];
        assert![TryInto::<Nnz8>::try_into(NonZeroI16::new(500).unwrap()).is_err()];

        /* complementary Integer conversions */

        // try_from bigger NonNegativeInteger (Self)
        assert_eq![Nnz8::new(200), Nnz16::new(200).try_into()?];
        assert_eq![Nnz8::new(200), Nnz8::new(200).try_into()?];
        assert![TryInto::<Nnz8>::try_into(Nnz16::new(500)).is_err()];

        // try_from bigger PositiveInteger
        assert_eq![Nnz8::new(200), Pz16::new(200)?.try_into()?];
        assert![TryInto::<Nnz8>::try_into(Pz16::new(500)?).is_err()];

        // try_from bigger Prime
        assert_eq![Nnz8::new(251), Pz16::new(251)?.try_into()?];
        assert![TryInto::<Nnz8>::try_into(Pz16::new(521)?).is_err()];

        /* remaining fallible integer conversions */

        // try_from Integer (only the non-negative values)
        assert_eq![Nnz8::new(0), Z8::new(0).try_into()?];
        assert_eq![Nnz8::new(100), Z8::new(100).try_into()?];
        assert_eq![Nnz8::new(200), Z16::new(200).try_into()?];
        assert_eq![Nnz16::new(100), Z8::new(100).try_into()?];
        assert![TryInto::<Nnz8>::try_into(Z8::new(-100)).is_err()];
        assert![TryInto::<Nnz8>::try_into(Z16::new(500)).is_err()];

        // try_from NonZeroInteger (only the positive values)
        assert_eq![Nnz8::new(100), N0z8::new(100)?.try_into()?];
        assert_eq![Nnz8::new(200), N0z16::new(200)?.try_into()?];
        assert_eq![Nnz16::new(100), N0z8::new(100)?.try_into()?];
        assert![TryInto::<Nnz8>::try_into(N0z8::new(-100)?).is_err()];
        assert![TryInto::<Nnz8>::try_into(N0z16::new(500)?).is_err()];

        // try_from NonPositiveInteger (only the 0)
        assert_eq![Nnz8::new(0), Npz8::new_neg(0).try_into()?];
        assert![TryInto::<Nnz8>::try_into(Npz8::new_neg(100)).is_err()];

        /* impossible Integer conversions */

        // try_from NegativeInteger (no valid values)
        assert![TryInto::<Nnz8>::try_into(Nz8::new_neg(100)?).is_err()];
        Ok(())
    }
}
