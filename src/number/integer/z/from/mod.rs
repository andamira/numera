// numera::number::integer::z::from

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

/* for Integer */

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
from_primitive![non0 for:Integer+16, from:NonZeroU+8];
from_primitive![non0 for:Integer+32, from:NonZeroU+8,16];
from_primitive![non0 for:Integer+64, from:NonZeroU+8,16,32];
from_primitive![non0 for:Integer+128, from:NonZeroU+8,16,32,64];
// try_from bigger or equal sized NonZeroU
try_from_primitive![non0 for:Integer+8, from:NonZeroU+8,16,32,64,128];
try_from_primitive![non0 for:Integer+16, from:NonZeroU+16,32,64,128];
try_from_primitive![non0 for:Integer+32, from:NonZeroU+32,64,128];
try_from_primitive![non0 for:Integer+64, from:NonZeroU+64,128];
try_from_primitive![non0 for:Integer+128, from:NonZeroU+128];

// from smaller or equal sized NonZeroI
from_primitive![non0 for:Integer+8, from:NonZeroI+8];
from_primitive![non0 for:Integer+16, from:NonZeroI+8,16];
from_primitive![non0 for:Integer+32, from:NonZeroI+8,16,32];
from_primitive![non0 for:Integer+64, from:NonZeroI+8,16,32,64];
from_primitive![non0 for:Integer+128, from:NonZeroI+8,16,32,64,128];
// try_from bigger NonZeroI
try_from_primitive![non0 for:Integer+8, from:NonZeroI+16,32,64,128];
try_from_primitive![non0 for:Integer+16, from:NonZeroI+32,64,128];
try_from_primitive![non0 for:Integer+32, from:NonZeroI+64,128];
try_from_primitive![non0 for:Integer+64, from:NonZeroI+128];

/* complementary Integer conversions */

// from smaller Integer (Self)
from_integer![int for:Integer+16, from:Integer+8];
from_integer![int for:Integer+32, from:Integer+8,16];
from_integer![int for:Integer+64, from:Integer+8,16,32];
from_integer![int for:Integer+128, from:Integer+8,16,32,64];
// try_from bigger Integer (Self)
try_from_integer![int for:Integer+8, from:Integer+16,32,64,128];
try_from_integer![int for:Integer+16, from:Integer+32,64,128];
try_from_integer![int for:Integer+32, from:Integer+64,128];
try_from_integer![int for:Integer+64, from:Integer+128];

// from smaller or equal sized NonZeroInteger
from_integer![non0 for:Integer+8, from:NonZeroInteger+8];
from_integer![non0 for:Integer+16, from:NonZeroInteger+8,16];
from_integer![non0 for:Integer+32, from:NonZeroInteger+8,16,32];
from_integer![non0 for:Integer+64, from:NonZeroInteger+8,16,32,64];
from_integer![non0 for:Integer+128, from:NonZeroInteger+8,16,32,64,128];
// try_from bigger NonZeroInteger
try_from_integer![non0 for:Integer+8, from:NonZeroInteger+16,32,64,128];
try_from_integer![non0 for:Integer+16, from:NonZeroInteger+32,64,128];
try_from_integer![non0 for:Integer+32, from:NonZeroInteger+64,128];
try_from_integer![non0 for:Integer+64, from:NonZeroInteger+128];

// from smaller NonNegativeInteger
from_integer![int for:Integer+16, from:NonNegativeInteger+8];
from_integer![int for:Integer+32, from:NonNegativeInteger+8,16];
from_integer![int for:Integer+64, from:NonNegativeInteger+8,16,32];
from_integer![int for:Integer+128, from:NonNegativeInteger+8,16,32,64];
// from bigger or equal sized NonNegativeInteger
try_from_integer![int for:Integer+8, from:NonNegativeInteger+8,16,32,64,128];
try_from_integer![int for:Integer+16, from:NonNegativeInteger+16,32,64,128];
try_from_integer![int for:Integer+32, from:NonNegativeInteger+32,64,128];
try_from_integer![int for:Integer+64, from:NonNegativeInteger+64,128];
try_from_integer![int for:Integer+128, from:NonNegativeInteger+128];

// from smaller PositiveInteger
from_integer![non0 for:Integer+16, from:PositiveInteger+8];
from_integer![non0 for:Integer+32, from:PositiveInteger+8,16];
from_integer![non0 for:Integer+64, from:PositiveInteger+8,16,32];
from_integer![non0 for:Integer+128, from:PositiveInteger+8,16,32,64];
// from bigger or equal sized PositiveInteger
try_from_integer![non0 for:Integer+8, from:PositiveInteger+8,16,32,64,128];
try_from_integer![non0 for:Integer+16, from:PositiveInteger+16,32,64,128];
try_from_integer![non0 for:Integer+32, from:PositiveInteger+32,64,128];
try_from_integer![non0 for:Integer+64, from:PositiveInteger+64,128];
try_from_integer![non0 for:Integer+128, from:PositiveInteger+128];

// from smaller NonPositiveInteger
from_integer![neg_signed for:Integer+i+16, from:NonPositiveInteger+8];
from_integer![neg_signed for:Integer+i+32, from:NonPositiveInteger+8,16];
from_integer![neg_signed for:Integer+i+64, from:NonPositiveInteger+8,16,32];
from_integer![neg_signed for:Integer+i+128, from:NonPositiveInteger+8,16,32,64];
// from bigger or equal sized NonPositiveInteger
try_from_integer![neg_signed for:Integer+i+8, from:NonPositiveInteger+8,16,32,64,128];
try_from_integer![neg_signed for:Integer+i+16, from:NonPositiveInteger+16,32,64,128];
try_from_integer![neg_signed for:Integer+i+32, from:NonPositiveInteger+32,64,128];
try_from_integer![neg_signed for:Integer+i+64, from:NonPositiveInteger+64,128];
try_from_integer![neg_signed for:Integer+i+128, from:NonPositiveInteger+128];

// from smaller NegativeInteger
from_integer![negnon0_signed for:Integer+i+16, from:NegativeInteger+8];
from_integer![negnon0_signed for:Integer+i+32, from:NegativeInteger+8,16];
from_integer![negnon0_signed for:Integer+i+64, from:NegativeInteger+8,16,32];
from_integer![negnon0_signed for:Integer+i+128, from:NegativeInteger+8,16,32,64];
// from bigger or equal sized NegativeInteger
try_from_integer![negnon0_signed for:Integer+i+8, from:NegativeInteger+8,16,32,64,128];
try_from_integer![negnon0_signed for:Integer+i+16, from:NegativeInteger+16,32,64,128];
try_from_integer![negnon0_signed for:Integer+i+32, from:NegativeInteger+32,64,128];
try_from_integer![negnon0_signed for:Integer+i+64, from:NegativeInteger+64,128];
try_from_integer![negnon0_signed for:Integer+i+128, from:NegativeInteger+128];

// from smaller Prime
from_integer![int for:Integer+16, from:Prime+8];
from_integer![int for:Integer+32, from:Prime+8,16];
from_integer![int for:Integer+64, from:Prime+8,16,32];
from_integer![int for:Integer+128, from:Prime+8,16,32,64];
// from bigger or equal sized Prime
try_from_integer![int for:Integer+8, from:Prime+8,16,32,64,128];
try_from_integer![int for:Integer+16, from:Prime+16,32,64,128];
try_from_integer![int for:Integer+32, from:Prime+32,64,128];
try_from_integer![int for:Integer+64, from:Prime+64,128];
try_from_integer![int for:Integer+128, from:Prime+128];

/* from Integer */

// for bigger or equal sized i (Self inner representation)
for_primitive![int for:i+8, from:Integer+8];
for_primitive![int for:i+16, from:Integer+8,16];
for_primitive![int for:i+32, from:Integer+8,16,32];
for_primitive![int for:i+64, from:Integer+8,16,32,64];
for_primitive![int for:i+128, from:Integer+8,16,32,64,128];
// try_for smaller i (Self inner representation)
try_for_primitive![int for:i+8, from:Integer+16,32,64,128];
try_for_primitive![int for:i+16, from:Integer+32,64,128];
try_for_primitive![int for:i+32, from:Integer+64,128];
try_for_primitive![int for:i+64, from:Integer+128];

// try_for u
try_for_primitive![int for:u+8, from:Integer+8,16,32,64,128];
try_for_primitive![int for:u+16, from:Integer+8,16,32,64,128];
try_for_primitive![int for:u+32, from:Integer+8,16,32,64,128];
try_for_primitive![int for:u+64, from:Integer+8,16,32,64,128];
try_for_primitive![int for:u+128, from:Integer+8,16,32,64,128];

// try_for NonZeroI
try_for_primitive![int_non0 for:NonZeroI+8, from:Integer+8,16,32,64,128];
try_for_primitive![int_non0 for:NonZeroI+16, from:Integer+8,16,32,64,128];
try_for_primitive![int_non0 for:NonZeroI+32, from:Integer+8,16,32,64,128];
try_for_primitive![int_non0 for:NonZeroI+64, from:Integer+8,16,32,64,128];
try_for_primitive![int_non0 for:NonZeroI+128, from:Integer+8,16,32,64,128];

// // try_for NonZeroU
try_for_primitive![int_non0 for:NonZeroU+8, from:Integer+8,16,32,64,128];
try_for_primitive![int_non0 for:NonZeroU+16, from:Integer+8,16,32,64,128];
try_for_primitive![int_non0 for:NonZeroU+32, from:Integer+8,16,32,64,128];
try_for_primitive![int_non0 for:NonZeroU+64, from:Integer+8,16,32,64,128];
try_for_primitive![int_non0 for:NonZeroU+128, from:Integer+8,16,32,64,128];
