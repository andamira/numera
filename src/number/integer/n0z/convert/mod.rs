// numera::number::integer::n0z::convert

#[cfg(test)]
mod tests;

use crate::number::{
    integer::{
        macros::{
            for_primitive, from_integer, from_primitive, try_for_primitive, try_from_integer,
            try_from_primitive,
        },
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

/* for NonZeroInteger */

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

// from smaller Prime
from_integer![int_non0 for:NonZeroInteger+16, from:Prime+8];
from_integer![int_non0 for:NonZeroInteger+32, from:Prime+8,16];
from_integer![int_non0 for:NonZeroInteger+64, from:Prime+8,16,32];
from_integer![int_non0 for:NonZeroInteger+128, from:Prime+8,16,32,64];
// from bigger or equal sized Prime
try_from_integer![int_non0 for:NonZeroInteger+8, from:Prime+8,16,32,64,128];
try_from_integer![int_non0 for:NonZeroInteger+16, from:Prime+16,32,64,128];
try_from_integer![int_non0 for:NonZeroInteger+32, from:Prime+32,64,128];
try_from_integer![int_non0 for:NonZeroInteger+64, from:Prime+64,128];
try_from_integer![int_non0 for:NonZeroInteger+128, from:Prime+128];

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

/* from NonZeroInteger */

// for bigger or equal sized NonZeroI (Self inner representation)
for_primitive![int for:NonZeroI+8, from:NonZeroInteger+8];
for_primitive![int for:NonZeroI+16, from:NonZeroInteger+8,16];
for_primitive![int for:NonZeroI+32, from:NonZeroInteger+8,16,32];
for_primitive![int for:NonZeroI+64, from:NonZeroInteger+8,16,32,64];
for_primitive![int for:NonZeroI+128, from:NonZeroInteger+8,16,32,64,128];
// try_for smaller NonZeroI (Self inner representation)
try_for_primitive![int for:NonZeroI+8, from:NonZeroInteger+16,32,64,128];
try_for_primitive![int for:NonZeroI+16, from:NonZeroInteger+32,64,128];
try_for_primitive![int for:NonZeroI+32, from:NonZeroInteger+64,128];
try_for_primitive![int for:NonZeroI+64, from:NonZeroInteger+128];

// for bigger or equal sized i
for_primitive![non0 for:i+8, from:NonZeroInteger+8];
for_primitive![non0 for:i+16, from:NonZeroInteger+8,16];
for_primitive![non0 for:i+32, from:NonZeroInteger+8,16,32];
for_primitive![non0 for:i+64, from:NonZeroInteger+8,16,32,64];
for_primitive![non0 for:i+128, from:NonZeroInteger+8,16,32,64,128];
// try_for smaller i
try_for_primitive![non0 for:i+8, from:NonZeroInteger+16,32,64,128];
try_for_primitive![non0 for:i+16, from:NonZeroInteger+32,64,128];
try_for_primitive![non0 for:i+32, from:NonZeroInteger+64,128];
try_for_primitive![non0 for:i+64, from:NonZeroInteger+128];

// try_for NonZeroU
try_for_primitive![int for:NonZeroU+8, from:NonZeroInteger+8,16,32,64,128];
try_for_primitive![int for:NonZeroU+16, from:NonZeroInteger+8,16,32,64,128];
try_for_primitive![int for:NonZeroU+32, from:NonZeroInteger+8,16,32,64,128];
try_for_primitive![int for:NonZeroU+64, from:NonZeroInteger+8,16,32,64,128];
try_for_primitive![int for:NonZeroU+128, from:NonZeroInteger+8,16,32,64,128];

// try_for u
try_for_primitive![non0 for:u+8, from:NonZeroInteger+8,16,32,64,128];
try_for_primitive![non0 for:u+16, from:NonZeroInteger+8,16,32,64,128];
try_for_primitive![non0 for:u+32, from:NonZeroInteger+8,16,32,64,128];
try_for_primitive![non0 for:u+64, from:NonZeroInteger+8,16,32,64,128];
try_for_primitive![non0 for:u+128, from:NonZeroInteger+8,16,32,64,128];
