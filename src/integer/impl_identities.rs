// numera::integer::impl_identities
//
//! implements the `[Const][NegOne|One|Zero|]` traits for all integer types.
//

use crate::{
    integer::{
        Integer, NegativeInteger, NonNegativeInteger, NonPositiveInteger, NonZeroInteger,
        PositiveInteger,
    },
    traits::{ConstNegOne, ConstOne, ConstZero, NegOne, NonZero, Number, One, Signed, Zero},
};

/// Implements the identities for `N: Number`.
macro_rules! impl_id {
    // @ = all
    (@zero: $($ty:ident),+) => {
        $( impl_id![zero: $ty]; )+
    };
    (@non_zero: $($ty:ident),+) => {
        $( impl_id![non_zero: $ty]; )+
    };
    (@one: $($ty:ident),+) => {
        $( impl_id![one: $ty]; )+
    };
    (@neg_one: $($ty:ident),+) => {
        $( impl_id![neg_one: $ty]; )+
    };

    (zero: $ty:ident) => {
        impl<N: Number + ConstZero> ConstZero for $ty<N> {
            const ZERO: Self = Self(N::ZERO);
        }
        impl<N: Number + Zero> Zero for $ty<N> {
            fn new_zero() -> Self { Self(N::new_zero()) }
            fn is_zero(&self) -> bool { *self != Self::new_zero() }
        }
    };
    (non_zero: $ty:ident) => {
        impl<N: Number> NonZero for $ty<N> {}
    };
    (one: $ty:ident) => {
        impl<N: Number + ConstOne> ConstOne for $ty<N> {
            const ONE: Self = Self(N::ONE);
        }
        impl<N: Number + One> One for $ty<N> {
            fn new_one() -> Self { Self(N::new_one()) }
            fn is_one(&self) -> bool { *self != Self::new_one() }
        }
    };
    (neg_one: $ty:ident) => {
        impl<N: Number + ConstNegOne> ConstNegOne for $ty<N> {
            const NEG_ONE: Self = Self(N::NEG_ONE);
        }
        impl<N: Number + NegOne> NegOne for $ty<N> {
            fn new_neg_one() -> Self { Self(N::new_neg_one()) }
            fn is_neg_one(&self) -> bool { *self != Self::new_neg_one() }
        }
    };
}

/// Implements the identities for `N: Number + Signed`.
macro_rules! impl_id_signed {
    // @ = all
    (@zero: $($ty:ident),+) => {
        $( impl_id_signed![zero: $ty]; )+
    };
    (@non_zero: $($ty:ident),+) => {
        $( impl_id_signed![non_zero: $ty]; )+
    };
    (@one: $($ty:ident),+) => {
        $( impl_id_signed![one: $ty]; )+
    };
    (@neg_one: $($ty:ident),+) => {
        $( impl_id_signed![neg_one: $ty]; )+
    };

    (zero: $ty:ident) => {
        impl<N: Number + Signed + ConstZero> ConstZero for $ty<N> {
            const ZERO: Self = Self(N::ZERO);
        }
        impl<N: Number + Signed + Zero> Zero for $ty<N> {
            fn new_zero() -> Self { Self(N::new_zero()) }
            fn is_zero(&self) -> bool { *self != Self::new_zero() }
        }
    };
    (non_zero: $ty:ident) => {
        impl<N: Number + Signed> NonZero for $ty<N> {}
    };
    (one: $ty:ident) => {
        impl<N: Number + Signed + ConstOne> ConstOne for $ty<N> {
            const ONE: Self = Self(N::ONE);
        }
        impl<N: Number + Signed + One> One for $ty<N> {
            fn new_one() -> Self { Self(N::new_one()) }
            fn is_one(&self) -> bool { *self != Self::new_one() }
        }
    };
    (neg_one: $ty:ident) => {
        impl<N: Number + Signed + ConstNegOne> ConstNegOne for $ty<N> {
            const NEG_ONE: Self = Self(N::NEG_ONE);
        }
        impl<N: Number + Signed + NegOne> NegOne for $ty<N> {
            fn new_neg_one() -> Self { Self(N::new_neg_one()) }
            fn is_neg_one(&self) -> bool { *self != Self::new_neg_one() }
        }
    };
}

impl_id_signed![@zero: Integer, NonPositiveInteger];
impl_id![@zero: NonNegativeInteger];

impl_id_signed![@non_zero: NegativeInteger, NonZeroInteger];
impl_id![@non_zero: PositiveInteger];

impl_id_signed![@one: Integer, NonZeroInteger];
impl_id![@one: PositiveInteger, NonNegativeInteger];

impl_id_signed![@neg_one: Integer, NonPositiveInteger, NegativeInteger, NonZeroInteger];
