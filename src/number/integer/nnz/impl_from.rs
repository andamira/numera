// numera::number::integer::nnz::impl_from
//
//!
//

use crate::number::{
    integer::{
        macros::{impl_from_integer, impl_from_primitive},
        nnz::*,
        pz::*,
    },
    traits::Number,
};
use core::num::{NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8};

/* from smaller or equal unsigned primitive */
impl_from_primitive![many for: NonNegativeInteger + 8, from: u + 8];
impl_from_primitive![many for: NonNegativeInteger + 16, from: u + 8, 16];
impl_from_primitive![many for: NonNegativeInteger + 32, from: u + 8, 16, 32];
impl_from_primitive![many for: NonNegativeInteger + 64, from: u + 8, 16, 32, 64];
impl_from_primitive![many for: NonNegativeInteger + 128, from: u + 8, 16, 32, 64, 128];

/* from smaller or equal nonzero primitive */
impl_from_primitive![many_nonzero
    for: NonNegativeInteger + 8, from: NonZeroU + 8];
impl_from_primitive![many_nonzero
    for: NonNegativeInteger + 16, from: NonZeroU + 8, 16];
impl_from_primitive![many_nonzero
    for: NonNegativeInteger + 32, from: NonZeroU + 8, 16, 32];
impl_from_primitive![many_nonzero
    for: NonNegativeInteger + 64, from: NonZeroU + 8, 16, 32, 64];
impl_from_primitive![many_nonzero
    for: NonNegativeInteger + 128, from: NonZeroU + 8, 16, 32, 64, 128];

/* from smaller sized NonNegativeInteger */
impl_from_integer![many_int
    for: NonNegativeInteger + u + 16, from: NonNegativeInteger + 8];
impl_from_integer![many_int
    for: NonNegativeInteger + u + 32, from: NonNegativeInteger + 8, 16];
impl_from_integer![many_int
    for: NonNegativeInteger + u + 64, from: NonNegativeInteger + 8, 16, 32];
impl_from_integer![many_int
    for: NonNegativeInteger + u + 128, from: NonNegativeInteger + 8, 16, 32, 64];

/* from smaller or equal sized NonNegativeInteger */
impl_from_integer![many_nonzero
    for: NonNegativeInteger + u + 8, from: PositiveInteger + 8];
impl_from_integer![many_nonzero
    for: NonNegativeInteger + u + 16, from: PositiveInteger + 8, 16];
impl_from_integer![many_nonzero
    for: NonNegativeInteger + u + 32, from: PositiveInteger + 8, 16, 32];
impl_from_integer![many_nonzero
    for: NonNegativeInteger + u + 64, from: PositiveInteger + 8, 16, 32, 64];
impl_from_integer![many_nonzero
    for: NonNegativeInteger + u + 128, from: PositiveInteger + 8, 16, 32, 64, 128];

#[cfg(test)]
mod tests {
    use crate::all::*;

    #[test]
    fn nnz_from() -> NumeraResult<()> {
        let _5 = NonNegativeInteger8::new(5)?;
        // 3 ways:
        assert_eq![<u8 as Into<NonNegativeInteger8>>::into(5), _5];
        assert_eq![Into::<NonNegativeInteger8>::into(5), _5];
        assert_eq![_5, 5.into()];

        /* from primitive */
        assert_eq![NonNegativeInteger16::new(100)?, 100_u8.into()];
        assert_eq![NonNegativeInteger16::new(100)?, 100_u16.into()];

        /* from smaller Integer */
        assert_eq![
            NonNegativeInteger16::new(100)?,
            NonNegativeInteger8::new(100)?.into()
        ];
        assert_eq![
            NonNegativeInteger32::new(100)?,
            NonNegativeInteger8::new(100)?.into()
        ];
        assert_eq![
            NonNegativeInteger32::new(100)?,
            NonNegativeInteger16::new(100)?.into()
        ];
        // ...
        assert_eq![
            NonNegativeInteger128::new(100)?,
            NonNegativeInteger8::new(100)?.into()
        ];
        assert_eq![
            NonNegativeInteger128::new(100)?,
            NonNegativeInteger64::new(100)?.into()
        ];

        Ok(())
    }
}
