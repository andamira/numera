// numera::number::integer::nnz::impl_from
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

// from smaller or equal sized u
from_primitive![int for:NonNegativeInteger+8, from:u+8];
from_primitive![int for:NonNegativeInteger+16, from:u+8,16];
from_primitive![int for:NonNegativeInteger+32, from:u+8,16,32];
from_primitive![int for:NonNegativeInteger+64, from:u+8,16,32,64];
from_primitive![int for:NonNegativeInteger+128, from:u+8,16,32,64,128];
// try_from bigger sized u
try_from_primitive![int for:NonNegativeInteger+8, from:u+16,32,64,128];
try_from_primitive![int for:NonNegativeInteger+16, from:u+32,64,128];
try_from_primitive![int for:NonNegativeInteger+32, from:u+64,128];
try_from_primitive![int for:NonNegativeInteger+64, from:u+128];

// from smaller or equal sized NonZeroU
from_primitive![nonzero for:NonNegativeInteger+8, from:NonZeroU+8];
from_primitive![nonzero for:NonNegativeInteger+16, from:NonZeroU+8,16];
from_primitive![nonzero for:NonNegativeInteger+32, from:NonZeroU+8,16,32];
from_primitive![nonzero for:NonNegativeInteger+64, from:NonZeroU+8,16,32,64];
from_primitive![nonzero for:NonNegativeInteger+128, from:NonZeroU+8,16,32,64,128];
// try_from bigger NonZeroU
try_from_primitive![nonzero for:NonNegativeInteger+8, from:NonZeroU+16,32,64,128];
try_from_primitive![nonzero for:NonNegativeInteger+16, from:NonZeroU+32,64,128];
try_from_primitive![nonzero for:NonNegativeInteger+32, from:NonZeroU+64,128];
try_from_primitive![nonzero for:NonNegativeInteger+64, from:NonZeroU+128];

/* remaining fallible primitive conversions */

// try_from i (only the non-negative values)
try_from_primitive![int for:NonNegativeInteger+8, from:i+8,16,32,64,128];
try_from_primitive![int for:NonNegativeInteger+16, from:i+8,16,32,64,128];
try_from_primitive![int for:NonNegativeInteger+32, from:i+8,16,32,64,128];
try_from_primitive![int for:NonNegativeInteger+64, from:i+8,16,32,64,128];
try_from_primitive![int for:NonNegativeInteger+128, from:i+8,16,32,64,128];

// try_from NonZeroI (only the non-negative values)
try_from_primitive![nonzero for:NonNegativeInteger+8, from:NonZeroI+8,16,32,64,128];
try_from_primitive![nonzero for:NonNegativeInteger+16, from:NonZeroI+8,16,32,64,128];
try_from_primitive![nonzero for:NonNegativeInteger+32, from:NonZeroI+8,16,32,64,128];
try_from_primitive![nonzero for:NonNegativeInteger+64, from:NonZeroI+8,16,32,64,128];
try_from_primitive![nonzero for:NonNegativeInteger+128, from:NonZeroI+8,16,32,64,128];

/* complementary Integer conversions */

// from smaller NonNegativeInteger (Self)
from_integer![int for:NonNegativeInteger+u+16, from:NonNegativeInteger+8];
from_integer![int for:NonNegativeInteger+u+32, from:NonNegativeInteger+8,16];
from_integer![int for:NonNegativeInteger+u+64, from:NonNegativeInteger+8,16,32];
from_integer![int for:NonNegativeInteger+u+128, from:NonNegativeInteger+8,16,32, 64];
// try_from bigger NonNegativeInteger (Self)
try_from_integer![int for:NonNegativeInteger+u+8, from:NonNegativeInteger+16,32,64,128];
try_from_integer![int for:NonNegativeInteger+u+16, from:NonNegativeInteger+32,64,128];
try_from_integer![int for:NonNegativeInteger+u+32, from:NonNegativeInteger+64,128];
try_from_integer![int for:NonNegativeInteger+u+64, from:NonNegativeInteger+128];

// from smaller or equal sized PositiveInteger
from_integer![nonzero for:NonNegativeInteger+u+8, from:PositiveInteger+8];
from_integer![nonzero for:NonNegativeInteger+u+16, from:PositiveInteger+8,16];
from_integer![nonzero for:NonNegativeInteger+u+32, from:PositiveInteger+8,16,32];
from_integer![nonzero for:NonNegativeInteger+u+64, from:PositiveInteger+8,16,32,64];
from_integer![nonzero for:NonNegativeInteger+u+128, from:PositiveInteger+8,16,32,64,128];
// try_from bigger PositiveInteger
try_from_integer![nonzero for:NonNegativeInteger+u+8, from:PositiveInteger+16,32,64,128];
try_from_integer![nonzero for:NonNegativeInteger+u+16, from:PositiveInteger+32,64,128];
try_from_integer![nonzero for:NonNegativeInteger+u+32, from:PositiveInteger+64,128];
try_from_integer![nonzero for:NonNegativeInteger+u+64, from:PositiveInteger+128];

/* remaining fallible integer conversions */

// try_from Integer (only the non-negative values)
try_from_integer![int for:NonNegativeInteger+u+8, from:Integer+8,16,32,64,128];
try_from_integer![int for:NonNegativeInteger+u+16, from:Integer+8,16,32,64,128];
try_from_integer![int for:NonNegativeInteger+u+32, from:Integer+8,16,32,64,128];
try_from_integer![int for:NonNegativeInteger+u+64, from:Integer+8,16,32,64,128];
try_from_integer![int for:NonNegativeInteger+u+128, from:Integer+8,16,32,64,128];

// try_from NonZeroInteger (only the positive values)
try_from_integer![nonzero for:NonNegativeInteger+u+8, from:NonZeroInteger+8,16,32,64,128];
try_from_integer![nonzero for:NonNegativeInteger+u+16, from:NonZeroInteger+8,16,32,64,128];
try_from_integer![nonzero for:NonNegativeInteger+u+32, from:NonZeroInteger+8,16,32,64,128];
try_from_integer![nonzero for:NonNegativeInteger+u+64, from:NonZeroInteger+8,16,32,64,128];
try_from_integer![nonzero for:NonNegativeInteger+u+128, from:NonZeroInteger+8,16,32,64,128];

// try_from NonPositiveInteger (only the 0)
try_from_integer![int for:NonNegativeInteger+u+8, from:NonPositiveInteger+8,16,32,64,128];
try_from_integer![int for:NonNegativeInteger+u+16, from:NonPositiveInteger+8,16,32,64,128];
try_from_integer![int for:NonNegativeInteger+u+32, from:NonPositiveInteger+8,16,32,64,128];
try_from_integer![int for:NonNegativeInteger+u+64, from:NonPositiveInteger+8,16,32,64,128];
try_from_integer![int for:NonNegativeInteger+u+128, from:NonPositiveInteger+8,16,32,64,128];

/* impossible Integer conversions */

// try_from NegativeInteger (no valid values)
try_from_integer![error for: NonNegativeInteger+u+8, from: NegativeInteger+8,16,32,64,128];
try_from_integer![error for: NonNegativeInteger+u+16, from: NegativeInteger+8,16,32,64,128];
try_from_integer![error for: NonNegativeInteger+u+32, from: NegativeInteger+8,16,32,64,128];
try_from_integer![error for: NonNegativeInteger+u+64, from: NegativeInteger+8,16,32,64,128];
try_from_integer![error for: NonNegativeInteger+u+128, from: NegativeInteger+8,16,32,64,128];

#[cfg(test)]
mod tests {
    use crate::all::{abbr::*, NegSigned, NumeraResult};

    #[test]
    fn nnz_from() -> NumeraResult<()> {
        let _5 = Nnz8::new(5);

        // 3 ways:
        assert_eq![<u8 as Into<Nnz8>>::into(5), _5];
        assert_eq![Into::<Nnz8>::into(5), _5];
        assert_eq![_5, 5.into()];

        // from smaller or equal sized u
        assert_eq![Nnz16::new(100), 100_u8.into()];
        assert_eq![Nnz16::new(100), 100_u16.into()];

        // from smaller NonNegativeInteger
        assert_eq![Nnz16::new(100), Nnz8::new(100).into()];
        assert_eq![Nnz32::new(100), Nnz8::new(100).into()];
        assert_eq![Nnz32::new(100), Nnz16::new(100).into()];
        // ...
        assert_eq![Nnz128::new(100), Nnz8::new(100).into()];
        assert_eq![Nnz128::new(100), Nnz64::new(100).into()];

        /* impossible conversions */
        assert![TryInto::<Nnz16>::try_into(Nz16::new_neg(100)?).is_err()];

        Ok(())
    }
}
