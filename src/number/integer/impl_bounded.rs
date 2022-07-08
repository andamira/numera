// numera::number::integer::impl_number
//
//! implements the Number trait for all integer types
//

use crate::number::integer::{Integer, Negative, NonNegative, NonPositive, Positive};
use crate::number::traits::{
    Bounded, ConstBounded, ConstLowerBounded, ConstNegOne, ConstOne, ConstUpperBounded, ConstZero,
    LowerBounded, NegOne, One, UpperBounded, Zero,
};
use crate::number::traits::{InnerNumber, Number};

// Integer (MIN..MAX)

impl<I: InnerNumber + ConstLowerBounded> ConstLowerBounded for Integer<I> {
    const MIN: Self = Self(I::MIN);
}
impl<I: InnerNumber + ConstUpperBounded> ConstUpperBounded for Integer<I> {
    const MAX: Self = Self(I::MAX);
}

impl<I: InnerNumber + LowerBounded> LowerBounded for Integer<I> {
    fn new_min() -> Self {
        Self(I::new_min())
    }
}
impl<I: InnerNumber + UpperBounded> UpperBounded for Integer<I> {
    fn new_max() -> Self {
        Self(I::new_max())
    }
}

// NonNegative (ZERO..=MAX)

impl<I: InnerNumber + ConstLowerBounded + ConstZero> ConstLowerBounded for NonNegative<I> {
    const MIN: Self = Self(I::ZERO);
}
impl<I: InnerNumber + ConstUpperBounded> ConstUpperBounded for NonNegative<I> {
    const MAX: Self = Self(I::MAX);
}

impl<I: InnerNumber + LowerBounded + Zero> LowerBounded for NonNegative<I> {
    fn new_min() -> Self {
        Self(I::new_zero())
    }
}
impl<I: InnerNumber + UpperBounded> UpperBounded for NonNegative<I> {
    fn new_max() -> Self {
        Self(I::new_max())
    }
}

// Positive (ONE..=MAX)

impl<I: InnerNumber + ConstLowerBounded + ConstOne> ConstLowerBounded for Positive<I> {
    const MIN: Self = Self(I::ONE);
}
impl<I: InnerNumber + ConstUpperBounded> ConstUpperBounded for Positive<I> {
    const MAX: Self = Self(I::MAX);
}

impl<I: InnerNumber + LowerBounded> LowerBounded for Positive<I> {
    fn new_min() -> Self {
        Self(I::new_one())
    }
}
impl<I: InnerNumber + UpperBounded> UpperBounded for Positive<I> {
    fn new_max() -> Self {
        Self(I::new_max())
    }
}

// NonPositive (MIN..=ZERO)

impl<I: InnerNumber + ConstLowerBounded> ConstLowerBounded for NonPositive<I> {
    const MIN: Self = Self(I::MIN);
}
impl<I: InnerNumber + ConstUpperBounded + ConstZero> ConstUpperBounded for NonPositive<I> {
    const MAX: Self = Self(I::ZERO);
}

impl<I: InnerNumber + LowerBounded> LowerBounded for NonPositive<I> {
    fn new_min() -> Self {
        Self(I::new_min())
    }
}
impl<I: InnerNumber + UpperBounded + Zero> UpperBounded for NonPositive<I> {
    fn new_max() -> Self {
        Self(I::new_zero())
    }
}

// Negative (MIN..=NEG_ONE)

impl<I: InnerNumber + ConstLowerBounded> ConstLowerBounded for Negative<I> {
    const MIN: Self = Self(I::MIN);
}
impl<I: InnerNumber + ConstUpperBounded + ConstNegOne> ConstUpperBounded for Negative<I> {
    const MAX: Self = Self(I::NEG_ONE);
}

impl<I: InnerNumber + LowerBounded> LowerBounded for Negative<I> {
    fn new_min() -> Self {
        Self(I::new_min())
    }
}
impl<I: InnerNumber + UpperBounded + NegOne> UpperBounded for Negative<I> {
    fn new_max() -> Self {
        Self(I::new_neg_one())
    }
}
