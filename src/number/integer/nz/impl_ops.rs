// numera::number::integer::nz::impl_ops
//
//!
//

use super::*;
use core::{
    num::{NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8},
    ops::{Add, Rem, Sub}, // Div, Mul, Neg
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
    // $t: integer type. e.g. NegativeInteger8
    (ops: $t:ident, $inner:ident) => {
        impl_integer_ops![bin_ops: $t, $inner, Add, add, Sub, sub, Rem, rem];
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
    // $t: integer type. e.g. NegativeInteger8
    // $inner: inner primitive type. e.g. NonZeroU8
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
    NegativeInteger8,
    NonZeroU8,
    NegativeInteger16,
    NonZeroU16,
    NegativeInteger32,
    NonZeroU32,
    NegativeInteger64,
    NonZeroU64,
    NegativeInteger128,
    NonZeroU128
];

#[cfg(test)]
mod tests {
    use crate::all::*;

    #[test]
    fn nz_ops() -> NumeraResult<()> {
        assert![NegativeInteger8::new(5).is_err()];
        let _n5 = NegativeInteger8::new_neg(5)?;
        let _n7 = NegativeInteger8::new_neg(7)?;

        assert_eq![_n7 + _n5, NegativeInteger8::new_neg(12)?];
        assert_eq![_n7 - _n5, NegativeInteger8::new_neg(2)?];

        #[cfg(feature = "std")]
        {
            use std::panic::catch_unwind;

            // positive
            assert![catch_unwind(|| _n5 - _n7).is_err()];
            // zero
            assert![catch_unwind(|| _n7 - _n7).is_err()];
        }
        Ok(())
    }
    #[test]
    #[should_panic]
    fn nz_ops_panic_underflow() {
        let _min = NegativeInteger8::MIN;
        let _ = _min + NegativeInteger8::new_neg(8).expect("new_neg(8)");
    }
    #[test]
    #[should_panic]
    fn nz_ops_panic_overflow() {
        let _max = NegativeInteger8::MAX;
        let _ = _max - NegativeInteger8::new_neg(9).expect("new_neg(9)");
    }
}
