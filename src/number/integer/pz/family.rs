// numera::integer::pz::family
//
//!
//

use super::{super::family::define_integer_family, *};
use crate::number::traits::{
    ConstLowerBounded, ConstOne, ConstUpperBounded, LowerBounded, NonNegative, NonZero, One,
    Positive, UpperBounded,
};

define_integer_family![build_variants:
    PositiveInteger,
    "The family of [positive integers][super], also known as [`Pz`][super::Pz].",
    common:
        PositiveInteger+8, PositiveInteger+16, PositiveInteger+32, PositiveInteger+64,
        PositiveInteger+128
    ;

    depending:
        Big, PositiveIntegerBig, "ibig-TODO" // placeholder, disabled
];

/* impl additional traits for the family */

/* sign */

impl NonNegative for PositiveInteger {}
impl Positive for PositiveInteger {}

/* ident */

impl NonZero for PositiveInteger {}
impl One for PositiveInteger {
    /// Returns a [`PositiveInteger8::new_one()`][PositiveInteger8#method.new_one].
    #[inline]
    fn new_one() -> Self {
        PositiveInteger8::new_one().into()
    }
}
impl ConstOne for PositiveInteger {
    /// Returns a [`PositiveInteger8::ONE`][PositiveInteger8#associatedconstant.ONE].
    const ONE: Self = PositiveInteger::_8(PositiveInteger8::ONE);
}

/* bound */

impl LowerBounded for PositiveInteger {
    /// Returns a [`PositiveInteger8::new_min()`][PositiveInteger8#method.new_min].
    #[inline]
    fn new_min() -> Self {
        PositiveInteger8::new_min().into()
    }
}
impl UpperBounded for PositiveInteger {
    /// Returns a [`PositiveInteger128::new_max()`][PositiveInteger128#method.new_max].
    #[inline]
    fn new_max() -> Self {
        PositiveInteger128::new_max().into()
    }
}
impl ConstLowerBounded for PositiveInteger {
    /// Returns a [`PositiveInteger8::MIN`][PositiveInteger8#associatedconstant.MIN].
    const MIN: Self = PositiveInteger::_8(PositiveInteger8::MIN);
}
impl ConstUpperBounded for PositiveInteger {
    /// Returns a [`PositiveInteger128::MAX`][PositiveInteger128#associatedconstant.MAX].
    const MAX: Self = PositiveInteger::_128(PositiveInteger128::MAX);
}
