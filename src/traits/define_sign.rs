// numera::traits::define_sign
//
//! defines the `Signed` and `Unsigned` traits.
//! and implements them for the primitives and supported external types.
//

/// Can represent negative numbers.
#[rustfmt::skip]
pub trait Signed {
    /// The value is > `0`.
    fn is_positive(&self) -> bool;
    /// The value is < `0`.
    fn is_negative(&self) -> bool;
}

/// Can *not* represent negative numbers.
pub trait Unsigned {}

/// Implements `Signed` on primitives
macro_rules! impl_sign {
    // Primitives that can not be negative.
    (all_unsigned: $($ty:ty),+) => {
        $( impl_sign![unsigned: $ty]; )+
    };
    (unsigned: $ty:ty) => {
        impl Unsigned for $ty { }
    };
    // Primitives that can be both positive and negative.
    (all_signed: $($ty:ty),+) => {
        $( impl_sign![signed: $ty]; )+
    };
    (signed: $ty:ty) => {
        impl Signed for $ty {
            fn is_negative(&self) -> bool { <$ty>::is_negative(*self) }
            fn is_positive(&self) -> bool { <$ty>::is_positive(*self) }
        }
    };
    // Floating point primitives that can be both positive and negative.
    (all_float: $($ty:ty),+) => {
        $( impl_sign![float: $ty]; )+
    };
    (float: $ty:ty) => {
        impl Signed for $ty {
            // −0.0 = +0.0
            fn is_negative(&self) -> bool { self.is_sign_negative() && *self != 0.0 }
            fn is_positive(&self) -> bool { self.is_sign_positive() && *self != 0.0 }
            // fn inverse(&self) -> Option<Self> { Some(-self) }
            // #[cfg(feature="std")]
            // fn abs(&self) -> Option<Self> { Some(<$ty>::abs(*self)) }
            // #[cfg(not(feature="std"))]
            // fn abs(&self) -> Option<Self> {
            //     if *self < 0.0 { Some(-*self) } else { Some(*self) }
            // }
        }
    };
}
impl_sign![all_signed: i8, i16, i32, i64, i128, isize];
impl_sign![all_float: f32, f64];

impl_sign![all_unsigned: u8, u16, u32, u64, u128, usize];

/// Implements `Sign` for TwoFloat
#[rustfmt::skip]
#[cfg(feature = "twofloat")]
mod impl_twofloat {
    use super::Signed;
    use twofloat::TwoFloat;
    impl Signed for TwoFloat {
        fn is_negative(&self) -> bool { self.is_sign_negative() && *self != 0.0 }
        fn is_positive(&self) -> bool { self.is_sign_positive() && *self != 0.0 }
    }
}

/// Implements `Sign` for bf16 & f16.
#[cfg(feature = "half")]
mod impl_half {
    use super::Signed;
    use half::{bf16, f16};
    macro_rules! impl_sign {
        ($($ty:ty),+) => {
            $(
            impl Signed for $ty {
                fn is_negative(&self) -> bool {
                    self.is_sign_negative() && *self != <$ty>::from_f32_const(0.0)
                }
                fn is_positive(&self) -> bool {
                    self.is_sign_positive() && *self != <$ty>::from_f32_const(0.0)
                }
            }
            )+
        };
    }
    impl_sign![bf16, f16];
}

/// Implements `Sign` for IBig & UBig.
#[rustfmt::skip]
#[cfg(feature = "ibig")]
mod impl_ibig {
    use super::{Signed, Unsigned};
    use ibig::{/*ops::Abs, */IBig, UBig};

    impl Unsigned for UBig { }
    impl Signed for IBig {
        fn is_negative(&self) -> bool { *self < IBig::from(0u8) }
        fn is_positive(&self) -> bool { *self > IBig::from(0u8) }
    }
}
