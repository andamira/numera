// numera::number::integer::z::ops::rem
//
//! Implement the division and remainder operations.
//

use crate::number::integer::*;
use core::ops::{Rem, RemAssign};
use devela::paste;

macro_rules! impl_integer_rem {
    // impl Div and Rem ops for multiple integer types
    //
    // # Args
    // $t: integer base name. e.g. Integer
    // $p: inner primitive base name. e.g. i
    // $b: integer and primitive bitsize. e.g. 8
    ( $($t:ident + $p:ident + $b:literal, cast: $bcast:literal);+ ) => {
        $(
            impl_integer_rem![rem: $t + $p + $b];
        )+
    };

    // impl the remainder operations
    //
    // impl variants:
    // - op
    // - overflowing TODO ? THINK
    // - wrapping_count TODO
    //
    // variants not implemented (since the remainder operation can't overflow):
    // - checked
    // - modulo
    // - wrapping
    // - saturating
    (rem: $t:ident + $p:ident + $b:literal) => { paste! {
        impl Rem<[<$t$b>]> for [<$t$b>] {
            type Output = [<$t$b>];
            /// Performs the `%` operation, using truncated remainder.
            #[inline]
            fn rem(self, rhs: [<$t$b>]) -> Self::Output {
                self.rem_trunc(rhs)
            }
        }
        impl RemAssign for [<$t$b>] {
            /// Performs the `%=` operation.
            #[inline]
            fn rem_assign(&mut self, rhs: [<$t$b>]) {
                self.0 %= rhs.0;
            }
        }
        /// # Remainder
        impl [<$t$b>] {
            /// Returns the truncated remainder operation.
            ///
            /// This is the default remainder operation in Rust, C and Java.
            ///
            /// It is based on truncated division, rounding the quotient towards
            /// zero, meaning the remainder has the same sign as the dividend.
            ///
            /// # Notation
            /// $ x \mod n $
            ///
            /// # Examples
            /// ```
            /// use numera::all::Z8;
            ///
            /// assert_eq![Z8::new(-5).rem_trunc(Z8::new(2)), Z8::new(-1)];
            /// ```
            #[inline]
            #[must_use]
            pub const fn rem_trunc(self, rhs: [<$t$b>]) -> [<$t$b>] {
                Self(self.0 % rhs.0)
            }

            /// Returns the euclidean remainder operation.
            ///
            /// This is often used in modular arithmetic, since it always
            /// returns a non-negative remainder.
            ///
            /// It is based on euclidean division, whih ensures that the same
            /// value of x will always result in the same remainder when divided
            /// by the same value of y.
            ///
            /// # Panics
            /// If `rhs` is 0 or the division results in overflow.
            ///
            /// # Examples
            /// ```
            /// use numera::all::Z8;
            ///
            /// assert_eq![Z8::new(-5).rem_euclid(Z8::new(2)), Z8::new(1)];
            /// ```
            #[inline]
            #[must_use]
            pub const fn rem_euclid(self, rhs: [<$t$b>]) -> [<$t$b>] {
                Self(self.0.rem_euclid(rhs.0))
            }

            /// Returns the checked Euclidean remainder operation.
            ///
            /// Computes [`rem_euclid`][Self#method.rem_euclid], returning `None`
            /// if `rhs` == 0 or the division results in overflow.
            ///
            /// # Examples
            /// ```
            /// use numera::all::Z8;
            ///
            /// assert_eq![Z8::new(-5).checked_rem_euclid(Z8::new(2)), Some(Z8::new(1))];
            /// assert_eq![Z8::new(-5).checked_rem_euclid(Z8::new(0)), None];
            /// ```
            #[inline]
            #[must_use]
            pub const fn checked_rem_euclid(self, rhs: [<$t$b>]) -> Option<[<$t$b>]> {
                if let Some(result) = self.0.checked_rem_euclid(rhs.0) {
                    Some(Self(result))
                } else {
                    None
                }
            }

            /// Returns the floored remainder operation.
            ///
            /// Rounds the quotient towards negative infinity, differing
            /// from the truncated division in regard to negative numbers.
            ///
            /// This is often used when the quotient represents the number of
            /// "groups" of a certain size that can be formed from a given
            /// number of items.
            ///
            /// # Examples
            /// ```
            /// use numera::all::Z8;
            ///
            /// // TODO
            /// ```
            #[inline]
            #[must_use]
            // FIXME: should return -4 for -7 / -3
            pub const fn rem_floor(self, rhs: [<$t$b>]) -> [<$t$b>] {
                let rem = self.rem_trunc(rhs);
                let rem_adjusted = if rem.0.signum() != rhs.0.signum() && rem.0 != 0 {
                    rem.0 + rhs.0
                } else {
                    rem.0
                };
                Self(rem_adjusted)
            }
        }
    }};
}

impl_integer_rem![
    Integer+i+8, cast:16;
    Integer+i+16, cast:32;
    Integer+i+32, cast:64;
    Integer+i+64, cast:128;
    Integer+i+128, cast:128
];
