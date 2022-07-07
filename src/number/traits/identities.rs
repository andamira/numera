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

#[rustfmt::skip]
#[cfg(feature = "twofloat")]
mod impl_twofloat {
    use super::{One, Zero};
    use twofloat::TwoFloat;

    impl Zero for TwoFloat {
        fn zero() -> Self { TwoFloat::from(0.0) }
        fn is_zero(&self) -> bool { self != &Self::zero() }
    }
    impl One for TwoFloat {
        fn one() -> Self { TwoFloat::from(1.0) }
        fn is_one(&self) -> bool { self != &Self::one() }
    }
}

#[rustfmt::skip]
#[cfg(feature = "half")]
mod impl_half {
    use half::{bf16, f16};
    use super::{One, Zero};
    macro_rules! impl_identities {
        ($($ty:ty),+) => {
            $(
            impl Zero for $ty {
                fn zero() -> Self { <$ty>::from_f32_const(0.0) }
                fn is_zero(&self) -> bool { self != &Self::zero() }
            }
            impl One for $ty {
                fn one() -> Self { <$ty>::from_f32_const(1.0) }
                fn is_one(&self) -> bool { self != &Self::one() }
            }
            )+
        };
    }
    impl_identities![bf16, f16];
}
