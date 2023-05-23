// numera::number::integer::q::from
//
//!
//

#[cfg(test)]
mod tests;

use crate::number::{
    integer::*,
    rational::{
        macros::{from_integer, from_rational, try_from_integer, try_from_rational},
        *,
    },
};
#[cfg(feature = "try_from")]
use core::num::NonZeroU128;
use core::num::{
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroU16, NonZeroU32, NonZeroU64,
    NonZeroU8,
};

/* complementary primitive conversions */

// from smaller u
from_integer![primint for:Rational+16,num:Z,den:N0z, from:u+8];
from_integer![primint for:Rational+32,num:Z,den:N0z, from:u+8,16];
from_integer![primint for:Rational+64,num:Z,den:N0z, from:u+8,16,32];
from_integer![primint for:Rational+128,num:Z,den:N0z, from:u+8,16,32,64];
// try_from bigger or equal sized u
try_from_integer![primint for:Rational+8,num:Z,den:N0z, from:u+8,16,32,64,128];
try_from_integer![primint for:Rational+16,num:Z,den:N0z, from:u+16,32,64,128];
try_from_integer![primint for:Rational+32,num:Z,den:N0z, from:u+32,64,128];
try_from_integer![primint for:Rational+64,num:Z,den:N0z, from:u+64,128];
try_from_integer![primint for:Rational+128,num:Z,den:N0z, from:u+128];

// from smaller or equal sized i
from_integer![primint for:Rational+8,num:Z,den:N0z, from:i+8];
from_integer![primint for:Rational+16,num:Z,den:N0z, from:i+8,16];
from_integer![primint for:Rational+32,num:Z,den:N0z, from:i+8,16,32];
from_integer![primint for:Rational+64,num:Z,den:N0z, from:i+8,16,32,64];
from_integer![primint for:Rational+128,num:Z,den:N0z, from:i+8,16,32,64,128];
// try_from bigger i
try_from_integer![primint for:Rational+8,num:Z,den:N0z, from:i+16,32,64,128];
try_from_integer![primint for:Rational+16,num:Z,den:N0z, from:i+32,64,128];
try_from_integer![primint for:Rational+32,num:Z,den:N0z, from:i+64,128];
try_from_integer![primint for:Rational+64,num:Z,den:N0z, from:i+128];

// from smaller NonZeroU
from_integer![nonzero for:Rational+16,num:Z,den:N0z, from:NonZeroU+8];
from_integer![nonzero for:Rational+32,num:Z,den:N0z, from:NonZeroU+8,16];
from_integer![nonzero for:Rational+64,num:Z,den:N0z, from:NonZeroU+8,16,32];
from_integer![nonzero for:Rational+128,num:Z,den:N0z, from:NonZeroU+8,16,32,64];
// try_from bigger or equal sized NonZeroU
try_from_integer![nonzero for:Rational+8,num:Z,den:N0z, from:NonZeroU+8,16,32,64,128];
try_from_integer![nonzero for:Rational+16,num:Z,den:N0z, from:NonZeroU+16,32,64,128];
try_from_integer![nonzero for:Rational+32,num:Z,den:N0z, from:NonZeroU+32,64,128];
try_from_integer![nonzero for:Rational+64,num:Z,den:N0z, from:NonZeroU+64,128];
try_from_integer![nonzero for:Rational+128,num:Z,den:N0z, from:NonZeroU+128];

// from smaller or equal sized NonZeroI
from_integer![nonzero for:Rational+8,num:Z,den:N0z, from:NonZeroI+8];
from_integer![nonzero for:Rational+16,num:Z,den:N0z, from:NonZeroI+8,16];
from_integer![nonzero for:Rational+32,num:Z,den:N0z, from:NonZeroI+8,16,32];
from_integer![nonzero for:Rational+64,num:Z,den:N0z, from:NonZeroI+8,16,32,64];
from_integer![nonzero for:Rational+128,num:Z,den:N0z, from:NonZeroI+8,16,32,64,128];
// try_from bigger NonZeroI
try_from_integer![nonzero for:Rational+8,num:Z,den:N0z, from:NonZeroI+16,32,64,128];
try_from_integer![nonzero for:Rational+16,num:Z,den:N0z, from:NonZeroI+32,64,128];
try_from_integer![nonzero for:Rational+32,num:Z,den:N0z, from:NonZeroI+64,128];
try_from_integer![nonzero for:Rational+64,num:Z,den:N0z, from:NonZeroI+128];

/* complementary Integer conversions */

// from smaller or equal sized Integer
from_integer![primint for:Rational+8,num:Z,den:N0z, from:Integer+8];
from_integer![primint for:Rational+16,num:Z,den:N0z, from:Integer+8,16];
from_integer![primint for:Rational+32,num:Z,den:N0z, from:Integer+8,16,32];
from_integer![primint for:Rational+64,num:Z,den:N0z, from:Integer+8,16,32,64];
from_integer![primint for:Rational+128,num:Z,den:N0z, from:Integer+8,16,32,64,128];
// try_from bigger Integer
try_from_integer![primint for:Rational+8,num:Z,den:N0z, from:Integer+16,32,64,128];
try_from_integer![primint for:Rational+16,num:Z,den:N0z, from:Integer+32,64,128];
try_from_integer![primint for:Rational+32,num:Z,den:N0z, from:Integer+64,128];
try_from_integer![primint for:Rational+64,num:Z,den:N0z, from:Integer+128];

// from smaller or equal sized NonZeroInteger
from_integer![primint for:Rational+8,num:Z,den:N0z, from:NonZeroInteger+8];
from_integer![primint for:Rational+16,num:Z,den:N0z, from:NonZeroInteger+8,16];
from_integer![primint for:Rational+32,num:Z,den:N0z, from:NonZeroInteger+8,16,32];
from_integer![primint for:Rational+64,num:Z,den:N0z, from:NonZeroInteger+8,16,32,64];
from_integer![primint for:Rational+128,num:Z,den:N0z, from:NonZeroInteger+8,16,32,64,128];
// try_from bigger NonZeroInteger
try_from_integer![primint for:Rational+8,num:Z,den:N0z, from:NonZeroInteger+16,32,64,128];
try_from_integer![primint for:Rational+16,num:Z,den:N0z, from:NonZeroInteger+32,64,128];
try_from_integer![primint for:Rational+32,num:Z,den:N0z, from:NonZeroInteger+64,128];
try_from_integer![primint for:Rational+64,num:Z,den:N0z, from:NonZeroInteger+128];

// from smaller NonNegativeInteger
from_integer![primint for:Rational+16,num:Z,den:N0z, from:NonNegativeInteger+8];
from_integer![primint for:Rational+32,num:Z,den:N0z, from:NonNegativeInteger+8,16];
from_integer![primint for:Rational+64,num:Z,den:N0z, from:NonNegativeInteger+8,16,32];
from_integer![primint for:Rational+128,num:Z,den:N0z, from:NonNegativeInteger+8,16,32,64];
// try_from bigger or equal sized NonNegativeInteger
try_from_integer![primint for:Rational+8,num:Z,den:N0z, from:NonNegativeInteger+8,16,32,64,128];
try_from_integer![primint for:Rational+16,num:Z,den:N0z, from:NonNegativeInteger+16,32,64,128];
try_from_integer![primint for:Rational+32,num:Z,den:N0z, from:NonNegativeInteger+32,64,128];
try_from_integer![primint for:Rational+64,num:Z,den:N0z, from:NonNegativeInteger+64,128];

// from smaller PositiveInteger
from_integer![primint for:Rational+16,num:Z,den:N0z, from:PositiveInteger+8];
from_integer![primint for:Rational+32,num:Z,den:N0z, from:PositiveInteger+8,16];
from_integer![primint for:Rational+64,num:Z,den:N0z, from:PositiveInteger+8,16,32];
from_integer![primint for:Rational+128,num:Z,den:N0z, from:PositiveInteger+8,16,32,64];
// try_from bigger or equal sized PositiveInteger
try_from_integer![primint for:Rational+8,num:Z,den:N0z, from:PositiveInteger+8,16,32,64,128];
try_from_integer![primint for:Rational+16,num:Z,den:N0z, from:PositiveInteger+16,32,64,128];
try_from_integer![primint for:Rational+32,num:Z,den:N0z, from:PositiveInteger+32,64,128];
try_from_integer![primint for:Rational+64,num:Z,den:N0z, from:PositiveInteger+64,128];
try_from_integer![primint for:Rational+128,num:Z,den:N0z, from:PositiveInteger+128];

// from smaller NonPositiveInteger
from_integer![primint for:Rational+16,num:Z,den:N0z, from:NonPositiveInteger+8];
from_integer![primint for:Rational+32,num:Z,den:N0z, from:NonPositiveInteger+8,16];
from_integer![primint for:Rational+64,num:Z,den:N0z, from:NonPositiveInteger+8,16,32];
from_integer![primint for:Rational+128,num:Z,den:N0z, from:NonPositiveInteger+8,16,32,64];
// try_from bigger or equal sized NonPositiveInteger
try_from_integer![primint for:Rational+8,num:Z,den:N0z, from:NonPositiveInteger+8,16,32,64,128];
try_from_integer![primint for:Rational+16,num:Z,den:N0z, from:NonPositiveInteger+16,32,64,128];
try_from_integer![primint for:Rational+32,num:Z,den:N0z, from:NonPositiveInteger+32,64,128];
try_from_integer![primint for:Rational+64,num:Z,den:N0z, from:NonPositiveInteger+64,128];
try_from_integer![primint for:Rational+128,num:Z,den:N0z, from:NonPositiveInteger+128];

// from smaller NegativeInteger
from_integer![primint for:Rational+16,num:Z,den:N0z, from:NegativeInteger+8];
from_integer![primint for:Rational+32,num:Z,den:N0z, from:NegativeInteger+8,16];
from_integer![primint for:Rational+64,num:Z,den:N0z, from:NegativeInteger+8,16,32];
from_integer![primint for:Rational+128,num:Z,den:N0z, from:NegativeInteger+8,16,32,64];
// try_from bigger or equal sized NegativeInteger
try_from_integer![primint for:Rational+8,num:Z,den:N0z, from:NegativeInteger+8,16,32,64,128];
try_from_integer![primint for:Rational+16,num:Z,den:N0z, from:NegativeInteger+16,32,64,128];
try_from_integer![primint for:Rational+32,num:Z,den:N0z, from:NegativeInteger+32,64,128];
try_from_integer![primint for:Rational+64,num:Z,den:N0z, from:NegativeInteger+64,128];
try_from_integer![primint for:Rational+128,num:Z,den:N0z, from:NegativeInteger+128];

/* complementary Rational conversions */

// from smaller sized Rational (Self)
from_rational![for:Rational+16,num:Z,den:N0z, from:Rational+8];
from_rational![for:Rational+32,num:Z,den:N0z, from:Rational+8,16];
from_rational![for:Rational+64,num:Z,den:N0z, from:Rational+8,16,32];
from_rational![for:Rational+128,num:Z,den:N0z, from:Rational+8,16,32,64];
// try_from bigger Rational (Self)
try_from_rational![for:Rational+8,num:Z,den:N0z, from:Rational+16,32,64,128];
try_from_rational![for:Rational+16,num:Z,den:N0z, from:Rational+32,64,128];
try_from_rational![for:Rational+32,num:Z,den:N0z, from:Rational+64,128];
try_from_rational![for:Rational+64,num:Z,den:N0z, from:Rational+128];

// // from smaller or equal sized NonZeroRational
// from_rational![nonzero for:Rational+i+8, from:NonZeroRational+8];
// from_rational![nonzero for:Rational+i+16, from:NonZeroRational+8,16];
// from_rational![nonzero for:Rational+i+32, from:NonZeroRational+8,16,32];
// from_rational![nonzero for:Rational+i+128, from:NonZeroRational+8,16,32,64,128];
// // try_from bigger NonZeroRational
// try_from_rational![nonzero for:Rational+i+8, from:NonZeroRational+16,32,64,128];
// try_from_rational![nonzero for:Rational+i+16, from:NonZeroRational+32,64,128];
// try_from_rational![nonzero for:Rational+i+32, from:NonZeroRational+64,128];
// try_from_rational![nonzero for:Rational+i+64, from:NonZeroRational+128];

// // from smaller sized NonNegativeRational
// from_rational![int for:Rational+i+16, from:NonNegativeRational+8];
// from_rational![int for:Rational+i+32, from:NonNegativeRational+8,16];
// from_rational![int for:Rational+i+64, from:NonNegativeRational+8,16,32];
// from_rational![int for:Rational+i+128, from:NonNegativeRational+8,16,32,64];
// // try_from bigger NonNegativeRational
// try_from_rational![nonzero for:Rational+i+8, from:NonNegativeRational+16,32,64,128];
// try_from_rational![nonzero for:Rational+i+16, from:NonNegativeRational+32,64,128];
// try_from_rational![nonzero for:Rational+i+32, from:NonNegativeRational+64,128];
// try_from_rational![nonzero for:Rational+i+64, from:NonNegativeRational+128];

// // from smaller sized PositiveRational
// from_rational![nonzero for:Rational+i+16, from:Pq+8];
// from_rational![nonzero for:Rational+i+32, from:Pq+8,16];
// from_rational![nonzero for:Rational+i+64, from:Pq+8,16,32];
// from_rational![nonzero for:Rational+i+128, from:Pq+8,16,32,64];
// // try_from bigger PositiveRational
// try_from_rational![nonzero for:Rational+i+8, from:PositiveRational+16,32,64,128];
// try_from_rational![nonzero for:Rational+i+16, from:PositiveRational+32,64,128];
// try_from_rational![nonzero for:Rational+i+32, from:PositiveRational+64,128];
// try_from_rational![nonzero for:Rational+i+64, from:PositiveRational+128];

// // from smaller sized NonPositiveRational
// from_rational![int_neg for:Rational+i+16, from:NonPositiveRational+8];
// from_rational![int_neg for:Rational+i+32, from:NonPositiveRational+8,16];
// from_rational![int_neg for:Rational+i+64, from:NonPositiveRational+8,16,32];
// from_rational![int_neg for:Rational+i+128, from:NonPositiveRational+8,16,32,64];
// // try_from bigger NonPositiveRational
// try_from_rational![nonzero for:Rational+i+8, from:NonPositiveRational+16,32,64,128];
// try_from_rational![nonzero for:Rational+i+16, from:NonPositiveRational+32,64,128];
// try_from_rational![nonzero for:Rational+i+32, from:NonPositiveRational+64,128];
// try_from_rational![nonzero for:Rational+i+64, from:NonPositiveRational+128];

// // from smaller sized NegativeRational
// from_rational![nonzero_neg for:Rational+i+16, from:NonNegativeRational+8];
// from_rational![nonzero_neg for:Rational+i+32, from:NonNegativeRational+8,16];
// from_rational![nonzero_neg for:Rational+i+64, from:NonNegativeRational+8,16,32];
// from_rational![nonzero_neg for:Rational+i+128, from:NonNegativeRational+8,16,32,64];
// // try_from bigger NegativeRational
// try_from_rational![nonzero for:Rational+i+8, from:NegativeRational+16,32,64,128];
// try_from_rational![nonzero for:Rational+i+16, from:NegativeRational+32,64,128];
// try_from_rational![nonzero for:Rational+i+32, from:NegativeRational+64,128];
// try_from_rational![nonzero for:Rational+i+64, from:NegativeRational+128];
