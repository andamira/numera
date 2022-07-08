// numera::number::integer::impl_number
//
//! implements the `[Const][Lower|Upper]Bounded` trait for all integer types.
//

use crate::number::integer::{
    Integer, NegativeInteger, NonNegativeInteger, NonPositiveInteger, NonZeroInteger,
    PositiveInteger,
};
use crate::number::traits::InnerNumber;
use crate::number::traits::{
    ConstLowerBounded, ConstNegOne, ConstOne, ConstUpperBounded, ConstZero, LowerBounded, NegOne,
    One, UpperBounded, Zero,
};

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

// NonZeroInteger (MIN..MAX)

impl<I: InnerNumber + ConstLowerBounded> ConstLowerBounded for NonZeroInteger<I> {
    const MIN: Self = Self(I::MIN);
}
impl<I: InnerNumber + ConstUpperBounded> ConstUpperBounded for NonZeroInteger<I> {
    const MAX: Self = Self(I::MAX);
}

impl<I: InnerNumber + LowerBounded> LowerBounded for NonZeroInteger<I> {
    fn new_min() -> Self {
        Self(I::new_min())
    }
}
impl<I: InnerNumber + UpperBounded> UpperBounded for NonZeroInteger<I> {
    fn new_max() -> Self {
        Self(I::new_max())
    }
}

// NonNegativeInteger (ZERO..=MAX)

impl<I: InnerNumber + ConstLowerBounded + ConstZero> ConstLowerBounded for NonNegativeInteger<I> {
    const MIN: Self = Self(I::ZERO);
}
impl<I: InnerNumber + ConstUpperBounded> ConstUpperBounded for NonNegativeInteger<I> {
    const MAX: Self = Self(I::MAX);
}

impl<I: InnerNumber + LowerBounded + Zero> LowerBounded for NonNegativeInteger<I> {
    fn new_min() -> Self {
        Self(I::new_zero())
    }
}
impl<I: InnerNumber + UpperBounded> UpperBounded for NonNegativeInteger<I> {
    fn new_max() -> Self {
        Self(I::new_max())
    }
}

// PositiveInteger (ONE..=MAX)

impl<I: InnerNumber + ConstLowerBounded + ConstOne> ConstLowerBounded for PositiveInteger<I> {
    const MIN: Self = Self(I::ONE);
}
impl<I: InnerNumber + ConstUpperBounded> ConstUpperBounded for PositiveInteger<I> {
    const MAX: Self = Self(I::MAX);
}

impl<I: InnerNumber + LowerBounded + One> LowerBounded for PositiveInteger<I> {
    fn new_min() -> Self {
        Self(I::new_one())
    }
}
impl<I: InnerNumber + UpperBounded> UpperBounded for PositiveInteger<I> {
    fn new_max() -> Self {
        Self(I::new_max())
    }
}

// NonPositiveInteger (MIN..=ZERO)

impl<I: InnerNumber + ConstLowerBounded> ConstLowerBounded for NonPositiveInteger<I> {
    const MIN: Self = Self(I::MIN);
}
impl<I: InnerNumber + ConstUpperBounded + ConstZero> ConstUpperBounded for NonPositiveInteger<I> {
    const MAX: Self = Self(I::ZERO);
}

impl<I: InnerNumber + LowerBounded> LowerBounded for NonPositiveInteger<I> {
    fn new_min() -> Self {
        Self(I::new_min())
    }
}
impl<I: InnerNumber + UpperBounded + Zero> UpperBounded for NonPositiveInteger<I> {
    fn new_max() -> Self {
        Self(I::new_zero())
    }
}

// NegativeInteger (MIN..=NEG_ONE)

impl<I: InnerNumber + ConstLowerBounded> ConstLowerBounded for NegativeInteger<I> {
    const MIN: Self = Self(I::MIN);
}
impl<I: InnerNumber + ConstUpperBounded + ConstNegOne> ConstUpperBounded for NegativeInteger<I> {
    const MAX: Self = Self(I::NEG_ONE);
}

impl<I: InnerNumber + LowerBounded> LowerBounded for NegativeInteger<I> {
    fn new_min() -> Self {
        Self(I::new_min())
    }
}
impl<I: InnerNumber + UpperBounded + NegOne> UpperBounded for NegativeInteger<I> {
    fn new_max() -> Self {
        Self(I::new_neg_one())
    }
}