// numera::number::integer::npz::impl_from
//
//!
//

use crate::number::{
    integer::{abbr::*, macros::impl_from_integer},
    traits::Number,
};

/* infallible conversions */

// from smaller sized NonPositiveInteger (Self)
impl_from_integer![int for: Npz + u + 16, from: Npz + 8];
impl_from_integer![int for: Npz + u + 32, from: Npz + 8, 16];
impl_from_integer![int for: Npz + u + 64, from: Npz + 8, 16, 32];
impl_from_integer![int for: Npz + u + 128, from: Npz + 8, 16, 32, 64];

// from smaller or equal sized NegativeInteger
impl_from_integer![nonzero for: Npz + u + 8, from: Nz + 8];
impl_from_integer![nonzero for: Npz + u + 16, from: Nz + 8, 16];
impl_from_integer![nonzero for: Npz + u + 32, from: Nz + 8, 16, 32];
impl_from_integer![nonzero for: Npz + u + 64, from: Nz + 8, 16, 32, 64];
impl_from_integer![nonzero for: Npz + u + 128, from: Nz + 8, 16, 32, 64, 128];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::all::NumeraResult;

    #[test]
    fn npz_from() -> NumeraResult<()> {
        // from smaller PositiveInteger
        assert_eq![Npz16::new_neg(100)?, Npz8::new_neg(100)?.into()];
        assert_eq![Npz32::new_neg(100)?, Npz8::new_neg(100)?.into()];
        assert_eq![Npz32::new_neg(100)?, Npz16::new_neg(100)?.into()];
        // ...
        assert_eq![Npz128::new_neg(100)?, Npz8::new_neg(100)?.into()];
        assert_eq![Npz128::new_neg(100)?, Npz64::new_neg(100)?.into()];

        Ok(())
    }
}
