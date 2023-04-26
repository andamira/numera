// numera::number::integer::pz::impl_from
//
//!
//

use crate::number::{
    integer::{
        macros::{impl_from_integer, impl_from_primitive},
        pz::*,
    },
    traits::Number,
};
use core::num::{NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8};

/* from smaller or equal nonzero primitive */
impl_from_primitive![many_nonzero
    for: PositiveInteger + 8, from: NonZeroU + 8];
impl_from_primitive![many_nonzero
    for: PositiveInteger + 16, from: NonZeroU + 8, 16];
impl_from_primitive![many_nonzero
    for: PositiveInteger + 32, from: NonZeroU + 8, 16, 32];
impl_from_primitive![many_nonzero
    for: PositiveInteger + 64, from: NonZeroU + 8, 16, 32, 64];
impl_from_primitive![many_nonzero
    for: PositiveInteger + 128, from: NonZeroU + 8, 16, 32, 64, 128];

/* from smaller sized PositiveInteger */
impl_from_integer![many_nonzero
    for: PositiveInteger + u + 16, from: PositiveInteger + 8];
impl_from_integer![many_nonzero
    for: PositiveInteger + u + 32, from: PositiveInteger + 8, 16];
impl_from_integer![many_nonzero
    for: PositiveInteger + u + 64, from: PositiveInteger + 8, 16, 32];
impl_from_integer![many_nonzero
    for: PositiveInteger + u + 128, from: PositiveInteger + 8, 16, 32, 64];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::NumeraResult;

    #[test]
    fn pz_from() -> NumeraResult<()> {
        assert_eq![PositiveInteger8::new(5)?, NonZeroU8::new(5).unwrap().into()];

        /* from primitive */
        assert_eq![
            PositiveInteger16::new(100)?,
            NonZeroU8::new(100).unwrap().into()
        ];
        assert_eq![
            PositiveInteger16::new(100)?,
            NonZeroU16::new(100).unwrap().into()
        ];

        /* from smaller PositiveInteger */
        assert_eq![
            PositiveInteger16::new(100)?,
            PositiveInteger8::new(100)?.into()
        ];
        assert_eq![
            PositiveInteger32::new(100)?,
            PositiveInteger8::new(100)?.into()
        ];
        assert_eq![
            PositiveInteger32::new(100)?,
            PositiveInteger16::new(100)?.into()
        ];
        // ...
        assert_eq![
            PositiveInteger128::new(100)?,
            PositiveInteger8::new(100)?.into()
        ];
        assert_eq![
            PositiveInteger128::new(100)?,
            PositiveInteger64::new(100)?.into()
        ];

        Ok(())
    }
}
