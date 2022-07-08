// numera::number::traits::bounded
//
//! defines arithmetic traits operations
//! and implements them for primitive and supported external types.
//

use core::ops::{Add, Div, Mul, Neg, Rem, Sub};

// Checked

/// Performs addition, returning `None` on overflow.
pub trait CheckedAdd: Sized + Add<Self, Output = Self> {
    /// Adds two numbers. Returns `None` on overflow.
    fn checked_add(&self, v: &Self) -> Option<Self>;
}

/// Performs substraction, returning `None` on underflow.
pub trait CheckedSub: Sized + Sub<Self, Output = Self> {
    /// Substracts two numbers. Returns `None` on underflow.
    fn checked_sub(&self, v: &Self) -> Option<Self>;
}

/// Performs multiplication, returning `None` on underflow or overflow.
pub trait CheckedMul: Sized + Mul<Self, Output = Self> {
    /// Adds two numbers. Returns `None` on underflow or overflow.
    fn checked_mul(&self, v: &Self) -> Option<Self>;
}

/// Performs division, returning `None` on underflow, overflow or division by 0.
pub trait CheckedDiv: Sized + Div<Self, Output = Self> {
    /// Divides two numbers. Returns `None` on underflow, overflow or division by 0.
    fn checked_div(&self, v: &Self) -> Option<Self>;
}

/// Performs negation, returning `None` if the value can't be represented.
pub trait CheckedNeg: Sized + Neg {
    /// Negates a number. Returns `None` if the value can't be represented.
    fn checked_neg(&self) -> Option<Self>;
}

/// Performs integral remainder, returning `None` on underflow, overflow
/// or division by 0.
pub trait CheckedRem: Sized + Rem<Self, Output = Self> {
    /// Finds the remainder of two numbers. Returns `None` on underflow,
    /// overflow or division by 0.
    fn checked_rem(&self, v: &Self) -> Option<Self>;
}

macro_rules! impl_checked {
    (all_primitives: $trait:ident, $method:ident) => {
        impl_checked![all: $trait, $method,
            i8, u8, i16, u16, i32, u32, i64, u64, isize, usize, i128, u128
        ];
    };
    (all: $trait:ident, $method:ident, $($ty:ty),+) => {
        $( impl_checked![$trait, $method, $ty]; )+
    };
    ($trait:ident, $method:ident, $ty:ty) => {
        impl $trait for $ty {
            #[inline]
            fn $method(&self, v: &$ty) -> Option<$ty> {
                <$ty>::$method(*self, *v)
            }
        }
    };
}

#[rustfmt::skip]
impl_checked![all_primitives: CheckedAdd, checked_add];

// Inv

