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
use core::{
    num::{
        NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroU128, NonZeroU16,
        NonZeroU32, NonZeroU64, NonZeroU8,
    },
    ops::Neg,
};

/* complementary primitive conversions */

// from smaller NonZeroU
from_primitive![nonzero for:NonZeroInteger+16, from:NonZeroU+8];
from_primitive![nonzero for:NonZeroInteger+32, from:NonZeroU+8,16];
from_primitive![nonzero for:NonZeroInteger+64, from:NonZeroU+8,16,32];
from_primitive![nonzero for:NonZeroInteger+128, from:NonZeroU+8,16,32,64];
// try_from bigger or equal sized NonZeroU
try_from_primitive![nonzero for:NonZeroInteger+8, from:NonZeroU+8,16,32,64,128];
try_from_primitive![nonzero for:NonZeroInteger+16, from:NonZeroU+16,32,64,128];
try_from_primitive![nonzero for:NonZeroInteger+32, from:NonZeroU+32,64,128];
try_from_primitive![nonzero for:NonZeroInteger+64, from:NonZeroU+64,128];
try_from_primitive![nonzero for:NonZeroInteger+128, from:NonZeroU+128];

// from smaller or equal sized NonZeroI
from_primitive![nonzero for:NonZeroInteger+8, from:NonZeroI+8];
from_primitive![nonzero for:NonZeroInteger+16, from:NonZeroI+8,16];
from_primitive![nonzero for:NonZeroInteger+32, from:NonZeroI+8,16,32];
from_primitive![nonzero for:NonZeroInteger+64, from:NonZeroI+8,16,32,64];
from_primitive![nonzero for:NonZeroInteger+128, from:NonZeroI+8,16,32,64,128];
// try_from bigger NonZeroI
try_from_primitive![nonzero for:NonZeroInteger+8, from:NonZeroI+16,32,64,128];
try_from_primitive![nonzero for:NonZeroInteger+16, from:NonZeroI+32,64,128];
try_from_primitive![nonzero for:NonZeroInteger+32, from:NonZeroI+64,128];
try_from_primitive![nonzero for:NonZeroInteger+64, from:NonZeroI+128];

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

// from smaller sized NonZeroInteger (Self)
from_integer![nonzero for:NonZeroInteger+i+16, from:NonZeroInteger+8];
from_integer![nonzero for:NonZeroInteger+i+32, from:NonZeroInteger+8,16];
from_integer![nonzero for:NonZeroInteger+i+64, from:NonZeroInteger+8,16,32];
from_integer![nonzero for:NonZeroInteger+i+128, from:NonZeroInteger+8,16,32,64];
// try_from bigger sized NonZeroInteger (Self)
try_from_integer![nonzero for:NonZeroInteger+i+8, from:NonZeroInteger+16,32,64,128];
try_from_integer![nonzero for:NonZeroInteger+i+16, from:NonZeroInteger+32,64,128];
try_from_integer![nonzero for:NonZeroInteger+i+32, from:NonZeroInteger+64,128];
try_from_integer![nonzero for:NonZeroInteger+i+64, from:NonZeroInteger+128];

// from smaller sized PositiveInteger
from_integer![nonzero for:NonZeroInteger+i+16, from:PositiveInteger+8];
from_integer![nonzero for:NonZeroInteger+i+32, from:PositiveInteger+8,16];
from_integer![nonzero for:NonZeroInteger+i+64, from:PositiveInteger+8,16,32];
from_integer![nonzero for:NonZeroInteger+i+128, from:PositiveInteger+8,16,32,64];
// try_from bigger or equal sized PositiveInteger
try_from_integer![nonzero for:NonZeroInteger+i+8, from:PositiveInteger+8,16,32,64,128];
try_from_integer![nonzero for:NonZeroInteger+i+16, from:PositiveInteger+16,32,64,128];
try_from_integer![nonzero for:NonZeroInteger+i+32, from:PositiveInteger+32,64,128];
try_from_integer![nonzero for:NonZeroInteger+i+64, from:PositiveInteger+64,128];
try_from_integer![nonzero for:NonZeroInteger+i+128, from:PositiveInteger+128];

// from smaller sized NegativeInteger
from_integer![nonzero_neg for:NonZeroInteger+i+16, from:NegativeInteger+8];
from_integer![nonzero_neg for:NonZeroInteger+i+32, from:NegativeInteger+8,16];
from_integer![nonzero_neg for:NonZeroInteger+i+64, from:NegativeInteger+8,16,32];
from_integer![nonzero_neg for:NonZeroInteger+i+128, from:NegativeInteger+8,16,32,64];
// try_from bigger or equal sized NegativeInteger
try_from_integer![nonzero_neg for:NonZeroInteger+i+8, from:NegativeInteger+8,16,32,64,128];
try_from_integer![nonzero_neg for:NonZeroInteger+i+16, from:NegativeInteger+16,32,64,128];
try_from_integer![nonzero_neg for:NonZeroInteger+i+32, from:NegativeInteger+32,64,128];
try_from_integer![nonzero_neg for:NonZeroInteger+i+64, from:NegativeInteger+64,128];
try_from_integer![nonzero_neg for:NonZeroInteger+i+128, from:NegativeInteger+128];

/* remaining fallible integer conversions */

// try_from Integer
try_from_integer![int_new for:NonZeroInteger+i+8, from:Integer+8,16,32,64,128];
try_from_integer![int_new for:NonZeroInteger+i+16, from:Integer+8,16,32,64,128];
try_from_integer![int_new for:NonZeroInteger+i+32, from:Integer+8,16,32,64,128];
try_from_integer![int_new for:NonZeroInteger+i+64, from:Integer+8,16,32,64,128];
try_from_integer![int_new for:NonZeroInteger+i+128, from:Integer+8,16,32,64,128];

// try_from NonNegativeInteger
try_from_integer![int_new for:NonZeroInteger+i+8, from:NonNegativeInteger+8,16,32,64,128];
try_from_integer![int_new for:NonZeroInteger+i+16, from:NonNegativeInteger+8,16,32,64,128];
try_from_integer![int_new for:NonZeroInteger+i+32, from:NonNegativeInteger+8,16,32,64,128];
try_from_integer![int_new for:NonZeroInteger+i+64, from:NonNegativeInteger+8,16,32,64,128];
try_from_integer![int_new for:NonZeroInteger+i+128, from:NonNegativeInteger+8,16,32,64,128];

// try_from NonPositiveInteger TODO:FIX
// try_from_integer![int_new_neg for:NonZeroInteger+i+8, from:NonPositiveInteger+16]; // TEST:FIX
try_from_integer![int_new for:NonZeroInteger+i+8, from:NonPositiveInteger+8,16,32,64,128];
try_from_integer![int_new for:NonZeroInteger+i+16, from:NonPositiveInteger+8,16,32,64,128];
try_from_integer![int_new for:NonZeroInteger+i+32, from:NonPositiveInteger+8,16,32,64,128];
try_from_integer![int_new for:NonZeroInteger+i+64, from:NonPositiveInteger+8,16,32,64,128];
try_from_integer![int_new for:NonZeroInteger+i+128, from:NonPositiveInteger+8,16,32,64,128];

#[cfg(test)]
mod tests {
    // use super::*;
    // use crate::all::NumeraResult;
    //
    // #[test]
    // fn n0z_from() -> NumeraResult<()> {
    //     Ok(())
    // }
}
