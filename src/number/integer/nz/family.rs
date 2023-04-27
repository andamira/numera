// numera::integer::nz::family
//
//!
//

use super::{super::family::define_integers_family, *};
use crate::{
    error::NumeraResult,
    number::traits::{
        ConstLowerBounded, ConstNegOne, ConstUpperBounded, LowerBounded, NegOne, NegSigned, NonOne,
        NonZero, UpperBounded,
    },
};

define_integers_family![build_variants:
    NegativeIntegers,
    "The family of negative integers.",
    no_std:
        NegativeInteger8, NegativeInteger16, NegativeInteger32, NegativeInteger64, NegativeInteger128
    ;

    depending:
        NegativeIntegerBig, "ibig-TODO" // placeholder, disabled
];

/* impl additional traits for the family */

/* sign */

/// This implementation is a no-op.
impl NegSigned for NegativeIntegers {
    type Inner = Self;
    fn new_neg(value: Self::Inner) -> NumeraResult<Self> {
        Ok(value)
    }
}

/* ident */

impl NonOne for NegativeIntegers {}
impl NonZero for NegativeIntegers {}
impl NegOne for NegativeIntegers {
    /// Returns a [`NegativeInteger8::new_neg_one()`][NegativeInteger8#method.new_neg_one].
    fn new_neg_one() -> Self {
        NegativeInteger8::new_neg_one().into()
    }
}
impl ConstNegOne for NegativeIntegers {
    /// Returns a [`NegativeInteger8::NEG_ONE`][NegativeInteger8#associatedconstant.NEG_ONE].
    const NEG_ONE: Self = NegativeIntegers::NegativeInteger8(NegativeInteger8::NEG_ONE);
}

/* bound */

impl LowerBounded for NegativeIntegers {
    /// Returns a [`NegativeInteger128::new_min()`][NegativeInteger128#method.new_min].
    fn new_min() -> Self {
        NegativeInteger128::new_min().into()
    }
}
impl UpperBounded for NegativeIntegers {
    /// Returns a [`NegativeInteger8::new_max()`][NegativeInteger8#method.new_max].
    fn new_max() -> Self {
        NegativeInteger8::new_max().into()
    }
}
impl ConstLowerBounded for NegativeIntegers {
    /// Returns a [`NegativeInteger128::MIN`][NegativeInteger128#associatedconstant.MIN].
    const MIN: Self = NegativeIntegers::NegativeInteger128(NegativeInteger128::MIN);
}
impl ConstUpperBounded for NegativeIntegers {
    /// Returns a [`NegativeInteger8::MAX`][NegativeInteger8#associatedconstant.MAX].
    const MAX: Self = NegativeIntegers::NegativeInteger8(NegativeInteger8::MAX);
}
