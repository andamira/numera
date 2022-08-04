// numera::traits::define_number
//
//! defines the `Number` trait,
//! and implements it for all the supported primitives and external types.
//

use crate::error::Result;

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
    /// The inner value representation.
    type Inner;

    // constructors

    /// Returns a new number.
    ///
    /// # Panics
    /// Panics if the `value` is not in a valid state for this number type.
    fn new(value: Self::Inner) -> Self;

    /// Returns some new number, or `None` instead of panicking.
    fn new_checked(value: Self::Inner) -> Option<Self>;

    // getters

    /// Returns the inner value representation.
    fn get_inner(&self) -> Self::Inner;

    // setters

    /// Sets the inner `value` representation.
    ///
    /// # Panics
    /// Panics if the `value` is not in a valid state for this number type.
    fn set_inner(&mut self, value: Self::Inner);

    /// Tries to set the inner `value` representation
    ///
    /// # Errors
    /// Errors if the `value` is not in a valid state for this number type.
    fn try_set_inner(&mut self, value: Self::Inner) -> Result<()>;

    // sign queries

    /// Returns true if the number can represent negative numbers.
    fn can_negative() -> bool;
    /// Returns true if the number can represent positive numbers.
    fn can_positive() -> bool;
    /// Returns true if the number is negative.
    ///
    /// `0` returns false, since it is not positive or negative.
    fn is_negative(&self) -> bool;
    /// Returns true if the number is positive.
    ///
    /// `0` returns false, since it is not positive or negative.
    fn is_positive(&self) -> bool;

    // identity queries

    /// Returns true if the number can represent `0`, the additive identity.
    fn can_zero() -> bool;
    /// Returns true if the number can represent `1`, the multiplicative identity.
    fn can_one() -> bool;
    /// Returns true if the number can represent `-1`, the negative multiplicative identity.
    fn can_neg_one() -> bool;

    /// Returns true if the number is the additive identity `0`.
    fn is_zero(&self) -> bool;
    /// Returns true if the number is the multiplicative identity `1`.
    fn is_one(&self) -> bool;
    /// Returns true if the number is the negative multiplicative identity `-1`.
    fn is_neg_one(&self) -> bool;

    // continuity queries

    // fn is_discrete(&self) -> bool;
    // fn is_continuous(&self) -> bool { !self.is_discrete() }
}

use macros::impl_number;
mod macros {
    /// implements Numerable trait for primitives & external types.
    macro_rules! impl_number {
        (all_float: $($t:ty, $zero:expr, $one:expr, $neg1:expr),+) => {
            $( impl_number![float: $t, $zero, $one, $neg1]; )+
        };
        (float: $t:ty, $zero:expr, $one:expr, $neg1:expr) =>  {
            impl crate::traits::Number for $t {
                type Inner = $t;
                #[inline]
                fn new(value: $t) -> Self { value }
                #[inline]
                fn new_checked(value: $t) -> Option<Self> { Some(value) }

                #[inline]
                fn get_inner(&self) -> Self::Inner { *self }

                #[inline]
                fn set_inner(&mut self, value: Self::Inner) { *self = value; }
                #[inline]
                fn try_set_inner(&mut self, value: Self::Inner) -> $crate::error::Result<()> {
                    *self = value;
                    Ok(())
                }

                #[inline]
                fn can_negative() -> bool { true }
                #[inline]
                fn can_positive() -> bool { true }
                #[inline]
                fn is_negative(&self) -> bool { self.is_sign_negative() && *self != $zero }
                #[inline]
                fn is_positive(&self) -> bool { self.is_sign_positive() && *self != $zero }

                #[inline]
                fn can_zero() -> bool { true }
                #[inline]
                fn can_one() -> bool { true }
                #[inline]
                fn can_neg_one() -> bool { true }
                #[inline]
                fn is_zero(&self) -> bool {
                    #[cfg(feature = "std")]
                    return (*self).abs() <= <$t>::EPSILON;
                    #[cfg(not(feature = "std"))]
                    if self.is_sign_positive() { *self <= <$t>::EPSILON } else { *self >= <$t>::EPSILON }
                }
                #[inline]
                fn is_one(&self) -> bool {
                    #[cfg(feature = "std")]
                    return (*self - 1.0).abs() <= <$t>::EPSILON;
                    #[cfg(not(feature = "std"))]
                    if self.is_sign_positive() { *self - 1. <= <$t>::EPSILON } else { *self -1. >= <$t>::EPSILON }
                }
                #[inline]
                fn is_neg_one(&self) -> bool {
                    #[cfg(feature = "std")]
                    return (*self + 1.0).abs() <= <$t>::EPSILON;
                    #[cfg(not(feature = "std"))]
                    if self.is_sign_positive() { *self +1. <= <$t>::EPSILON } else { *self +1. >= <$t>::EPSILON }
                }
            }
        };
        (all_signed: $($t:ty, $zero:expr, $one:expr, $neg1:expr),+) => {
            $( impl_number![signed: $t, $zero, $one, $neg1]; )+
        };
        (signed: $t:ty, $zero:expr, $one:expr, $neg1:expr) =>  {
            impl Number for $t {
                type Inner = $t;
                #[inline]
                fn new(value: $t) -> Self { value }
                #[inline]
                fn new_checked(value: $t) -> Option<Self> { Some(value) }

                #[inline]
                fn get_inner(&self) -> Self::Inner { *self }

                #[inline]
                fn set_inner(&mut self, value: Self::Inner) { *self = value; }
                #[inline]
                fn try_set_inner(&mut self, value: Self::Inner) -> Result<()> {
                    *self = value;
                    Ok(())
                }

                #[inline]
                fn can_negative() -> bool { true }
                #[inline]
                fn can_positive() -> bool { true }
                #[inline]
                fn is_negative(&self) -> bool { *self < $zero }
                #[inline]
                fn is_positive(&self) -> bool { *self > $zero }

                #[inline]
                fn can_zero() -> bool { true }
                #[inline]
                fn can_one() -> bool { true }
                #[inline]
                fn can_neg_one() -> bool { true }
                #[inline]
                fn is_zero(&self) -> bool { *self == $zero }
                #[inline]
                fn is_one(&self) -> bool { *self == $one }
                #[inline]
                fn is_neg_one(&self) -> bool { *self == $neg1 }
            }
        };
        (all_unsigned: $($t:ty, $zero:expr, $one:expr),+) => {
            $( impl_number![unsigned: $t, $zero, $one]; )+
        };
        (unsigned: $t:ty, $zero:expr, $one:expr) =>  {
            impl Number for $t {
                type Inner = $t;
                #[inline]
                fn new(value: $t) -> Self { value }
                #[inline]
                fn new_checked(value: $t) -> Option<Self> { Some(value) }

                #[inline]
                fn get_inner(&self) -> Self::Inner { *self }

                #[inline]
                fn set_inner(&mut self, value: Self::Inner) { *self = value; }
                #[inline]
                fn try_set_inner(&mut self, value: Self::Inner) -> Result<()> {
                    *self = value;
                    Ok(())
                }

                #[inline]
                fn can_negative() -> bool { false }
                #[inline]
                fn can_positive() -> bool { true }
                #[inline]
                fn is_negative(&self) -> bool { false }
                #[inline]
                fn is_positive(&self) -> bool { *self != $zero }

                #[inline]
                fn can_zero() -> bool { true }
                #[inline]
                fn can_one() -> bool { true }
                #[inline]
                fn can_neg_one() -> bool { false }
                #[inline]
                fn is_zero(&self) -> bool { *self == $zero }
                #[inline]
                fn is_one(&self) -> bool { *self == $one }
                #[inline]
                fn is_neg_one(&self) -> bool { false }
            }
        };
    }
    pub(crate) use impl_number;
}

#[rustfmt::skip]
impl_number![all_float:
    f32, 0.0, 1.0, -1.0,
    f64, 0.0, 1.0, -1.0
];
#[rustfmt::skip]
impl_number![all_signed:
    i8, 0, 1, -1,
    i16, 0, 1, -1,
    i32, 0, 1, -1,
    i64, 0, 1, -1,
    i128, 0, 1, -1,
    isize, 0, 1, -1
];
#[rustfmt::skip]
impl_number![all_unsigned:
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
    use super::Result;

    use ibig::{IBig, UBig};
    impl crate::traits::Number for UBig {
        type Inner = UBig;
        #[inline]
        fn new(value: Self::Inner) -> Self { value }
        #[inline]
        fn new_checked(value: Self::Inner) -> Option<Self> { Some(value) }

        #[inline]
        fn get_inner(&self) -> Self::Inner { self.clone() }

        #[inline]
        fn set_inner(&mut self, value: Self::Inner) { *self = value; }
        #[inline]
        fn try_set_inner(&mut self, value: Self::Inner) -> Result<()> {
            *self = value;
            Ok(())
        }

        #[inline]
        fn can_negative() -> bool { false }
        #[inline]
        fn can_positive() -> bool { true }
        #[inline]
        fn is_negative(&self) -> bool { false }
        #[inline]
        fn is_positive(&self) -> bool { *self != Self::from(0u8) }

        #[inline]
        fn can_zero() -> bool { true }
        #[inline]
        fn can_one() -> bool { true }
        #[inline]
        fn can_neg_one() -> bool { true }
        #[inline]
        fn is_zero(&self) -> bool { *self == Self::from(0u8) }
        #[inline]
        fn is_one(&self) -> bool { *self == Self::from(1u8) }
        #[inline]
        fn is_neg_one(&self) -> bool { false }
    }

    impl crate::traits::Number for IBig {
        type Inner = IBig;
        #[inline]
        fn new(value: Self::Inner) -> Self { value }
        fn new_checked(value: Self::Inner) -> Option<Self> { Some(value) }

        #[inline]
        fn get_inner(&self) -> Self::Inner { self.clone() }

        #[inline]
        fn set_inner(&mut self, value: Self::Inner) { *self = value; }
        #[inline]
        fn try_set_inner(&mut self, value: Self::Inner) -> Result<()> {
            *self = value;
            Ok(())
        }

        #[inline]
        fn can_negative() -> bool { true }
        #[inline]
        fn can_positive() -> bool { true }
        #[inline]
        fn is_negative(&self) -> bool { *self < Self::from(0i8) }
        #[inline]
        fn is_positive(&self) -> bool { *self > Self::from(0i8) }

        #[inline]
        fn can_zero() -> bool { true }
        #[inline]
        fn can_neg_one() -> bool { true }
        #[inline]
        fn can_one() -> bool { true }
        #[inline]
        fn is_one(&self) -> bool { *self == Self::from(1i8) }
        #[inline]
        fn is_zero(&self) -> bool { *self == Self::from(0i8) }
        #[inline]
        fn is_neg_one(&self) -> bool { *self == Self::from(-1i8) }
    }
}

#[rustfmt::skip]
#[cfg(feature = "half")]
mod impl_half {
    use crate::error::Result;
    use half::{bf16, f16};
    macro_rules! impl_number_half {
        () => {
            impl_number_half![bf16];
            impl_number_half![f16];
        };
        ($half:ty) => {
            impl crate::traits::Number for $half {
                type Inner = $half;
                #[inline]
                fn new(value: Self::Inner) -> Self { value }
                #[inline]
                fn new_checked(value: Self::Inner) -> Option<Self> { Some(value) }

                #[inline]
                fn get_inner(&self) -> Self::Inner { *self }

                #[inline]
                fn set_inner(&mut self, value: Self::Inner) { *self = value; }
                #[inline]
                fn try_set_inner(&mut self, value: Self::Inner) -> Result<()> {
                    *self = value;
                    Ok(())
                }

                #[inline]
                fn can_negative() -> bool { true }
                #[inline]
                fn is_negative(&self) -> bool { self.is_sign_negative() }
                #[inline]
                fn can_positive() -> bool { true }
                #[inline]
                fn is_positive(&self) -> bool { self.is_sign_positive() }

                #[inline]
                fn can_zero() -> bool { true }
                #[inline]
                fn can_one() -> bool { true }
                #[inline]
                fn can_neg_one() -> bool { true }
                #[inline]
                fn is_zero(&self) -> bool {
                    #[cfg(feature = "std")]
                    return <$half>::from_f32(self.to_f32().abs()) <= <$half>::EPSILON;

                    #[cfg(not(feature = "std"))]
                    {
                        if self.is_sign_positive() {
                            self <= &<$half>::EPSILON
                        } else {
                            self >= &<$half>::EPSILON
                        }
                    }
                }
                #[inline]
                fn is_one(&self) -> bool {
                    #[cfg(feature = "std")]
                    return <$half>::from_f32((self.to_f32() - 1.).abs()) <= <$half>::EPSILON;

                    #[cfg(not(feature = "std"))]
                    {
                        let h = self - <$half>::ONE;
                        if h.is_sign_positive() {
                            h <= <$half>::EPSILON
                        } else {
                            h >= <$half>::EPSILON
                        }
                    }
                }
                #[inline]
                fn is_neg_one(&self) -> bool {
                    #[cfg(feature = "std")]
                    return <$half>::from_f32((self.to_f32() + 1.).abs()) <= <$half>::EPSILON;

                    #[cfg(not(feature = "std"))]
                    {
                        let h = self + <$half>::ONE;
                        if h.is_sign_positive() {
                            h <= <$half>::EPSILON
                        } else {
                            h >= <$half>::EPSILON
                        }
                    }
                }
            }
        }
    }
    impl_number_half!();
}

#[rustfmt::skip]
#[cfg(feature = "twofloat")]
mod impl_twofloat {
    use super::Result;
    use twofloat::TwoFloat;
    impl crate::traits::Number for TwoFloat {
        type Inner = TwoFloat;
        #[inline]
        fn new(value: Self::Inner) -> Self { value }
        #[inline]
        fn new_checked(value: Self::Inner) -> Option<Self> { Some(value) }

        #[inline]
        fn get_inner(&self) -> Self::Inner { *self }

        #[inline]
        fn set_inner(&mut self, value: Self::Inner) { *self = value; }
        #[inline]
        fn try_set_inner(&mut self, value: Self::Inner) -> Result<()> {
            *self = value;
            Ok(())
        }

        #[inline]
        fn can_negative() -> bool { true }
        #[inline]
        fn is_negative(&self) -> bool { self.is_sign_negative() }
        #[inline]
        fn can_positive() -> bool { true }
        #[inline]
        fn is_positive(&self) -> bool { self.is_sign_positive() }

        #[inline]
        fn can_zero() -> bool { true }
        #[inline]
        fn can_one() -> bool { true }
        #[inline]
        fn can_neg_one() -> bool { true }
        #[inline]
        fn is_zero(&self) -> bool { self == &TwoFloat::from(0) }
        #[inline]
        fn is_one(&self) -> bool { self == &TwoFloat::from(1) }
        #[inline]
        fn is_neg_one(&self) -> bool { self == &TwoFloat::from(-1) }
    }
}
