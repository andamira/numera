// numera::number::integer::n0z::impl_ops
//
//!
//

use super::*;
use core::{
    num::{NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8},
    ops::{Add, Div, Mul, Neg, Rem, Sub},
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
        impl_integer_ops![bin_ops: $t, $inner, Add, add, Sub, sub, Mul, mul, Div, div, Rem, rem];
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

#[cfg(test)]
mod tests {
    use crate::all::{abbr::*, *};

    #[test]
    fn n0z_ops() -> NumeraResult<()> {
        let _5 = N0z8::from_parts(5)?;
        let _7 = N0z8::from_parts(7)?;

        assert_eq![_7 + _5, N0z8::from_parts(12)?];
        assert_eq![_7 - _5, N0z8::from_parts(2)?];
        assert_eq![_5 - _7, N0z8::from_parts(-2)?];
        assert_eq![_7 * _5, N0z8::from_parts(35)?];
        assert_eq![_7 / _5, N0z8::from_parts(1)?];
        assert_eq![-_7, N0z8::from_parts(-7)?];

        #[cfg(feature = "std")]
        {
            use std::panic::catch_unwind;
            // overflow
            assert![catch_unwind(|| _7 * _7 * _7).is_err()];
            // underflow
            assert![catch_unwind(|| N0z8::MIN - _5).is_err()];
            // zero
            assert![catch_unwind(|| _5 / _7).is_err()];
        }
        Ok(())
    }
}
