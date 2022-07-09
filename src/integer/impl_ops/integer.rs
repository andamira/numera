// numera::integer::impl_ops::integer
//
//! implements the arithmetic operations on `Integer`.

use crate::traits::{
    CheckedAdd, CheckedDiv, CheckedMul, CheckedNeg, CheckedRem, CheckedSub, NumberAble,
};
use crate::Integer;
use core::ops::{Add, Div, Mul, Neg, Rem, Sub};

// Core

macro_rules! impl_core_ops {
    ($trait:ident, $method:ident, $ty:ident) => {
        impl_core_ops![unary: ]
    };
    (unary: $trait:ident, $method:ident, $ty:ident, $($bounds:tt)+) => {
        impl <I: $($bounds)+> $trait for $ty<I> {
            type Output = Self;
            fn $method(self) -> Self {
                Self(self.0.$method())
            }
        }
    };
    (binary: $trait:ident, $method:ident, $ty:ident, $($bounds:tt)+) => {
        impl <I: $($bounds)+> $trait for $ty<I> {
            type Output = Self;
            fn $method(self, other: Self) -> Self::Output {
                Self(self.0.$method(other.0))
            }
        }
    };
}
impl_core_ops![unary: Neg, neg, Integer, NumberAble + Neg<Output = I>];
impl_core_ops![binary: Add, add, Integer, NumberAble + Add<Output = I>];
impl_core_ops![binary: Sub, sub, Integer, NumberAble + Sub<Output = I>];
impl_core_ops![binary: Mul, mul, Integer, NumberAble + Mul<Output = I>];
impl_core_ops![binary: Div, div, Integer, NumberAble + Div<Output = I>];
impl_core_ops![binary: Rem, rem, Integer, NumberAble + Rem<Output = I>];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Number;

    #[test]
    fn core() {
        let im3 = Integer::new(-3);
        let i5 = Integer::new(5);

        assert_eq![im3 + i5, Integer::new(2)]; // Add
        assert_eq![-im3, Integer::new(3)]; // Neg
    }
}
