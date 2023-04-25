// numera::number::integer::z::define_big
//
//!
//
// TOC
//
// - definition
// - impls
//   - Sign
//   - Bound
//   - Count
//   - Ident
//   - Number

use crate::{
    error::NumeraResult,
    number::traits::{
        Bound, Count, Countable, Ident, NegOne, NonLowerBounded, NonUpperBounded, Number, One,
        Sign, Signed, Zero,
    },
};
use ibig::IBig;

/* definition */

/// A big integer number, from the set $\\Z$.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct IntegerBig(pub(crate) IBig);

/* sign */

impl Sign for IntegerBig {
    fn can_negative(&self) -> bool {
        true
    }
    fn can_positive(&self) -> bool {
        true
    }
    fn is_negative(&self) -> bool {
        self.0.is_negative()
    }
    fn is_positive(&self) -> bool {
        self.0.is_positive()
    }
}
impl Signed for IntegerBig {}

/* bound */

impl Bound for IntegerBig {
    fn is_lower_bounded(&self) -> bool {
        false
    }
    fn is_upper_bounded(&self) -> bool {
        false
    }
    fn lower_bound(&self) -> Option<Self> {
        None
    }
    fn upper_bound(&self) -> Option<Self> {
        None
    }
}
impl NonLowerBounded for IntegerBig {}
impl NonUpperBounded for IntegerBig {}

/* count */

impl Count for IntegerBig {
    fn is_countable(&self) -> bool {
        true
    }
}

impl Countable for IntegerBig {
    fn next(&self) -> NumeraResult<Self> {
        self.0.next().map(Self)
    }
    fn previous(&self) -> NumeraResult<Self> {
        self.0.previous().map(Self)
    }
}

/* ident */

impl Ident for IntegerBig {
    fn can_zero(&self) -> bool {
        true
    }
    fn can_one(&self) -> bool {
        true
    }
    fn can_neg_one(&self) -> bool {
        true
    }

    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
    fn is_one(&self) -> bool {
        self.0.is_one()
    }
    fn is_neg_one(&self) -> bool {
        self.0.is_neg_one()
    }
}
impl Zero for IntegerBig {
    fn new_zero() -> Self {
        Self(IBig::new_zero())
    }
}
impl One for IntegerBig {
    fn new_one() -> Self {
        Self(IBig::new_one())
    }
}
impl NegOne for IntegerBig {
    fn new_neg_one() -> Self {
        Self(IBig::new_neg_one())
    }
}

/* number */

impl Number for IntegerBig {
    type Inner = IBig;

    fn new(value: Self::Inner) -> NumeraResult<Self> {
        Ok(Self(value))
    }
    unsafe fn new_unchecked(value: Self::Inner) -> Self {
        Self(value)
    }
}
