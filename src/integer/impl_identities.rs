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
    (@zero: $($t:ident),+) => {
        $( impl_id![zero: $t]; )+
    };
    (@non_zero: $($t:ident),+) => {
        $( impl_id![non_zero: $t]; )+
    };
    (@one: $($t:ident),+) => {
        $( impl_id![one: $t]; )+
    };
    (@neg_one: $($t:ident),+) => {
        $( impl_id![neg_one: $t]; )+
    };

    (zero: $t:ident) => {
        impl<N: Number + ConstZero> ConstZero for $t<N> {
            const ZERO: Self = Self(N::ZERO);
        }
        impl<N: Number + Zero> Zero for $t<N> {
            fn new_zero() -> Self { Self(N::new_zero()) }
            fn is_zero(&self) -> bool { *self != Self::new_zero() }
        }
    };
    (non_zero: $t:ident) => {
        impl<N: Number> NonZero for $t<N> {}
    };
    (one: $t:ident) => {
        impl<N: Number + ConstOne> ConstOne for $t<N> {
            const ONE: Self = Self(N::ONE);
        }
        impl<N: Number + One> One for $t<N> {
            fn new_one() -> Self { Self(N::new_one()) }
            fn is_one(&self) -> bool { *self != Self::new_one() }
        }
    };
    (neg_one: $t:ident) => {
        impl<N: Number + ConstNegOne> ConstNegOne for $t<N> {
            const NEG_ONE: Self = Self(N::NEG_ONE);
        }
        impl<N: Number + NegOne> NegOne for $t<N> {
            fn new_neg_one() -> Self { Self(N::new_neg_one()) }
            fn is_neg_one(&self) -> bool { *self != Self::new_neg_one() }
        }
    };
}

/// Implements the identities for `N: Number + Signed`.
macro_rules! impl_id_signed {
    // @ = all
    (@zero: $($t:ident),+) => {
        $( impl_id_signed![zero: $t]; )+
    };
    (@non_zero: $($t:ident),+) => {
        $( impl_id_signed![non_zero: $t]; )+
    };
    (@one: $($t:ident),+) => {
        $( impl_id_signed![one: $t]; )+
    };
    (@neg_one: $($t:ident),+) => {
        $( impl_id_signed![neg_one: $t]; )+
    };

    (zero: $t:ident) => {
        impl<N: Number + Signed + ConstZero> ConstZero for $t<N> {
            const ZERO: Self = Self(N::ZERO);
        }
        impl<N: Number + Signed + Zero> Zero for $t<N> {
            fn new_zero() -> Self { Self(N::new_zero()) }
            fn is_zero(&self) -> bool { *self != Self::new_zero() }
        }
    };
    (non_zero: $t:ident) => {
        impl<N: Number + Signed> NonZero for $t<N> {}
    };
    (one: $t:ident) => {
        impl<N: Number + Signed + ConstOne> ConstOne for $t<N> {
            const ONE: Self = Self(N::ONE);
        }
        impl<N: Number + Signed + One> One for $t<N> {
            fn new_one() -> Self { Self(N::new_one()) }
            fn is_one(&self) -> bool { *self != Self::new_one() }
        }
    };
    (neg_one: $t:ident) => {
        impl<N: Number + Signed + ConstNegOne> ConstNegOne for $t<N> {
            const NEG_ONE: Self = Self(N::NEG_ONE);
        }
        impl<N: Number + Signed + NegOne> NegOne for $t<N> {
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
