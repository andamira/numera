// numera::integer::z::family
//
//!
//

use super::{
    super::family::define_integer_family, Integer128, Integer16, Integer32, Integer64, Integer8,
};
use crate::number::traits::{
    ConstLowerBounded, ConstNegOne, ConstOne, ConstUpperBounded, ConstZero, LowerBounded, NegOne,
    Negative, One, Positive, UpperBounded, Zero,
};

#[cfg(feature = "dashu-int")]
use super::IntegerBig;

define_integer_family![build_variants:
    Integers, // the enum name
    "The family of [integer][super] numbers, also known as [`Z`][super::Z].",
    common:
        Integer+8, Integer+16, Integer+32, Integer+64, Integer+128
    ;

    // feature-gated variants
    depending:
        Big, IntegerBig, "dashu-int"
];

/* impl additional traits for the family */

/* sign */

impl Positive for Integers {}
impl Negative for Integers {}

/* ident */

impl Zero for Integers {
    /// Returns a [`Integer8::new_zero()`][Integer8#method.new_zero].
    #[inline]
    fn new_zero() -> Self {
        Integer8::new_zero().into()
    }
}
impl One for Integers {
    /// Returns a [`Integer8::new_one()`][Integer8#method.new_one].
    #[inline]
    fn new_one() -> Self {
        Integer8::new_one().into()
    }
}
impl NegOne for Integers {
    /// Returns a [`Integer8::new_neg_one()`][Integer8#method.new_neg_one].
    #[inline]
    fn new_neg_one() -> Self {
        Integer8::new_neg_one().into()
    }
}
impl ConstZero for Integers {
    /// Returns a [`Integer8::ZERO`][Integer8#associatedconstant.ZERO].
    const ZERO: Self = Integers::_8(Integer8::ZERO);
}
impl ConstOne for Integers {
    /// Returns a [`Integer8::ONE`][Integer8#associatedconstant.ONE].
    const ONE: Self = Integers::_8(Integer8::ONE);
}
impl ConstNegOne for Integers {
    /// Returns a [`Integer8::NEG_ONE`][Integer8#associatedconstant.NEG_ONE].
    const NEG_ONE: Self = Integers::_8(Integer8::NEG_ONE);
}

/* bound */

impl LowerBounded for Integers {
    /// Returns a [`Integer8::new_min()`][Integer8#method.new_min].
    #[inline]
    fn new_min() -> Self {
        Integer8::new_min().into()
    }
}
impl UpperBounded for Integers {
    /// Returns a [`Integer128::new_max()`][Integer128#method.new_max].
    #[inline]
    fn new_max() -> Self {
        Integer128::new_max().into()
    }
}
impl ConstLowerBounded for Integers {
    /// Returns a [`Integer128::MIN`][Integer128#associatedconstant.MIN].
    const MIN: Self = Integers::_128(Integer128::MIN);
}
impl ConstUpperBounded for Integers {
    /// Returns a [`Integer128::MAX`][Integer128#associatedconstant.MAX].
    const MAX: Self = Integers::_128(Integer128::MAX);
}
