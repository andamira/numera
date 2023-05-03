// numera::number::integer::z::impl_from
//
//!
//
// TOC
// - complementary primitive conversions
// - complementary integer conversions

use crate::number::{
    integer::{
        macros::{
            from_integer, from_primitive, try_from_integer, try_from_primitive,
        },
        *,
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
from_primitive![int for:Integer+16, from:u+8];
from_primitive![int for:Integer+32, from:u+8,16];
from_primitive![int for:Integer+64, from:u+8,16,32];
from_primitive![int for:Integer+128, from:u+8,16,32,64];
// try_from bigger or equal sized u
try_from_primitive![int for:Integer+8, from:u+8,16,32,64,128];
try_from_primitive![int for:Integer+16, from:u+16,32,64,128];
try_from_primitive![int for:Integer+32, from:u+32,64,128];
try_from_primitive![int for:Integer+64, from:u+64,128];
try_from_primitive![int for:Integer+128, from:u+128];

// from smaller or equal sized i
from_primitive![int for:Integer+8, from:i+8];
from_primitive![int for:Integer+16, from:i+8,16];
from_primitive![int for:Integer+32, from:i+8,16,32];
from_primitive![int for:Integer+64, from:i+8,16,32,64];
from_primitive![int for:Integer+128, from:i+8,16,32,64,128];
// try_from bigger i
try_from_primitive![int for:Integer+8, from:i+16,32,64,128];
try_from_primitive![int for:Integer+16, from:i+32,64,128];
try_from_primitive![int for:Integer+32, from:i+64,128];
try_from_primitive![int for:Integer+64, from:i+128];

// from smaller NonZeroU
from_primitive![nonzero for:Integer+16, from:NonZeroU+8];
from_primitive![nonzero for:Integer+32, from:NonZeroU+8,16];
from_primitive![nonzero for:Integer+64, from:NonZeroU+8,16,32];
from_primitive![nonzero for:Integer+128, from:NonZeroU+8,16,32,64];
// try_from bigger or equal sized NonZeroU
try_from_primitive![nonzero for:Integer+8, from:NonZeroU+8,16,32,64,128];
try_from_primitive![nonzero for:Integer+16, from:NonZeroU+16,32,64,128];
try_from_primitive![nonzero for:Integer+32, from:NonZeroU+32,64,128];
try_from_primitive![nonzero for:Integer+64, from:NonZeroU+64,128];
try_from_primitive![nonzero for:Integer+128, from:NonZeroU+128];

// from smaller or equal sized NonZeroI
from_primitive![nonzero for:Integer+8, from:NonZeroI+8];
from_primitive![nonzero for:Integer+16, from:NonZeroI+8,16];
from_primitive![nonzero for:Integer+32, from:NonZeroI+8,16,32];
from_primitive![nonzero for:Integer+64, from:NonZeroI+8,16,32,64];
from_primitive![nonzero for:Integer+128, from:NonZeroI+8,16,32,64,128];
// try_from bigger NonZeroI
try_from_primitive![nonzero for:Integer+8, from:NonZeroI+16,32,64,128];
try_from_primitive![nonzero for:Integer+16, from:NonZeroI+32,64,128];
try_from_primitive![nonzero for:Integer+32, from:NonZeroI+64,128];
try_from_primitive![nonzero for:Integer+64, from:NonZeroI+128];

/* complementary Integer conversions */

// from smaller or equal sized Integer (Self)
from_integer![int for:Integer+i+16, from:Integer+8];
from_integer![int for:Integer+i+32, from:Integer+8,16];
from_integer![int for:Integer+i+64, from:Integer+8,16,32];
from_integer![int for:Integer+i+128, from:Integer+8,16,32,64];
// try_from bigger Integer (Self)
try_from_integer![int for:Integer+i+8, from:Integer+16,32,64,128];
try_from_integer![int for:Integer+i+16, from:Integer+32,64,128];
try_from_integer![int for:Integer+i+32, from:Integer+64,128];
try_from_integer![int for:Integer+i+64, from:Integer+128];

// from smaller or equal sized NonZeroInteger
from_integer![nonzero for:Integer+i+8, from:NonZeroInteger+8];
from_integer![nonzero for:Integer+i+16, from:NonZeroInteger+8,16];
from_integer![nonzero for:Integer+i+32, from:NonZeroInteger+8,16,32];
from_integer![nonzero for:Integer+i+64, from:NonZeroInteger+8,16,32,64];
from_integer![nonzero for:Integer+i+128, from:NonZeroInteger+8,16,32,64,128];
// try_from bigger NonZeroInteger
try_from_integer![nonzero for:Integer+i+8, from:NonZeroInteger+16,32,64,128];
try_from_integer![nonzero for:Integer+i+16, from:NonZeroInteger+32,64,128];
try_from_integer![nonzero for:Integer+i+32, from:NonZeroInteger+64,128];
try_from_integer![nonzero for:Integer+i+64, from:NonZeroInteger+128];

// from smaller NonNegativeInteger
from_integer![int for:Integer+i+16, from:NonNegativeInteger+8];
from_integer![int for:Integer+i+32, from:NonNegativeInteger+8,16];
from_integer![int for:Integer+i+64, from:NonNegativeInteger+8,16,32];
from_integer![int for:Integer+i+128, from:NonNegativeInteger+8,16,32,64];
// from bigger or equal sized NonNegativeInteger
try_from_integer![int for:Integer+i+8, from:NonNegativeInteger+8,16,32,64,128];
try_from_integer![int for:Integer+i+16, from:NonNegativeInteger+16,32,64,128];
try_from_integer![int for:Integer+i+32, from:NonNegativeInteger+32,64,128];
try_from_integer![int for:Integer+i+64, from:NonNegativeInteger+64,128];
try_from_integer![int for:Integer+i+128, from:NonNegativeInteger+128];

// from smaller PositiveInteger
from_integer![nonzero for:Integer+i+16, from:PositiveInteger+8];
from_integer![nonzero for:Integer+i+32, from:PositiveInteger+8,16];
from_integer![nonzero for:Integer+i+64, from:PositiveInteger+8,16,32];
from_integer![nonzero for:Integer+i+128, from:PositiveInteger+8,16,32,64];
// from bigger or equal sized PositiveInteger
try_from_integer![nonzero for:Integer+i+8, from:PositiveInteger+8,16,32,64,128];
try_from_integer![nonzero for:Integer+i+16, from:PositiveInteger+16,32,64,128];
try_from_integer![nonzero for:Integer+i+32, from:PositiveInteger+32,64,128];
try_from_integer![nonzero for:Integer+i+64, from:PositiveInteger+64,128];
try_from_integer![nonzero for:Integer+i+128, from:PositiveInteger+128];

// from smaller NonPositiveInteger
from_integer![int_neg for:Integer+i+16, from:NonPositiveInteger+8];
from_integer![int_neg for:Integer+i+32, from:NonPositiveInteger+8,16];
from_integer![int_neg for:Integer+i+64, from:NonPositiveInteger+8,16,32];
from_integer![int_neg for:Integer+i+128, from:NonPositiveInteger+8,16,32,64];
// from bigger or equal sized NonPositiveInteger
try_from_integer![int_neg for:Integer+i+8, from:NonPositiveInteger+8,16,32,64,128];
try_from_integer![int_neg for:Integer+i+16, from:NonPositiveInteger+16,32,64,128];
try_from_integer![int_neg for:Integer+i+32, from:NonPositiveInteger+32,64,128];
try_from_integer![int_neg for:Integer+i+64, from:NonPositiveInteger+64,128];
try_from_integer![int_neg for:Integer+i+128, from:NonPositiveInteger+128];

// from smaller NegativeInteger
from_integer![nonzero_neg for:Integer+i+16, from:NegativeInteger+8];
from_integer![nonzero_neg for:Integer+i+32, from:NegativeInteger+8,16];
from_integer![nonzero_neg for:Integer+i+64, from:NegativeInteger+8,16,32];
from_integer![nonzero_neg for:Integer+i+128, from:NegativeInteger+8,16,32,64];
// from bigger or equal sized NegativeInteger
try_from_integer![nonzero_neg for:Integer+i+8, from:NegativeInteger+8,16,32,64,128];
try_from_integer![nonzero_neg for:Integer+i+16, from:NegativeInteger+16,32,64,128];
try_from_integer![nonzero_neg for:Integer+i+32, from:NegativeInteger+32,64,128];
try_from_integer![nonzero_neg for:Integer+i+64, from:NegativeInteger+64,128];
try_from_integer![nonzero_neg for:Integer+i+128, from:NegativeInteger+128];

#[cfg(test)]
mod tests {
    use crate::all::{abbr::*, NegSigned, NumeraResult};

    #[test]
    fn z_from() -> NumeraResult<()> {
        let _5 = Z8::new(5);

        // 3 ways:
        assert_eq![<i8 as Into<Z8>>::into(5), _5];
        assert_eq![Into::<Z8>::into(5), _5];
        assert_eq![_5, 5.into()];

        // from u,i
        assert_eq![Z16::new(100), 100_u8.into()];
        assert_eq![Z16::new(100), 100_i16.into()];

        // from smaller Integer
        assert_eq![Z16::new(100), Z8::new(100).into()];
        assert_eq![Z32::new(100), Z8::new(100).into()];
        assert_eq![Z32::new(100), Z16::new(100).into()];
        // ...
        assert_eq![Z128::new(100), Z8::new(100).into()];
        assert_eq![Z128::new(100), Z64::new(100).into()];

        // from smaller or equal sized NonZeroInteger
        assert_eq![Z16::new(100), N0z8::new(100)?.into()];
        assert_eq![Z16::new(100), N0z16::new(100)?.into()];
        assert_eq![Z128::new(100), N0z128::new(100)?.into()];

        // from smaller NonNegativeInteger
        assert_eq![Z16::new(100), Nnz8::new(100).into()];
        assert_eq![Z128::new(100), Nnz64::new(100).into()];

        // from smaller NonPositiveInteger
        assert_eq![Z16::new(-100), Npz8::new_neg(100)?.into()];
        assert_eq![Z128::new(-100), Npz64::new_neg(100)?.into()];

        Ok(())
    }
}
