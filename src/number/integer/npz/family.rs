// numera::integer::npz::family
//
//!
//

use super::{super::family::define_integers_family, *};
use crate::number::traits::{
    ConstLowerBounded, ConstNegOne, ConstUpperBounded, ConstZero, LowerBounded, NegOne, NegSigned,
    NonOne, UpperBounded, Zero,
};

define_integers_family![build_variants:
    NonPositiveIntegers,
    "The family of non-negative integers.",
    no_std:
        NonPositiveInteger8, NonPositiveInteger16, NonPositiveInteger32, NonPositiveInteger64, NonPositiveInteger128
    ;

    depending:
        NonPositiveIntegerBig, "ibig-TODO" // placeholder, disabled
];

/* impl additional traits for the family */

/* sign */

/// This implementation is a no-op.
impl NegSigned for NonPositiveIntegers {
    type Inner = Self;
    fn new_neg(value: Self::Inner) -> Self {
        value
    }
}

/* ident */

impl NonOne for NonPositiveIntegers {}
impl Zero for NonPositiveIntegers {
    /// Returns a [`NonPositiveInteger8::new_zero()`][NonPositiveInteger8#method.new_zero].
    fn new_zero() -> Self {
        NonPositiveInteger8::new_zero().into()
    }
}
impl NegOne for NonPositiveIntegers {
    /// Returns a [`NonPositiveInteger8::new_neg_one()`][NonPositiveInteger8#method.new_neg_one].
    fn new_neg_one() -> Self {
        NonPositiveInteger8::new_neg_one().into()
    }
}
impl ConstZero for NonPositiveIntegers {
    /// Returns a [`NonPositiveInteger8::ZERO`][NonPositiveInteger8#associatedconstant.ZERO].
    const ZERO: Self = NonPositiveIntegers::NonPositiveInteger8(NonPositiveInteger8::ZERO);
}
impl ConstNegOne for NonPositiveIntegers {
    /// Returns a [`NonPositiveInteger8::NEG_ONE`][NonPositiveInteger8#associatedconstant.NEG_ONE].
    const NEG_ONE: Self = NonPositiveIntegers::NonPositiveInteger8(NonPositiveInteger8::NEG_ONE);
}

/* bound */

impl LowerBounded for NonPositiveIntegers {
    /// Returns a [`NonPositiveInteger8::new_min()`][NonPositiveInteger8#method.new_min].
    fn new_min() -> Self {
        NonPositiveInteger8::new_min().into()
    }
}
impl UpperBounded for NonPositiveIntegers {
    /// Returns a [`NonPositiveInteger128::new_max()`][NonPositiveInteger128#method.new_max].
    fn new_max() -> Self {
        NonPositiveInteger128::new_max().into()
    }
}
impl ConstLowerBounded for NonPositiveIntegers {
    /// Returns a [`NonPositiveInteger8::MIN`][NonPositiveInteger8#associatedconstant.MIN].
    const MIN: Self = NonPositiveIntegers::NonPositiveInteger8(NonPositiveInteger8::MIN);
}
impl ConstUpperBounded for NonPositiveIntegers {
    /// Returns a [`NonPositiveInteger128::MAX`][NonPositiveInteger128#associatedconstant.MAX].
    const MAX: Self = NonPositiveIntegers::NonPositiveInteger128(NonPositiveInteger128::MAX);
}
