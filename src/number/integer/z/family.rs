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

#[cfg(feature = "ibig")]
use super::IntegerBig;

define_integer_family![build_variants:
    Integer,
    "The family of [integer][super] numbers, also known as [`Z`][super::Z].",
    no_std:
        Integer8, Integer16, Integer32, Integer64, Integer128
    ;

    // feature-gated variants
    depending:
        IntegerBig, "ibig"
];

/* impl additional traits for the family */

/* sign */

impl Positive for Integer {}
impl Negative for Integer {}

/* ident */

impl Zero for Integer {
    /// Returns a [`Integer8::new_zero()`][Integer8#method.new_zero].
    #[inline]
    fn new_zero() -> Self {
        Integer8::new_zero().into()
    }
}
impl One for Integer {
    /// Returns a [`Integer8::new_one()`][Integer8#method.new_one].
    #[inline]
    fn new_one() -> Self {
        Integer8::new_one().into()
    }
}
impl NegOne for Integer {
    /// Returns a [`Integer8::new_neg_one()`][Integer8#method.new_neg_one].
    #[inline]
    fn new_neg_one() -> Self {
        Integer8::new_neg_one().into()
    }
}
impl ConstZero for Integer {
    /// Returns a [`Integer8::ZERO`][Integer8#associatedconstant.ZERO].
    const ZERO: Self = Integer::Integer8(Integer8::ZERO);
}
impl ConstOne for Integer {
    /// Returns a [`Integer8::ONE`][Integer8#associatedconstant.ONE].
    const ONE: Self = Integer::Integer8(Integer8::ONE);
}
impl ConstNegOne for Integer {
    /// Returns a [`Integer8::NEG_ONE`][Integer8#associatedconstant.NEG_ONE].
    const NEG_ONE: Self = Integer::Integer8(Integer8::NEG_ONE);
}

/* bound */

impl LowerBounded for Integer {
    /// Returns a [`Integer8::new_min()`][Integer8#method.new_min].
    #[inline]
    fn new_min() -> Self {
        Integer8::new_min().into()
    }
}
impl UpperBounded for Integer {
    /// Returns a [`Integer128::new_max()`][Integer128#method.new_max].
    #[inline]
    fn new_max() -> Self {
        Integer128::new_max().into()
    }
}
impl ConstLowerBounded for Integer {
    /// Returns a [`Integer128::MIN`][Integer128#associatedconstant.MIN].
    const MIN: Self = Integer::Integer128(Integer128::MIN);
}
impl ConstUpperBounded for Integer {
    /// Returns a [`Integer128::MAX`][Integer128#associatedconstant.MAX].
    const MAX: Self = Integer::Integer128(Integer128::MAX);
}
