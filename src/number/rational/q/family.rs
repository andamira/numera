// numera::rational::q::family
//
//!
//

use super::{
    super::family::define_rational_family, Rational128, Rational16, Rational32, Rational64,
    Rational8,
};
use crate::number::traits::{
    ConstLowerBounded, ConstNegOne, ConstOne, ConstUpperBounded, ConstZero, LowerBounded, NegOne,
    Negative, One, Positive, UpperBounded, Zero,
};

define_rational_family![build_variants:
    Rationals,
    "The family of [rational][super] numbers, also known as [`Q`][super::Q].",
    common:
        Rational+8, Rational+16, Rational+32, Rational+64, Rational+128
    ;

    // feature-gated variants
    depending:
        Big, RationalBig, "---"
];

/* impl additional traits for the family */

/* sign */

impl Positive for Rationals {}
impl Negative for Rationals {}

/* ident */

impl Zero for Rationals {
    /// Returns a [`Rational8::new_zero()`][Rational8#method.new_zero].
    #[inline]
    fn new_zero() -> Self {
        Rational8::new_zero().into()
    }
}
impl One for Rationals {
    /// Returns a [`Rational8::new_one()`][Rational8#method.new_one].
    #[inline]
    fn new_one() -> Self {
        Rational8::new_one().into()
    }
}
impl NegOne for Rationals {
    /// Returns a [`Rational8::new_neg_one()`][Rational8#method.new_neg_one].
    #[inline]
    fn new_neg_one() -> Self {
        Rational8::new_neg_one().into()
    }
}
impl ConstZero for Rationals {
    /// Returns a [`Rational8::ZERO`][Rational8#associatedconstant.ZERO].
    const ZERO: Self = Rationals::_8(Rational8::ZERO);
}
impl ConstOne for Rationals {
    /// Returns a [`Rational8::ONE`][Rational8#associatedconstant.ONE].
    const ONE: Self = Rationals::_8(Rational8::ONE);
}
impl ConstNegOne for Rationals {
    /// Returns a [`Rational8::NEG_ONE`][Rational8#associatedconstant.NEG_ONE].
    const NEG_ONE: Self = Rationals::_8(Rational8::NEG_ONE);
}

/* bound */

impl LowerBounded for Rationals {
    /// Returns a [`Rational8::new_min()`][Rational8#method.new_min].
    #[inline]
    fn new_min() -> Self {
        Rational8::new_min().into()
    }
}
impl UpperBounded for Rationals {
    /// Returns a [`Rational128::new_max()`][Rational128#method.new_max].
    #[inline]
    fn new_max() -> Self {
        Rational128::new_max().into()
    }
}
impl ConstLowerBounded for Rationals {
    /// Returns a [`Rational128::MIN`][Rational128#associatedconstant.MIN].
    const MIN: Self = Rationals::_128(Rational128::MIN);
}
impl ConstUpperBounded for Rationals {
    /// Returns a [`Rational128::MAX`][Rational128#associatedconstant.MAX].
    const MAX: Self = Rationals::_128(Rational128::MAX);
}
