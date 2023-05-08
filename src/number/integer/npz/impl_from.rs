// numera::number::integer::npz::impl_from
//
//!
//

use crate::number::{
    integer::{
        macros::{from_integer, try_from_integer, try_from_primitive},
        *,
    },
    traits::Number,
};
use core::num::{
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroU128, NonZeroU16,
    NonZeroU32, NonZeroU64, NonZeroU8,
};

/* fallible primitive conversions */

// try_from i (only the non-positive values)
try_from_primitive![int for: NonPositiveInteger+8, from: i+8,16,32,64,128];
try_from_primitive![int for: NonPositiveInteger+16, from: i+8,16,32,64,128];
try_from_primitive![int for: NonPositiveInteger+32, from: i+8,16,32,64,128];
try_from_primitive![int for: NonPositiveInteger+64, from: i+8,16,32,64,128];
try_from_primitive![int for: NonPositiveInteger+128, from: i+8,16,32,64,128];

// try_from u (only the 0)
try_from_primitive![int for: NonPositiveInteger+8, from: u+8,16,32,64,128];
try_from_primitive![int for: NonPositiveInteger+16, from: u+8,16,32,64,128];
try_from_primitive![int for: NonPositiveInteger+32, from: u+8,16,32,64,128];
try_from_primitive![int for: NonPositiveInteger+64, from: u+8,16,32,64,128];
try_from_primitive![int for: NonPositiveInteger+128, from: u+8,16,32,64,128];

// try_from NonZeroI (only the negative values)
try_from_primitive![nonzero for: NonPositiveInteger+8, from: NonZeroI+8,16,32,64,128];
try_from_primitive![nonzero for: NonPositiveInteger+16, from: NonZeroI+8,16,32,64,128];
try_from_primitive![nonzero for: NonPositiveInteger+32, from: NonZeroI+8,16,32,64,128];
try_from_primitive![nonzero for: NonPositiveInteger+64, from: NonZeroI+8,16,32,64,128];
try_from_primitive![nonzero for: NonPositiveInteger+128, from: NonZeroI+8,16,32,64,128];

/* impossible primitive conversions */

// try_from NonZeroU (no valid values)
try_from_primitive![error for: NonPositiveInteger+8, from: NonZeroU+8,16,32,64,128];
try_from_primitive![error for: NonPositiveInteger+16, from: NonZeroU+8,16,32,64,128];
try_from_primitive![error for: NonPositiveInteger+32, from: NonZeroU+8,16,32,64,128];
try_from_primitive![error for: NonPositiveInteger+64, from: NonZeroU+8,16,32,64,128];
try_from_primitive![error for: NonPositiveInteger+128, from: NonZeroU+8,16,32,64,128];

/* complementary Integer conversions */

// from smaller NonPositiveInteger (Self)
from_integer![int for: NonPositiveInteger+u+16, from: NonPositiveInteger+8];
from_integer![int for: NonPositiveInteger+u+32, from: NonPositiveInteger+8,16];
from_integer![int for: NonPositiveInteger+u+64, from: NonPositiveInteger+8,16,32];
from_integer![int for: NonPositiveInteger+u+128, from: NonPositiveInteger+8,16,32,64];
// try_from bigger NonPositiveInteger (Self)
try_from_integer![int for: NonPositiveInteger+u+8, from: NonPositiveInteger+16,32,64,128];
try_from_integer![int for: NonPositiveInteger+u+16, from: NonPositiveInteger+32,64,128];
try_from_integer![int for: NonPositiveInteger+u+32, from: NonPositiveInteger+64,128];
try_from_integer![int for: NonPositiveInteger+u+64, from: NonPositiveInteger+128];

// from smaller or equal sized NegativeInteger
from_integer![nonzero for: NonPositiveInteger+u+8, from: NegativeInteger+8];
from_integer![nonzero for: NonPositiveInteger+u+16, from: NegativeInteger+8,16];
from_integer![nonzero for: NonPositiveInteger+u+32, from: NegativeInteger+8,16,32];
from_integer![nonzero for: NonPositiveInteger+u+64, from: NegativeInteger+8,16,32,64];
from_integer![nonzero for: NonPositiveInteger+u+128, from: NegativeInteger+8,16,32,64,128];
// try_from bigger NegativeInteger
try_from_integer![nonzero for: NonPositiveInteger+u+8, from: NegativeInteger+16,32,64,128];
try_from_integer![nonzero for: NonPositiveInteger+u+16, from: NegativeInteger+32,64,128];
try_from_integer![nonzero for: NonPositiveInteger+u+32, from: NegativeInteger+64,128];
try_from_integer![nonzero for: NonPositiveInteger+u+64, from: NegativeInteger+128];

/* fallible Integer conversions */

// try_from Integer (only the non-positive values)
try_from_integer![int for: NonPositiveInteger+u+8, from: Integer+8,16,32,64,128];
try_from_integer![int for: NonPositiveInteger+u+16, from: Integer+8,16,32,64,128];
try_from_integer![int for: NonPositiveInteger+u+32, from: Integer+8,16,32,64,128];
try_from_integer![int for: NonPositiveInteger+u+64, from: Integer+8,16,32,64,128];
try_from_integer![int for: NonPositiveInteger+u+128, from: Integer+8,16,32,64,128];

// try_from NonZeroInteger (only the negative values)
try_from_integer![nonzero for: NonPositiveInteger+u+8, from: NonZeroInteger+8,16,32,64,128];
try_from_integer![nonzero for: NonPositiveInteger+u+16, from: NonZeroInteger+8,16,32,64,128];
try_from_integer![nonzero for: NonPositiveInteger+u+32, from: NonZeroInteger+8,16,32,64,128];
try_from_integer![nonzero for: NonPositiveInteger+u+64, from: NonZeroInteger+8,16,32,64,128];
try_from_integer![nonzero for: NonPositiveInteger+u+128, from: NonZeroInteger+8,16,32,64,128];

// try_from NonNegativeInteger (only the 0)
try_from_integer![int for: NonPositiveInteger+u+8, from: NonNegativeInteger+8,16,32,64,128];
try_from_integer![int for: NonPositiveInteger+u+16, from: NonNegativeInteger+8,16,32,64,128];
try_from_integer![int for: NonPositiveInteger+u+32, from: NonNegativeInteger+8,16,32,64,128];
try_from_integer![int for: NonPositiveInteger+u+64, from: NonNegativeInteger+8,16,32,64,128];
try_from_integer![int for: NonPositiveInteger+u+128, from: NonNegativeInteger+8,16,32,64,128];

/* impossible Integer conversions */

// try_from PositiveInteger (no valid values)
try_from_integer![error for: NonPositiveInteger+u+8, from: PositiveInteger+8,16,32,64,128];
try_from_integer![error for: NonPositiveInteger+u+16, from: PositiveInteger+8,16,32,64,128];
try_from_integer![error for: NonPositiveInteger+u+32, from: PositiveInteger+8,16,32,64,128];
try_from_integer![error for: NonPositiveInteger+u+64, from: PositiveInteger+8,16,32,64,128];
try_from_integer![error for: NonPositiveInteger+u+128, from: PositiveInteger+8,16,32,64,128];

#[cfg(test)]
mod tests {
    use super::NonZeroU16;
    use crate::all::{abbr::*, NegSigned, NumeraResult};

    #[test]
    fn npz_from() -> NumeraResult<()> {
        // from smaller PositiveInteger
        assert_eq![Npz16::new_neg(100)?, Npz8::new_neg(100)?.into()];
        assert_eq![Npz32::new_neg(100)?, Npz8::new_neg(100)?.into()];
        assert_eq![Npz32::new_neg(100)?, Npz16::new_neg(100)?.into()];
        // ...
        assert_eq![Npz128::new_neg(100)?, Npz8::new_neg(100)?.into()];
        assert_eq![Npz128::new_neg(100)?, Npz64::new_neg(100)?.into()];

        /* impossible conversions */
        assert![TryInto::<Npz16>::try_into(NonZeroU16::new(100).unwrap()).is_err()];
        assert![TryInto::<Npz16>::try_into(Pz16::new(100)?).is_err()];

        Ok(())
    }
}
