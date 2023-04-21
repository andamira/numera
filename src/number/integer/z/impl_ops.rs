// numera::integer::z::impl_ops
//
//!
//

use super::*;
use core::ops::{Add, Div, Mul, Neg, Rem, Sub};

// impl ops (which panic on overflow)
macro_rules! impl_integer_ops {
    // impl all ops for multiple integer types
    ($($t:ident),+) => {
        $(
            impl_integer_ops![ops: $t];
        )+
    };

    // impl all ops for a single integer type
    //
    // $t: integer type. e.g. Integer8
    (ops: $t:ident) => {
        impl_integer_ops![bin_ops: $t, Add, add, Sub, sub, Mul, mul, Div, div, Rem, rem];
        impl_integer_ops![un_op: $t, Neg, neg];
    };

    // impl multiple binary ops for a single integer type
    //
    // $t: integer type. e.g. Integer8
    // $(
    //   $op: operation. e.g. Add
    //   $fn: operation fn. e.g. add
    // )
    (bin_ops: $t:ident, $($op:ident, $fn:ident),+) => {
        $(
            impl_integer_ops![bin_op: $t, $op, $fn];
        )+
    };

    // impl a binary op for a single integer type
    //
    // $t: integer type. e.g. Integer8
    // $op: operation. e.g. Add
    // $fn: operation fn. e.g. add
    (bin_op: $t:ident, $op:ident, $fn:ident) => {
        impl $op for $t {
            type Output = $t;

            fn $fn(self, rhs: Self::Output) -> Self::Output {
                $t(self.0.$fn(rhs.0))
            }
        }
    };

    // impl a unary op for a single integer type
    //
    // $t: integer type. e.g. Integer8
    // $op: operation. e.g. Neg
    // $fn: operation fn. e.g. neg
    (un_op: $t:ident, $op:ident, $fn:ident) => {
        impl $op for $t {
            type Output = $t;

            fn $fn(self) -> Self::Output {
                $t(self.0.$fn())
            }
        }
    };
}
impl_integer_ops![Integer8, Integer16, Integer32, Integer64, Integer128];
