// numera::number::integer::pz::ops
//
//!
//

use super::*;
use core::{
    num::{NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8},
    ops::{Add, Div, Mul, Rem, Sub},
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
}
impl_integer_ops![
    PositiveInteger8,
    NonZeroU8,
    PositiveInteger16,
    NonZeroU16,
    PositiveInteger32,
    NonZeroU32,
    PositiveInteger64,
    NonZeroU64,
    PositiveInteger128,
    NonZeroU128
];

#[cfg(test)]
mod tests {
    use crate::all::*;

    #[test]
    fn pz_ops() -> NumeraResult<()> {
        let _5 = PositiveInteger8::new(5)?;
        let _7 = PositiveInteger8::new(7)?;

        assert_eq![_7 + _5, PositiveInteger8::new(12)?];
        assert_eq![_7 - _5, PositiveInteger8::new(2)?];
        assert_eq![_7 * _5, PositiveInteger8::new(35)?];
        assert_eq![_7 / _5, PositiveInteger8::new(1)?];

        #[cfg(feature = "std")]
        {
            use std::panic::catch_unwind;
            // overflow
            assert![catch_unwind(|| _7 * _7 * _7).is_err()];
            // underflow
            assert![catch_unwind(|| PositiveInteger8::MIN - _5).is_err()];
            // zero
            assert![catch_unwind(|| _5 / _7).is_err()];
        }
        Ok(())
    }
}
