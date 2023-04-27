// numera::number::integer::npz::impl_from
//
//!
//

use crate::number::{
    integer::{macros::impl_from_integer, npz::*, nz::*},
    traits::Number,
};

/* from smaller sized NonPositiveInteger */
impl_from_integer![many_int
    for: NonPositiveInteger + u + 16, from: NonPositiveInteger + 8];
impl_from_integer![many_int
    for: NonPositiveInteger + u + 32, from: NonPositiveInteger + 8, 16];
impl_from_integer![many_int
    for: NonPositiveInteger + u + 64, from: NonPositiveInteger + 8, 16, 32];
impl_from_integer![many_int
    for: NonPositiveInteger + u + 128, from: NonPositiveInteger + 8, 16, 32, 64];

/* from smaller or equal sized NegativeInteger */
impl_from_integer![many_nonzero
    for: NonPositiveInteger + u + 8, from: NegativeInteger + 8];
impl_from_integer![many_nonzero
    for: NonPositiveInteger + u + 16, from: NegativeInteger + 8, 16];
impl_from_integer![many_nonzero
    for: NonPositiveInteger + u + 32, from: NegativeInteger + 8, 16, 32];
impl_from_integer![many_nonzero
    for: NonPositiveInteger + u + 64, from: NegativeInteger + 8, 16, 32, 64];
impl_from_integer![many_nonzero
    for: NonPositiveInteger + u + 128, from: NegativeInteger + 8, 16, 32, 64, 128];

#[cfg(test)]
mod tests {
    use crate::all::*;

    #[test]
    fn npz_from() -> NumeraResult<()> {
        /* from smaller PositiveInteger */
        assert_eq![
            NonPositiveInteger16::new_neg(100)?,
            NonPositiveInteger8::new_neg(100)?.into()
        ];
        assert_eq![
            NonPositiveInteger32::new_neg(100)?,
            NonPositiveInteger8::new_neg(100)?.into()
        ];
        assert_eq![
            NonPositiveInteger32::new_neg(100)?,
            NonPositiveInteger16::new_neg(100)?.into()
        ];
        // ...
        assert_eq![
            NonPositiveInteger128::new_neg(100)?,
            NonPositiveInteger8::new_neg(100)?.into()
        ];
        assert_eq![
            NonPositiveInteger128::new_neg(100)?,
            NonPositiveInteger64::new_neg(100)?.into()
        ];

        Ok(())
    }
}
