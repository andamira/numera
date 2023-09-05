// numera::integer::nnz::family
//
//!
//

use super::{super::family::define_integer_family, *};
use crate::number::traits::{
    ConstLowerBounded, ConstOne, ConstUpperBounded, ConstZero, LowerBounded, NonNegative, One,
    Positive, UpperBounded, Zero,
};

define_integer_family![build_variants:
    NonNegativeIntegers, // the enum name
    "The family of [non-negative integers][super], also known as [`Nnz`][super::Nnz].",
    common:
        NonNegativeInteger+8, NonNegativeInteger+16, NonNegativeInteger+32, NonNegativeInteger+64,
        NonNegativeInteger+128
    ;

    depending:
        Big, NonNegativeIntegerBig, "dashu-int-TODO" // placeholder, disabled
];

/* impl additional traits for the family */

/* sign */

impl Positive for NonNegativeIntegers {}
impl NonNegative for NonNegativeIntegers {}

/* ident */

impl Zero for NonNegativeIntegers {
    /// Returns a [`NonNegativeInteger8::new_zero()`][NonNegativeInteger8#method.new_zero].
    #[inline]
    fn new_zero() -> Self {
        NonNegativeInteger8::new_zero().into()
    }
}
impl One for NonNegativeIntegers {
    /// Returns a [`NonNegativeInteger8::new_one()`][NonNegativeInteger8#method.new_one].
    #[inline]
    fn new_one() -> Self {
        NonNegativeInteger8::new_one().into()
    }
}
impl ConstZero for NonNegativeIntegers {
    /// Returns a [`NonNegativeInteger8::ZERO`][NonNegativeInteger8#associatedconstant.ZERO].
    const ZERO: Self = NonNegativeIntegers::_8(NonNegativeInteger8::ZERO);
}
impl ConstOne for NonNegativeIntegers {
    /// Returns a [`NonNegativeInteger8::ONE`][NonNegativeInteger8#associatedconstant.ONE].
    const ONE: Self = NonNegativeIntegers::_8(NonNegativeInteger8::ONE);
}

/* bound */

impl LowerBounded for NonNegativeIntegers {
    /// Returns a [`NonNegativeInteger8::new_min()`][NonNegativeInteger8#method.new_min].
    #[inline]
    fn new_min() -> Self {
        NonNegativeInteger8::new_min().into()
    }
}
impl UpperBounded for NonNegativeIntegers {
    /// Returns a [`NonNegativeInteger128::new_max()`][NonNegativeInteger128#method.new_max].
    #[inline]
    fn new_max() -> Self {
        NonNegativeInteger128::new_max().into()
    }
}
impl ConstLowerBounded for NonNegativeIntegers {
    /// Returns a [`NonNegativeInteger8::MIN`][NonNegativeInteger8#associatedconstant.MIN].
    const MIN: Self = NonNegativeIntegers::_8(NonNegativeInteger8::MIN);
}
impl ConstUpperBounded for NonNegativeIntegers {
    /// Returns a [`NonNegativeInteger128::MAX`][NonNegativeInteger128#associatedconstant.MAX].
    const MAX: Self = NonNegativeIntegers::_128(NonNegativeInteger128::MAX);
}
