// numera::traits::define_sign
//
//! defines the `Signed` and `Unsigned` traits.
//! and implements them for the primitives and supported external types.
//

/// Can represent both positive and negative numbers.
#[rustfmt::skip]
pub trait Signed {
    /// The value is > `0`.
    fn is_positive(&self) -> bool;
    /// The value is < `0`.
    fn is_negative(&self) -> bool;
}

/// Can *not* represent negative numbers.
pub trait Unsigned {}

/// Can *not* represent positive numbers.
pub trait NegSigned {
    /// The number's [`Inner`][crate::Number::Inner] value representation.
    type Inner;
    /// Returns a new number that contains the negation of the `value`.
    ///
    /// This allows using an unsigned type value to store only negative numbers.
    fn new_neg(value: Self::Inner) -> Self;
}

/// Implements `Signed` on primitives
macro_rules! impl_sign {
    // Primitives that can not be negative.
    (all_unsigned: $($t:ty),+) => {
        $( impl_sign![unsigned: $t]; )+
    };
    (unsigned: $t:ty) => {
        impl Unsigned for $t { }
    };
    // Primitives that can be both positive and negative.
    (all_signed: $($t:ty),+) => {
        $( impl_sign![signed: $t]; )+
    };
    (signed: $t:ty) => {
        impl Signed for $t {
            fn is_negative(&self) -> bool { <$t>::is_negative(*self) }
            fn is_positive(&self) -> bool { <$t>::is_positive(*self) }
        }
    };
    // Floating point primitives that can be both positive and negative.
    (all_float: $($t:ty),+) => {
        $( impl_sign![float: $t]; )+
    };
    (float: $t:ty) => {
        impl Signed for $t {
            // −0.0 = +0.0
            fn is_negative(&self) -> bool { self.is_sign_negative() && *self != 0.0 }
            fn is_positive(&self) -> bool { self.is_sign_positive() && *self != 0.0 }
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
        ($($t:ty),+) => {
            $(
            impl Signed for $t {
                fn is_negative(&self) -> bool {
                    self.is_sign_negative() && *self != <$t>::from_f32_const(0.0)
                }
                fn is_positive(&self) -> bool {
                    self.is_sign_positive() && *self != <$t>::from_f32_const(0.0)
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

#[cfg(test)]
mod tests {
    use super::*;
    use static_assertions::*;

    /// Checks types only have one trait
    #[test]
    fn only_one_sign_trait() {
        macro_rules! assert_impl1 {
            (all: $($t:ty),+) => {
                $( assert_impl_one![$t: NegSigned, Signed, Unsigned]; )+
            };
        }
        #[rustfmt::skip]
        assert_impl1![all:
            f32, f64, i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize];
    }
}
