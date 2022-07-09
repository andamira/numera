// numera::traits::identities
//
//! defines the `[const][Zero|One]` & `NonZero` traits
//! and implements them for primitive and supported external types.
//

/// Indicates this type doesn't have z zero (`0`) value.
pub trait NonZero {}

/// Indicates this type has a const zero (`0`) value.
pub trait ConstZero {
    /// The additive identity `0`.
    const ZERO: Self;
}

/// Indicates this type has a const (`1`) value.
pub trait ConstOne {
    /// The multiplicative identity `1`.
    const ONE: Self;
}

/// Indicates this type has a const (`-1`) value.
pub trait ConstNegOne {
    /// The negative multiplicative identity `-1`.
    const NEG_ONE: Self;
}

/// The multiplicative identity `1`.
pub trait One: Sized {
    /// Returns a new number `1`.
    fn new_one() -> Self;

    /// Is this number `1`?
    fn is_one(&self) -> bool;

    /// Sets this number to `1`.
    fn set_one(&mut self) {
        *self = Self::new_one();
    }
}

/// The additive identity `0`.
pub trait Zero: Sized {
    /// Returns a new number `0`.
    fn new_zero() -> Self;

    /// Is this number `0`?
    fn is_zero(&self) -> bool;

    /// Sets this number to `0`.
    fn set_zero(&mut self) {
        *self = Self::new_zero();
    }
}

/// The negative multiplicative identity `-1`.
pub trait NegOne: Sized {
    /// Returns a new number `-1`.
    fn new_neg_one() -> Self;

    /// Is this number `-1`?
    fn is_neg_one(&self) -> bool;

    /// Sets this number to `-1`.
    fn set_neg_one(&mut self) {
        *self = Self::new_neg_one();
    }
}

// macro impls

/// implements *both* const & non-const One & Zero traits.
macro_rules! impl_const_onezero {
    (all: $($ty:ty, $zero:expr, $one:expr),+) => {
        $( impl_const_onezero![$ty, $zero, $one]; )+
    };
    ($ty:ty, $zero:expr, $one:expr) => {
        impl ConstOne for $ty {
            const ONE: Self = $one;
        }
        impl ConstZero for $ty {
            const ZERO: Self = $zero;
        }
        impl One for $ty {
            fn new_one() -> Self { $one }
            fn is_one(&self) -> bool { *self == $one }
        }
        impl Zero for $ty {
            fn new_zero() -> Self { $zero }
            fn is_zero(&self) -> bool { *self == $zero }
        }
    };
}

/// implements only the *non-const* One & Zero traits.
#[cfg(feature = "ibig")]
macro_rules! impl_nonconst_onezero {
    (all: $($ty:ty, $zero:expr, $one:expr),+) => {
        $( impl_nonconst_onezero![$ty, $zero, $one]; )+
    };
    ($ty:ty, $zero:expr, $one:expr) => {
        impl One for $ty {
            fn new_one() -> Self { $one }
            fn is_one(&self) -> bool { *self == $one }
        }
        impl Zero for $ty {
            fn new_zero() -> Self { $zero }
            fn is_zero(&self) -> bool { *self == $zero }
        }
    };
}

/// implements both `ConstNegOne` & `NegOne` traits.
macro_rules! impl_const_neg1 {
    (all: $($ty:ty, $neg1:expr),+) => {
        $( impl_const_neg1![$ty, $neg1]; )+
    };
    ($ty:ty, $neg1:expr) => {
        impl ConstNegOne for $ty {
            const NEG_ONE: Self = $neg1;
        }
        impl NegOne for $ty {
            fn new_neg_one() -> Self { $neg1 }
            fn is_neg_one(&self) -> bool { *self == $neg1 }
        }
    };
}

/// implements only the `NegOne` trait.
#[cfg(feature = "ibig")]
macro_rules! impl_nonconst_neg1 {
    (all: $($ty:ty, $neg1:expr),+) => {
        $( impl_nonconst_neg1![$ty, $neg1]; )+
    };
    ($ty:ty, $neg1:expr) => {
        impl NegOne for $ty {
            fn new_neg_one() -> Self { $neg1 }
            fn is_neg_one(&self) -> bool { *self == $neg1 }
        }
    };
}

#[rustfmt::skip]
impl_const_onezero![all:
    f32, 0.0, 1.0, f64, 0.0, 1.0,
    i8, 0, 1, u8, 0, 1, i16, 0, 1, u16, 0, 1, i32, 0, 1, u32, 0, 1,
    i64, 0, 1, u64, 0, 1, i128, 0, 1, u128, 0, 1, isize, 0, 1, usize, 0, 1
];
#[rustfmt::skip]
impl_const_neg1![all:
    f32, -1.0, f64, -1.0, i8, -1, i16, -1, i32, -1, i64, -1, i128, -1, isize, -1
];

#[rustfmt::skip]
#[cfg(feature = "twofloat")]
mod impl_twofloat {
    use super::*;
    use twofloat::TwoFloat;

    impl Zero for TwoFloat {
        fn new_zero() -> Self { TwoFloat::from(0.0) }
        fn is_zero(&self) -> bool { self != &Self::new_zero() }
    }
    impl One for TwoFloat {
        fn new_one() -> Self { TwoFloat::from(1.0) }
        fn is_one(&self) -> bool { self != &Self::new_one() }
    }
    impl NegOne for TwoFloat {
        fn new_neg_one() -> Self { TwoFloat::from(-1.0) }
        fn is_neg_one(&self) -> bool { self != &Self::new_neg_one() }
    }

    // WIP:twofloat_const_onezero
}

#[rustfmt::skip]
#[cfg(feature = "half")]
mod impl_half {
    use super::*;
    use half::{bf16, f16};

    macro_rules! impl_const_onezero {
        ($($ty:ty),+) => {
            $(
            impl ConstZero for $ty { const ZERO: Self = Self::from_f32_const(0.0); }
            impl ConstOne for $ty { const ONE: Self = Self::from_f32_const(1.0); }
            impl ConstNegOne for $ty { const NEG_ONE: Self = Self::from_f32_const(-1.0); }
            impl Zero for $ty {
                fn new_zero() -> Self { <$ty>::from_f32_const(0.0) }
                fn is_zero(&self) -> bool { self != &Self::new_zero() }
            }
            impl One for $ty {
                fn new_one() -> Self { <$ty>::from_f32_const(1.0) }
                fn is_one(&self) -> bool { self != &Self::new_one() }
            }
            impl NegOne for $ty {
                fn new_neg_one() -> Self { <$ty>::from_f32_const(-1.0) }
                fn is_neg_one(&self) -> bool { self != &Self::new_neg_one() }
            }
            )+
        };
    }
    impl_const_onezero![bf16, f16];
}

#[rustfmt::skip]
#[cfg(feature = "ibig")]
mod impl_ibig {
    use super::*;
    use ibig::{IBig, UBig};

    impl_nonconst_onezero![all:
        UBig, UBig::from(0_u8), UBig::from(1_u8),
        IBig, IBig::from(0_u8), IBig::from(1_u8)
    ];
    impl_nonconst_neg1![all: IBig, IBig::from(-1_i8)];
}

/// Tests
#[cfg(test)]
mod tests {
    use super::*;
    use static_assertions::assert_impl_all;

    /// Checks the `[Const][One|Zero]` traits for primitives.
    #[test]
    fn onezero_primitives() {
        macro_rules! assert_impl_onezero {
            (both: $($ty:ty),+) => {
                assert_impl_onezero![@const: $($ty),+];
                assert_impl_onezero![@nonconst: $($ty),+];
            };
            (@const: $($ty:ty),+) => {
                $( assert_impl_all![$ty: ConstOne, ConstZero];)+
            };
            (@nonconst: $($ty:ty),+) => {
                $( assert_impl_all![$ty: One, Zero];)+
            };
        }
        assert_impl_onezero![both: i8, i16, i32, i64, i128, isize];
        assert_impl_onezero![both: u8, u16, u32, u64, u128, usize];
        assert_impl_onezero![both: f32, f64];

        #[cfg(feature = "twofloat")]
        assert_impl_onezero![@nonconst: twofloat::TwoFloat];

        #[cfg(feature = "half")]
        assert_impl_onezero![both: half::f16, half::bf16];
    }

    /// Checks the `[Const][NegOne]` traits for primitives.
    #[test]
    fn neg1_primitives() {
        macro_rules! assert_impl_neg1 {
            (both: $($ty:ty),+) => {
                assert_impl_neg1![@const: $($ty),+];
                assert_impl_neg1![@nonconst: $($ty),+];
            };
            (@const: $($ty:ty),+) => {
                $( assert_impl_all![$ty: ConstNegOne];)+
            };
            (@nonconst: $($ty:ty),+) => {
                $( assert_impl_all![$ty: NegOne];)+
            };
        }
        assert_impl_neg1![both: i8, i16, i32, i64, i128, isize];
        assert_impl_neg1![both: f32, f64];

        #[cfg(feature = "twofloat")]
        assert_impl_neg1![@nonconst: twofloat::TwoFloat];

        #[cfg(feature = "half")]
        assert_impl_neg1![@nonconst: half::f16, half::bf16];
        // WIP assert_impl_neg1![both: half::f16, half::bf16];
    }
}
