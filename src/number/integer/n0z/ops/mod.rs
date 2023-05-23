// numera::number::integer::n0z::ops
//
//!
//

#[cfg(test)]
mod tests;

mod add;

// TODO
// mod div_rem;
// mod mul;
// mod neg;
// mod sub;

// OLD:

use super::*;
use core::{
    num::{NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8},
    ops::{Div, Mul, Neg, Rem, Sub},
};

// impl ops (will panic on overflow, and when result is 0)
macro_rules! impl_integer_ops {
    // impl all ops for multiple integer types
    ($($t:ident, $inner:ident),+) => {
        $(
            impl_integer_ops![ops: $t, $inner];
        )+
    };

    // impl all ops for a single integer type
    //
    // $t: integer type. e.g. Integer8
    (ops: $t:ident, $inner:ident) => {
        impl_integer_ops![bin_ops: $t, $inner, Sub, sub, Mul, mul, Div, div, Rem, rem];
        impl_integer_ops![un_op: $t, $inner, Neg, neg];
    };

    // impl multiple binary ops for a single integer type
    //
    // $t: integer type. e.g. Integer8
    // $(
    //   $op: operation. e.g. Add
    //   $fn: operation fn. e.g. add
    // )
    (bin_ops: $t:ident, $inner:ident, $($op:ident, $fn:ident),+) => {
        $(
            impl_integer_ops![bin_op: $t, $inner, $op, $fn];
        )+
    };

    // impl a binary op for a single integer type
    //
    // $t: integer type. e.g. NonZeroInteger8
    // $inner: inner primitive type. e.g. NonZeroI8
    // $op: operation. e.g. Add
    // $fn: operation fn. e.g. add
    (bin_op: $t:ident, $inner:ident, $op:ident, $fn:ident) => {
        impl $op for $t {
            type Output = $t;

            fn $fn(self, rhs: Self::Output) -> Self::Output {
                $t($inner::new(self.0.get().$fn(rhs.0.get())).expect("Invalid value 0."))
            }
        }
    };

    // impl a unary op for a single integer type
    //
    // $t: integer type. e.g. Integer8
    // $op: operation. e.g. Neg
    // $fn: operation fn. e.g. neg
    (un_op: $t:ident, $inner:ident, $op:ident, $fn:ident) => {
        impl $op for $t {
            type Output = $t;

            fn $fn(self) -> Self::Output {
                $t($inner::new(self.0.get().$fn()).expect("Invalid value 0."))
            }
        }
    };
}
impl_integer_ops![
    NonZeroInteger8,
    NonZeroI8,
    NonZeroInteger16,
    NonZeroI16,
    NonZeroInteger32,
    NonZeroI32,
    NonZeroInteger64,
    NonZeroI64,
    NonZeroInteger128,
    NonZeroI128
];
