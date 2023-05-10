// numera::number::integer::nz::impl_from
//
//!
//

use crate::number::{
    integer::{
        macros::{from_integer, try_from_any, try_from_integer, try_from_primitive},
        *,
    },
    traits::Number,
};
use core::num::{
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroU128, NonZeroU16,
    NonZeroU32, NonZeroU64, NonZeroU8,
};

/* fallible primitive conversions */

// try_from i (only the negative values)
try_from_primitive![int for: NegativeInteger+8, from: i+8,16,32,64,128];
try_from_primitive![int for: NegativeInteger+16, from: i+8,16,32,64,128];
try_from_primitive![int for: NegativeInteger+32, from: i+8,16,32,64,128];
try_from_primitive![int for: NegativeInteger+64, from: i+8,16,32,64,128];
try_from_primitive![int for: NegativeInteger+128, from: i+8,16,32,64,128];

// try_from NonZeroI (only the negative values)
try_from_primitive![nonzero for: NegativeInteger+8, from: NonZeroI+8,16,32,64,128];
try_from_primitive![nonzero for: NegativeInteger+16, from: NonZeroI+8,16,32,64,128];
try_from_primitive![nonzero for: NegativeInteger+32, from: NonZeroI+8,16,32,64,128];
try_from_primitive![nonzero for: NegativeInteger+64, from: NonZeroI+8,16,32,64,128];
try_from_primitive![nonzero for: NegativeInteger+128, from: NonZeroI+8,16,32,64,128];

/* impossible primitive conversions */

// try_from u (no valid values)
try_from_any![error for: NegativeInteger+8, from: u+8,16,32,64,128];
try_from_any![error for: NegativeInteger+16, from: u+8,16,32,64,128];
try_from_any![error for: NegativeInteger+32, from: u+8,16,32,64,128];
try_from_any![error for: NegativeInteger+64, from: u+8,16,32,64,128];
try_from_any![error for: NegativeInteger+128, from: u+8,16,32,64,128];

// try_from NonZeroU (no valid values)
try_from_any![error for: NegativeInteger+8, from: NonZeroU+8,16,32,64,128];
try_from_any![error for: NegativeInteger+16, from: NonZeroU+8,16,32,64,128];
try_from_any![error for: NegativeInteger+32, from: NonZeroU+8,16,32,64,128];
try_from_any![error for: NegativeInteger+64, from: NonZeroU+8,16,32,64,128];
try_from_any![error for: NegativeInteger+128, from: NonZeroU+8,16,32,64,128];

/* complementary Integer conversions */

// from smaller NegativeInteger (Self)
from_integer![nonzero for: NegativeInteger+16, from: NegativeInteger+8];
from_integer![nonzero for: NegativeInteger+32, from: NegativeInteger+8,16];
from_integer![nonzero for: NegativeInteger+64, from: NegativeInteger+8,16,32];
from_integer![nonzero for: NegativeInteger+128, from: NegativeInteger+8,16,32,64];
// try_from bigger NegativeInteger (Self)
try_from_integer![nonzero for: NegativeInteger+8, from: NegativeInteger+16,32,64,128];
try_from_integer![nonzero for: NegativeInteger+16, from: NegativeInteger+32,64,128];
try_from_integer![nonzero for: NegativeInteger+32, from: NegativeInteger+64,128];
try_from_integer![nonzero for: NegativeInteger+64, from: NegativeInteger+128];

/* fallible Integer conversions */

// try_from Integer (only the negative values) // TODO

// try_from NonZeroInteger (only the negative values) // TODO

/* impossible Integer conversions */

// try_from NonNegativeInteger (no valid values)
try_from_any![error for: NegativeInteger+8, from: NonNegativeInteger+8,16,32,64,128];
try_from_any![error for: NegativeInteger+16, from: NonNegativeInteger+8,16,32,64,128];
try_from_any![error for: NegativeInteger+32, from: NonNegativeInteger+8,16,32,64,128];
try_from_any![error for: NegativeInteger+64, from: NonNegativeInteger+8,16,32,64,128];
try_from_any![error for: NegativeInteger+128, from: NonNegativeInteger+8,16,32,64,128];

// try_from PositiveInteger (no valid values)
try_from_any![error for: NegativeInteger+8, from: PositiveInteger+8,16,32,64,128];
try_from_any![error for: NegativeInteger+16, from: PositiveInteger+8,16,32,64,128];
try_from_any![error for: NegativeInteger+32, from: PositiveInteger+8,16,32,64,128];
try_from_any![error for: NegativeInteger+64, from: PositiveInteger+8,16,32,64,128];
try_from_any![error for: NegativeInteger+128, from: PositiveInteger+8,16,32,64,128];

#[cfg(test)]
mod tests {
    use crate::all::{abbr::*, NegSigned, NumeraResult};

    #[test]
    fn nz_from() -> NumeraResult<()> {
        /* fallible primitive conversions */

        // try_from i (only the negative values)

        // try_from NonZeroI (only the negative values)

        /* impossible primitive conversions */

        // try_from u (no valid values)

        // try_from NonZeroU (no valid values)

        /* complementary Integer conversions */

        // from smaller NegativeInteger (Self)
        // try_from bigger NegativeInteger (Self)

        /* fallible Integer conversions */

        // try_from Integer (only the negative values) // TODO

        // try_from NonZeroInteger (only the negative values) // TODO

        /* impossible Integer conversions */

        // try_from NonNegativeInteger (no valid values)

        // try_from PositiveInteger (no valid values)

        Ok(())
    }
}
