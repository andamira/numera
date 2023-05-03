// numera::number::integer::q::impl_from
//
//!
//

use crate::number::{
    integer::{abbr::*, *},
    rational::{
        macros::{from_integer, from_rational},
        *,
    },
};
use core::num::{
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroU16, NonZeroU32, NonZeroU64,
    NonZeroU8,
}; // NonZeroU128,

/* infallible conversions */

// from smaller u
from_integer![prim for:Rational+16,num:Z16,den:N0z16, from:u+8];
from_integer![prim for:Rational+32,num:Z32,den:N0z32, from:u+8,16];
from_integer![prim for:Rational+64,num:Z64,den:N0z64, from:u+8,16,32];
from_integer![prim for:Rational+128,num:Z128,den:N0z128, from:u+8,16,32,64];
// from smaller or equal sized i
from_integer![prim for:Rational+8,num:Z8,den:N0z8, from:i+8];
from_integer![prim for:Rational+16,num:Z16,den:N0z16, from:i+8,16];
from_integer![prim for:Rational+32,num:Z32,den:N0z32, from:i+8,16,32];
from_integer![prim for:Rational+64,num:Z64,den:N0z64, from:i+8,16,32,64];
from_integer![prim for:Rational+128,num:Z128,den:N0z128, from:i+8,16,32,64,128];

// from smaller NonZeroU
from_integer![nonzero for:Rational+16,num:Z16,den:N0z16, from:NonZeroU+8];
from_integer![nonzero for:Rational+32,num:Z32,den:N0z32, from:NonZeroU+8,16];
from_integer![nonzero for:Rational+64,num:Z64,den:N0z64, from:NonZeroU+8,16,32];
from_integer![nonzero for:Rational+128,num:Z128,den:N0z128, from:NonZeroU+8,16,32,64];
// from smaller or equal sized NonZeroI
from_integer![nonzero for:Rational+8,num:Z8,den:N0z8, from:NonZeroI+8];
from_integer![nonzero for:Rational+16,num:Z16,den:N0z16, from:NonZeroI+8,16];
from_integer![nonzero for:Rational+32,num:Z32,den:N0z32, from:NonZeroI+8,16,32];
from_integer![nonzero for:Rational+64,num:Z64,den:N0z64, from:NonZeroI+8,16,32,64];
from_integer![nonzero for:Rational+128,num:Z128,den:N0z128, from:NonZeroI+8,16,32,64,128];

// from smaller or equal sized Integer
from_integer![integer for:Rational+8,num:Z8,den:N0z8, from:Z+8];
from_integer![integer for:Rational+16,num:Z16,den:N0z16, from:Z+8,16];
from_integer![integer for:Rational+32,num:Z32,den:N0z32, from:Z+8,16,32];
from_integer![integer for:Rational+64,num:Z64,den:N0z64, from:Z+8,16,32,64];
from_integer![integer for:Rational+128,num:Z128,den:N0z128, from:Z+8,16,32,64,128];
// from smaller or equal sized NonZeroInteger
from_integer![integer for:Rational+8,num:Z8,den:N0z8, from:N0z+8];
from_integer![integer for:Rational+16,num:Z16,den:N0z16, from:N0z+8,16];
from_integer![integer for:Rational+32,num:Z32,den:N0z32, from:N0z+8,16,32];
from_integer![integer for:Rational+64,num:Z64,den:N0z64, from:N0z+8,16,32,64];
from_integer![integer for:Rational+128,num:Z128,den:N0z128, from:N0z+8,16,32,64,128];

// from smaller PositiveInteger
from_integer![integer for:Rational+16,num:Z16,den:N0z16, from:PositiveInteger+8];
from_integer![integer for:Rational+32,num:Z32,den:N0z32, from:PositiveInteger+8,16];
from_integer![integer for:Rational+64,num:Z64,den:N0z64, from:PositiveInteger+8,16,32];
from_integer![integer for:Rational+128,num:Z128,den:N0z128, from:PositiveInteger+8,16,32,64];
// from smaller NonNegativeInteger
from_integer![integer for:Rational+16,num:Z16,den:N0z16, from:Nnz+8];
from_integer![integer for:Rational+32,num:Z32,den:N0z32, from:Nnz+8,16];
from_integer![integer for:Rational+64,num:Z64,den:N0z64, from:Nnz+8,16,32];
from_integer![integer for:Rational+128,num:Z128,den:N0z128, from:Nnz+8,16,32,64];
// from smaller NegativeInteger
from_integer![integer for:Rational+16,num:Z16,den:N0z16, from:NegativeInteger+8];
from_integer![integer for:Rational+32,num:Z32,den:N0z32, from:NegativeInteger+8,16];
from_integer![integer for:Rational+64,num:Z64,den:N0z64, from:NegativeInteger+8,16,32];
from_integer![integer for:Rational+128,num:Z128,den:N0z128, from:NegativeInteger+8,16,32,64];
// from smaller NonPositiveInteger
from_integer![integer for:Rational+16,num:Z16,den:N0z16, from:Npz+8];
from_integer![integer for:Rational+32,num:Z32,den:N0z32, from:Npz+8,16];
from_integer![integer for:Rational+64,num:Z64,den:N0z64, from:Npz+8,16,32];
from_integer![integer for:Rational+128,num:Z128,den:N0z128, from:Npz+8,16,32,64];

// from smaller sized Rational (Self)
from_rational![for:Rational+16,num:Z16,den:N0z16, from:Rational+8];
from_rational![for:Rational+32,num:Z32,den:N0z32, from:Rational+8,16];
from_rational![for:Rational+64,num:Z64,den:N0z64, from:Rational+8,16,32];
from_rational![for:Rational+128,num:Z128,den:N0z128, from:Rational+8,16,32,64];

// // from smaller or equal sized NonZeroRational
// from_rational![nonzero for:Rational+i+8, from:NonZeroRational+8];
// from_rational![nonzero for:Rational+i+16, from:NonZeroRational+8,16];
// from_rational![nonzero for:Rational+i+32, from:NonZeroRational+8,16,32];
// from_rational![nonzero for:Rational+i+128, from:NonZeroRational+8,16,32,64,128];
//
// // from smaller sized NonNegativeRational
// from_rational![int for:Rational+i+16, from:NonNegativeRational+8];
// from_rational![int for:Rational+i+32, from:NonNegativeRational+8,16];
// from_rational![int for:Rational+i+64, from:NonNegativeRational+8,16,32];
// from_rational![int for:Rational+i+128, from:NonNegativeRational+8,16,32,64];
//
// // from smaller sized PositiveRational
// from_rational![nonzero for:Rational+i+16, from:Pq+8];
// from_rational![nonzero for:Rational+i+32, from:Pq+8,16];
// from_rational![nonzero for:Rational+i+64, from:Pq+8,16,32];
// from_rational![nonzero for:Rational+i+128, from:Pq+8,16,32,64];
//
// // from smaller sized NonPositiveRational
// from_rational![int_neg for:Rational+i+16, from:NonPositiveRational+8];
// from_rational![int_neg for:Rational+i+32, from:NonPositiveRational+8,16];
// from_rational![int_neg for:Rational+i+64, from:NonPositiveRational+8,16,32];
// from_rational![int_neg for:Rational+i+128, from:NonPositiveRational+8,16,32,64];
//
// // from smaller sized NegativeRational
// from_rational![nonzero_neg for:Rational+i+16, from:NonNegativeRational+8];
// from_rational![nonzero_neg for:Rational+i+32, from:NonNegativeRational+8,16];
// from_rational![nonzero_neg for:Rational+i+64, from:NonNegativeRational+8,16,32];
// from_rational![nonzero_neg for:Rational+i+128, from:NonNegativeRational+8,16,32,64];

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::all::NumeraResult;
//
//     #[test]
//     fn q_from() -> NumeraResult<()> { // TODO
//         // let _q5 = Rational8::new((5, 1))?;
//         Ok(())
//     }
// }
