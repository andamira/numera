// numera::number::integer::npz::impl_from
//
//!
//

use crate::number::{
    integer::{macros::from_integer, *},
    traits::Number,
};

/* infallible conversions */

// from smaller sized NonPositiveInteger (Self)
from_integer![int for: NonPositiveInteger+u+16, from: NonPositiveInteger+8];
from_integer![int for: NonPositiveInteger+u+32, from: NonPositiveInteger+8,16];
from_integer![int for: NonPositiveInteger+u+64, from: NonPositiveInteger+8,16,32];
from_integer![int for: NonPositiveInteger+u+128, from: NonPositiveInteger+8,16,32,64];

// from smaller or equal sized NegativeInteger
from_integer![nonzero for: NonPositiveInteger+u+8, from: NegativeInteger+8];
from_integer![nonzero for: NonPositiveInteger+u+16, from: NegativeInteger+8,16];
from_integer![nonzero for: NonPositiveInteger+u+32, from: NegativeInteger+8,16,32];
from_integer![nonzero for: NonPositiveInteger+u+64, from: NegativeInteger+8,16,32,64];
from_integer![nonzero for: NonPositiveInteger+u+128, from: NegativeInteger+8,16,32,64,128];

#[cfg(test)]
mod tests {
    use crate::all::{abbr::*, NegSigned, NumeraResult};

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
