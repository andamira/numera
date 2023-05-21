// numera::number::integer::q::impl_from
//
//!
//

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

#[cfg(test)]
mod tests {
    use crate::all::*;
    use core::num::{NonZeroI8, NonZeroU8};

    #[test]
    fn q_from() -> NumeraResult<()> {
        /* complementary primitive conversions */

        // from smaller u
        assert_eq![Q16::new(100, 1)?, 100_u8.into()];

        // from smaller or equal sized i
        assert_eq![Q8::new(100, 1)?, 100_i8.into()];
        assert_eq![Q16::new(100, 1)?, 100_i8.into()];
        // from smaller NonZeroU
        assert_eq![Q16::new(100, 1)?, NonZeroU8::new(100).unwrap().into()];

        // from smaller or equal sized NonZeroI
        assert_eq![Q8::new(100, 1)?, NonZeroI8::new(100).unwrap().into()];
        assert_eq![Q16::new(100, 1)?, NonZeroI8::new(100).unwrap().into()];

        /* complementary Integer conversions */

        // from smaller or equal sized Integer
        assert_eq![Q16::new(100, 1)?, Z8::new(100).into()];
        assert_eq![Q8::new(100, 1)?, Z8::new(100).into()];

        // from smaller or equal sized NonZeroInteger
        assert_eq![Q16::new(100, 1)?, N0z8::new(100)?.into()];
        assert_eq![Q8::new(100, 1)?, N0z8::new(100)?.into()];

        // from smaller NonNegativeInteger
        assert_eq![Q16::new(100, 1)?, Nnz8::new(100).into()];

        // from smaller PositiveInteger
        assert_eq![Q16::new(100, 1)?, Pz8::new(100)?.into()];

        // from smaller NonPositiveInteger
        assert_eq![Q16::new(-100, 1)?, Npz8::new_neg(100).into()];

        // from smaller NegativeInteger
        assert_eq![Q16::new(-100, 1)?, Nz8::new_neg(100)?.into()];

        /* complementary Rational conversions */

        // from smaller sized Rational (Self)
        assert_eq![Q16::new(41, 107)?, Q16::new(41, 107)?.into()];

        // ...

        Ok(())
    }

    #[test]
    #[cfg(feature = "try_from")]
    fn q_try_from() -> NumeraResult<()> {
        use core::num::{NonZeroI16, NonZeroU16};
        /* complementary primitive conversions */

        // try_from bigger or equal sized u
        assert_eq![Q8::new(100, 1)?, 100_u8.try_into()?];
        assert_eq![Q8::new(100, 1)?, 100_u16.try_into()?];
        assert![TryInto::<Q8>::try_into(200_u16).is_err()];

        // try_from bigger i
        assert_eq![Q8::new(100, 1)?, 100_i16.try_into()?];
        assert![TryInto::<Q8>::try_into(200i16).is_err()];

        // try_from bigger or equal sized NonZeroU
        assert_eq![Q8::new(100, 1)?, NonZeroU8::new(100).unwrap().try_into()?];
        assert_eq![Q8::new(100, 1)?, NonZeroU16::new(100).unwrap().try_into()?];
        assert![TryInto::<Q8>::try_into(NonZeroU16::new(200).unwrap()).is_err()];

        // try_from bigger NonZeroI
        assert_eq![Q8::new(100, 1)?, NonZeroI16::new(100).unwrap().try_into()?];
        assert![TryInto::<Q8>::try_into(NonZeroI16::new(200).unwrap()).is_err()];

        /* complementary Integer conversions */

        // try_from bigger Integer
        assert_eq![Q8::new(100, 1)?, Z16::new(100).try_into()?];
        assert![TryInto::<Q8>::try_into(Z16::new(200)).is_err()];

        // try_from bigger NonZeroInteger
        assert_eq![Q8::new(100, 1)?, N0z16::new(100)?.try_into()?];
        assert![TryInto::<Q8>::try_into(N0z16::new(200)?).is_err()];

        // from bigger or equal sized NonNegativeInteger
        assert_eq![Q8::new(100, 1)?, Nnz16::new(100).try_into()?];
        assert_eq![Q8::new(100, 1)?, Nnz8::new(100).try_into()?];
        assert![TryInto::<Q8>::try_into(Nnz16::new(200)).is_err()];

        // from bigger or equal sized PositiveInteger
        assert_eq![Q8::new(100, 1)?, Pz16::new(100)?.try_into()?];
        assert_eq![Q8::new(100, 1)?, Pz8::new(100)?.try_into()?];
        assert![TryInto::<Q8>::try_into(Pz16::new(200)?).is_err()];

        // from bigger or equal sized NonPositiveInteger
        assert_eq![Q8::new(-100, 1)?, Npz16::new_neg(100).try_into()?];
        assert_eq![Q8::new(-100, 1)?, Npz8::new_neg(100).try_into()?];
        assert![TryInto::<Q8>::try_into(Npz16::new_neg(200)).is_err()];

        // from bigger or equal sized NegativeInteger
        assert_eq![Q8::new(-100, 1)?, Nz16::new_neg(100)?.try_into()?];
        assert_eq![Q8::new(-100, 1)?, Nz8::new_neg(100)?.try_into()?];
        assert![TryInto::<Q8>::try_into(Nz16::new_neg(200)?).is_err()];

        /* complementary Rational conversions */

        // try_from bigger Rational (Self)
        assert_eq![Q8::new(41, 107)?, Q16::new(41, 107)?.try_into()?];

        // ...

        Ok(())
    }
}
