// numera::integer::n0z::family
//
//!
//

use super::{
    super::family::define_integer_family, NonZeroInteger128, NonZeroInteger16, NonZeroInteger32,
    NonZeroInteger64, NonZeroInteger8,
};
use crate::number::traits::{
    ConstLowerBounded, ConstNegOne, ConstOne, ConstUpperBounded, LowerBounded, NegOne, Negative,
    NonZero, One, Positive, UpperBounded,
};

define_integer_family![build_variants:
    NonZeroIntegers, // the enum name
    "The family of [non-zero integers][super], also known as [`N0z`][super::N0z].",
    common:
        NonZeroInteger+8, NonZeroInteger+16, NonZeroInteger+32, NonZeroInteger+64,
        NonZeroInteger+128
    ;

    depending:
        Big, NonZeroIntegerBig, "dashu-int-TODO" // placeholder, disabled
];

/* impl additional traits for the family */

/* sign */

impl Positive for NonZeroIntegers {}
impl Negative for NonZeroIntegers {}

/* ident */

impl NonZero for NonZeroIntegers {}
impl NegOne for NonZeroIntegers {
    /// Returns a [`NonZeroInteger8::new_neg_one()`][NonZeroInteger8#method.new_neg_one].
    #[inline]
    fn new_neg_one() -> Self {
        NonZeroInteger8::new_neg_one().into()
    }
}
impl One for NonZeroIntegers {
    /// Returns a [`NonZeroInteger8::new_one()`][NonZeroInteger8#method.new_one].
    #[inline]
    fn new_one() -> Self {
        NonZeroInteger8::new_one().into()
    }
}
impl ConstNegOne for NonZeroIntegers {
    /// Returns a [`NonZeroInteger8::NEG_ONE`][NonZeroInteger8#associatedconstant.NEG_ONE].
    const NEG_ONE: Self = NonZeroIntegers::_8(NonZeroInteger8::NEG_ONE);
}
impl ConstOne for NonZeroIntegers {
    /// Returns a [`NonZeroInteger8::ONE`][NonZeroInteger8#associatedconstant.ONE].
    const ONE: Self = NonZeroIntegers::_8(NonZeroInteger8::ONE);
}

/* bound */

impl LowerBounded for NonZeroIntegers {
    /// Returns a [`NonZeroInteger8::new_min()`][NonZeroInteger8#method.new_min].
    #[inline]
    fn new_min() -> Self {
        NonZeroInteger8::new_min().into()
    }
}
impl UpperBounded for NonZeroIntegers {
    /// Returns a [`NonZeroInteger128::new_max()`][NonZeroInteger128#method.new_max].
    #[inline]
    fn new_max() -> Self {
        NonZeroInteger128::new_max().into()
    }
}
impl ConstLowerBounded for NonZeroIntegers {
    /// Returns a [`NonZeroInteger128::MIN`][NonZeroInteger128#associatedconstant.MIN].
    const MIN: Self = NonZeroIntegers::_128(NonZeroInteger128::MIN);
}
impl ConstUpperBounded for NonZeroIntegers {
    /// Returns a [`NonZeroInteger128::MAX`][NonZeroInteger128#associatedconstant.MAX].
    const MAX: Self = NonZeroIntegers::_128(NonZeroInteger128::MAX);
}
