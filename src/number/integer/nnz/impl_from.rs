// numera::number::integer::nnz::impl_from
//
//!
//

use crate::number::{
    integer::{
        macros::{from_integer, from_primitive},
        nnz::*,
        pz::*,
    },
    traits::Number,
};
use core::num::{NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8};

/* infallible conversions */

// from smaller or equal sized u
from_primitive![int for:NonNegativeInteger+8, from:u+8];
from_primitive![int for:NonNegativeInteger+16, from:u+8,16];
from_primitive![int for:NonNegativeInteger+32, from:u+8,16,32];
from_primitive![int for:NonNegativeInteger+64, from:u+8,16,32,64];
from_primitive![int for:NonNegativeInteger+128, from:u+8,16,32,64,128];

// from smaller or equal sized NonZeroU
from_primitive![nonzero for:NonNegativeInteger+8, from:NonZeroU+8];
from_primitive![nonzero for:NonNegativeInteger+16, from:NonZeroU+8,16];
from_primitive![nonzero for:NonNegativeInteger+32, from:NonZeroU+8,16,32];
from_primitive![nonzero for:NonNegativeInteger+64, from:NonZeroU+8,16,32,64];
from_primitive![nonzero for:NonNegativeInteger+128, from:NonZeroU+8,16,32,64,128];

// from smaller NonNegativeInteger (Self)
from_integer![int for:NonNegativeInteger+u+16, from:NonNegativeInteger+8];
from_integer![int for:NonNegativeInteger+u+32, from:NonNegativeInteger+8,16];
from_integer![int for:NonNegativeInteger+u+64, from:NonNegativeInteger+8,16, 32];
from_integer![int for:NonNegativeInteger+u+128, from:NonNegativeInteger+8,16, 32, 64];

// from smaller or equal sized PositiveInteger
from_integer![nonzero for:NonNegativeInteger+u+8, from:PositiveInteger+8];
from_integer![nonzero for:NonNegativeInteger+u+16, from:PositiveInteger+8,16];
from_integer![nonzero for:NonNegativeInteger+u+32, from:PositiveInteger+8,16,32];
from_integer![nonzero for:NonNegativeInteger+u+64, from:PositiveInteger+8,16,32,64];
from_integer![nonzero for:NonNegativeInteger+u+128, from:PositiveInteger+8,16,32,64,128];

#[cfg(test)]
mod tests {
    use crate::all::abbr::*;
    use crate::error::NumeraResult;

    #[test]
    fn nnz_from() -> NumeraResult<()> {
        let _5 = Nnz8::new(5);

        // 3 ways:
        assert_eq![<u8 as Into<Nnz8>>::into(5), _5];
        assert_eq![Into::<Nnz8>::into(5), _5];
        assert_eq![_5, 5.into()];

        // from smaller or equal sized u
        assert_eq![Nnz16::new(100), 100_u8.into()];
        assert_eq![Nnz16::new(100), 100_u16.into()];

        // from smaller NonNegativeInteger
        assert_eq![Nnz16::new(100), Nnz8::new(100).into()];
        assert_eq![Nnz32::new(100), Nnz8::new(100).into()];
        assert_eq![Nnz32::new(100), Nnz16::new(100).into()];
        // ...
        assert_eq![Nnz128::new(100), Nnz8::new(100).into()];
        assert_eq![Nnz128::new(100), Nnz64::new(100).into()];

        Ok(())
    }
}
