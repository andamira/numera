// numera::number::integer::pz::impl_from
//
//!
//

use crate::number::{
    integer::{
        macros::{from_integer, from_primitive},
        pz::*,
    },
    traits::Number,
};
use core::num::{NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8};

/* infallibe conversions*/

// from smaller or equal sized NonZeroU
from_primitive![nonzero for: PositiveInteger+8, from: NonZeroU+8];
from_primitive![nonzero for: PositiveInteger+16, from: NonZeroU+8,16];
from_primitive![nonzero for: PositiveInteger+32, from: NonZeroU+8,16,32];
from_primitive![nonzero for: PositiveInteger+64, from: NonZeroU+8,16,32,64];
from_primitive![nonzero for: PositiveInteger+128, from: NonZeroU+8,16,32,64,128];

// from smaller sized PositiveInteger (Self)
from_integer![nonzero for: PositiveInteger+u+16, from: PositiveInteger+8];
from_integer![nonzero for: PositiveInteger+u+32, from: PositiveInteger+8,16];
from_integer![nonzero for: PositiveInteger+u+64, from: PositiveInteger+8,16,32];
from_integer![nonzero for: PositiveInteger+u+128, from: PositiveInteger+8,16,32,64];

#[cfg(test)]
mod tests {
    use crate::all::{abbr::*, NumeraResult};
    use core::num::{NonZeroU16, NonZeroU8};

    #[test]
    fn pz_from() -> NumeraResult<()> {
        assert_eq![Pz8::new(5)?, NonZeroU8::new(5).unwrap().into()];

        // from smaller or equal sized NonZeroU
        assert_eq![Pz16::new(100)?, NonZeroU8::new(100).unwrap().into()];
        assert_eq![Pz16::new(100)?, NonZeroU16::new(100).unwrap().into()];

        // from smaller sized PositiveInteger (Self)
        assert_eq![Pz16::new(100)?, Pz8::new(100)?.into()];
        assert_eq![Pz32::new(100)?, Pz8::new(100)?.into()];
        assert_eq![Pz32::new(100)?, Pz16::new(100)?.into()];
        // ...
        assert_eq![Pz128::new(100)?, Pz8::new(100)?.into()];
        assert_eq![Pz128::new(100)?, Pz64::new(100)?.into()];

        Ok(())
    }
}
