// numera::integer::nz::family
//
//!
//

use super::{super::family::define_integer_family, *};
use crate::{
    error::NumeraResult,
    number::traits::{
        ConstLowerBounded, ConstNegOne, ConstUpperBounded, LowerBounded, NegOne, Negative,
        NonPositive, NonZero, UpperBounded,
    },
};

define_integer_family![build_variants:
    NegativeIntegers,
    "The family of [negative integers][super], also known as [`Nz`][super::Nz].",
    common:
        NegativeInteger+8, NegativeInteger+16, NegativeInteger+32, NegativeInteger+64,
        NegativeInteger+128
    ;

    depending:
        Big, NegativeIntegerBig, "dashu-int-TODO" // placeholder, disabled
];

/* impl additional traits for the family */

/* sign */

impl Negative for NegativeIntegers {}

/// This implementation is a no-op.
impl NonPositive for NegativeIntegers {
    type InnerRepr = Self;

    /// Returns `value` unchanged.
    #[inline]
    fn new_neg(value: Self::InnerRepr) -> NumeraResult<Self> {
        Ok(value)
    }
}

/* ident */

impl NonZero for NegativeIntegers {}
impl NegOne for NegativeIntegers {
    /// Returns a [`NegativeInteger8::new_neg_one()`][NegativeInteger8#method.new_neg_one].
    #[inline]
    fn new_neg_one() -> Self {
        NegativeInteger8::new_neg_one().into()
    }
}
impl ConstNegOne for NegativeIntegers {
    /// Returns a [`NegativeInteger8::NEG_ONE`][NegativeInteger8#associatedconstant.NEG_ONE].
    const NEG_ONE: Self = NegativeIntegers::_8(NegativeInteger8::NEG_ONE);
}

/* bound */

impl LowerBounded for NegativeIntegers {
    /// Returns a [`NegativeInteger128::new_min()`][NegativeInteger128#method.new_min].
    #[inline]
    fn new_min() -> Self {
        NegativeInteger128::new_min().into()
    }
}
impl UpperBounded for NegativeIntegers {
    /// Returns a [`NegativeInteger8::new_max()`][NegativeInteger8#method.new_max].
    #[inline]
    fn new_max() -> Self {
        NegativeInteger8::new_max().into()
    }
}
impl ConstLowerBounded for NegativeIntegers {
    /// Returns a [`NegativeInteger128::MIN`][NegativeInteger128#associatedconstant.MIN].
    const MIN: Self = NegativeIntegers::_128(NegativeInteger128::MIN);
}
impl ConstUpperBounded for NegativeIntegers {
    /// Returns a [`NegativeInteger8::MAX`][NegativeInteger8#associatedconstant.MAX].
    const MAX: Self = NegativeIntegers::_8(NegativeInteger8::MAX);
}
