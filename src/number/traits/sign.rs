// numera::number::traits::sign
//
//!
//

/// Information about the sign of a number.
#[rustfmt::skip]
pub trait Sign {
    /// Returns true if the number type can be made negative.
    fn can_negative() -> bool;
    /// Returns true if the number type can be made positive.
    fn can_positive() -> bool;

    /// Returns true if the number is $<0$.
    fn is_negative(&self) -> bool;
    /// Returns true if the number is $>0$.
    fn is_positive(&self) -> bool;

    /// Returns the inverse of the number, if possible.
    fn inverse(&self) -> Option<Self> where Self: Sized;

    /// Returns the absolute value of the number, if possible.
    fn abs(&self) -> Option<Self> where Self: Sized;
}

/// Implements `Sign` on primitives
macro_rules! impl_sign {
    // Primitives that can not be negative.
    (all_unsigned: $($ty:ty),+) => {
        $( impl_sign![unsigned: $ty]; )+
    };
    (unsigned: $ty:ty) => {
        impl Sign for $ty {
            fn can_negative() -> bool { false }
            fn can_positive() -> bool { true }
            fn is_negative(&self) -> bool { false }
            fn is_positive(&self) -> bool { true }
            fn inverse(&self) -> Option<Self> { None }
            fn abs(&self) -> Option<Self> { Some(*self) }
        }
    };
    // Primitives that can be both positive and negative.
    (all_signed: $($ty:ty),+) => {
        $( impl_sign![signed: $ty]; )+
    };
    (signed: $ty:ty) => {
        impl Sign for $ty {
            fn can_negative() -> bool { true }
            fn can_positive() -> bool { true }
            fn is_negative(&self) -> bool { <$ty>::is_negative(*self) }
            fn is_positive(&self) -> bool { <$ty>::is_positive(*self) }
            fn inverse(&self) -> Option<Self> { self.checked_neg() }
            fn abs(&self) -> Option<Self> { self.checked_abs() }
        }
    };
    // Floating point primitives that can be both positive and negative.
    (all_float: $($ty:ty),+) => {
        $( impl_sign![float: $ty]; )+
    };
    (float: $ty:ty) => {
        impl Sign for $ty {
            fn can_negative() -> bool { true }
            fn can_positive() -> bool { true }
            fn is_negative(&self) -> bool { self.is_sign_negative() && *self != 0.0 }
            fn is_positive(&self) -> bool { self.is_sign_positive() && *self != 0.0 }
            fn inverse(&self) -> Option<Self> { Some(-self) }
            fn abs(&self) -> Option<Self> { Some(<$ty>::abs(*self)) }
        }
    };
}
impl_sign![all_unsigned: u8, u16, u32, u64, u128, usize];
impl_sign![all_signed: i8, i16, i32, i64, i128, isize];
impl_sign![all_float: f32, f64];

#[rustfmt::skip]
#[cfg(feature = "twofloat")]
mod impl_twofloat {
    use super::Sign;
    use twofloat::TwoFloat;
    impl Sign for TwoFloat {
        fn can_negative() -> bool { true }
        fn can_positive() -> bool { true }
        fn is_negative(&self) -> bool { self.is_sign_negative() && *self != 0.0 }
        fn is_positive(&self) -> bool { self.is_sign_positive() && *self != 0.0 }
        fn inverse(&self) -> Option<Self> { Some(-self) }
        fn abs(&self) -> Option<Self> { Some(TwoFloat::abs(self)) }
    }
}

#[cfg(feature = "half")]
mod impl_half {
    use super::Sign;
    use half::{bf16, f16};
    macro_rules! impl_sign {
        ($($ty:ty),+) => {
            $(
            impl Sign for $ty {
                fn can_negative() -> bool { true }
                fn can_positive() -> bool { true }
                fn is_negative(&self) -> bool {
                    self.is_sign_negative() && *self != <$ty>::from_f32_const(0.0)
                }
                fn is_positive(&self) -> bool {
                    self.is_sign_positive() && *self != <$ty>::from_f32_const(0.0)
                }
                fn inverse(&self) -> Option<Self> { Some(-self) }
                fn abs(&self) -> Option<Self> {
                    Some(<$ty>::from_f32(self.to_f32().abs()))
                }
            }
            )+
        };
    }
    impl_sign![bf16, f16];
}
