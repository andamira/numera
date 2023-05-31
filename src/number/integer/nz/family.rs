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
    NegativeInteger,
    "The family of negative integers.",
    no_std:
        NegativeInteger8, NegativeInteger16, NegativeInteger32, NegativeInteger64, NegativeInteger128
    ;

    depending:
        NegativeIntegerBig, "ibig-TODO" // placeholder, disabled
];

/* impl additional traits for the family */

/* sign */

impl Negative for NegativeInteger {}

/// This implementation is a no-op.
impl NonPositive for NegativeInteger {
    type Parts = Self;

    /// Returns `value` unchanged.
    #[inline]
    fn new_neg(value: Self::Parts) -> NumeraResult<Self> {
        Ok(value)
    }
}

/* ident */

impl NonZero for NegativeInteger {}
impl NegOne for NegativeInteger {
    /// Returns a [`NegativeInteger8::new_neg_one()`][NegativeInteger8#method.new_neg_one].
    #[inline]
    fn new_neg_one() -> Self {
        NegativeInteger8::new_neg_one().into()
    }
}
impl ConstNegOne for NegativeInteger {
    /// Returns a [`NegativeInteger8::NEG_ONE`][NegativeInteger8#associatedconstant.NEG_ONE].
    const NEG_ONE: Self = NegativeInteger::NegativeInteger8(NegativeInteger8::NEG_ONE);
}

/* bound */

impl LowerBounded for NegativeInteger {
    /// Returns a [`NegativeInteger128::new_min()`][NegativeInteger128#method.new_min].
    #[inline]
    fn new_min() -> Self {
        NegativeInteger128::new_min().into()
    }
}
impl UpperBounded for NegativeInteger {
    /// Returns a [`NegativeInteger8::new_max()`][NegativeInteger8#method.new_max].
    #[inline]
    fn new_max() -> Self {
        NegativeInteger8::new_max().into()
    }
}
impl ConstLowerBounded for NegativeInteger {
    /// Returns a [`NegativeInteger128::MIN`][NegativeInteger128#associatedconstant.MIN].
    const MIN: Self = NegativeInteger::NegativeInteger128(NegativeInteger128::MIN);
}
impl ConstUpperBounded for NegativeInteger {
    /// Returns a [`NegativeInteger8::MAX`][NegativeInteger8#associatedconstant.MAX].
    const MAX: Self = NegativeInteger::NegativeInteger8(NegativeInteger8::MAX);
}
