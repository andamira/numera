// numera::number::integer::z::impl_ops::neg
//
//! Implement the negation operations.
//

use crate::number::integer::*;
use core::ops::Neg;
use devela::paste;

macro_rules! impl_integer_neg {
    // impl Add ops for multiple integer types
    //
    // # Args
    // $t: integer base name. e.g. Integer
    // $p: inner primitive base name. e.g. i
    // $b: integer and primitive bitsize. e.g. 8
    ( $($t:ident + $p:ident + $b:literal, cast: $bcast:literal);+ ) => {
        $(
            impl_integer_neg![neg: $t + $p + $b];
        )+
    };

    // impl the negation operations
    //
    // impl variants:
    // - op
    // - wrapping
    (neg: $t:ident + $p:ident + $b:literal) => { paste! {
        impl Neg for [<$t$b>] {
            type Output = [<$t$b>];
            /// Performs the unary `-` operation.
            #[inline]
            fn neg(self) -> Self::Output {
                self.op_neg()
            }
        }
        /// # Integer negation
        impl [<$t$b>] {
            /// Basic negation.
            #[inline]
            #[must_use]
            pub const fn op_neg(self) -> [<$t$b>] {
                [<$t$b>](-self.0)
            }

            /// Wrapping negation.
            #[inline]
            #[must_use]
            pub const fn wrapping_neg(self) -> [<$t$b>] {
                Self(self.0.wrapping_neg())
            }
        }
    }};
}
impl_integer_neg![
    Integer+i+8, cast:16;
    Integer+i+16, cast:32;
    Integer+i+32, cast:64;
    Integer+i+64, cast:128;
    Integer+i+128, cast:128
];
