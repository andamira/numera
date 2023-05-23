// numera::number::integer::npz::ops
//
//!
//

use super::*;
use core::ops::{Add, Rem, Sub}; // Div, Mul, Neg

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
    // $t: integer type. e.g. NonPositiveInteger8
    (ops: $t:ident) => {
        impl_integer_ops![bin_ops: $t, Add, add, Sub, sub, Rem, rem];
    };

    // impl multiple binary ops for a single integer type
    //
    // $t: integer type. e.g. NonPositiveInteger8
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
    // $t: integer type. e.g. NonPositiveInteger8
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
    NonPositiveInteger8,
    NonPositiveInteger16,
    NonPositiveInteger32,
    NonPositiveInteger64,
    NonPositiveInteger128
];

#[cfg(test)]
mod tests {
    use crate::all::*;

    #[test]
    fn npz_ops() -> NumeraResult<()> {
        // assert![NonPositiveInteger8::from_parts(5).is_err()]; // not an error currently
        let _n5 = NonPositiveInteger8::new_neg(5);
        let _n7 = NonPositiveInteger8::new_neg(7);

        assert_eq![_n7 + _n5, Npz8::new_neg(12)];
        assert_eq![_n7 - _n5, Npz8::new_neg(2)];
        assert_eq![_n7 - _n7, Npz8::new_neg(0)];

        #[cfg(feature = "std")]
        {
            use std::panic::catch_unwind;

            // positive
            assert![catch_unwind(|| _n5 - _n7).is_err()];
        }
        Ok(())
    }
    #[test]
    #[should_panic]
    fn npz_ops_panic_underflow() {
        let _min = Npz8::MIN;
        let _ = _min + Npz8::new_neg(8);
    }
    #[test]
    #[should_panic]
    fn npz_ops_panic_overflow() {
        let _max = Npz8::MAX;
        let _ = _max - Npz8::new_neg(9);
    }
}
