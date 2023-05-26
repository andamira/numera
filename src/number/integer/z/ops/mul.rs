// numera::number::integer::z::ops::mul
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
    // impl Mul ops for multiple integer types
    //
    // # Args
    // $t: integer base name. e.g. Integer
    // $p: inner primitive base name. e.g. i
    // $b: integer and primitive bitsize. e.g. 8
    ( $($t:ident + $p:ident + $b:literal, cast: $bcast:literal);+ ) => {
        $( impl_integer_mul![mul: $t + $p + $b]; )+
    };

    // multiplication operations
    //
    // impl variants:
    // - mul
    // - checked_mul_
    // - saturating_mul_
    // - wrapping_mul_
    // - overflowing_mul_
    // - modular_mul_ TODO
    // - modular_counting_mul_ TODO
    (mul: $t:ident + $p:ident + $b:literal) => { paste! {
        impl Mul<[<$t$b>]> for [<$t$b>] {
            /// Performs the `*` operation.
            ///
            /// # Panics
            /// Panics in debug, on overflow.
            /// While in release, it performs two's complement wrapping.
            type Output = [<$t$b>];
            #[inline]
            fn mul(self, rhs: [<$t$b>]) -> Self::Output {
                self.mul(rhs)
            }
        }
        impl MulAssign for [<$t$b>] {
            /// Performs the `*=` operation.
            ///
            /// # Panics
            /// Panics in debug, on overflow.
            /// While in release, it performs two's complement wrapping.
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
            /// Panics in debug, on overflow.
            /// While in release, it performs two's complement wrapping.
            #[inline]
            #[must_use]
            pub const fn mul(self, rhs: [<$t$b>]) -> [<$t$b>] {
                Self(self.0 * rhs.0)
            }

            /// Checked integer multiplication.
            ///
            /// Returns `None` on overflow.
            #[inline]
            #[must_use]
            pub const fn checked_mul(self, rhs: [<$t$b>]) -> Option<[<$t$b>]> {
                if let Some(result) = self.0.checked_mul(rhs.0) {
                    Some(Self(result))
                } else {
                    None
                }
            }

            /// Saturating multiplication.
            /// Computes `self + rhs`, saturating at the numeric bounds instead of overflowing.
            #[inline]
            #[must_use]
            pub const fn saturating_mul(self, rhs: [<$t$b>]) -> [<$t$b>] {
                Self(self.0.saturating_mul(rhs.0))
            }

            /// Wrapping (modular) multiplication.
            /// Computes `self + rhs`, wrapping around at the boundary of the type.
            #[inline]
            #[must_use]
            pub const fn wrapping_mul(self, rhs: [<$t$b>]) -> [<$t$b>] {
                Self(self.0.wrapping_mul(rhs.0))
            }

            /// Overflowing multiplication.
            ///
            /// Returns a tuple of the multiplication along with a boolean indicating
            /// whether an arithmetic overflow would occur. If an overflow would
            /// have occurred then the wrapped value is returned.
            #[inline]
            #[must_use]
            pub const fn overflowing_mul(self, rhs: [<$t$b>]) -> ([<$t$b>], bool) {
                let (result, overflown) = self.0.overflowing_mul(rhs.0);
                (Self(result), overflown)
            }

            // /// Modular multiplication with a custom `modulo`.
            // #[inline]
            // #[must_use]
            // // TEST
            // pub const fn modular_mul(self, rhs: [<$t$b>], modulo: [<$t$b>]) -> [<$t$b>] {
            //     self.mul(rhs).rem_euclid(modulo)
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

#[cfg(feature = "ibig")]
mod big {
    use super::*;

    impl Mul<IntegerBig> for IntegerBig {
        type Output = IntegerBig;
        /// Performs the `*` operation.
        #[inline]
        #[must_use]
        fn mul(self, rhs: IntegerBig) -> Self::Output {
            Self(self.0 * rhs.0)
        }
    }
    impl MulAssign for IntegerBig {
        /// Performs the `*=` operation.
        #[inline]
        fn mul_assign(&mut self, rhs: IntegerBig) {
            self.0 *= rhs.0;
        }
    }

    impl Product for IntegerBig {
        fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
            iter.fold(IntegerBig::new(1), |a, b| a * b)
        }
    }
    impl<'a> Product<&'a IntegerBig> for IntegerBig {
        fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
            iter.fold(
                IntegerBig::new(1),
                |a, b| a * b.clone(), // CHECK performance
            )
        }
    }
}
