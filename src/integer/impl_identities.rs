// numera::integer::impl_identities
//
//! implements the `[Const][NegOne|One|Zero|]` traits for all integer types.
//

use crate::integer::{
    Integer, NegativeInteger, NonNegativeInteger, NonPositiveInteger, NonZeroInteger,
    PositiveInteger,
};
use crate::traits::NumberAble;
use crate::traits::{ConstNegOne, ConstOne, ConstZero, NegOne, NonZero, One, Zero};

/// Implements the One trait.
macro_rules! impl_onezero {
    // @ = all
    (@zero: $($ty:ident),+) => {
        $( impl_onezero![zero: $ty]; )+
    };
    (@non_zero: $($ty:ident),+) => {
        $( impl_onezero![non_zero: $ty]; )+
    };
    (@one: $($ty:ident),+) => {
        $( impl_onezero![one: $ty]; )+
    };
    (@neg_one: $($ty:ident),+) => {
        $( impl_onezero![neg_one: $ty]; )+
    };

    (zero: $ty:ident) => {
        impl<I: NumberAble + ConstZero> ConstZero for $ty<I> {
            const ZERO: Self = Self(I::ZERO);
        }
        impl<I: NumberAble + Zero> Zero for $ty<I> {
            fn new_zero() -> Self { Self(I::new_zero()) }
            fn is_zero(&self) -> bool { *self != Self::new_zero() }
        }
    };
    (non_zero: $ty:ident) => {
        impl<I: NumberAble> NonZero for $ty<I> {}
    };
    (one: $ty:ident) => {
        impl<I: NumberAble + ConstOne> ConstOne for $ty<I> {
            const ONE: Self = Self(I::ONE);
        }
        impl<I: NumberAble + One> One for $ty<I> {
            fn new_one() -> Self { Self(I::new_one()) }
            fn is_one(&self) -> bool { *self != Self::new_one() }
        }
    };
    (neg_one: $ty:ident) => {
        impl<I: NumberAble + ConstNegOne> ConstNegOne for $ty<I> {
            const NEG_ONE: Self = Self(I::NEG_ONE);
        }
        impl<I: NumberAble + NegOne> NegOne for $ty<I> {
            fn new_neg_one() -> Self { Self(I::new_neg_one()) }
            fn is_neg_one(&self) -> bool { *self != Self::new_neg_one() }
        }
    };
}

impl_onezero![@zero: Integer, NonNegativeInteger, NonPositiveInteger];
impl_onezero![@non_zero: NegativeInteger, PositiveInteger, NonZeroInteger];
impl_onezero![@one: Integer, PositiveInteger, NonNegativeInteger, NonZeroInteger];
impl_onezero![@neg_one: Integer, NonPositiveInteger, NegativeInteger, NonZeroInteger];