// numera::number::integer::nz::impl_from
//
//!
//

use crate::number::{
    integer::{
        macros::{from_integer, try_from_any, try_from_integer, try_from_primitive},
        *,
    },
    traits::Number,
};
#[cfg(feature = "try_from")]
use core::num::{
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroU128, NonZeroU16,
    NonZeroU32, NonZeroU64, NonZeroU8,
};

/* fallible primitive conversions */

// try_from i (only the negative values)
try_from_primitive![negnon0_int for: NegativeInteger+8, from: i+8,16,32,64,128];
try_from_primitive![negnon0_int for: NegativeInteger+16, from: i+8,16,32,64,128];
try_from_primitive![negnon0_int for: NegativeInteger+32, from: i+8,16,32,64,128];
try_from_primitive![negnon0_int for: NegativeInteger+64, from: i+8,16,32,64,128];
try_from_primitive![negnon0_int for: NegativeInteger+128, from: i+8,16,32,64,128];

// try_from NonZeroI (only the negative values)
try_from_primitive![neg_non0 for: NegativeInteger+8, from: NonZeroI+8,16,32,64,128];
try_from_primitive![neg_non0 for: NegativeInteger+16, from: NonZeroI+8,16,32,64,128];
try_from_primitive![neg_non0 for: NegativeInteger+32, from: NonZeroI+8,16,32,64,128];
try_from_primitive![neg_non0 for: NegativeInteger+64, from: NonZeroI+8,16,32,64,128];
try_from_primitive![neg_non0 for: NegativeInteger+128, from: NonZeroI+8,16,32,64,128];

/* impossible primitive conversions */

// try_from u (no valid values)
try_from_any![error for: NegativeInteger+8, from: u+8,16,32,64,128];
try_from_any![error for: NegativeInteger+16, from: u+8,16,32,64,128];
try_from_any![error for: NegativeInteger+32, from: u+8,16,32,64,128];
try_from_any![error for: NegativeInteger+64, from: u+8,16,32,64,128];
try_from_any![error for: NegativeInteger+128, from: u+8,16,32,64,128];

// try_from NonZeroU (no valid values)
try_from_any![error for: NegativeInteger+8, from: NonZeroU+8,16,32,64,128];
try_from_any![error for: NegativeInteger+16, from: NonZeroU+8,16,32,64,128];
try_from_any![error for: NegativeInteger+32, from: NonZeroU+8,16,32,64,128];
try_from_any![error for: NegativeInteger+64, from: NonZeroU+8,16,32,64,128];
try_from_any![error for: NegativeInteger+128, from: NonZeroU+8,16,32,64,128];

/* complementary Integer conversions */

// from smaller NegativeInteger (Self)
from_integer![non0 for: NegativeInteger+16, from: NegativeInteger+8];
from_integer![non0 for: NegativeInteger+32, from: NegativeInteger+8,16];
from_integer![non0 for: NegativeInteger+64, from: NegativeInteger+8,16,32];
from_integer![non0 for: NegativeInteger+128, from: NegativeInteger+8,16,32,64];
// try_from bigger NegativeInteger (Self)
try_from_integer![non0 for: NegativeInteger+8, from: NegativeInteger+16,32,64,128];
try_from_integer![non0 for: NegativeInteger+16, from: NegativeInteger+32,64,128];
try_from_integer![non0 for: NegativeInteger+32, from: NegativeInteger+64,128];
try_from_integer![non0 for: NegativeInteger+64, from: NegativeInteger+128];

/* fallible Integer conversions */

// try_from Integer (only the negative values)
try_from_integer![negnon0_int for: NegativeInteger+8, from: Integer+8,16,32,64,128];
try_from_integer![negnon0_int for: NegativeInteger+16, from: Integer+8,16,32,64,128];
try_from_integer![negnon0_int for: NegativeInteger+32, from: Integer+8,16,32,64,128];
try_from_integer![negnon0_int for: NegativeInteger+64, from: Integer+8,16,32,64,128];
try_from_integer![negnon0_int for: NegativeInteger+128, from: Integer+8,16,32,64,128];
// try_from NonZeroInteger (only the negative values)
try_from_integer![neg_non0 for:NegativeInteger+8, from: NonZeroInteger+8,16,32,64,128];
try_from_integer![neg_non0 for:NegativeInteger+16, from: NonZeroInteger+8,16,32,64,128];
try_from_integer![neg_non0 for:NegativeInteger+32, from: NonZeroInteger+8,16,32,64,128];
try_from_integer![neg_non0 for:NegativeInteger+64, from: NonZeroInteger+8,16,32,64,128];
try_from_integer![neg_non0 for:NegativeInteger+128, from: NonZeroInteger+8,16,32,64,128];
// try_from NonPositiveInteger (only the non-zero values)
try_from_integer![neg_non0neg for:NegativeInteger+8, from: NonPositiveInteger+8,16,32,64,128];
try_from_integer![neg_non0neg for:NegativeInteger+16, from: NonPositiveInteger+8,16,32,64,128];
try_from_integer![neg_non0neg for:NegativeInteger+32, from: NonPositiveInteger+8,16,32,64,128];
try_from_integer![neg_non0neg for:NegativeInteger+64, from: NonPositiveInteger+8,16,32,64,128];
try_from_integer![neg_non0neg for:NegativeInteger+128, from: NonPositiveInteger+8,16,32,64,128];

/* impossible Integer conversions */

// try_from NonNegativeInteger (no valid values)
try_from_any![error for: NegativeInteger+8, from: NonNegativeInteger+8,16,32,64,128];
try_from_any![error for: NegativeInteger+16, from: NonNegativeInteger+8,16,32,64,128];
try_from_any![error for: NegativeInteger+32, from: NonNegativeInteger+8,16,32,64,128];
try_from_any![error for: NegativeInteger+64, from: NonNegativeInteger+8,16,32,64,128];
try_from_any![error for: NegativeInteger+128, from: NonNegativeInteger+8,16,32,64,128];

// try_from PositiveInteger (no valid values)
try_from_any![error for: NegativeInteger+8, from: PositiveInteger+8,16,32,64,128];
try_from_any![error for: NegativeInteger+16, from: PositiveInteger+8,16,32,64,128];
try_from_any![error for: NegativeInteger+32, from: PositiveInteger+8,16,32,64,128];
try_from_any![error for: NegativeInteger+64, from: PositiveInteger+8,16,32,64,128];
try_from_any![error for: NegativeInteger+128, from: PositiveInteger+8,16,32,64,128];

#[cfg(test)]
mod tests {
    use crate::all::*;

    #[test]
    fn nz_from() -> NumeraResult<()> {
        /* complementary Integer conversions */

        // from smaller NegativeInteger (Self)
        assert_eq![Nz16::new_neg(200)?, Nz8::new_neg(200)?.into()];

        Ok(())
    }

    #[test]
    #[cfg(feature = "try_from")]
    fn nz_try_from() -> NumeraResult<()> {
        use core::num::{NonZeroI16, NonZeroI8, NonZeroU8};

        /* fallible primitive conversions */

        // try_from i (only the negative values)
        assert_eq![Nz8::new_neg(100)?, (-100_i8).try_into()?];
        assert_eq![Nz8::new_neg(200)?, (-200_i16).try_into()?];
        assert_eq![Nz16::new_neg(100)?, (-100_i8).try_into()?];
        assert![TryInto::<Nz8>::try_into(0_i8).is_err()];
        assert![TryInto::<Nz8>::try_into(100_i8).is_err()];
        assert![TryInto::<Nz8>::try_into(-500_i16).is_err()];

        // try_from NonZeroI (only the negative values)
        assert_eq![Nz8::new_neg(100)?, NonZeroI8::new(-100).unwrap().try_into()?];
        assert_eq![Nz8::new_neg(200)?, NonZeroI16::new(-200).unwrap().try_into()?];
        assert_eq![Nz16::new_neg(100)?, NonZeroI8::new(-100).unwrap().try_into()?];
        assert![TryInto::<Nz8>::try_into(NonZeroI8::new(100).unwrap()).is_err()];
        assert![TryInto::<Nz8>::try_into(NonZeroI16::new(-500).unwrap()).is_err()];

        /* impossible primitive conversions */

        // try_from u (no valid values)
        assert![TryInto::<Nz8>::try_into(100_u8).is_err()];

        // try_from NonZeroU (no valid values)
        assert![TryInto::<Nz8>::try_into(NonZeroU8::new(100).unwrap()).is_err()];

        /* complementary Integer conversions */

        // try_from bigger NegativeInteger (Self)
        assert_eq![Nz8::new_neg(200)?, Nz16::new_neg(200)?.try_into()?];
        assert_eq![Nz8::new_neg(200)?, Nz8::new_neg(200)?.try_into()?];
        assert![TryInto::<Nz8>::try_into(Nz16::new_neg(500)?).is_err()];

        /* fallible Integer conversions */

        // try_from Integer (only the negative values)
        assert_eq![Nz8::new_neg(100)?, Z8::new(-100).try_into()?];
        assert_eq![Nz8::new_neg(200)?, Z16::new(-200).try_into()?];
        assert_eq![Nz16::new_neg(100)?, Z8::new(-100).try_into()?];
        assert![TryInto::<Nz8>::try_into(Z8::new(0)).is_err()];
        assert![TryInto::<Nz8>::try_into(Z8::new(100)).is_err()];
        assert![TryInto::<Nz8>::try_into(Z16::new(-500)).is_err()];

        // try_from NonZeroInteger (only the negative values)
        assert_eq![Nz8::new_neg(100)?, N0z8::new(-100)?.try_into()?];
        assert_eq![Nz8::new_neg(200)?, N0z16::new(-200)?.try_into()?];
        assert_eq![Nz16::new_neg(100)?, N0z8::new(-100)?.try_into()?];
        assert![TryInto::<Nz8>::try_into(N0z8::new(100)?).is_err()];
        assert![TryInto::<Nz8>::try_into(N0z16::new(-500)?).is_err()];

        // try_from NonPositiveInteger (only the non-zero values)
        assert_eq![Nz8::new_neg(200)?, Npz8::new_neg(200).try_into()?];
        assert_eq![Nz8::new_neg(200)?, Npz16::new_neg(200).try_into()?];
        assert_eq![Nz16::new_neg(200)?, Npz8::new_neg(200).try_into()?];
        assert![TryInto::<Nz8>::try_into(Npz8::new_neg(0)).is_err()];
        assert![TryInto::<Nz8>::try_into(Npz16::new_neg(500)).is_err()];

        /* impossible Integer conversions */

        // try_from NonNegativeInteger (no valid values)
        assert![TryInto::<Nz8>::try_into(Nnz8::new(0)).is_err()];
        assert![TryInto::<Nz8>::try_into(Nnz8::new(100)).is_err()];

        // try_from PositiveInteger (no valid values)
        assert![TryInto::<Nz8>::try_into(Pz8::new(100)?).is_err()];

        Ok(())
    }
}
