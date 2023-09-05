// numera::integer::npz::family
//
//!
//

use super::{super::family::define_integer_family, *};
use crate::{
    error::NumeraResult,
    number::traits::{
        ConstLowerBounded, ConstNegOne, ConstUpperBounded, ConstZero, LowerBounded, NegOne,
        Negative, NonPositive, UpperBounded, Zero,
    },
};

define_integer_family![build_variants:
    NonPositiveIntegers,
    "The family of [non-positive integers][super], also known as [`Npz`][super::Npz].",
    common:
        NonPositiveInteger+8, NonPositiveInteger+16, NonPositiveInteger+32, NonPositiveInteger+64,
        NonPositiveInteger+128
    ;

    depending:
        Big, NonPositiveIntegerBig, "dashu-int-TODO" // placeholder, disabled
];

/* impl additional traits for the family */

/* sign */

impl Negative for NonPositiveIntegers {}

/// This implementation is a no-op.
impl NonPositive for NonPositiveIntegers {
    type InnerRepr = Self;

    /// Returns `value` unchanged.
    #[inline]
    fn new_neg(value: Self::InnerRepr) -> NumeraResult<Self> {
        Ok(value)
    }
}

/* ident */

impl Zero for NonPositiveIntegers {
    /// Returns a [`NonPositiveInteger8::new_zero()`][NonPositiveInteger8#method.new_zero].
    #[inline]
    fn new_zero() -> Self {
        NonPositiveInteger8::new_zero().into()
    }
}
impl NegOne for NonPositiveIntegers {
    /// Returns a [`NonPositiveInteger8::new_neg_one()`][NonPositiveInteger8#method.new_neg_one].
    #[inline]
    fn new_neg_one() -> Self {
        NonPositiveInteger8::new_neg_one().into()
    }
}
impl ConstZero for NonPositiveIntegers {
    /// Returns a [`NonPositiveInteger8::ZERO`][NonPositiveInteger8#associatedconstant.ZERO].
    const ZERO: Self = NonPositiveIntegers::_8(NonPositiveInteger8::ZERO);
}
impl ConstNegOne for NonPositiveIntegers {
    /// Returns a [`NonPositiveInteger8::NEG_ONE`][NonPositiveInteger8#associatedconstant.NEG_ONE].
    const NEG_ONE: Self = NonPositiveIntegers::_8(NonPositiveInteger8::NEG_ONE);
}

/* bound */

impl LowerBounded for NonPositiveIntegers {
    /// Returns a [`NonPositiveInteger128::new_min()`][NonPositiveInteger128#method.new_min].
    #[inline]
    fn new_min() -> Self {
        NonPositiveInteger128::new_min().into()
    }
}
impl UpperBounded for NonPositiveIntegers {
    /// Returns a [`NonPositiveInteger8::new_max()`][NonPositiveInteger8#method.new_max].
    #[inline]
    fn new_max() -> Self {
        NonPositiveInteger8::new_max().into()
    }
}
impl ConstLowerBounded for NonPositiveIntegers {
    /// Returns a [`NonPositiveInteger128::MIN`][NonPositiveInteger128#associatedconstant.MIN].
    const MIN: Self = NonPositiveIntegers::_128(NonPositiveInteger128::MIN);
}
impl ConstUpperBounded for NonPositiveIntegers {
    /// Returns a [`NonPositiveInteger8::MAX`][NonPositiveInteger8#associatedconstant.MAX].
    const MAX: Self = NonPositiveIntegers::_8(NonPositiveInteger8::MAX);
}
