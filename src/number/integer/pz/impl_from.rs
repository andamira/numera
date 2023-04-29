// numera::number::integer::pz::impl_from
//
//!
//

use crate::number::{
    integer::{
        macros::{impl_from_integer, impl_from_primitive},
        pz::abbr::*,
    },
    traits::Number,
};
use core::num::{NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8};

/* infallibe conversions*/

// from smaller or equal sized NonZeroU
impl_from_primitive![nonzero for: Pz + 8, from: NonZeroU + 8];
impl_from_primitive![nonzero for: Pz + 16, from: NonZeroU + 8, 16];
impl_from_primitive![nonzero for: Pz + 32, from: NonZeroU + 8, 16, 32];
impl_from_primitive![nonzero for: Pz + 64, from: NonZeroU + 8, 16, 32, 64];
impl_from_primitive![nonzero for: Pz + 128, from: NonZeroU + 8, 16, 32, 64, 128];

// from smaller sized PositiveInteger (Self)
impl_from_integer![nonzero for: Pz + u + 16, from: Pz + 8];
impl_from_integer![nonzero for: Pz + u + 32, from: Pz + 8, 16];
impl_from_integer![nonzero for: Pz + u + 64, from: Pz + 8, 16, 32];
impl_from_integer![nonzero for: Pz + u + 128, from: Pz + 8, 16, 32, 64];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::all::NumeraResult;

    #[test]
    fn pz_from() -> NumeraResult<()> {
        assert_eq![Pz8::new(5)?, NonZeroU8::new(5).unwrap().into()];

        /* from primitive */
        assert_eq![Pz16::new(100)?, NonZeroU8::new(100).unwrap().into()];
        assert_eq![Pz16::new(100)?, NonZeroU16::new(100).unwrap().into()];

        /* from smaller Pz */
        assert_eq![Pz16::new(100)?, Pz8::new(100)?.into()];
        assert_eq![Pz32::new(100)?, Pz8::new(100)?.into()];
        assert_eq![Pz32::new(100)?, Pz16::new(100)?.into()];
        // ...
        assert_eq![Pz128::new(100)?, Pz8::new(100)?.into()];
        assert_eq![Pz128::new(100)?, Pz64::new(100)?.into()];

        Ok(())
    }
}
