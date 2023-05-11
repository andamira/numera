// numera::number::integer::npz::impl_from
//
//!
//

use crate::number::{
    integer::{
        macros::{from_integer, try_from_any, try_from_integer, try_from_primitive},
        *,
    },
    traits::{ConstZero, Ident, Number},
};
use core::num::{
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroU128, NonZeroU16,
    NonZeroU32, NonZeroU64, NonZeroU8,
};

/* fallible primitive conversions */

// try_from i (only the non-positive values)
try_from_primitive![neg_int for: NonPositiveInteger+8, from: i+8,16,32,64,128];
try_from_primitive![neg_int for: NonPositiveInteger+16, from: i+8,16,32,64,128];
try_from_primitive![neg_int for: NonPositiveInteger+32, from: i+8,16,32,64,128];
try_from_primitive![neg_int for: NonPositiveInteger+64, from: i+8,16,32,64,128];
try_from_primitive![neg_int for: NonPositiveInteger+128, from: i+8,16,32,64,128];

// try_from u (only the 0)
try_from_any![zero for: NonPositiveInteger+8, from: u+8,16,32,64,128];
try_from_any![zero for: NonPositiveInteger+16, from: u+8,16,32,64,128];
try_from_any![zero for: NonPositiveInteger+32, from: u+8,16,32,64,128];
try_from_any![zero for: NonPositiveInteger+64, from: u+8,16,32,64,128];
try_from_any![zero for: NonPositiveInteger+128, from: u+8,16,32,64,128];

// try_from NonZeroI (only the negative values)
try_from_primitive![neg_nonzero for: NonPositiveInteger+8, from: NonZeroI+8,16,32,64,128];
try_from_primitive![neg_nonzero for: NonPositiveInteger+16, from: NonZeroI+8,16,32,64,128];
try_from_primitive![neg_nonzero for: NonPositiveInteger+32, from: NonZeroI+8,16,32,64,128];
try_from_primitive![neg_nonzero for: NonPositiveInteger+64, from: NonZeroI+8,16,32,64,128];
try_from_primitive![neg_nonzero for: NonPositiveInteger+128, from: NonZeroI+8,16,32,64,128];

/* impossible primitive conversions */

// try_from NonZeroU (no valid values)
try_from_any![error for: NonPositiveInteger+8, from: NonZeroU+8,16,32,64,128];
try_from_any![error for: NonPositiveInteger+16, from: NonZeroU+8,16,32,64,128];
try_from_any![error for: NonPositiveInteger+32, from: NonZeroU+8,16,32,64,128];
try_from_any![error for: NonPositiveInteger+64, from: NonZeroU+8,16,32,64,128];
try_from_any![error for: NonPositiveInteger+128, from: NonZeroU+8,16,32,64,128];

/* complementary Integer conversions */

// from smaller NonPositiveInteger (Self)
from_integer![int for: NonPositiveInteger+16, from: NonPositiveInteger+8];
from_integer![int for: NonPositiveInteger+32, from: NonPositiveInteger+8,16];
from_integer![int for: NonPositiveInteger+64, from: NonPositiveInteger+8,16,32];
from_integer![int for: NonPositiveInteger+128, from: NonPositiveInteger+8,16,32,64];
// try_from bigger NonPositiveInteger (Self)
try_from_integer![int for: NonPositiveInteger+8, from: NonPositiveInteger+16,32,64,128];
try_from_integer![int for: NonPositiveInteger+16, from: NonPositiveInteger+32,64,128];
try_from_integer![int for: NonPositiveInteger+32, from: NonPositiveInteger+64,128];
try_from_integer![int for: NonPositiveInteger+64, from: NonPositiveInteger+128];

// from smaller or equal sized NegativeInteger
from_integer![nonzero for: NonPositiveInteger+8, from: NegativeInteger+8];
from_integer![nonzero for: NonPositiveInteger+16, from: NegativeInteger+8,16];
from_integer![nonzero for: NonPositiveInteger+32, from: NegativeInteger+8,16,32];
from_integer![nonzero for: NonPositiveInteger+64, from: NegativeInteger+8,16,32,64];
from_integer![nonzero for: NonPositiveInteger+128, from: NegativeInteger+8,16,32,64,128];
// try_from bigger NegativeInteger
try_from_integer![nonzero for: NonPositiveInteger+8, from: NegativeInteger+16,32,64,128];
try_from_integer![nonzero for: NonPositiveInteger+16, from: NegativeInteger+32,64,128];
try_from_integer![nonzero for: NonPositiveInteger+32, from: NegativeInteger+64,128];
try_from_integer![nonzero for: NonPositiveInteger+64, from: NegativeInteger+128];

/* fallible Integer conversions */

// try_from Integer (only the non-positive values)
try_from_integer![neg_int for: NonPositiveInteger+8, from: Integer+8,16,32,64,128];
try_from_integer![neg_int for: NonPositiveInteger+16, from: Integer+8,16,32,64,128];
try_from_integer![neg_int for: NonPositiveInteger+32, from: Integer+8,16,32,64,128];
try_from_integer![neg_int for: NonPositiveInteger+64, from: Integer+8,16,32,64,128];
try_from_integer![neg_int for: NonPositiveInteger+128, from: Integer+8,16,32,64,128];

// try_from NonZeroInteger (only the negative values)
try_from_integer![neg_nonzero for: NonPositiveInteger+8, from: NonZeroInteger+8,16,32,64,128];
try_from_integer![neg_nonzero for: NonPositiveInteger+16, from: NonZeroInteger+8,16,32,64,128];
try_from_integer![neg_nonzero for: NonPositiveInteger+32, from: NonZeroInteger+8,16,32,64,128];
try_from_integer![neg_nonzero for: NonPositiveInteger+64, from: NonZeroInteger+8,16,32,64,128];
try_from_integer![neg_nonzero for: NonPositiveInteger+128, from: NonZeroInteger+8,16,32,64,128];

// try_from NonNegativeInteger (only the 0)
try_from_any![zero for: NonPositiveInteger+8, from: NonNegativeInteger+8,16,32,64,128];
try_from_any![zero for: NonPositiveInteger+16, from: NonNegativeInteger+8,16,32,64,128];
try_from_any![zero for: NonPositiveInteger+32, from: NonNegativeInteger+8,16,32,64,128];
try_from_any![zero for: NonPositiveInteger+64, from: NonNegativeInteger+8,16,32,64,128];
try_from_any![zero for: NonPositiveInteger+128, from: NonNegativeInteger+8,16,32,64,128];

/* impossible Integer conversions */

// try_from PositiveInteger (no valid values)
try_from_any![error for: NonPositiveInteger+8, from: PositiveInteger+8,16,32,64,128];
try_from_any![error for: NonPositiveInteger+16, from: PositiveInteger+8,16,32,64,128];
try_from_any![error for: NonPositiveInteger+32, from: PositiveInteger+8,16,32,64,128];
try_from_any![error for: NonPositiveInteger+64, from: PositiveInteger+8,16,32,64,128];
try_from_any![error for: NonPositiveInteger+128, from: PositiveInteger+8,16,32,64,128];

#[cfg(test)]
mod tests {
    use crate::all::{abbr::*, NumeraResult};
    use core::num::{NonZeroI16, NonZeroI8, NonZeroU16, NonZeroU8};

    #[test]
    fn npz_from() -> NumeraResult<()> {
        /* fallible primitive conversions */

        // try_from i (only the non-positive values)
        assert_eq![Npz8::new(0), (0_i8).try_into()?];
        assert_eq![Npz8::new(100), (-100_i8).try_into()?];
        assert_eq![Npz8::new(200), (-200_i16).try_into()?];
        assert_eq![Npz16::new(100), (-100_i8).try_into()?];
        assert![TryInto::<Npz8>::try_into(-500_i16).is_err()];

        // try_from u (only the 0)
        assert_eq![Npz8::new(0), 0_u8.try_into()?];
        assert![TryInto::<Npz8>::try_into(100_u8).is_err()];

        // try_from NonZeroI (only the negative values)
        assert_eq![Npz8::new(100), NonZeroI8::new(-100).unwrap().try_into()?];
        assert_eq![Npz8::new(200), NonZeroI16::new(-200).unwrap().try_into()?];
        assert_eq![Npz16::new(100), NonZeroI8::new(-100).unwrap().try_into()?];
        assert![TryInto::<Npz8>::try_into(NonZeroI8::new(100).unwrap()).is_err()];
        assert![TryInto::<Npz8>::try_into(NonZeroI16::new(-500).unwrap()).is_err()];

        /* impossible primitive conversions */

        // try_from NonZeroU (no valid values)
        assert![TryInto::<Npz8>::try_into(NonZeroU8::new(100).unwrap()).is_err()];

        /* complementary Integer conversions */

        // from smaller NonPositiveInteger (Self)
        assert_eq![Npz16::new(200), Npz8::new(200).into()];
        // try_from bigger NonPositiveInteger (Self)
        assert_eq![Npz8::new(200), Npz16::new(200).try_into()?];
        assert_eq![Npz8::new(200), Npz8::new(200).try_into()?];
        assert![TryInto::<Npz8>::try_into(Npz16::new(500)).is_err()];

        // from smaller or equal sized NegativeInteger
        assert_eq![Npz16::new(200), Nz8::new(200)?.into()];
        assert_eq![Npz8::new(200), Nz8::new(200)?.into()];
        // try_from bigger NegativeInteger
        assert_eq![Npz8::new(200), Nz16::new(200)?.try_into()?];
        assert![TryInto::<Npz8>::try_into(Nz16::new(500)?).is_err()];

        /* fallible Integer conversions */

        // try_from Integer (only the non-positive values)
        assert_eq![Npz8::new(0), Z8::new(0).try_into()?];
        assert_eq![Npz8::new(100), Z8::new(-100).try_into()?];
        assert_eq![Npz8::new(200), Z16::new(-200).try_into()?];
        assert_eq![Npz16::new(100), Z8::new(-100).try_into()?];
        assert![TryInto::<Npz8>::try_into(Z8::new(100)).is_err()];
        assert![TryInto::<Npz8>::try_into(Z16::new(-500)).is_err()];

        // try_from NonZeroInteger (only the negative values)
        assert_eq![Npz8::new(100), N0z8::new(-100)?.try_into()?];
        assert_eq![Npz8::new(200), N0z16::new(-200)?.try_into()?];
        assert_eq![Npz16::new(100), N0z8::new(-100)?.try_into()?];
        assert![TryInto::<Npz8>::try_into(N0z8::new(100)?).is_err()];
        assert![TryInto::<Npz8>::try_into(N0z16::new(-500)?).is_err()];

        // try_from NonNegativeInteger (only the 0)
        assert_eq![Npz8::new(0), Nnz8::new(0).try_into()?];
        assert![TryInto::<Npz8>::try_into(Nnz8::new(100)).is_err()];

        /* impossible Integer conversions */

        // try_from PositiveInteger (no valid values)
        assert![TryInto::<Npz8>::try_into(Pz8::new(100)?).is_err()];

        Ok(())
    }
}
