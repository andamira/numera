// numera::number::integer::pz::impl_from
//
//!
//

use crate::number::{
    integer::{
        macros::{
            from_integer, from_primitive, try_from_any, try_from_integer, try_from_primitive,
        },
        *,
    },
    traits::Number,
};
use core::num::{
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroU128, NonZeroU16,
    NonZeroU32, NonZeroU64, NonZeroU8,
};

/* complementary primitive conversions */

// from smaller or equal sized NonZeroU
from_primitive![nonzero for: PositiveInteger+8, from: NonZeroU+8];
from_primitive![nonzero for: PositiveInteger+16, from: NonZeroU+8,16];
from_primitive![nonzero for: PositiveInteger+32, from: NonZeroU+8,16,32];
from_primitive![nonzero for: PositiveInteger+64, from: NonZeroU+8,16,32,64];
from_primitive![nonzero for: PositiveInteger+128, from: NonZeroU+8,16,32,64,128];
// try_from bigger NonZeroU
try_from_primitive![nonzero for: PositiveInteger+8, from: NonZeroU+16,32,64,128];
try_from_primitive![nonzero for: PositiveInteger+16, from: NonZeroU+32,64,128];
try_from_primitive![nonzero for: PositiveInteger+32, from: NonZeroU+64,128];
try_from_primitive![nonzero for: PositiveInteger+64, from: NonZeroU+128];

/* remaining fallible primitive conversions */

// try_from i (only the positive values)
try_from_primitive![int for:PositiveInteger+8, from:i+8,16,32,64,128];
try_from_primitive![int for:PositiveInteger+16, from:i+8,16,32,64,128];
try_from_primitive![int for:PositiveInteger+32, from:i+8,16,32,64,128];
try_from_primitive![int for:PositiveInteger+64, from:i+8,16,32,64,128];
try_from_primitive![int for:PositiveInteger+128, from:i+8,16,32,64,128];

// try_from u (only the positive values)
try_from_primitive![int for:PositiveInteger+8, from:u+8,16,32,64,128];
try_from_primitive![int for:PositiveInteger+16, from:u+8,16,32,64,128];
try_from_primitive![int for:PositiveInteger+32, from:u+8,16,32,64,128];
try_from_primitive![int for:PositiveInteger+64, from:u+8,16,32,64,128];
try_from_primitive![int for:PositiveInteger+128, from:u+8,16,32,64,128];

// try_from NonZeroI (only the positive values)
try_from_primitive![nonzero for: PositiveInteger+8, from: NonZeroI+8,16,32,64,128];
try_from_primitive![nonzero for: PositiveInteger+16, from: NonZeroI+8,16,32,64,128];
try_from_primitive![nonzero for: PositiveInteger+32, from: NonZeroI+8,16,32,64,128];
try_from_primitive![nonzero for: PositiveInteger+64, from: NonZeroI+8,16,32,64,128];
try_from_primitive![nonzero for: PositiveInteger+128, from: NonZeroI+8,16,32,64,128];

/* complementary Integer conversions */

// from smaller PositiveInteger (Self)
from_integer![nonzero for: PositiveInteger+16, from: PositiveInteger+8];
from_integer![nonzero for: PositiveInteger+32, from: PositiveInteger+8,16];
from_integer![nonzero for: PositiveInteger+64, from: PositiveInteger+8,16,32];
from_integer![nonzero for: PositiveInteger+128, from: PositiveInteger+8,16,32,64];
// try_from bigger PositiveInteger (Self)
try_from_integer![nonzero for: PositiveInteger+8, from: PositiveInteger+16,32,64,128];
try_from_integer![nonzero for: PositiveInteger+16, from: PositiveInteger+32,64,128];
try_from_integer![nonzero for: PositiveInteger+32, from: PositiveInteger+64,128];
try_from_integer![nonzero for: PositiveInteger+64, from: PositiveInteger+128];

/* remaining fallible integer conversions */

// try_from Integer (only the positive values)
try_from_integer![int_new for: PositiveInteger+8, from: Integer+8,16,32,64,128];
try_from_integer![int_new for: PositiveInteger+16, from: Integer+8,16,32,64,128];
try_from_integer![int_new for: PositiveInteger+32, from: Integer+8,16,32,64,128];
try_from_integer![int_new for: PositiveInteger+64, from: Integer+8,16,32,64,128];
try_from_integer![int_new for: PositiveInteger+128, from: Integer+8,16,32,64,128];

// try_from NonZeroInteger (only the positive values)
try_from_integer![int for: PositiveInteger+8, from: NonZeroInteger+8,16,32,64,128];
try_from_integer![int for: PositiveInteger+16, from: NonZeroInteger+8,16,32,64,128];
try_from_integer![int for: PositiveInteger+32, from: NonZeroInteger+8,16,32,64,128];
try_from_integer![int for: PositiveInteger+64, from: NonZeroInteger+8,16,32,64,128];
try_from_integer![int for: PositiveInteger+128, from: NonZeroInteger+8,16,32,64,128];

/* impossible conversions */

// try_from NegativeInteger (no valid values)
try_from_any![error for: PositiveInteger+8, from: NegativeInteger+8,16,32,64,128];
try_from_any![error for: PositiveInteger+16, from: NegativeInteger+8,16,32,64,128];
try_from_any![error for: PositiveInteger+32, from: NegativeInteger+8,16,32,64,128];
try_from_any![error for: PositiveInteger+64, from: NegativeInteger+8,16,32,64,128];
try_from_any![error for: PositiveInteger+128, from: NegativeInteger+8,16,32,64,128];

// try_from NonPositiveInteger (no valid values)
try_from_any![error for: PositiveInteger+8, from: NonPositiveInteger+8,16,32,64,128];
try_from_any![error for: PositiveInteger+16, from: NonPositiveInteger+8,16,32,64,128];
try_from_any![error for: PositiveInteger+32, from: NonPositiveInteger+8,16,32,64,128];
try_from_any![error for: PositiveInteger+64, from: NonPositiveInteger+8,16,32,64,128];
try_from_any![error for: PositiveInteger+128, from: NonPositiveInteger+8,16,32,64,128];

#[cfg(test)]
mod tests {
    use crate::all::{abbr::*, NumeraResult};
    use core::num::{NonZeroI16, NonZeroI8, NonZeroU16, NonZeroU8};

    #[test]
    fn pz_from() -> NumeraResult<()> {
        /* complementary primitive conversions */

        // from smaller or equal sized NonZeroU
        assert_eq![Pz8::new(100)?, NonZeroU8::new(100).unwrap().into()];
        assert_eq![Pz16::new(100)?, NonZeroU8::new(100).unwrap().into()];
        // try_from bigger NonZeroU
        assert_eq![Pz8::new(100)?, NonZeroU16::new(100).unwrap().try_into()?];
        assert![TryInto::<Pz8>::try_into(NonZeroU16::new(500).unwrap()).is_err()];

        /* remaining fallible primitive conversions */

        // try_from i (only the positive values)
        assert_eq![Pz8::new(100)?, 100_i8.try_into()?];
        assert_eq![Pz8::new(200)?, 200_i16.try_into()?];
        assert_eq![Pz16::new(100)?, 100_i8.try_into()?];
        assert![TryInto::<Pz8>::try_into(0_i8).is_err()];
        assert![TryInto::<Pz8>::try_into(-100_i8).is_err()];
        assert![TryInto::<Pz8>::try_into(500_i16).is_err()];

        // try_from u (only the positive values)
        assert_eq![Pz8::new(200)?, 200_u8.try_into()?];
        assert_eq![Pz8::new(200)?, 200_u16.try_into()?];
        assert_eq![Pz16::new(200)?, 200_u8.try_into()?];
        assert![TryInto::<Pz8>::try_into(0_u8).is_err()];
        assert![TryInto::<Pz8>::try_into(500_u16).is_err()];

        // try_from NonZeroI (only the positive values)
        assert_eq![Pz8::new(100)?, NonZeroI8::new(100).unwrap().try_into()?];
        assert_eq![Pz8::new(200)?, NonZeroI16::new(200).unwrap().try_into()?];
        assert_eq![Pz16::new(100)?, NonZeroI8::new(100).unwrap().try_into()?];
        assert![TryInto::<Pz8>::try_into(NonZeroI8::new(-100).unwrap()).is_err()];
        assert![TryInto::<Pz8>::try_into(NonZeroI16::new(500).unwrap()).is_err()];

        /* complementary Integer conversions */

        // from smaller PositiveInteger (Self)
        assert_eq![Pz16::new(200)?, Pz8::new(200)?.into()];
        // try_from bigger PositiveInteger (Self)
        assert_eq![Pz8::new(200)?, Pz16::new(200)?.try_into()?];
        assert![TryInto::<Pz8>::try_into(Pz16::new(500)?).is_err()];

        /* remaining fallible integer conversions */

        // try_from Integer (only the positive values)
        assert_eq![Pz8::new(100)?, Z8::new(100).try_into()?];
        assert_eq![Pz8::new(100)?, Z16::new(100).try_into()?];
        assert_eq![Pz16::new(100)?, Z8::new(100).try_into()?];
        assert![TryInto::<Pz8>::try_into(Z8::new(0)).is_err()];
        assert![TryInto::<Pz8>::try_into(Z8::new(-100)).is_err()];
        assert![TryInto::<Pz8>::try_into(Z16::new(500)).is_err()];

        // try_from NonZeroInteger (only the positive values)
        assert_eq![Pz8::new(100)?, N0z8::new(100)?.try_into()?];
        assert_eq![Pz8::new(100)?, N0z16::new(100)?.try_into()?];
        assert_eq![Pz16::new(100)?, N0z8::new(100)?.try_into()?];
        assert![TryInto::<Pz8>::try_into(N0z8::new(-100)?).is_err()];
        assert![TryInto::<Pz8>::try_into(N0z16::new(500)?).is_err()];

        /* impossible conversions */

        // try_from NegativeInteger (no valid values)
        assert![TryInto::<Pz8>::try_into(Nz8::new(100)?).is_err()];

        // try_from NonPositiveInteger (no valid values)
        assert![TryInto::<Pz8>::try_into(Npz8::new(100)).is_err()];

        Ok(())
    }
}
