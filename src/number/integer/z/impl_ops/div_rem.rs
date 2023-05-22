// numera::number::integer::z::impl_ops::div_rem
//
//! Implement the division and remainder operations.
//

use crate::number::integer::*;
use core::ops::{Div, DivAssign, Rem, RemAssign};
use devela::paste;

macro_rules! impl_integer_div_rem {
    // impl Div and Rem ops for multiple integer types
    //
    // # Args
    // $t: integer base name. e.g. Integer
    // $p: inner primitive base name. e.g. i
    // $b: integer and primitive bitsize. e.g. 8
    ( $($t:ident + $p:ident + $b:literal, cast: $bcast:literal);+ ) => {
        $(
            impl_integer_div_rem![div: $t + $p + $b, cast:$bcast];
            impl_integer_div_rem![rem: $t + $p + $b];
            impl_integer_div_rem![div_rem: $t + $p + $b];
        )+
    };

    // impl variants:
    // - div_trunc
    // - div_euclid
    // - div_ceil
    // - div_floor
    // - div_half_away
    // - div_half_even
    // - checked_div_trunc
    // - checked_div_euclid
    // - checked_div_ceil
    // - checked_div_floor
    // - checked_div_half_away
    // - checked_div_half_even
    // - saturating_div_trunc TODO
    // - saturating_div_euclid TODO
    // - saturating_div_ceil TODO
    // - saturating_div_floor TODO
    // - saturating_div_half_away TODO
    // - saturating_div_half_even TODO
    // - wrapping_div_trunc TODO
    // - wrapping_div_euclid TODO
    // - wrapping_div_ceil TODO
    // - wrapping_div_floor TODO
    // - wrapping_div_half_away TODO
    // - wrapping_div_half_even TODO
    // - overflowing_div_trunc TODO
    // - overflowing_div_euclid TODO
    // - overflowing_div_ceil TODO
    // - overflowing_div_floor TODO
    // - overflowing_div_half_away TODO
    // - overflowing_div_half_even TODO
    // - modular_div_trunc TODO
    // - modular_div_euclid TEST (alias modular_div)
    // - …
    // - modular_counting_div_trunc TODO
    // - …
    (div: $t:ident + $p:ident + $b:literal, cast: $bcast:literal) => { paste! {
        impl Div<[<$t$b>]> for [<$t$b>] {
            type Output = [<$t$b>];
            /// Performs the `/` operation, using truncated division.
            ///
            /// # Panics
            /// In debug, on overflow.
            ///
            /// In release, it performs two's complement wrapping.
            #[inline]
            fn div(self, rhs: [<$t$b>]) -> Self::Output {
                self.div_trunc(rhs)
            }
        }
        impl DivAssign for [<$t$b>] {
            /// Performs the `/=` operation.
            ///
            /// # Panics
            /// In debug, on overflow.
            ///
            /// In release, it performs two's complement wrapping.
            #[inline]
            fn div_assign(&mut self, rhs: [<$t$b>]) {
                self.0 /= rhs.0
            }
        }

        /// # Integer division
        ///
        /// ## Comparison of division functions
        ///
        /// ### Only the quotient:
        /// dividend|divisor||*float*||[trunc]|[euclid]|[floor]|[ceil]||[away]|[even]|
        /// :------:|:-----:||:-----:||:-----:|:------:|:----:|:-----:||:----:|:----:|
        ///     7   |    3  || 2.33… ||   2   |   2    |   2  |    3  ||   2  |   2  |
        ///     7   |   -3  ||   "   ||  -2   |  -2    |  -3  |   -2  ||  -2  |  -2  |
        ///    -7   |    3  ||   "   ||  -2   |  -3    |  -3  |   -2  ||  -2  |  -2  |
        ///    -7   |   -3  ||   "   ||   2   |   3    |   2  |    3  ||   2  |   2  |
        ///         |       ||       ||       |        |      |       ||      |      |
        ///     8   |    5  || 1.6   ||   1   |   1    |   1  |    2  ||   2  |   2  |
        ///     6   |    4  || 1.5   ||   1   |   1    |   1  |    2  ||   2  |   2  |
        ///     7   |    5  || 1.25  ||   1   |   1    |   1  |    2  ||   1  |   2  |
        ///
        /// ### Showing the quotient and the remainder:
        /// dividend|divisor||*float*||[trunc]|[euclid]|[floor]|[ceil]||[away]|[even]|
        /// :------:|:-----:||:-----:||:-----:|:------:|:----:|:-----:||:----:|:----:|
        ///     7   |    3  || 2.33… ||(2, ) | (2, )  | ( 2, )|  ( 3, )|| (2, )| (2, )|
        ///     7   |   -3  ||   "   ||(-2, ) | (-2, )  | (-3, )|  (-2, )|| (-2, )| (-2, )|
        ///    -7   |    3  ||   "   ||(-2, ) | (-3, )  | (-3, )|  (-2, )|| (-2, )| (-2, )|
        ///    -7   |   -3  ||   "   ||(2, ) | (3, )  | ( 2, )|  ( 3, )|| (2, )| (2, )|
        ///         |       ||       ||       |        |      |       ||      |      |
        ///     8   |    5  || 1.6   ||(1, ) | (1, )  | (1, )|  (2, )|| (2, )| (2, )|
        ///     6   |    4  || 1.5   ||(1, ) | (1, )  | (1, )|  (2, )|| (2, )| (2, )|
        ///     7   |    5  || 1.25  ||(1, ) | (1, )  | (1, )|  (2, )|| (1, )| (2, )|
        ///
        /// [trunc]: Self#method.div_trunc
        /// [euclid]: Self#method.div_euclid
        /// [floor]: Self#method.div_floor
        /// [ceil]: Self#method.div_ceil
        /// [away]: Self#method.div_half_away
        /// [even]: Self#method.div_half_even
        impl [<$t$b>] {
            /// Truncated division.
            ///
            /// Rounds the quotient towards zero, or away from infinity.
            ///
            /// # Panics
            /// If `rhs` is 0 or if division results in overflow.
            ///
            /// # Examples
            /// ```
            /// # use numera::all::*;
            /// assert_eq![Z8::new(7).div_trunc(Z8::new(3)), Z8::new(2)];
            /// assert_eq![Z8::new(7).div_trunc(Z8::new(-3)), Z8::new(-2)];
            /// assert_eq![Z8::new(-7).div_trunc(Z8::new(3)), Z8::new(-2)];
            /// assert_eq![Z8::new(-7).div_trunc(Z8::new(-3)), Z8::new(2)];
            ///
            /// // tie-breaking
            /// assert_eq![Z8::new(8).div_trunc(Z8::new(5)), Z8::new(1)]; // 8/5 = 1.6 => 1
            /// assert_eq![Z8::new(6).div_trunc(Z8::new(4)), Z8::new(1)]; // 6/4 = 1.5 => 1
            /// assert_eq![Z8::new(7).div_trunc(Z8::new(5)), Z8::new(1)]; // 7/5 = 1.4 => 1
            /// ```
            #[inline]
            #[must_use]
            pub const fn div_trunc(self, rhs: [<$t$b>]) -> [<$t$b>] {
                Self(self.0 / rhs.0)
            }

            /// Checked truncated division.
            ///
            /// Rounds the quotient towards zero, or away from infinity.
            ///
            /// # Examples
            /// ```
            /// # use numera::all::*;
            /// // invalid operands
            /// assert_eq![Z8::new(7).checked_div_trunc(Z8::new(0)), None]; // division by 0
            /// assert_eq![Z8::MIN.checked_div_trunc(Z8::new(-1)), None]; // overflow
            /// ```
            #[inline]
            #[must_use]
            pub const fn checked_div_trunc(self, rhs: [<$t$b>]) -> Option<[<$t$b>]> {
                if let Some(result) = self.0.checked_div(rhs.0) {
                    Some(Self(result))
                } else {
                    None
                }
            }

            /// Euclidean division.
            ///
            /// Ensures that the remainder is always non-negative and smaller
            /// than the divisor.
            ///
            /// # Panics
            /// If `rhs` is 0 or if division results in overflow.
            ///
            /// # Examples
            /// ```
            /// # use numera::all::*;
            /// assert_eq![Z8::new(7).div_euclid(Z8::new(3)), Z8::new(2)];
            /// assert_eq![Z8::new(7).div_euclid(Z8::new(-3)), Z8::new(-2)];
            /// assert_eq![Z8::new(-7).div_euclid(Z8::new(3)), Z8::new(-3)];
            /// assert_eq![Z8::new(-7).div_euclid(Z8::new(-3)), Z8::new(3)];
            ///
            /// // tie-breaking
            /// assert_eq![Z8::new(8).div_trunc(Z8::new(5)), Z8::new(1)]; // 8/5 = 1.6 => 1
            /// assert_eq![Z8::new(6).div_trunc(Z8::new(4)), Z8::new(1)]; // 6/4 = 1.5 => 1
            /// assert_eq![Z8::new(7).div_trunc(Z8::new(5)), Z8::new(1)]; // 7/5 = 1.4 => 1
            /// ```
            #[inline]
            #[must_use]
            pub const fn div_euclid(self, rhs: [<$t$b>]) -> [<$t$b>] {
                Self(self.0.div_euclid(rhs.0))
            }

            /// Checked euclidean division.
            ///
            /// Ensures that the remainder is always non-negative and smaller
            /// than the divisor.
            ///
            /// # Examples
            /// ```
            /// # use numera::all::*;
            /// // invalid operands
            /// assert_eq![Z8::new(7).checked_div_euclid(Z8::new(0)), None]; // division by 0
            /// assert_eq![Z8::MIN.checked_div_euclid(Z8::new(-1)), None]; // overflow
            /// ```
            #[inline]
            #[must_use]
            pub const fn checked_div_euclid(self, rhs: [<$t$b>]) -> Option<[<$t$b>]> {
                if let Some(result) = self.0.checked_div_euclid(rhs.0) {
                    Some(Self(result))
                } else {
                    None
                }
            }

            /// Floored division
            ///
            /// Rounds the quotient towards negative infinity.
            ///
            /// # Panics
            /// If `rhs` is 0 or if the division results in overflow.
            ///
            /// # Notation
            /// $ \left\lfloor \frac{x}{y} \right\rfloor $
            ///
            /// # Examples
            /// ```
            /// # use numera::all::*;
            /// assert_eq![Z8::new(7).div_floor(Z8::new(3)), Z8::new(2)];
            /// assert_eq![Z8::new(7).div_floor(Z8::new(-3)), Z8::new(-3)];
            /// assert_eq![Z8::new(-7).div_floor(Z8::new(3)), Z8::new(-3)];
            /// assert_eq![Z8::new(-7).div_floor(Z8::new(-3)), Z8::new(2)];
            ///
            /// // tie-breaking
            /// assert_eq![Z8::new(8).div_trunc(Z8::new(5)), Z8::new(1)]; // 8/5 = 1.6 => 1
            /// assert_eq![Z8::new(6).div_trunc(Z8::new(4)), Z8::new(1)]; // 6/4 = 1.5 => 1
            /// assert_eq![Z8::new(7).div_trunc(Z8::new(5)), Z8::new(1)]; // 7/5 = 1.4 => 1
            /// ```
            #[inline]
            #[must_use]
            pub const fn div_floor(self, rhs: [<$t$b>]) -> [<$t$b>] {
                if self.0 > 0 && rhs.0 < 0 {
                    Self(((self.0 - 1) / rhs.0) - 1)
                } else if self.0 < 0 && rhs.0 > 0 {
                    Self(((self.0 + 1) / rhs.0) - 1)
                } else {
                    self.div_trunc(rhs)
                }

                // ALTERNATIVE:TODO:BENCH
                // let result = self.0 / rhs.0;
                // if self.0 % rhs.0 != 0 && (self.0 < 0) != (rhs.0 < 0) {
                //     Self(result - 1)
                // } else {
                //     Self(result)
                // }
            }

            /// Checked floored division.
            ///
            /// Rounds the quotient towards negative infinity.
            ///
            /// # Notation
            /// $ \left\lfloor \frac{x}{y} \right\rfloor $
            ///
            /// # Examples
            /// ```
            /// # use numera::all::*;
            /// // invalid operands
            /// assert_eq![Z8::new(7).checked_div_floor(Z8::new(0)), None]; // division by 0
            /// assert_eq![Z8::MIN.checked_div_floor(Z8::new(-1)), None]; // overflow
            /// ```
            #[inline]
            #[must_use]
            pub const fn checked_div_floor(self, rhs: [<$t$b>]) -> Option<[<$t$b>]> {
                if rhs.0 == 0 || ((self.0 == [<$p$b>]::MIN) & (rhs.0 == -1)) {
                    None
                } else {
                    Some(self.div_floor(rhs))
                }
            }

            /// Ceiled division.
            ///
            /// Rounds the quotient towards positive infinity.
            ///
            /// # Notation
            /// $ \left\lceil \frac{x}{y} \right\rceil $
            ///
            /// # Examples
            /// ```
            /// # use numera::all::*;
            /// assert_eq![Z8::new(7).div_ceil(Z8::new(3)), Z8::new(3)];
            /// assert_eq![Z8::new(7).div_ceil(Z8::new(-3)), Z8::new(-2)];
            /// assert_eq![Z8::new(-7).div_ceil(Z8::new(3)), Z8::new(-2)];
            /// assert_eq![Z8::new(-7).div_ceil(Z8::new(-3)), Z8::new(3)];
            ///
            /// // tie-breaking
            /// assert_eq![Z8::new(8).div_ceil(Z8::new(5)), Z8::new(2)]; // 8/5 = 1.6 => 2
            /// assert_eq![Z8::new(6).div_ceil(Z8::new(4)), Z8::new(2)]; // 6/4 = 1.5 => 2
            /// assert_eq![Z8::new(7).div_ceil(Z8::new(5)), Z8::new(2)]; // 7/5 = 1.4 => 2
            /// ```
            #[inline]
            #[must_use]
            pub const fn div_ceil(self, rhs: [<$t$b>]) -> [<$t$b>] {
                if self.0 > 0 && rhs.0 > 0 {
                    Self(((self.0 - 1) / rhs.0) + 1)
                } else if self.0 < 0 && rhs.0 < 0 {
                    Self(((self.0 + 1) / rhs.0) + 1)
                } else {
                    self.div_trunc(rhs)
                }
            }

            /// Checked ceiled division.
            ///
            /// Rounds the quotient towards positive infinity.
            ///
            /// # Notation
            /// $ \left\lceil \frac{x}{y} \right\rceil $
            ///
            /// # Examples
            /// ```
            /// # use numera::all::*;
            /// // invalid operands
            /// assert_eq![Z8::new(7).checked_div_ceil(Z8::new(0)), None]; // division by 0
            /// assert_eq![Z8::MIN.checked_div_ceil(Z8::new(-1)), None]; // overflow
            /// ```
            #[inline]
            #[must_use]
            pub const fn checked_div_ceil(self, rhs: [<$t$b>]) -> Option<[<$t$b>]> {
                if rhs.0 == 0 || ((self.0 == [<$p$b>]::MIN) & (rhs.0 == -1)) {
                    None
                } else {
                    Some(self.div_ceil(rhs))
                }
            }

            /// Rounded division half away from 0.
            ///
            /// Rounds the quotient to the nearest integer, tie-breaking away
            /// from 0 ([w]).
            ///
            /// [w]: https://en.wikipedia.org/wiki/Rounding#Rounding_half_away_from_zero
            ///
            /// # Panics
            /// If `rhs` is 0 or if the division results in overflow.
            ///
            /// # Examples
            /// ```
            /// # use numera::all::*;
            /// assert_eq![Z8::new(7).div_half_away(Z8::new(3)), Z8::new(2)];
            /// assert_eq![Z8::new(7).div_half_away(Z8::new(-3)), Z8::new(-2)];
            /// assert_eq![Z8::new(-7).div_half_away(Z8::new(3)), Z8::new(-2)];
            /// assert_eq![Z8::new(-7).div_half_away(Z8::new(-3)), Z8::new(2)];
            ///
            /// // half tie-breaking
            /// assert_eq![Z8::new(6).div_half_away(Z8::new(4)), Z8::new(2)]; // 6/4 = 1.5 => 2
            /// assert_eq![Z8::new(7).div_half_away(Z8::new(5)), Z8::new(1)]; // 7/5 = 1.25 => 1
            /// ```
            #[inline]
            #[must_use]
            // NOTE: this implementation assumes the input integer is not larger than MAX/2
            // TODO: IMPROVE it by casting first to the next larger bit-size
            pub const fn div_half_away(self, rhs: [<$t$b>]) -> [<$t$b>] {
                let (q, r) = [<div_rem_trunc_$p$b>](self.0, rhs.0);

                // Check if the remainder is greater than or equal to half of the divisor
                if 2 * r.abs() >= rhs.0.abs() {
                    if (self.0 > 0) == (rhs.0 > 0) {
                        Self(q + 1)
                    } else {
                        Self(q - 1)
                    }
                } else {
                    Self(q)
                }
            }

            /// Checked rounded division half away from 0.
            ///
            /// Rounds the quotient to the nearest integer, tie-breaking away
            /// from 0 ([w]).
            ///
            /// [w]: https://en.wikipedia.org/wiki/Rounding#Rounding_half_away_from_zero
            ///
            /// # Examples
            /// ```
            /// # use numera::all::*;
            /// // invalid operands
            /// assert_eq![Z8::new(7).checked_div_half_away(Z8::new(0)), None]; // division by 0
            /// assert_eq![Z8::MIN.checked_div_half_away(Z8::new(-1)), None]; // overflow
            /// ```
            #[inline]
            #[must_use]
            pub const fn checked_div_half_away(self, rhs: [<$t$b>]) -> Option<[<$t$b>]> {
                if rhs.0 == 0 || ((self.0 == [<$p$b>]::MIN) & (rhs.0 == -1)) {
                    None
                } else {
                    Some(self.div_half_away(rhs))
                }
            }

            /// Rounded division half to even.
            ///
            /// Rounds the quotient to the nearest integer, tie-breaking to the
            /// nearest even number.
            ///
            /// This is also known as *bankers' rounding* and is often the
            /// default rounding method, since it helps eliminate
            /// positive/negative bias and bias towards/away from zero ([w]).
            ///
            /// [w]: https://en.wikipedia.org/wiki/Rounding#Rounding_half_to_even
            ///
            /// # Panics
            /// If `rhs` is 0 or if the division results in overflow.
            ///
            /// # Examples
            /// ```
            /// # use numera::all::*;
            /// assert_eq![Z8::new(7).div_half_even(Z8::new(3)), Z8::new(2)];
            /// assert_eq![Z8::new(7).div_half_even(Z8::new(-3)), Z8::new(-2)];
            /// assert_eq![Z8::new(-7).div_half_even(Z8::new(3)), Z8::new(-2)];
            /// assert_eq![Z8::new(-7).div_half_even(Z8::new(-3)), Z8::new(2)];
            ///
            /// // half tie-breaking
            /// assert_eq![Z8::new(6).div_half_even(Z8::new(4)), Z8::new(2)]; // 6/4 = 1.5 => 2
            /// assert_eq![Z8::new(7).div_half_even(Z8::new(5)), Z8::new(2)]; // 7/5 = 1.25 => 2
            /// ```
            #[inline]
            #[must_use]
            pub const fn div_half_even(self, rhs: [<$t$b>]) -> [<$t$b>] {
                let (q, r) = [<div_rem_trunc_$p$b>](self.0, rhs.0);

                let qround = if r == 0 {
                    q
                } else if (q.abs() % 2) == 1 {
                    q + self.0.signum() * rhs.0.signum()
                } else {
                    q
                };
                Self(qround)
            }
            // TODO:BENCH using intermediate floating-point
            // NOTE: this can't be const
            // pub fn div_half_even(self, rhs: [<$t$b>]) -> [<$t$b>] {
            //     Self(crate::all::round_half_even64(self.0 as f64 / rhs.0 as f64) as [<$p$b>])
            // }

            /// Checked rounded division half to even.
            ///
            /// Rounds the quotient to the nearest integer, tie-breaking to the
            /// nearest even number.
            ///
            /// This is also known as *bankers' rounding* and is often the
            /// default rounding method, since it helps eliminate
            /// positive/negative bias and bias towards/away from zero ([w]).
            ///
            /// [w]: https://en.wikipedia.org/wiki/Rounding#Rounding_half_to_even
            ///
            /// # Examples
            /// ```
            /// # use numera::all::*;
            /// // invalid operands
            /// assert_eq![Z8::new(7).checked_div_half_away(Z8::new(0)), None]; // division by 0
            /// assert_eq![Z8::MIN.checked_div_half_away(Z8::new(-1)), None]; // overflow
            /// ```
            #[inline]
            #[must_use]
            pub const fn checked_div_half_even(self, rhs: [<$t$b>]) -> Option<[<$t$b>]> {
                if rhs.0 == 0 || ((self.0 == [<$p$b>]::MIN) & (rhs.0 == -1)) {
                    None
                } else {
                    Some(self.div_half_even(rhs))
                }
            }

            // MAYBE
            // /// Alias of [`modular_div_euclid`][Self#method.modular_div_euclid].
            // #[inline(always)]
            // pub const fn modular_div(self, rhs: [<$t$b>], modulo: [<$t$b>]) -> [<$t$b>] {
            //     self.modular_div_euclid(rhs, modulo)
            // }

            // // TEST
            // /// Modular euclidian division with custom `modulo`.
            // #[inline]
            // #[must_use]
            // pub const fn modular_div_euclid(self, rhs: [<$t$b>], modulo: [<$t$b>]) -> [<$t$b>] {
            //     self.div_euclid(rhs).rem_euclid(modulo)
            // }
        }
    }};

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
                self.0 %= rhs.0
            }
        }
        /// # Integer remainder
        impl [<$t$b>] {
            /// Truncated remained operation.
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

            /// Euclidean remainder operation.
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

            /// Checked Euclidean remainder operation.
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

            /// Floored remainder operation.
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

    // impl the division + remainder operations together
    // // MAYBE join with op_div & op_rem
    (div_rem: $t:ident + $p:ident + $b:literal) => { paste! {
        // private function for the inner primitive
        #[inline]
        const fn [<div_rem_trunc_$p$b>](lhs: [<$p$b>], rhs: [<$p$b>]) -> ([<$p$b>], [<$p$b>]) {
            (lhs / rhs, lhs % rhs)
        }

        /// # Integer division and remainder
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
