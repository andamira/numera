// numera::number::integer::npz::from

#[cfg(test)]
mod tests;

#[cfg(feature = "try_from")]
use crate::number::traits::{ConstZero, Ident};
use crate::number::{
    integer::{
        macros::{
            from_integer, try_for_primitive, try_from_any, try_from_integer, try_from_primitive,
        },
        *,
    },
    traits::Number,
};
#[cfg(feature = "try_from")]
use core::{
    num::{
        NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroU128, NonZeroU16,
        NonZeroU32, NonZeroU64, NonZeroU8,
    },
    ops::Neg,
};

/* for NonPositive Integer */

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
try_from_primitive![non0_pos for: NonPositiveInteger+8, from: NonZeroI+8,16,32,64,128];
try_from_primitive![non0_pos for: NonPositiveInteger+16, from: NonZeroI+8,16,32,64,128];
try_from_primitive![non0_pos for: NonPositiveInteger+32, from: NonZeroI+8,16,32,64,128];
try_from_primitive![non0_pos for: NonPositiveInteger+64, from: NonZeroI+8,16,32,64,128];
try_from_primitive![non0_pos for: NonPositiveInteger+128, from: NonZeroI+8,16,32,64,128];

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
from_integer![non0 for: NonPositiveInteger+8, from: NegativeInteger+8];
from_integer![non0 for: NonPositiveInteger+16, from: NegativeInteger+8,16];
from_integer![non0 for: NonPositiveInteger+32, from: NegativeInteger+8,16,32];
from_integer![non0 for: NonPositiveInteger+64, from: NegativeInteger+8,16,32,64];
from_integer![non0 for: NonPositiveInteger+128, from: NegativeInteger+8,16,32,64,128];
// try_from bigger NegativeInteger
try_from_integer![non0 for: NonPositiveInteger+8, from: NegativeInteger+16,32,64,128];
try_from_integer![non0 for: NonPositiveInteger+16, from: NegativeInteger+32,64,128];
try_from_integer![non0 for: NonPositiveInteger+32, from: NegativeInteger+64,128];
try_from_integer![non0 for: NonPositiveInteger+64, from: NegativeInteger+128];

/* fallible Integer conversions */

// try_from Integer (only the non-positive values)
try_from_integer![neg_int for: NonPositiveInteger+8, from: Integer+8,16,32,64,128];
try_from_integer![neg_int for: NonPositiveInteger+16, from: Integer+8,16,32,64,128];
try_from_integer![neg_int for: NonPositiveInteger+32, from: Integer+8,16,32,64,128];
try_from_integer![neg_int for: NonPositiveInteger+64, from: Integer+8,16,32,64,128];
try_from_integer![neg_int for: NonPositiveInteger+128, from: Integer+8,16,32,64,128];

// try_from NonZeroInteger (only the negative values)
try_from_integer![non0_pos for: NonPositiveInteger+8, from: NonZeroInteger+8,16,32,64,128];
try_from_integer![non0_pos for: NonPositiveInteger+16, from: NonZeroInteger+8,16,32,64,128];
try_from_integer![non0_pos for: NonPositiveInteger+32, from: NonZeroInteger+8,16,32,64,128];
try_from_integer![non0_pos for: NonPositiveInteger+64, from: NonZeroInteger+8,16,32,64,128];
try_from_integer![non0_pos for: NonPositiveInteger+128, from: NonZeroInteger+8,16,32,64,128];

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

// try_from Prime (no valid values)
try_from_any![error for: NonPositiveInteger+8, from: Prime+8,16,32,64,128];
try_from_any![error for: NonPositiveInteger+16, from: Prime+8,16,32,64,128];
try_from_any![error for: NonPositiveInteger+32, from: Prime+8,16,32,64,128];
try_from_any![error for: NonPositiveInteger+64, from: Prime+8,16,32,64,128];
try_from_any![error for: NonPositiveInteger+128, from: Prime+8,16,32,64,128];

/* from NonPositive Integer */

try_for_primitive![neg for: i+8, from: NonPositiveInteger+8,16,32,64,128];
try_for_primitive![neg for: i+16, from: NonPositiveInteger+8,16,32,64,128];
try_for_primitive![neg for: i+32, from: NonPositiveInteger+8,16,32,64,128];
try_for_primitive![neg for: i+64, from: NonPositiveInteger+8,16,32,64,128];
try_for_primitive![neg for: i+128, from: NonPositiveInteger+8,16,32,64,128];

// (only the 0)
try_from_any![zero for: u+8, from: NonPositiveInteger+8,16,32,64,128];
try_from_any![zero for: u+16, from: NonPositiveInteger+8,16,32,64,128];
try_from_any![zero for: u+32, from: NonPositiveInteger+8,16,32,64,128];
try_from_any![zero for: u+64, from: NonPositiveInteger+8,16,32,64,128];
try_from_any![zero for: u+128, from: NonPositiveInteger+8,16,32,64,128];

try_for_primitive![neg_non0 for: NonZeroI+8, from: NonPositiveInteger+8,16,32,64,128];
try_for_primitive![neg_non0 for: NonZeroI+16, from: NonPositiveInteger+8,16,32,64,128];
try_for_primitive![neg_non0 for: NonZeroI+32, from: NonPositiveInteger+8,16,32,64,128];
try_for_primitive![neg_non0 for: NonZeroI+64, from: NonPositiveInteger+8,16,32,64,128];
try_for_primitive![neg_non0 for: NonZeroI+128, from: NonPositiveInteger+8,16,32,64,128];

try_from_any![error for: NonZeroU+8, from: NonPositiveInteger+8,16,32,64,128];
try_from_any![error for: NonZeroU+16, from: NonPositiveInteger+8,16,32,64,128];
try_from_any![error for: NonZeroU+32, from: NonPositiveInteger+8,16,32,64,128];
try_from_any![error for: NonZeroU+64, from: NonPositiveInteger+8,16,32,64,128];
try_from_any![error for: NonZeroU+128, from: NonPositiveInteger+8,16,32,64,128];
