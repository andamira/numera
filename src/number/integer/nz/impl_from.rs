// numera::number::integer::nz::impl_from
//
//!
//

use crate::number::{
    integer::{macros::impl_from_integer, nz::*},
    traits::Number,
};

/* from smaller sized NegativeInteger */
impl_from_integer![many_nonzero
    for: NegativeInteger + u + 16, from: NegativeInteger + 8];
impl_from_integer![many_nonzero
    for: NegativeInteger + u + 32, from: NegativeInteger + 8, 16];
impl_from_integer![many_nonzero
    for: NegativeInteger + u + 64, from: NegativeInteger + 8, 16, 32];
impl_from_integer![many_nonzero
    for: NegativeInteger + u + 128, from: NegativeInteger + 8, 16, 32, 64];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::all::{NegSigned, NumeraResult};

    #[test]
    fn nz_from() -> NumeraResult<()> {
        /* from smaller NegativeInteger */
        assert_eq![
            NegativeInteger16::new_neg(100)?,
            NegativeInteger8::new_neg(100)?.into()
        ];
        assert_eq![
            NegativeInteger32::new_neg(100)?,
            NegativeInteger8::new_neg(100)?.into()
        ];
        assert_eq![
            NegativeInteger32::new_neg(100)?,
            NegativeInteger16::new_neg(100)?.into()
        ];
        // ...
        assert_eq![
            NegativeInteger128::new_neg(100)?,
            NegativeInteger8::new_neg(100)?.into()
        ];
        assert_eq![
            NegativeInteger128::new_neg(100)?,
            NegativeInteger64::new_neg(100)?.into()
        ];

        Ok(())
    }
}
