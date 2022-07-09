// numera::integer::impl_bounded
//
//! implements the `[Const][Lower|Upper]Bounded` trait for all integer types.
//

use crate::integer::{
    Integer, NegativeInteger, NonNegativeInteger, NonPositiveInteger, NonZeroInteger,
    PositiveInteger,
};
use crate::traits::{
    ConstLowerBounded, ConstNegOne, ConstOne, ConstUpperBounded, ConstZero, LowerBounded, NegOne,
    Number, One, Signed, UpperBounded, Zero,
};

// Integer (MIN..MAX)

impl<N: Number + Signed + ConstLowerBounded> ConstLowerBounded for Integer<N> {
    const MIN: Self = Self(N::MIN);
}
impl<N: Number + Signed + ConstUpperBounded> ConstUpperBounded for Integer<N> {
    const MAX: Self = Self(N::MAX);
}

impl<N: Number + Signed + LowerBounded> LowerBounded for Integer<N> {
    fn new_min() -> Self {
        Self(N::new_min())
    }
}
impl<N: Number + Signed + UpperBounded> UpperBounded for Integer<N> {
    fn new_max() -> Self {
        Self(N::new_max())
    }
}

// NonZeroInteger (MIN..MAX)

impl<N: Number + Signed + ConstLowerBounded> ConstLowerBounded for NonZeroInteger<N> {
    const MIN: Self = Self(N::MIN);
}
impl<N: Number + Signed + ConstUpperBounded> ConstUpperBounded for NonZeroInteger<N> {
    const MAX: Self = Self(N::MAX);
}

impl<N: Number + Signed + LowerBounded> LowerBounded for NonZeroInteger<N> {
    fn new_min() -> Self {
        Self(N::new_min())
    }
}
impl<N: Number + Signed + UpperBounded> UpperBounded for NonZeroInteger<N> {
    fn new_max() -> Self {
        Self(N::new_max())
    }
}

// NonNegativeInteger (ZERO..=MAX)

impl<N: Number + ConstLowerBounded + ConstZero> ConstLowerBounded for NonNegativeInteger<N> {
    const MIN: Self = Self(N::ZERO);
}
impl<N: Number + ConstUpperBounded> ConstUpperBounded for NonNegativeInteger<N> {
    const MAX: Self = Self(N::MAX);
}

impl<N: Number + LowerBounded + Zero> LowerBounded for NonNegativeInteger<N> {
    fn new_min() -> Self {
        Self(N::new_zero())
    }
}
impl<N: Number + UpperBounded> UpperBounded for NonNegativeInteger<N> {
    fn new_max() -> Self {
        Self(N::new_max())
    }
}

// PositiveInteger (ONE..=MAX)

impl<N: Number + ConstLowerBounded + ConstOne> ConstLowerBounded for PositiveInteger<N> {
    const MIN: Self = Self(N::ONE);
}
impl<N: Number + ConstUpperBounded> ConstUpperBounded for PositiveInteger<N> {
    const MAX: Self = Self(N::MAX);
}

impl<N: Number + LowerBounded + One> LowerBounded for PositiveInteger<N> {
    fn new_min() -> Self {
        Self(N::new_one())
    }
}
impl<N: Number + UpperBounded> UpperBounded for PositiveInteger<N> {
    fn new_max() -> Self {
        Self(N::new_max())
    }
}

// NonPositiveInteger (MIN..=ZERO)

impl<N: Number + Signed + ConstLowerBounded> ConstLowerBounded for NonPositiveInteger<N> {
    const MIN: Self = Self(N::MIN);
}
impl<N: Number + Signed + ConstUpperBounded + ConstZero> ConstUpperBounded
    for NonPositiveInteger<N>
{
    const MAX: Self = Self(N::ZERO);
}

impl<N: Number + Signed + LowerBounded> LowerBounded for NonPositiveInteger<N> {
    fn new_min() -> Self {
        Self(N::new_min())
    }
}
impl<N: Number + Signed + UpperBounded + Zero> UpperBounded for NonPositiveInteger<N> {
    fn new_max() -> Self {
        Self(N::new_zero())
    }
}

// NegativeInteger (MIN..=NEG_ONE)

impl<N: Number + Signed + ConstLowerBounded> ConstLowerBounded for NegativeInteger<N> {
    const MIN: Self = Self(N::MIN);
}
impl<N: Number + Signed + ConstUpperBounded + ConstNegOne> ConstUpperBounded
    for NegativeInteger<N>
{
    const MAX: Self = Self(N::NEG_ONE);
}

impl<N: Number + Signed + LowerBounded> LowerBounded for NegativeInteger<N> {
    fn new_min() -> Self {
        Self(N::new_min())
    }
}
impl<N: Number + Signed + UpperBounded + NegOne> UpperBounded for NegativeInteger<N> {
    fn new_max() -> Self {
        Self(N::new_neg_one())
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
        assert_impl_all![PositiveInteger<UBig>: LowerBounded];
        assert_not_impl_any![NonNegativeInteger<UBig>: UpperBounded];
        assert_not_impl_any![NegativeInteger<IBig>: LowerBounded, UpperBounded];
    }
}
