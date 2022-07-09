// numera::integer::impl_bounded
//
//! implements the `[Const][Lower|Upper]Bounded` trait for all integer types.
//

use crate::integer::{
    Integer, NegativeInteger, NonNegativeInteger, NonPositiveInteger, NonZeroInteger,
    PositiveInteger,
};
use crate::traits::NumberAble;
use crate::traits::{
    ConstLowerBounded, ConstNegOne, ConstOne, ConstUpperBounded, ConstZero, LowerBounded, NegOne,
    One, UpperBounded, Zero,
};

// Integer (MIN..MAX)

impl<I: NumberAble + ConstLowerBounded> ConstLowerBounded for Integer<I> {
    const MIN: Self = Self(I::MIN);
}
impl<I: NumberAble + ConstUpperBounded> ConstUpperBounded for Integer<I> {
    const MAX: Self = Self(I::MAX);
}

impl<I: NumberAble + LowerBounded> LowerBounded for Integer<I> {
    fn new_min() -> Self {
        Self(I::new_min())
    }
}
impl<I: NumberAble + UpperBounded> UpperBounded for Integer<I> {
    fn new_max() -> Self {
        Self(I::new_max())
    }
}

// NonZeroInteger (MIN..MAX)

impl<I: NumberAble + ConstLowerBounded> ConstLowerBounded for NonZeroInteger<I> {
    const MIN: Self = Self(I::MIN);
}
impl<I: NumberAble + ConstUpperBounded> ConstUpperBounded for NonZeroInteger<I> {
    const MAX: Self = Self(I::MAX);
}

impl<I: NumberAble + LowerBounded> LowerBounded for NonZeroInteger<I> {
    fn new_min() -> Self {
        Self(I::new_min())
    }
}
impl<I: NumberAble + UpperBounded> UpperBounded for NonZeroInteger<I> {
    fn new_max() -> Self {
        Self(I::new_max())
    }
}

// NonNegativeInteger (ZERO..=MAX)

impl<I: NumberAble + ConstLowerBounded + ConstZero> ConstLowerBounded for NonNegativeInteger<I> {
    const MIN: Self = Self(I::ZERO);
}
impl<I: NumberAble + ConstUpperBounded> ConstUpperBounded for NonNegativeInteger<I> {
    const MAX: Self = Self(I::MAX);
}

impl<I: NumberAble + LowerBounded + Zero> LowerBounded for NonNegativeInteger<I> {
    fn new_min() -> Self {
        Self(I::new_zero())
    }
}
impl<I: NumberAble + UpperBounded> UpperBounded for NonNegativeInteger<I> {
    fn new_max() -> Self {
        Self(I::new_max())
    }
}

// PositiveInteger (ONE..=MAX)

impl<I: NumberAble + ConstLowerBounded + ConstOne> ConstLowerBounded for PositiveInteger<I> {
    const MIN: Self = Self(I::ONE);
}
impl<I: NumberAble + ConstUpperBounded> ConstUpperBounded for PositiveInteger<I> {
    const MAX: Self = Self(I::MAX);
}

impl<I: NumberAble + LowerBounded + One> LowerBounded for PositiveInteger<I> {
    fn new_min() -> Self {
        Self(I::new_one())
    }
}
impl<I: NumberAble + UpperBounded> UpperBounded for PositiveInteger<I> {
    fn new_max() -> Self {
        Self(I::new_max())
    }
}

// NonPositiveInteger (MIN..=ZERO)

impl<I: NumberAble + ConstLowerBounded> ConstLowerBounded for NonPositiveInteger<I> {
    const MIN: Self = Self(I::MIN);
}
impl<I: NumberAble + ConstUpperBounded + ConstZero> ConstUpperBounded for NonPositiveInteger<I> {
    const MAX: Self = Self(I::ZERO);
}

impl<I: NumberAble + LowerBounded> LowerBounded for NonPositiveInteger<I> {
    fn new_min() -> Self {
        Self(I::new_min())
    }
}
impl<I: NumberAble + UpperBounded + Zero> UpperBounded for NonPositiveInteger<I> {
    fn new_max() -> Self {
        Self(I::new_zero())
    }
}

// NegativeInteger (MIN..=NEG_ONE)

impl<I: NumberAble + ConstLowerBounded> ConstLowerBounded for NegativeInteger<I> {
    const MIN: Self = Self(I::MIN);
}
impl<I: NumberAble + ConstUpperBounded + ConstNegOne> ConstUpperBounded for NegativeInteger<I> {
    const MAX: Self = Self(I::NEG_ONE);
}

impl<I: NumberAble + LowerBounded> LowerBounded for NegativeInteger<I> {
    fn new_min() -> Self {
        Self(I::new_min())
    }
}
impl<I: NumberAble + UpperBounded + NegOne> UpperBounded for NegativeInteger<I> {
    fn new_max() -> Self {
        Self(I::new_neg_one())
    }
}

/// Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::{Bounded, ConstBounded};
    use static_assertions::*;

    macro_rules! assert_impl_bounded {
        (both: $($ty:ty),+) => {
            assert_impl_bounded![@const: $($ty),+];
            assert_impl_bounded![@nonconst: $($ty),+];
        };
        (@const: $($ty:ty),+) => {
            $( assert_impl_all![$ty: ConstLowerBounded, ConstUpperBounded, ConstBounded];)+
        };
        (@nonconst: $($ty:ty),+) => {
            $( assert_impl_all![$ty: LowerBounded, UpperBounded, Bounded];)+
        };
    }

    /// Checks the `[Const][Lower|Upper]Bounded]` traits for integers.
    #[test]
    fn bounded_integers() {
        assert_impl_bounded![
            both: Integer<i8>,
            NonNegativeInteger<i8>,
            NonPositiveInteger<i8>,
            NegativeInteger<i8>,
            PositiveInteger<i8>,
            NonZeroInteger<i8>
        ];
    }

    /// Checks the bounds for `half` types.
    #[test]
    #[cfg(feature = "half")]
    fn bounded_integers_half() {
        use half::{bf16, f16};
        assert_impl_all![Integer<bf16>:
            ConstLowerBounded, ConstUpperBounded, LowerBounded, UpperBounded];
        assert_impl_all![Integer<f16>:
            ConstLowerBounded, ConstUpperBounded, LowerBounded, UpperBounded];
    }

    /// Checks the bounds for `twofloat` type.
    #[test]
    #[cfg(feature = "twofloat")]
    fn bounded_integers_twoflat() {
        use twofloat::TwoFloat;
        assert_impl_all![Integer<TwoFloat>:
            ConstLowerBounded, ConstUpperBounded, LowerBounded, UpperBounded];
    }

    /// Checks the bounds for `ibig` big integers.
    #[test]
    #[cfg(feature = "ibig")]
    fn bounded_integers_ibig() {
        use ibig::{IBig, UBig};
        assert_impl_all![Integer<UBig>: LowerBounded];
        assert_not_impl_any![Integer<UBig>: UpperBounded];
        assert_not_impl_any![Integer<IBig>: LowerBounded, UpperBounded];
    }
}
