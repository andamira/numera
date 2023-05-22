// numera::number::integer::z::impl_ops::mul
//
//! Implement the multiplication operations, and the Product trait.
//

use crate::number::{integer::*, traits::ConstOne};
use core::{
    iter::Product,
    ops::{Mul, MulAssign},
};
use devela::paste;

macro_rules! impl_integer_mul {
    // impl Add ops for multiple integer types
    //
    // # Args
    // $t: integer base name. e.g. Integer
    // $p: inner primitive base name. e.g. i
    // $b: integer and primitive bitsize. e.g. 8
    ( $($t:ident + $p:ident + $b:literal, cast: $bcast:literal);+ ) => {
        $(
            impl_integer_mul![mul: $t + $p + $b];
        )+
    };

    // multiplication operations
    //
    // impl variants:
    // - basic_mul
    // - checked_mul
    // - saturating_mul TODO
    // - wrapping_mul TODO
    // - overflowing_mul TODO
    // - modular_mul TEST
    // - modular_counting_mul TODO
    (mul: $t:ident + $p:ident + $b:literal) => { paste! {
        impl Mul<[<$t$b>]> for [<$t$b>] {
            /// Performs the `*` operation.
            ///
            /// # Panics
            /// In debug, on overflow.
            ///
            /// In release, it performs two's complement wrapping.
            type Output = [<$t$b>];
            #[inline]
            fn mul(self, rhs: [<$t$b>]) -> Self::Output {
                self.basic_mul(rhs)
            }
        }
        impl MulAssign for [<$t$b>] {
            /// Performs the `*=` operation.
            ///
            /// # Panics
            /// In debug, on overflow.
            ///
            /// In release, it performs two's complement wrapping.
            #[inline]
            fn mul_assign(&mut self, rhs: [<$t$b>]) {
                self.0 *= rhs.0
            }
        }

        impl Product for [<$t$b>] {
            fn product<I: Iterator<Item=Self>>(iter: I) -> Self {
                iter.fold(
                    [<$t$b>]::ONE,
                    |a, b| a * b,
                )
            }
        }
        impl<'a> Product<&'a [<$t$b>]> for [<$t$b>] {
            fn product<I: Iterator<Item=&'a Self>>(iter: I) -> Self {
                iter.fold(
                    [<$t$b>]::ONE,
                    |a, b| a * *b,
                )
            }
        }

        /// # Integer multiplication
        impl [<$t$b>] {
            /// Integer multiplication.
            ///
            /// # Panics
            /// If the multiplication results in overflow.
            #[inline]
            #[must_use]
            pub const fn basic_mul(self, rhs: [<$t$b>]) -> [<$t$b>] {
                Self(self.0 * rhs.0)
            }

            /// Checked integer multiplication.
            #[inline]
            #[must_use]
            pub const fn checked_mul(self, rhs: [<$t$b>]) -> Option<[<$t$b>]> {
                if let Some(result) = self.0.checked_mul(rhs.0) {
                    Some(Self(result))
                } else {
                    None
                }
            }

            // /// Modular multiplication with a custom `modulo`.
            // #[inline]
            // #[must_use]
            // // TEST
            // pub const fn modular_mul(self, rhs: [<$t$b>], modulo: [<$t$b>]) -> [<$t$b>] {
            //     self.basic_mul(rhs).rem_euclid(modulo)
            //
            // }
        }
    }};

}
impl_integer_mul![
    Integer+i+8, cast:16;
    Integer+i+16, cast:32;
    Integer+i+32, cast:64;
    Integer+i+64, cast:128;
    Integer+i+128, cast:128
];
