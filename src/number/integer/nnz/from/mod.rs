// numera::number::integer::nnz::from
//
//!
//

#[cfg(test)]
mod tests;

#[cfg(feature = "try_from")]
use crate::number::traits::{ConstZero, Ident};
use crate::number::{
    integer::{
        macros::{
            for_primitive, from_integer, from_primitive, try_for_primitive, try_from_any,
            try_from_integer, try_from_primitive,
        },
        *,
    },
    traits::Number,
};
#[cfg(feature = "try_from")]
use core::num::{NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8};
use core::num::{NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8};

/* for NonNegativeInteger */

/* complementary primitive conversions */

// from smaller or equal sized u
from_primitive![int for:NonNegativeInteger+8, from:u+8];
from_primitive![int for:NonNegativeInteger+16, from:u+8,16];
from_primitive![int for:NonNegativeInteger+32, from:u+8,16,32];
from_primitive![int for:NonNegativeInteger+64, from:u+8,16,32,64];
from_primitive![int for:NonNegativeInteger+128, from:u+8,16,32,64,128];
// try_from bigger u
try_from_primitive![int for:NonNegativeInteger+8, from:u+16,32,64,128];
try_from_primitive![int for:NonNegativeInteger+16, from:u+32,64,128];
try_from_primitive![int for:NonNegativeInteger+32, from:u+64,128];
try_from_primitive![int for:NonNegativeInteger+64, from:u+128];

// from smaller or equal sized NonZeroU
from_primitive![non0 for:NonNegativeInteger+8, from:NonZeroU+8];
from_primitive![non0 for:NonNegativeInteger+16, from:NonZeroU+8,16];
from_primitive![non0 for:NonNegativeInteger+32, from:NonZeroU+8,16,32];
from_primitive![non0 for:NonNegativeInteger+64, from:NonZeroU+8,16,32,64];
from_primitive![non0 for:NonNegativeInteger+128, from:NonZeroU+8,16,32,64,128];
// try_from bigger NonZeroU
try_from_primitive![non0 for:NonNegativeInteger+8, from:NonZeroU+16,32,64,128];
try_from_primitive![non0 for:NonNegativeInteger+16, from:NonZeroU+32,64,128];
try_from_primitive![non0 for:NonNegativeInteger+32, from:NonZeroU+64,128];
try_from_primitive![non0 for:NonNegativeInteger+64, from:NonZeroU+128];

/* remaining fallible primitive conversions */

// try_from i (only the non-negative values)
try_from_primitive![int for:NonNegativeInteger+8, from:i+8,16,32,64,128];
try_from_primitive![int for:NonNegativeInteger+16, from:i+8,16,32,64,128];
try_from_primitive![int for:NonNegativeInteger+32, from:i+8,16,32,64,128];
try_from_primitive![int for:NonNegativeInteger+64, from:i+8,16,32,64,128];
try_from_primitive![int for:NonNegativeInteger+128, from:i+8,16,32,64,128];

// try_from NonZeroI (only the non-negative values)
try_from_primitive![non0 for:NonNegativeInteger+8, from:NonZeroI+8,16,32,64,128];
try_from_primitive![non0 for:NonNegativeInteger+16, from:NonZeroI+8,16,32,64,128];
try_from_primitive![non0 for:NonNegativeInteger+32, from:NonZeroI+8,16,32,64,128];
try_from_primitive![non0 for:NonNegativeInteger+64, from:NonZeroI+8,16,32,64,128];
try_from_primitive![non0 for:NonNegativeInteger+128, from:NonZeroI+8,16,32,64,128];

/* complementary Integer conversions */

// from smaller NonNegativeInteger (Self)
from_integer![int for:NonNegativeInteger+16, from:NonNegativeInteger+8];
from_integer![int for:NonNegativeInteger+32, from:NonNegativeInteger+8,16];
from_integer![int for:NonNegativeInteger+64, from:NonNegativeInteger+8,16,32];
from_integer![int for:NonNegativeInteger+128, from:NonNegativeInteger+8,16,32, 64];
// try_from bigger NonNegativeInteger (Self)
try_from_integer![int for:NonNegativeInteger+8, from:NonNegativeInteger+16,32,64,128];
try_from_integer![int for:NonNegativeInteger+16, from:NonNegativeInteger+32,64,128];
try_from_integer![int for:NonNegativeInteger+32, from:NonNegativeInteger+64,128];
try_from_integer![int for:NonNegativeInteger+64, from:NonNegativeInteger+128];

// from smaller or equal sized PositiveInteger
from_integer![non0 for:NonNegativeInteger+8, from:PositiveInteger+8];
from_integer![non0 for:NonNegativeInteger+16, from:PositiveInteger+8,16];
from_integer![non0 for:NonNegativeInteger+32, from:PositiveInteger+8,16,32];
from_integer![non0 for:NonNegativeInteger+64, from:PositiveInteger+8,16,32,64];
from_integer![non0 for:NonNegativeInteger+128, from:PositiveInteger+8,16,32,64,128];
// try_from bigger PositiveInteger
try_from_integer![non0 for:NonNegativeInteger+8, from:PositiveInteger+16,32,64,128];
try_from_integer![non0 for:NonNegativeInteger+16, from:PositiveInteger+32,64,128];
try_from_integer![non0 for:NonNegativeInteger+32, from:PositiveInteger+64,128];
try_from_integer![non0 for:NonNegativeInteger+64, from:PositiveInteger+128];

// from smaller or equal sized Prime
from_integer![int for:NonNegativeInteger+8, from:Prime+8];
from_integer![int for:NonNegativeInteger+16, from:Prime+8,16];
from_integer![int for:NonNegativeInteger+32, from:Prime+8,16,32];
from_integer![int for:NonNegativeInteger+64, from:Prime+8,16,32,64];
from_integer![int for:NonNegativeInteger+128, from:Prime+8,16,32,64,128];
// try_from bigger Prime
try_from_integer![int for:NonNegativeInteger+8, from:Prime+16,32,64,128];
try_from_integer![int for:NonNegativeInteger+16, from:Prime+32,64,128];
try_from_integer![int for:NonNegativeInteger+32, from:Prime+64,128];
try_from_integer![int for:NonNegativeInteger+64, from:Prime+128];

/* remaining fallible integer conversions */

// try_from Integer (only the non-negative values)
try_from_integer![int for:NonNegativeInteger+8, from:Integer+8,16,32,64,128];
try_from_integer![int for:NonNegativeInteger+16, from:Integer+8,16,32,64,128];
try_from_integer![int for:NonNegativeInteger+32, from:Integer+8,16,32,64,128];
try_from_integer![int for:NonNegativeInteger+64, from:Integer+8,16,32,64,128];
try_from_integer![int for:NonNegativeInteger+128, from:Integer+8,16,32,64,128];

// try_from NonZeroInteger (only the positive values)
try_from_integer![non0 for:NonNegativeInteger+8, from:NonZeroInteger+8,16,32,64,128];
try_from_integer![non0 for:NonNegativeInteger+16, from:NonZeroInteger+8,16,32,64,128];
try_from_integer![non0 for:NonNegativeInteger+32, from:NonZeroInteger+8,16,32,64,128];
try_from_integer![non0 for:NonNegativeInteger+64, from:NonZeroInteger+8,16,32,64,128];
try_from_integer![non0 for:NonNegativeInteger+128, from:NonZeroInteger+8,16,32,64,128];

// try_from NonPositiveInteger (only the 0)
try_from_any![zero for:NonNegativeInteger+8, from:NonPositiveInteger+8,16,32,64,128];
try_from_any![zero for:NonNegativeInteger+16, from:NonPositiveInteger+8,16,32,64,128];
try_from_any![zero for:NonNegativeInteger+32, from:NonPositiveInteger+8,16,32,64,128];
try_from_any![zero for:NonNegativeInteger+64, from:NonPositiveInteger+8,16,32,64,128];
try_from_any![zero for:NonNegativeInteger+128, from:NonPositiveInteger+8,16,32,64,128];

/* impossible Integer conversions */

// try_from NegativeInteger (no valid values)
try_from_any![error for: NonNegativeInteger+8, from: NegativeInteger+8,16,32,64,128];
try_from_any![error for: NonNegativeInteger+16, from: NegativeInteger+8,16,32,64,128];
try_from_any![error for: NonNegativeInteger+32, from: NegativeInteger+8,16,32,64,128];
try_from_any![error for: NonNegativeInteger+64, from: NegativeInteger+8,16,32,64,128];
try_from_any![error for: NonNegativeInteger+128, from: NegativeInteger+8,16,32,64,128];

/* from NonNegativeInteger */

// for smaller or equal sized u
for_primitive![int for:u+8, from:NonNegativeInteger+8];
for_primitive![int for:u+16, from:NonNegativeInteger+8,16];
for_primitive![int for:u+32, from:NonNegativeInteger+8,16,32];
for_primitive![int for:u+64, from:NonNegativeInteger+8,16,32,64];
for_primitive![int for:u+128, from:NonNegativeInteger+8,16,32,64,128];
// try_for bigger i
try_for_primitive![int for:u+8, from:NonNegativeInteger+16,32,64,128];
try_for_primitive![int for:u+16, from:NonNegativeInteger+32,64,128];
try_for_primitive![int for:u+32, from:NonNegativeInteger+64,128];
try_for_primitive![int for:u+64, from:NonNegativeInteger+128];

// try_for i
try_for_primitive![int for:i+8, from:NonNegativeInteger+8,16,32,64,128];
try_for_primitive![int for:i+16, from:NonNegativeInteger+8,16,32,64,128];
try_for_primitive![int for:i+32, from:NonNegativeInteger+8,16,32,64,128];
try_for_primitive![int for:i+64, from:NonNegativeInteger+8,16,32,64,128];
try_for_primitive![int for:i+128, from:NonNegativeInteger+8,16,32,64,128];

// try_for NonZeroI
try_for_primitive![int_non0 for:NonZeroI+8, from:NonNegativeInteger+8,16,32,64,128];
try_for_primitive![int_non0 for:NonZeroI+16, from:NonNegativeInteger+8,16,32,64,128];
try_for_primitive![int_non0 for:NonZeroI+32, from:NonNegativeInteger+8,16,32,64,128];
try_for_primitive![int_non0 for:NonZeroI+64, from:NonNegativeInteger+8,16,32,64,128];
try_for_primitive![int_non0 for:NonZeroI+128, from:NonNegativeInteger+8,16,32,64,128];

// try_for NonZeroU
try_for_primitive![int_non0 for:NonZeroU+8, from:NonNegativeInteger+8,16,32,64,128];
try_for_primitive![int_non0 for:NonZeroU+16, from:NonNegativeInteger+8,16,32,64,128];
try_for_primitive![int_non0 for:NonZeroU+32, from:NonNegativeInteger+8,16,32,64,128];
try_for_primitive![int_non0 for:NonZeroU+64, from:NonNegativeInteger+8,16,32,64,128];
try_for_primitive![int_non0 for:NonZeroU+128, from:NonNegativeInteger+8,16,32,64,128];
