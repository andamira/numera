// numera::number::integer::nz::impl_from
//
//!
//

use crate::number::{
    integer::{macros::from_integer, *},
    traits::Number,
};

/* infallible conversions */

// from smaller sized NegativeInteger (Self)
from_integer![nonzero for: NegativeInteger+u+16, from: NegativeInteger+8];
from_integer![nonzero for: NegativeInteger+u+32, from: NegativeInteger+8,16];
from_integer![nonzero for: NegativeInteger+u+64, from: NegativeInteger+8,16,32];
from_integer![nonzero for: NegativeInteger+u+128, from: NegativeInteger+8,16,32,64];

#[cfg(test)]
mod tests {
    use crate::all::{abbr::*, NegSigned, NumeraResult};

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
