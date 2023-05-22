// numera::number::integer::z::impl_ops::sub
//
//! Implement the substraction operations.
//

use crate::number::integer::*;
use core::ops::{Sub, SubAssign};
use devela::paste;

macro_rules! impl_integer_sub {
    // impl Add ops for multiple integer types
    //
    // # Args
    // $t: integer base name. e.g. Integer
    // $p: inner primitive base name. e.g. i
    // $b: integer and primitive bitsize. e.g. 8
    ( $($t:ident + $p:ident + $b:literal, cast: $bcast:literal);+ ) => {
        $(
            impl_integer_sub![sub: $t + $p + $b];
        )+
    };

    // substraction operations
    //
    // impl variants:
    // - basic
    // - checked
    // - saturating TODO
    // - wrapping TODO
    // - overflowing TODO
    // - modular TEST
    // - modular_counting TODO
    (sub: $t:ident + $p:ident + $b:literal) => { paste! {
        impl Sub<[<$t$b>]> for [<$t$b>] {
            type Output = [<$t$b>];
            /// Performs the `-` operation.
            ///
            /// # Panics
            /// In debug, on overflow.
            ///
            /// In release, it performs two's complement wrapping.
            #[inline]
            fn sub(self, rhs: [<$t$b>]) -> Self::Output {
                self.basic_sub(rhs)
            }
        }
        impl SubAssign for [<$t$b>] {
            /// Performs the `-=` operation.
            ///
            /// # Panics
            /// In debug, on overflow.
            ///
            /// In release, it performs two's complement wrapping.
            #[inline]
            fn sub_assign(&mut self, rhs: [<$t$b>]) {
                self.0 -= rhs.0
            }
        }
        /// # Integer substraction
        impl [<$t$b>] {
            /// Integer substraction.
            ///
            /// # Panics
            /// If the substraction results in overflow.
            #[inline]
            #[must_use]
            pub const fn basic_sub(self, rhs: [<$t$b>]) -> [<$t$b>] {
                Self(self.0 - rhs.0)
            }

            /// Checked substraction.
            #[inline]
            #[must_use]
            pub const fn checked_sub(self, rhs: [<$t$b>]) -> Option<[<$t$b>]> {
                if let Some(result) = self.0.checked_sub(rhs.0) {
                    Some(Self(result))
                } else {
                    None
                }
            }

            /// Saturating substraction.
            /// Computes `self + rhs`, saturating at the numeric bounds instead of overflowing.
            #[inline]
            #[must_use]
            pub const fn saturating_sub(self, rhs: [<$t$b>]) -> [<$t$b>] {
                Self(self.0.saturating_sub(rhs.0))
            }

            /// Wrapping (modular) substraction.
            /// Computes `self + rhs`, wrapping around at the boundary of the type.
            #[inline]
            #[must_use]
            pub const fn wrapping_sub(self, rhs: [<$t$b>]) -> [<$t$b>] {
                Self(self.0.wrapping_sub(rhs.0))
            }

            /// Overflowing substraction.
            ///
            /// Returns a tuple of the substraction along with a boolean indicating
            /// whether an arithmetic overflow would occur. If an overflow would
            /// have occurred then the wrapped value is returned.
            #[inline]
            #[must_use]
            pub const fn overflowing_sub(self, rhs: [<$t$b>]) -> ([<$t$b>], bool) {
                let (result, overflown) = self.0.overflowing_sub(rhs.0);
                (Self(result), overflown)
            }

            // /// Modular subtraction with a custom `modulo`.
            // #[inline]
            // #[must_use]
            // pub const fn modular_sub(self, rhs: [<$t$b>], modulo: [<$t$b>]) -> [<$t$b>] {
            //     self.basic_sub(rhs).rem_euclid(modulo)
            //
            // }
        }
    }};
}
impl_integer_sub![
    Integer+i+8, cast:16;
    Integer+i+16, cast:32;
    Integer+i+32, cast:64;
    Integer+i+64, cast:128;
    Integer+i+128, cast:128
];
