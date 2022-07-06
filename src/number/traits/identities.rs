// numera::number::traits::identities
//
//!
//

/// The multiplicative identity `1`.
pub trait One: Sized {
    ///
    fn one() -> Self;

    ///
    fn is_one(&self) -> bool;

    ///
    fn set_one(&mut self) {
        *self = Self::one();
    }
}

/// The additive identity `0`.
pub trait Zero: Sized {
    ///
    fn zero() -> Self;

    ///
    fn is_zero(&self) -> bool;

    ///
    fn set_zero(&mut self) {
        *self = Self::zero();
    }
}

macro_rules! impl_identities {
    (all: $($ty:ty, $zero:expr, $one:expr),+) => {
        $( impl_identities![$ty, $zero, $one]; )+
    };
    ($ty:ty, $zero:expr, $one:expr) => {
        impl Zero for $ty {
            fn zero() -> Self { $zero }
            fn is_zero(&self) -> bool { *self == $zero }
        }
        impl One for $ty {
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
