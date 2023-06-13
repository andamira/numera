// numera::number::integer::z::ops::div_rem
//
//! Implement the division and remainder operations.
//

use crate::number::integer::*;
use devela::paste;

macro_rules! impl_integer_div_rem {
    // impl Div and Rem ops together for multiple integer types
    //
    // # Args
    // $t: integer base name. e.g. Integer
    // $p: inner primitive base name. e.g. i
    // $b: integer and primitive bitsize. e.g. 8
    ( $($t:ident + $p:ident + $b:literal, cast: $bcast:literal);+ ) => {
        $(
            impl_integer_div_rem![div_rem: $t + $p + $b];
        )+
    };

    // impl the division + remainder operations together
    (div_rem: $t:ident + $p:ident + $b:literal) => { paste! {
        // helper function for the inner primitive
        #[inline]
        pub(crate) const fn [<div_rem_trunc_$p$b>](lhs: [<$p$b>], rhs: [<$p$b>]) -> ([<$p$b>], [<$p$b>]) {
            (lhs / rhs, lhs % rhs)
        }

        /// # Division and remainder
        impl [<$t$b>] {
            /// Alias of [`div_rem_trunc`][Self#method.div_rem_trunc].
            #[inline(always)]
            pub const fn div_rem(self, rhs: [<$t$b>]) -> ([<$t$b>], [<$t$b>]) {
                self.div_rem_trunc(rhs)
            }

            /// Returns the truncated (`quotient`, `remainder`).
            ///
            /// # Panics
            /// If `rhs` is 0 or if division results in overflow.
            /// ```
            /// # use numera::all::Z8;
            /// assert_eq![Z8::new(7).div_rem_trunc(Z8::new(3)), (Z8::new(2), Z8::new(1))];
            /// assert_eq![Z8::new(7).div_rem_trunc(Z8::new(-3)), (Z8::new(-2), Z8::new(1))];
            /// assert_eq![Z8::new(-7).div_rem_trunc(Z8::new(3)), (Z8::new(-2), Z8::new(-1))];
            /// assert_eq![Z8::new(-7).div_rem_trunc(Z8::new(-3)), (Z8::new(2), Z8::new(-1))];
            /// ```
            #[inline]
            #[must_use]
            pub const fn div_rem_trunc(self, rhs: [<$t$b>]) -> ([<$t$b>], [<$t$b>]) {
                (self.div_trunc(rhs), self.rem_trunc(rhs))
            }

            /// Returns the euclidean (`quotient`, `remainder`).
            ///
            /// # Panics
            /// If `rhs` is 0 or if division results in overflow.
            ///
            /// # Examples
            /// ```
            /// # use numera::all::Z8;
            /// assert_eq![Z8::new(7).div_rem_euclid(Z8::new(3)), (Z8::new(2), Z8::new(1))];
            /// assert_eq![Z8::new(7).div_rem_euclid(Z8::new(-3)), (Z8::new(-2), Z8::new(1))];
            /// assert_eq![Z8::new(-7).div_rem_euclid(Z8::new(3)), (Z8::new(-3), Z8::new(2))];
            /// assert_eq![Z8::new(-7).div_rem_euclid(Z8::new(-3)), (Z8::new(3), Z8::new(2))];
            /// ```
            pub const fn div_rem_euclid(self, rhs: [<$t$b>]) -> ([<$t$b>], [<$t$b>]) {
                let (q, r) = [<div_rem_trunc_$p$b>](self.0, rhs.0);
                if r < 0 {
                    if rhs.0 > 0 {
                        (Self(q - 1), Self(r + rhs.0))
                    } else {
                        (Self(q + 1), Self(r - rhs.0))
                    }
                } else {
                    (Self(q), Self(r))
                }
                // NOTE: must use primitives because PartialOrd is not const
                // let (q, r) = self.div_rem_trunc(rhs);
                // if r < Self::ZERO {
                //     if rhs > Self::ZERO {
                //         (q - Self::ONE, r + rhs)
                //     } else {
                //         (q + Self::ONE, r - rhs)
                //     }
                // } else {
                //     (q, r)
                // }
            }

            /// Returns the ceiled (`quotient`, `remainder`).
            ///
            /// # Panics
            /// If `rhs` is 0 or if division results in overflow.
            ///
            /// # Examples
            /// ```
            /// # use numera::all::Z8;
            /// assert_eq![Z8::new(7).div_rem_ceil(Z8::new(3)), (Z8::new(3), Z8::new(-2))];
            /// assert_eq![Z8::new(7).div_rem_ceil(Z8::new(-3)), (Z8::new(-2), Z8::new(1))];
            /// assert_eq![Z8::new(-7).div_rem_ceil(Z8::new(3)), (Z8::new(-2), Z8::new(-1))];
            /// assert_eq![Z8::new(-7).div_rem_ceil(Z8::new(-3)), (Z8::new(3), Z8::new(2))];
            /// ```
            pub const fn div_rem_ceil(self, rhs: [<$t$b>]) -> ([<$t$b>], [<$t$b>]) {
                if self.0 > 0 && rhs.0 > 0 {
                    let (q, r) = [<div_rem_trunc_$p$b>]((self.0 - 1), rhs.0);
                    (Self(q + 1), Self(r - rhs.0 + 1))
                } else if self.0 < 0 && rhs.0 < 0 {
                    let (q, r) = [<div_rem_trunc_$p$b>]((self.0 + 1), rhs.0);
                    (Self(q + 1), Self(r - rhs.0 - 1))
                } else {
                    let (q, r) = [<div_rem_trunc_$p$b>](self.0, rhs.0);
                    (Self(q), Self(r))
                }
            }

            /// Returns the floored (`quotient`, `remainder`).
            ///
            /// # Panics
            /// If `rhs` is 0 or if division results in overflow.
            ///
            /// # Examples
            /// ```
            /// # use numera::all::Z8;
            /// assert_eq![Z8::new(7).div_rem_floor(Z8::new(3)), (Z8::new(2), Z8::new(1))];
            /// assert_eq![Z8::new(7).div_rem_floor(Z8::new(-3)), (Z8::new(-3), Z8::new(-2))];
            /// assert_eq![Z8::new(-7).div_rem_floor(Z8::new(3)), (Z8::new(-3), Z8::new(2))];
            /// assert_eq![Z8::new(-7).div_rem_floor(Z8::new(-3)), (Z8::new(2), Z8::new(-1))];
            /// ```
            pub const fn div_rem_floor(self, rhs: [<$t$b>]) -> ([<$t$b>], [<$t$b>]) {
                if self.0 > 0 && rhs.0 < 0 {
                    let (q, r) = [<div_rem_trunc_$p$b>](self.0 - 1, rhs.0);
                    (Self(q - 1), Self(r + rhs.0 + 1))
                } else if self.0 < 0 && rhs.0 > 0 {
                    let (q, r) = [<div_rem_trunc_$p$b>](self.0 + 1, rhs.0);
                    (Self(q - 1), Self(r + rhs.0 - 1))
                } else {
                    let (q, r) = [<div_rem_trunc_$p$b>](self.0, rhs.0);
                    (Self(q), Self(r))
                }
            }
        }
    }};
}

impl_integer_div_rem![
    Integer+i+8, cast:16;
    Integer+i+16, cast:32;
    Integer+i+32, cast:64;
    Integer+i+64, cast:128;
    Integer+i+128, cast:128
];
