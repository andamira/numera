// numera::number::traits::identities
//
//!
//

use core::ops::{Add, Div, Mul, Sub};

/// Defines both the additive and multiplicative identity elements.
pub trait Identities:
    Sized
    + Add<Self, Output = Self>
    + Mul<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Div<Self, Output = Self>
{
    ///
    fn zero() -> Self;

    ///
    fn is_zero(&self) -> bool;

    ///
    fn set_zero(&mut self) {
        *self = Self::zero();
    }

    ///
    fn one() -> Self;

    ///
    fn is_one(&self) -> bool;

    ///
    fn set_one(&mut self) {
        *self = Self::one();
    }
}
macro_rules! impl_identities {
    (all: $($ty:ty, $zero:expr, $one:expr),+) => {
        $( impl_identities![$ty, $zero, $one]; )+
    };
    ($ty:ty, $zero:expr, $one:expr) => {
        impl Identities for $ty {
            fn zero() -> Self { $zero }
            fn is_zero(&self) -> bool { *self == $zero }
            fn one() -> Self { $one }
            fn is_one(&self) -> bool { *self == $one }
        }
    };
}
#[rustfmt::skip]
impl_identities![all:
    f32, 0.0, 1.0, f64, 0.0, 1.0,
    i8, 0, 1, u8, 0, 1, i16, 0, 1, u16, 0, 1, i32, 0, 1, u32, 0, 1,
    i64, 0, 1, u64, 0, 1, i128, 0, 1, u128, 0, 1, isize, 0, 1, usize, 0, 1
];

#[cfg(feature = "twofloat")]
impl_identities![twofloat::TwoFloat, 0.0, 1.0];

#[cfg(feature = "half")]
impl_identities![all: half::f16, 0.0, 1.0, half::bf16, 0.0, 1.0];
