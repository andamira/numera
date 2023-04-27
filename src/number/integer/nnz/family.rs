// numera::integer::nnz::family
//
//!
//

use super::{super::family::define_integers_family, *};
use crate::number::traits::{
    ConstLowerBounded, ConstOne, ConstUpperBounded, ConstZero, LowerBounded, NonNegOne, One,
    Unsigned, UpperBounded, Zero,
};

define_integers_family![build_variants:
    NonNegativeIntegers,
    "The family of non-negative integers.",
    no_std:
        NonNegativeInteger8, NonNegativeInteger16, NonNegativeInteger32, NonNegativeInteger64, NonNegativeInteger128
    ;

    depending:
        NonNegativeIntegerBig, "ibig-TODO" // placeholder, disabled
];

/* impl additional traits for the family */

/* sign */
impl Unsigned for NonNegativeIntegers {}

/* ident */
impl NonNegOne for NonNegativeIntegers {}
impl Zero for NonNegativeIntegers {
    /// Returns a [`NonNegativeInteger8::new_zero()`][NonNegativeInteger8#method.new_zero].
    fn new_zero() -> Self {
        NonNegativeInteger8::new_zero().into()
    }
}
impl One for NonNegativeIntegers {
    /// Returns a [`NonNegativeInteger8::new_one()`][NonNegativeInteger8#method.new_one].
    fn new_one() -> Self {
        NonNegativeInteger8::new_one().into()
    }
}
impl ConstZero for NonNegativeIntegers {
    /// Returns a [`NonNegativeInteger8::ZERO`][NonNegativeInteger8#associatedconstant.ZERO].
    const ZERO: Self = NonNegativeIntegers::NonNegativeInteger8(NonNegativeInteger8::ZERO);
}
impl ConstOne for NonNegativeIntegers {
    /// Returns a [`NonNegativeInteger8::ONE`][NonNegativeInteger8#associatedconstant.ONE].
    const ONE: Self = NonNegativeIntegers::NonNegativeInteger8(NonNegativeInteger8::ONE);
}

/* bound */
impl LowerBounded for NonNegativeIntegers {
    /// Returns a [`NonNegativeInteger8::new_min()`][NonNegativeInteger8#method.new_min].
    fn new_min() -> Self {
        NonNegativeInteger8::new_min().into()
    }
}
impl UpperBounded for NonNegativeIntegers {
    /// Returns a [`NonNegativeInteger128::new_max()`][NonNegativeInteger128#method.new_max].
    fn new_max() -> Self {
        NonNegativeInteger128::new_max().into()
    }
}
impl ConstLowerBounded for NonNegativeIntegers {
    /// Returns a [`NonNegativeInteger8::MIN`][NonNegativeInteger8#associatedconstant.MIN].
    const MIN: Self = NonNegativeIntegers::NonNegativeInteger8(NonNegativeInteger8::MIN);
}
impl ConstUpperBounded for NonNegativeIntegers {
    /// Returns a [`NonNegativeInteger128::MAX`][NonNegativeInteger128#associatedconstant.MAX].
    const MAX: Self = NonNegativeIntegers::NonNegativeInteger128(NonNegativeInteger128::MAX);
}
