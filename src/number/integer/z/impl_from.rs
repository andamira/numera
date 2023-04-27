// numera::number::integer::z::impl_from
//
//!
//

use crate::number::{
    integer::{
        macros::{impl_from_integer, impl_from_primitive},
        n0z::*,
        nnz::*,
        npz::*,
        nz::*,
        pz::*,
        z::*,
    },
    traits::Number,
};
use core::ops::Neg;

/* from smaller unsigned primitive, and from smaller or equal signed primitive */
impl_from_primitive![many for: Integer + 8, from: i + 8];
impl_from_primitive![many for: Integer + 16, from: u + 8];
impl_from_primitive![many for: Integer + 16, from: i + 8, 16];
impl_from_primitive![many for: Integer + 32, from: u + 8, 16];
impl_from_primitive![many for: Integer + 32, from: i + 8, 16, 32];
impl_from_primitive![many for: Integer + 64, from: u + 8, 16, 32];
impl_from_primitive![many for: Integer + 64, from: i + 8, 16, 32, 64];
impl_from_primitive![many for: Integer + 128, from: u + 8, 16, 32, 64];
impl_from_primitive![many for: Integer + 128, from: i + 8, 16, 32, 64, 128];

/* from smaller sized Integer */
impl_from_integer![many_int for: Integer + i + 16, from: Integer + 8];
impl_from_integer![many_int for: Integer + i + 32, from: Integer + 8, 16];
impl_from_integer![many_int for: Integer + i + 64, from: Integer + 8, 16, 32];
impl_from_integer![many_int for: Integer + i + 128, from: Integer + 8, 16, 32, 64];

/* from smaller or equal sized NonZeroInteger */
impl_from_integer![many_nonzero for: Integer + i + 8, from: NonZeroInteger + 8];
impl_from_integer![many_nonzero for: Integer + i + 16, from: NonZeroInteger + 8, 16];
impl_from_integer![many_nonzero for: Integer + i + 32, from: NonZeroInteger + 8, 16, 32];
impl_from_integer![many_nonzero for: Integer + i + 128, from: NonZeroInteger + 8, 16, 32, 64, 128];

/* from smaller sized NonNegativeInteger */
impl_from_integer![many_int for: Integer + i + 16, from: NonNegativeInteger + 8];
impl_from_integer![many_int for: Integer + i + 32, from: NonNegativeInteger + 8, 16];
impl_from_integer![many_int for: Integer + i + 64, from: NonNegativeInteger + 8, 16, 32];
impl_from_integer![many_int for: Integer + i + 128, from: NonNegativeInteger + 8, 16, 32, 64];

/* from smaller sized PositiveInteger */
impl_from_integer![many_nonzero for: Integer + i + 16, from: PositiveInteger + 8];
impl_from_integer![many_nonzero for: Integer + i + 32, from: PositiveInteger + 8, 16];
impl_from_integer![many_nonzero for: Integer + i + 64, from: PositiveInteger + 8, 16, 32];
impl_from_integer![many_nonzero for: Integer + i + 128, from: PositiveInteger + 8, 16, 32, 64];

/* from smaller sized NonPositiveInteger */
impl_from_integer![many_int_neg for: Integer + i + 16, from: NonPositiveInteger + 8];
impl_from_integer![many_int_neg for: Integer + i + 32, from: NonPositiveInteger + 8, 16];
impl_from_integer![many_int_neg for: Integer + i + 64, from: NonPositiveInteger + 8, 16, 32];
impl_from_integer![many_int_neg for: Integer + i + 128, from: NonPositiveInteger + 8, 16, 32, 64];

/* from smaller sized NegativeInteger */
impl_from_integer![many_nonzero_neg for: Integer + i + 16, from: NegativeInteger + 8];
impl_from_integer![many_nonzero_neg for: Integer + i + 32, from: NegativeInteger + 8, 16];
impl_from_integer![many_nonzero_neg for: Integer + i + 64, from: NegativeInteger + 8, 16, 32];
impl_from_integer![many_nonzero_neg for: Integer + i + 128, from: NegativeInteger + 8, 16, 32, 64];

#[cfg(test)]
mod tests {
    use crate::all::*;

    #[test]
    fn z_from() -> NumeraResult<()> {
        let _5 = Integer8::new(5)?;
        // 3 ways:
        assert_eq![<i8 as Into<Integer8>>::into(5), _5];
        assert_eq![Into::<Integer8>::into(5), _5];
        assert_eq![_5, 5.into()];

        /* from primitive */
        assert_eq![Integer16::new(100)?, 100_u8.into()];
        assert_eq![Integer16::new(100)?, 100_i16.into()];

        /* from smaller Integer */
        assert_eq![Integer16::new(100)?, Integer8::new(100)?.into()];
        assert_eq![Integer32::new(100)?, Integer8::new(100)?.into()];
        assert_eq![Integer32::new(100)?, Integer16::new(100)?.into()];
        // ...
        assert_eq![Integer128::new(100)?, Integer8::new(100)?.into()];
        assert_eq![Integer128::new(100)?, Integer64::new(100)?.into()];

        /* from smaller or equal non-zero Integer */
        assert_eq![Integer16::new(100)?, NonZeroInteger8::new(100)?.into()];
        assert_eq![Integer16::new(100)?, NonZeroInteger16::new(100)?.into()];
        assert_eq![Integer128::new(100)?, NonZeroInteger128::new(100)?.into()];

        /* from smaller non-negative Integer */
        assert_eq![Integer16::new(100)?, NonNegativeInteger8::new(100)?.into()];
        assert_eq![
            Integer128::new(100)?,
            NonNegativeInteger64::new(100)?.into()
        ];

        /* from smaller non-positive Integer */
        assert_eq![
            Integer16::new(-100)?,
            NonPositiveInteger8::new_neg(100)?.into()
        ];
        assert_eq![
            Integer128::new(-100)?,
            NonPositiveInteger64::new_neg(100)?.into()
        ];

        Ok(())
    }
}
