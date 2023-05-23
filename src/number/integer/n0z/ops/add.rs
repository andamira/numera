// numera::number::integer::n0z::ops::add
//
//! Implement the addition operations, and the Sum trait.
//!
//! This is a tricky operation for a signed non-zero integer, since addittion
//! can result in 0. That's why there are only checked versions of non-basic
//! addition operations.
//!
//! We also can't use the additive identity 0.
//

use crate::number::integer::*;
use core::{
    iter::Sum,
    num::*,
    ops::{Add, AddAssign},
};
use devela::paste;

macro_rules! impl_integer_add {
    // impl Add ops for multiple integer types
    //
    // # Args
    // $t: integer base name. e.g. NonZeroInteger
    // $p: inner primitive base name. e.g. NonZeroI
    // $b: integer and primitive bitsize. e.g. 8
    ( $($t:ident + $p:ident + $b:literal, cast: $bcast:literal);+ ) => {
        $( impl_integer_add![add: $t + $p + $b]; )+
    };

    // addition operations
    //
    // impl variants:
    // - basic
    // - checked
    // - checked_saturating
    // - checked_wrapping
    // - checked_overflowing
    (add: $t:ident + $p:ident + $b:literal) => { paste! {
        impl Add<[<$t$b>]> for [<$t$b>] {
            type Output = [<$t$b>];
            /// Performs the `+` operation.
            ///
            /// Panics in debug, on overflow.
            /// While in release, it performs two's complement wrapping.
            #[inline]
            fn add(self, rhs: [<$t$b>]) -> Self::Output {
                self.basic_add(rhs)
            }
        }
        // impl<'a> Add<&'a[<$t$b>]> for &'a [<$t$b>] {} // MAYBE

        impl AddAssign for [<$t$b>] {
            /// Performs the `+=` operation.
            ///
            /// # Panics
            /// Panics in debug, on overflow.
            /// While in release, it performs two's complement wrapping.
            #[inline]
            fn add_assign(&mut self, rhs: [<$t$b>]) {
                *self = self.basic_add(rhs)
            }
        }

        impl Sum for [<$t$b>] {
            fn sum<I: Iterator<Item=Self>>(iter: I) -> Self {
                iter.reduce(
                    |a, b| a + b,
                ).expect("The iterator is empty")
            }
        }

        /// # Integer addition
        impl [<$t$b>] {
            /// Integer addition.
            ///
            /// # Panics
            /// If the addition results in 0, or overflows.
            #[inline]
            #[must_use]
            pub const fn basic_add(self, rhs: [<$t$b>]) -> [<$t$b>] {
                if let Some(result) = [<$p$b>]::new(self.0.get() + rhs.0.get()) {
                    Self(result)
                } else {
                    panic!["NonZero addition resulted in 0"]
                }
            }

            /// Checked addition.
            ///
            /// Returns `None` if the result is 0, or it overflows.
            #[inline]
            #[must_use]
            pub const fn checked_add(self, rhs: [<$t$b>]) -> Option<[<$t$b>]> {
                if let Some(result) = self.0.get().checked_add(rhs.0.get()) {
                    if let Some(non0_result) = [<$p$b>]::new(result) {
                        Some(Self(non0_result))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }

            /* THINK: create checked versions */

            /// Saturating addition.
            /// Computes `self + rhs`, saturating at the numeric bounds instead of overflowing.
            ///
            /// # Panics
            /// If the addition results in 0.
            #[inline]
            #[must_use]
            pub const fn checked_saturating_add(self, rhs: [<$t$b>]) -> Option<[<$t$b>]> {
                let result = self.0.get().saturating_add(rhs.0.get());

                if let Some(non0_result) = [<$p$b>]::new(result) {
                    Some(Self(non0_result))
                } else {
                    None
                }
            }

            /// Wrapping (modular) addition.
            /// Computes `self + rhs`, wrapping around at the boundary of the type.
            #[inline]
            #[must_use]
            pub const fn checked_wrapping_add(self, rhs: [<$t$b>]) -> Option<[<$t$b>]> {
                let result = self.0.get().wrapping_add(rhs.0.get());

                if let Some(non0_result) = [<$p$b>]::new(result) {
                    Some(Self(non0_result))
                } else {
                    None
                }
            }

            /// Overflowing addition.
            ///
            /// Returns a tuple of the addition along with a boolean indicating
            /// whether an arithmetic overflow would occur. If an overflow would
            /// have occurred then the wrapped value is returned.
            #[inline]
            #[must_use]
            pub const fn overflowing_add(self, rhs: [<$t$b>]) -> (Option<[<$t$b>]>, bool) {
                let (result, overflown) = self.0.get().overflowing_add(rhs.0.get());

                if let Some(non0_result) = [<$p$b>]::new(result) {
                    (Some(Self(non0_result)), overflown)
                } else {
                    (None, overflown)
                }
            }
        }
    }};
}

impl_integer_add![
    NonZeroInteger+NonZeroI+8, cast:16;
    NonZeroInteger+NonZeroI+16, cast:32;
    NonZeroInteger+NonZeroI+32, cast:64;
    NonZeroInteger+NonZeroI+64, cast:128;
    NonZeroInteger+NonZeroI+128, cast:128
];
