// numera::number::integer::z::impl_ops::add
//
//! Implement the addition operations, and the Sum trait.
//

use crate::number::integer::*;
use core::{
    iter::Sum,
    ops::{Add, AddAssign},
};
use devela::paste;

macro_rules! impl_integer_add {
    // impl Add ops for multiple integer types
    //
    // # Args
    // $t: integer base name. e.g. Integer
    // $p: inner primitive base name. e.g. i
    // $b: integer and primitive bitsize. e.g. 8
    ( $($t:ident + $p:ident + $b:literal, cast: $bcast:literal);+ ) => {
        $(
            impl_integer_add![add: $t + $p + $b];
        )+
    };

    // addition operations
    //
    // impl variants:
    // - basic
    // - checked
    // - saturating
    // - wrapping
    // - overflowing
    // - modular TODO TEST
    // - modular_counting TODO TEST
    (add: $t:ident + $p:ident + $b:literal) => { paste! {
        impl Add<[<$t$b>]> for [<$t$b>] {
            type Output = [<$t$b>];
            /// Performs the `+` operation.
            ///
            /// # Panics
            /// In debug, on overflow.
            ///
            /// In release, it performs two's complement wrapping.
            #[inline]
            fn add(self, rhs: [<$t$b>]) -> Self::Output {
                self.basic_add(rhs)
            }
        }
        impl AddAssign for [<$t$b>] {
            /// Performs the `+=` operation.
            ///
            /// # Panics
            /// In debug, on overflow.
            ///
            /// In release, it performs two's complement wrapping.
            #[inline]
            fn add_assign(&mut self, rhs: [<$t$b>]) {
                self.0 += rhs.0
            }
        }
        /// # Integer addition
        impl [<$t$b>] {
            /// Integer addition.
            ///
            /// # Panics
            /// If the addition results in overflow.
            #[inline]
            #[must_use]
            pub const fn basic_add(self, rhs: [<$t$b>]) -> [<$t$b>] {
                Self(self.0 + rhs.0)
            }

            /// Checked addition.
            #[inline]
            #[must_use]
            pub const fn checked_add(self, rhs: [<$t$b>]) -> Option<[<$t$b>]> {
                if let Some(result) = self.0.checked_add(rhs.0) {
                    Some(Self(result))
                } else {
                    None
                }
            }

            /// Saturating addition.
            /// Computes `self + rhs`, saturating at the numeric bounds instead of overflowing.
            #[inline]
            #[must_use]
            pub const fn saturating_add(self, rhs: [<$t$b>]) -> [<$t$b>] {
                Self(self.0.saturating_add(rhs.0))
            }

            /// Wrapping (modular) addition.
            /// Computes `self + rhs`, wrapping around at the boundary of the type.
            #[inline]
            #[must_use]
            pub const fn wrapping_add(self, rhs: [<$t$b>]) -> [<$t$b>] {
                Self(self.0.wrapping_add(rhs.0))
            }

            /// Overflowing addition.
            ///
            /// Returns a tuple of the addition along with a boolean indicating
            /// whether an arithmetic overflow would occur. If an overflow would
            /// have occurred then the wrapped value is returned.
            #[inline]
            #[must_use]
            pub const fn overflowing_add(self, rhs: [<$t$b>]) -> ([<$t$b>], bool) {
                let (result, overflown) = self.0.overflowing_add(rhs.0);
                (Self(result), overflown)
            }

            // /// Modular addition with a custom `modulo`.
            // #[inline]
            // #[must_use]
            // pub const fn modular_add(self, rhs: [<$t$b>], modulo: [<$t$b>]) -> [<$t$b>] {
            //     // self.wrapping_add(rhs).op_rem(modulo) // CHECK wrapping
            //
            //     // use core::num::Wrapping; // MAYBE?
            //     // let wrapped_sum = Wrapping(self.0).wrapping_add(Wrapping(rhs.0));
            //
            //     // let wrapped_sum = self.0.wrapping_add(rhs.0);
            //     // Self((wrapped_sum % modulo.0 + modulo.0) % modulo.0)
            //
            //     // TEST
            //     Self(self.0.wrapping_add(rhs.0) % modulo.0)
            // }

            // TEST
            // /// Addition with a custom `modulo`, and counting of the number of wraps.
            // ///
            // /// Returns the wrapped result and the number of times the modulo has wrapped around.
            // #[inline]
            // #[must_use]
            // pub fn modular_counting_add(&self, other: [<$t$b>], modulo: [<$t$b>]) -> ([<$t$b>], [<$t$b>]) {
            //     let modulo_abs = modulo.0.abs();
            //     let mut result = self.0 + other.0;
            //     if result < 0 {
            //         result += modulo_abs;
            //     }
            //     let count = result / modulo_abs;
            //     (Self(result), Self(count))
            // }
        }
    }};
}
impl_integer_add![
    Integer+i+8, cast:16;
    Integer+i+16, cast:32;
    Integer+i+32, cast:64;
    Integer+i+64, cast:128;
    Integer+i+128, cast:128
];
