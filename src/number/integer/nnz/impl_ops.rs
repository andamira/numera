// numera::number::integer::nnz::impl_ops
//
//!
//

use super::*;
use core::ops::{Add, Div, Mul, Rem, Sub};

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
}

impl_integer_ops![
    NonNegativeInteger8,
    NonNegativeInteger16,
    NonNegativeInteger32,
    NonNegativeInteger64,
    NonNegativeInteger128
];

#[cfg(test)]
mod tests {
    use crate::all::*;

    #[test]
    fn nnz_ops() -> NumeraResult<()> {
        let _5 = NonNegativeInteger8::from_parts(5)?;
        let _7 = NonNegativeInteger8::from_parts(7)?;
        assert_eq![_7 + _5, NonNegativeInteger8::from_parts(12)?];
        assert_eq![_7 - _5, NonNegativeInteger8::from_parts(2)?];
        assert_eq![_7 * _5, NonNegativeInteger8::from_parts(35)?];
        assert_eq![_7 / _5, NonNegativeInteger8::from_parts(1)?];
        assert_eq![_5 / _7, NonNegativeInteger8::from_parts(0)?];

        #[cfg(feature = "std")]
        {
            use std::panic::catch_unwind;

            // negative
            assert![catch_unwind(|| _5 - _7).is_err()];
            // overflow
            assert![catch_unwind(|| _7 * _7 * _7).is_err()];
            // underflow
            assert![catch_unwind(|| NonNegativeInteger8::MIN - _5).is_err()];
        }
        Ok(())
    }
}
