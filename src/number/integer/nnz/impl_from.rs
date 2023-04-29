// numera::number::integer::nnz::impl_from
//
//!
//

use crate::number::{
    integer::{
        abbr::*,
        macros::{impl_from_integer, impl_from_primitive},
    },
    traits::Number,
};
use core::num::{NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8};

/* infallible conversions */

// from smaller or equal sized u
impl_from_primitive![int for: Nnz + 8, from: u + 8];
impl_from_primitive![int for: Nnz + 16, from: u + 8, 16];
impl_from_primitive![int for: Nnz + 32, from: u + 8, 16, 32];
impl_from_primitive![int for: Nnz + 64, from: u + 8, 16, 32, 64];
impl_from_primitive![int for: Nnz + 128, from: u + 8, 16, 32, 64, 128];

// from smaller or equal sized NonZeroU
impl_from_primitive![nonzero for: Nnz + 8, from: NonZeroU + 8];
impl_from_primitive![nonzero for: Nnz + 16, from: NonZeroU + 8, 16];
impl_from_primitive![nonzero for: Nnz + 32, from: NonZeroU + 8, 16, 32];
impl_from_primitive![nonzero for: Nnz + 64, from: NonZeroU + 8, 16, 32, 64];
impl_from_primitive![nonzero for: Nnz + 128, from: NonZeroU + 8, 16, 32, 64, 128];

// from smaller NonNegativeInteger (Self)
impl_from_integer![int for: Nnz + u + 16, from: Nnz + 8];
impl_from_integer![int for: Nnz + u + 32, from: Nnz + 8, 16];
impl_from_integer![int for: Nnz + u + 64, from: Nnz + 8, 16, 32];
impl_from_integer![int for: Nnz + u + 128, from: Nnz + 8, 16, 32, 64];

// from smaller or equal sized PositiveInteger
impl_from_integer![nonzero for: Nnz + u + 8, from: Pz + 8];
impl_from_integer![nonzero for: Nnz + u + 16, from: Pz + 8, 16];
impl_from_integer![nonzero for: Nnz + u + 32, from: Pz + 8, 16, 32];
impl_from_integer![nonzero for: Nnz + u + 64, from: Pz + 8, 16, 32, 64];
impl_from_integer![nonzero for: Nnz + u + 128, from: Pz + 8, 16, 32, 64, 128];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::all::NumeraResult;

    #[test]
    fn nnz_from() -> NumeraResult<()> {
        let _5 = Nnz8::new(5)?;

        // 3 ways:
        assert_eq![<u8 as Into<Nnz8>>::into(5), _5];
        assert_eq![Into::<Nnz8>::into(5), _5];
        assert_eq![_5, 5.into()];

        // from smaller or equal sized u
        assert_eq![Nnz16::new(100)?, 100_u8.into()];
        assert_eq![Nnz16::new(100)?, 100_u16.into()];

        // from smaller NonNegativeInteger
        assert_eq![Nnz16::new(100)?, Nnz8::new(100)?.into()];
        assert_eq![Nnz32::new(100)?, Nnz8::new(100)?.into()];
        assert_eq![Nnz32::new(100)?, Nnz16::new(100)?.into()];
        // ...
        assert_eq![Nnz128::new(100)?, Nnz8::new(100)?.into()];
        assert_eq![Nnz128::new(100)?, Nnz64::new(100)?.into()];

        Ok(())
    }
}
