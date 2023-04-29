// numera::number::integer::nz::impl_from
//
//!
//

use crate::number::{
    integer::{abbr::*, macros::impl_from_integer},
    traits::Number,
};

/* infallible conversions */

// from smaller sized NegativeInteger (Self)
impl_from_integer![nonzero for: Nz + u + 16, from: Nz + 8];
impl_from_integer![nonzero for: Nz + u + 32, from: Nz + 8, 16];
impl_from_integer![nonzero for: Nz + u + 64, from: Nz + 8, 16, 32];
impl_from_integer![nonzero for: Nz + u + 128, from: Nz + 8, 16, 32, 64];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::all::{NegSigned, NumeraResult};

    #[test]
    fn nz_from() -> NumeraResult<()> {
        // from smaller NegativeInteger (Self)
        assert_eq![Nz16::new_neg(100)?, Nz8::new_neg(100)?.into()];
        assert_eq![Nz32::new_neg(100)?, Nz8::new_neg(100)?.into()];
        assert_eq![Nz32::new_neg(100)?, Nz16::new_neg(100)?.into()];
        // ...
        assert_eq![Nz128::new_neg(100)?, Nz8::new_neg(100)?.into()];
        assert_eq![Nz128::new_neg(100)?, Nz64::new_neg(100)?.into()];

        Ok(())
    }
}
