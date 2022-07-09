// numera::traits::define_number
//
//! defines the `Number` trait,
//! and implements it for all the supported primitives and external types.
//

/// A *number* is a mathematical object used to count, measure, and label
/// ([w][1w]/[m][1m]).
///
/// A general term which refers to a member of a given set.
///
/// [1w]: https://en.wikipedia.org/wiki/Number
/// [1m]: https://mathworld.wolfram.com/Number.html
//
#[rustfmt::skip]
pub trait Number: PartialOrd + Clone {
    /// The inner value representation for this number.
    type Value;

    /// Returns a new number.
    ///
    /// Must ensure the inner value is in a valid format.
    fn new(value: Self::Value) -> Self;

    // sign

    /// Returns true if the number can represent negative numbers.
    fn can_negative() -> bool;
    /// Returns true if the number is negative.
    ///
    /// `0` returns false, since it is not positive or negative.
    fn is_negative(&self) -> bool;

    /// Returns true if the number can represent positive numbers.
    fn can_positive() -> bool;
    /// Returns true if the number is positive.
    ///
    /// `0` returns false, since it is not positive or negative.
    fn is_positive(&self) -> bool;

    // identities

    /// Returns true if the number can represent `0`, the additive identity.
    fn can_zero() -> bool;
    /// Returns true if the number is the additive identity `0`.
    fn is_zero(&self) -> bool;

    /// Returns true if the number can represent `1`, the multiplicative identity.
    fn can_one() -> bool;
    /// Returns true if the number is the multiplicative identity `1`.
    fn is_one(&self) -> bool;

    /// Returns true if the number can represent `-1`, the negative multiplicative identity.
    fn can_neg_one() -> bool;
    /// Returns true if the number is the negative multiplicative identity `-1`.
    fn is_neg_one(&self) -> bool;

    // continuity
    // fn is_discrete(&self) -> bool;
    // fn is_continuous(&self) -> bool { !self.is_discrete() }
}

use macros::impl_numberable;
mod macros {
    /// implements Numerable trait for primitives & external types.
    macro_rules! impl_numberable {
        (all_float: $($ty:ty, $zero:expr, $one:expr, $neg1:expr),+) => {
            $( impl_numberable![float: $ty, $zero, $one, $neg1]; )+
        };
        (float: $ty:ty, $zero:expr, $one:expr, $neg1:expr) =>  {
            impl crate::traits::Number for $ty {
                type Value = $ty;
                fn new(value: $ty) -> Self { value }

                fn can_negative() -> bool { true }
                fn is_negative(&self) -> bool { self.is_sign_negative() && *self != $zero }
                fn can_positive() -> bool { true }
                fn is_positive(&self) -> bool { self.is_sign_positive() && *self != $zero }

                fn can_zero() -> bool { true }
                fn is_zero(&self) -> bool { *self == $zero }

                fn can_one() -> bool { true }
                fn is_one(&self) -> bool { *self == $one }

                fn can_neg_one() -> bool { true }
                fn is_neg_one(&self) -> bool { *self == $neg1 }
            }
        };
        (all_signed: $($ty:ty, $zero:expr, $one:expr, $neg1:expr),+) => {
            $( impl_numberable![signed: $ty, $zero, $one, $neg1]; )+
        };
        (signed: $ty:ty, $zero:expr, $one:expr, $neg1:expr) =>  {
            impl Number for $ty {
                type Value = $ty;
                fn new(value: $ty) -> Self { value }

                fn can_negative() -> bool { true }
                fn is_negative(&self) -> bool { *self < $zero }

                fn can_positive() -> bool { true }
                fn is_positive(&self) -> bool { *self > $zero }

                fn can_zero() -> bool { true }
                fn is_zero(&self) -> bool { *self == $zero }

                fn can_one() -> bool { true }
                fn is_one(&self) -> bool { *self == $one }

                fn can_neg_one() -> bool { true }
                fn is_neg_one(&self) -> bool { *self == $neg1 }
            }
        };
        (all_unsigned: $($ty:ty, $zero:expr, $one:expr),+) => {
            $( impl_numberable![unsigned: $ty, $zero, $one]; )+
        };
        (unsigned: $ty:ty, $zero:expr, $one:expr) =>  {
            impl Number for $ty {
                type Value = $ty;
                fn new(value: $ty) -> Self { value }

                fn can_negative() -> bool { false }
                fn is_negative(&self) -> bool { false }

                fn can_positive() -> bool { true }
                fn is_positive(&self) -> bool { *self != $zero }

                fn can_zero() -> bool { true }
                fn is_zero(&self) -> bool { *self == $zero }

                fn can_one() -> bool { true }
                fn is_one(&self) -> bool { *self == $one }

                fn can_neg_one() -> bool { false }
                fn is_neg_one(&self) -> bool { false }
            }
        };
    }
    pub(crate) use impl_numberable;
}

#[rustfmt::skip]
impl_numberable![all_float:
    f32, 0.0, 1.0, -1.0,
    f64, 0.0, 1.0, -1.0
];
#[rustfmt::skip]
impl_numberable![all_signed:
    i8, 0, 1, -1,
    i16, 0, 1, -1,
    i32, 0, 1, -1,
    i64, 0, 1, -1,
    i128, 0, 1, -1,
    isize, 0, 1, -1
];
#[rustfmt::skip]
impl_numberable![all_unsigned:
    u8, 0, 1,
    u16, 0, 1,
    u32, 0, 1,
    u64, 0, 1,
    u128, 0, 1,
    usize, 0, 1
];

#[rustfmt::skip]
#[cfg(feature = "ibig")]
mod impl_ibig {
    use ibig::{IBig, UBig};
    impl crate::traits::Number for UBig {
        type Value = UBig;
        fn new(value: Self::Value) -> Self { value }

        fn can_negative() -> bool { false }
        fn is_negative(&self) -> bool { false }
        fn can_positive() -> bool { true }
        fn is_positive(&self) -> bool { *self != Self::from(0u8) }

        fn can_zero() -> bool { true }
        fn is_zero(&self) -> bool { *self == Self::from(0u8) }

        fn can_one() -> bool { true }
        fn is_one(&self) -> bool { *self == Self::from(1u8) }

        fn can_neg_one() -> bool { true }
        fn is_neg_one(&self) -> bool { false }
    }

    impl crate::traits::Number for IBig {
        type Value = IBig;
        fn new(value: Self::Value) -> Self { value }

        fn can_negative() -> bool { true }
        fn is_negative(&self) -> bool { *self < Self::from(0i8) }
        fn can_positive() -> bool { true }
        fn is_positive(&self) -> bool { *self > Self::from(0i8) }

        fn can_zero() -> bool { true }
        fn is_zero(&self) -> bool { *self == Self::from(0i8) }

        fn can_one() -> bool { true }
        fn is_one(&self) -> bool { *self == Self::from(1i8) }

        fn can_neg_one() -> bool { true }
        fn is_neg_one(&self) -> bool { *self == Self::from(-1i8) }
    }
}

#[rustfmt::skip]
#[cfg(feature = "half")]
mod impl_half {
    use super::impl_numberable;
    use half::{bf16, f16};
    impl_numberable![all_float:
        bf16, bf16::from_f32_const(0.0), bf16::from_f32_const(1.0), bf16::from_f32_const(-1.0),
        f16, f16::from_f32_const(0.0), f16::from_f32_const(1.0), f16::from_f32_const(-1.0)
    ];
}

#[rustfmt::skip]
#[cfg(feature = "twofloat")]
mod impl_twofloat {
    use twofloat::TwoFloat;
    impl crate::traits::Number for TwoFloat {
        type Value = TwoFloat;
        fn new(value: Self::Value) -> Self { value }

        fn can_negative() -> bool { true }
        fn is_negative(&self) -> bool { self.is_sign_negative() }
        fn can_positive() -> bool { true }
        fn is_positive(&self) -> bool { self.is_sign_positive() }

        fn can_zero() -> bool { true }
        fn is_zero(&self) -> bool { self == &TwoFloat::from(0) }

        fn can_one() -> bool { true }
        fn is_one(&self) -> bool { self == &TwoFloat::from(1) }

        fn can_neg_one() -> bool { true }
        fn is_neg_one(&self) -> bool { self == &TwoFloat::from(-1) }
    }
}
