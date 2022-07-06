// numera::number::traits::sign
//
//!
//

/// Information about the sign of a number.
pub trait Sign {
    /// Returns true if the number can be made negative.
    fn can_negative() -> bool;
    /// Returns true if the number can be made positive.
    fn can_positive() -> bool;

    /// Returns true if the number is negative and different from `0`.
    fn is_negative(&self) -> bool;
    /// Returns true if the number is positive and different from `0`.
    fn is_positive(&self) -> bool;
}
///
/// type, can_neg, can_pos, is_neg, is_pos,
macro_rules! impl_sign {
    (all_unsigned: $($ty:ty),+) => {
        $( impl_sign![unsigned: $ty]; )+
    };
    (unsigned: $ty:ty) => {
        impl Sign for $ty {
            fn can_negative() -> bool { false }
            fn can_positive() -> bool { true }
            fn is_negative(&self) -> bool { false }
            fn is_positive(&self) -> bool { true }
        }
    };
    //
    (all_signed: $($ty:ty),+) => {
        $( impl_sign![signed: $ty]; )+
    };
    (signed: $ty:ty) => {
        impl Sign for $ty {
            fn can_negative() -> bool { true }
            fn can_positive() -> bool { true }
            fn is_negative(&self) -> bool { <$ty>::is_negative(*self) }
            fn is_positive(&self) -> bool { <$ty>::is_positive(*self) }
        }
    };
    //
    (all_float: $($ty:ty),+) => {
        $( impl_sign![float: $ty]; )+
    };
    (float: $ty:ty) => {
        impl Sign for $ty {
            fn can_negative() -> bool { true }
            fn can_positive() -> bool { true }
            fn is_negative(&self) -> bool { self.is_sign_negative() && *self != 0.0 }
            fn is_positive(&self) -> bool { self.is_sign_positive() && *self != 0.0 }
        }
    };
}
impl_sign![all_unsigned: u8, u16, u32, u64, u128, usize];
impl_sign![all_signed: i8, i16, i32, i64, i128, isize];
impl_sign![all_float: f32, f64];

#[cfg(feature = "twofloat")]
impl_sign![all_float: twofloat::TwoFloat];

#[rustfmt::skip]
#[cfg(feature = "half")]
impl Sign for half::f16 {
    fn can_negative() -> bool { true }
    fn can_positive() -> bool { true }
    fn is_negative(&self) -> bool {
        self.is_sign_negative() && *self != half::f16::from_f32_const(0.0)
    }
    fn is_positive(&self) -> bool {
        self.is_sign_positive() && *self != half::f16::from_f32_const(0.0)
    }
}
