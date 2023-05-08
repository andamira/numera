// numera::number::integer::pz::impl_from
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
// from bigger NonZeroU
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
from_integer![nonzero for: PositiveInteger+u+16, from: PositiveInteger+8];
from_integer![nonzero for: PositiveInteger+u+32, from: PositiveInteger+8,16];
from_integer![nonzero for: PositiveInteger+u+64, from: PositiveInteger+8,16,32];
from_integer![nonzero for: PositiveInteger+u+128, from: PositiveInteger+8,16,32,64];
// from bigger PositiveInteger (Self)
try_from_integer![nonzero for: PositiveInteger+u+8, from: PositiveInteger+16,32,64,128];
try_from_integer![nonzero for: PositiveInteger+u+16, from: PositiveInteger+32,64,128];
try_from_integer![nonzero for: PositiveInteger+u+32, from: PositiveInteger+64,128];
try_from_integer![nonzero for: PositiveInteger+u+64, from: PositiveInteger+128];

/* remaining fallible integer conversions */

// from Integer (only the positive values)
// try_from_integer![int for: PositiveInteger+u+8, from: Integer+8]; // TODO:FIX
//
// try_from_integer![int for: PositiveInteger+u+8, from: Integer+8,16,32,64,128];
// try_from_integer![int for: PositiveInteger+u+16, from: Integer+8,16,32,64,128];
// try_from_integer![int for: PositiveInteger+u+32, from: Integer+8,16,32,64,128];
// try_from_integer![int for: PositiveInteger+u+64, from: Integer+8,16,32,64,128];
// try_from_integer![int for: PositiveInteger+u+128, from: Integer+8,16,32,64,128];

// from NonZeroInteger (only the positive values)
try_from_integer![int for: PositiveInteger+u+8, from: NonZeroInteger+8,16,32,64,128];
try_from_integer![int for: PositiveInteger+u+16, from: NonZeroInteger+8,16,32,64,128];
try_from_integer![int for: PositiveInteger+u+32, from: NonZeroInteger+8,16,32,64,128];
try_from_integer![int for: PositiveInteger+u+64, from: NonZeroInteger+8,16,32,64,128];
try_from_integer![int for: PositiveInteger+u+128, from: NonZeroInteger+8,16,32,64,128];

/* impossible conversions */

// try_from NegativeInteger (no valid values)
try_from_integer![error for: PositiveInteger+u+8, from: NegativeInteger+8,16,32,64,128];
try_from_integer![error for: PositiveInteger+u+16, from: NegativeInteger+8,16,32,64,128];
try_from_integer![error for: PositiveInteger+u+32, from: NegativeInteger+8,16,32,64,128];
try_from_integer![error for: PositiveInteger+u+64, from: NegativeInteger+8,16,32,64,128];
try_from_integer![error for: PositiveInteger+u+128, from: NegativeInteger+8,16,32,64,128];

// try_from NonPositiveInteger (no valid values)
try_from_integer![error for: PositiveInteger+u+8, from: NonPositiveInteger+8,16,32,64,128];
try_from_integer![error for: PositiveInteger+u+16, from: NonPositiveInteger+8,16,32,64,128];
try_from_integer![error for: PositiveInteger+u+32, from: NonPositiveInteger+8,16,32,64,128];
try_from_integer![error for: PositiveInteger+u+64, from: NonPositiveInteger+8,16,32,64,128];
try_from_integer![error for: PositiveInteger+u+128, from: NonPositiveInteger+8,16,32,64,128];

#[cfg(test)]
mod tests {
    use crate::all::{abbr::*, NegSigned, NumeraResult};
    use core::num::{NonZeroU16, NonZeroU8};

    #[test]
    fn pz_from() -> NumeraResult<()> {
        assert_eq![Pz8::new(5)?, NonZeroU8::new(5).unwrap().into()];

        // from smaller or equal sized NonZeroU
        assert_eq![Pz16::new(100)?, NonZeroU8::new(100).unwrap().into()];
        assert_eq![Pz16::new(100)?, NonZeroU16::new(100).unwrap().into()];

        // from smaller sized PositiveInteger (Self)
        assert_eq![Pz16::new(100)?, Pz8::new(100)?.into()];
        assert_eq![Pz32::new(100)?, Pz8::new(100)?.into()];
        assert_eq![Pz32::new(100)?, Pz16::new(100)?.into()];
        // ...
        assert_eq![Pz128::new(100)?, Pz8::new(100)?.into()];
        assert_eq![Pz128::new(100)?, Pz64::new(100)?.into()];

        /* impossible conversions */
        assert![TryInto::<Pz16>::try_into(Nz16::new_neg(100)?).is_err()];
        assert![TryInto::<Pz16>::try_into(Npz16::new_neg(100)?).is_err()];

        Ok(())
    }
}
