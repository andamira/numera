// numera::number::integer::z::impl_ops
//
//!
//
// - IMPROVE
// - make doc examples use the current type
// - join all division and remainder in a single block (there'r 3 blocks now)

use super::*;
use core::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};
use devela::paste;

// impl ops (which panic on overflow)
macro_rules! impl_integer_ops {
    // impl all ops for multiple integer types
    //
    // # Args
    // $t: integer base name. e.g. Integer
    // $p: inner primitive base name. e.g. i
    // $b: integer and primitive bitsize. e.g. 8
    ( $($t:ident + $p:ident + $b:literal),+ ) => {
        $(
            impl_integer_ops![ops: $t + $p + $b];
        )+
    };

    // impl all ops for a single integer type
    //
    // # Args
    // $t: integer base name. e.g. Integer
    // $p: inner primitive base name. e.g. i
    // $b: integer and primitive bitsize. e.g. 8
    (ops: $t:ident + $p:ident + $b:literal) => {
        impl_integer_ops![op_add: $t + $p + $b];
        impl_integer_ops![op_sub: $t + $p + $b];
        impl_integer_ops![op_mul: $t + $p + $b];
        impl_integer_ops![op_div: $t + $p + $b];
        impl_integer_ops![op_rem: $t + $p + $b];
        impl_integer_ops![op_div_rem: $t + $p + $b];
        impl_integer_ops![op_neg: $t + $p + $b];
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
    (op_add: $t:ident + $p:ident + $b:literal) => { paste! {
        impl Add<[<$t$b>]> for [<$t$b>] {
            type Output = [<$t$b>];
            /// Performs the `+` operation.
            #[inline]
            fn add(self, rhs: [<$t$b>]) -> Self::Output {
                self.basic_add(rhs)
            }
        }
        impl AddAssign for [<$t$b>] {
            /// Performs the `+=` operation.
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
    (op_sub: $t:ident + $p:ident + $b:literal) => { paste! {
        impl Sub<[<$t$b>]> for [<$t$b>] {
            type Output = [<$t$b>];
            /// Performs the `-` operation.
            #[inline]
            fn sub(self, rhs: [<$t$b>]) -> Self::Output {
                self.basic_sub(rhs)
            }
        }
        impl SubAssign for [<$t$b>] {
            /// Performs the `-=` operation.
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
    (op_mul: $t:ident + $p:ident + $b:literal) => { paste! {
        impl Mul<[<$t$b>]> for [<$t$b>] {
            /// Performs the `*` operation.
            type Output = [<$t$b>];
            #[inline]
            fn mul(self, rhs: [<$t$b>]) -> Self::Output {
                self.basic_mul(rhs)
            }
        }
        impl MulAssign for [<$t$b>] {
            /// Performs the `*=` operation.
            #[inline]
            fn mul_assign(&mut self, rhs: [<$t$b>]) {
                self.0 *= rhs.0
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

    // implement the division operations
    //
    // impl variants:
    // - div_trunc
    // - div_euclid
    // - div_ceil
    // - div_floor
    // - div_round
    // - checked_div_trunc
    // - checked_div_euclid
    // - checked_div_ceil
    // - checked_div_floor
    // - checked_div_round
    // - saturating_div_trunc TODO
    // - saturating_div_euclid TODO
    // - saturating_div_ceil TODO
    // - saturating_div_floor TODO
    // - saturating_div_round TODO
    // - wrapping_div_trunc TODO
    // - wrapping_div_euclid TODO
    // - wrapping_div_ceil TODO
    // - wrapping_div_floor TODO
    // - wrapping_div_round TODO
    // - overflowing_div_trunc TODO
    // - overflowing_div_euclid TODO
    // - overflowing_div_ceil TODO
    // - overflowing_div_floor TODO
    // - overflowing_div_round TODO
    // - modular_div_trunc TODO
    // - modular_div_euclid TEST (alias modular_div)
    // - modular_div_ceil TODO
    // - modular_div_floor TODO
    // - modular_div_round TODO
    // - modular_counting_div_trunc TODO
    // - modular_counting_div_euclid TODO
    // - modular_counting_div_ceil TODO
    // - modular_counting_div_floor TODO
    // - modular_counting_div_round TODO
    (op_div: $t:ident + $p:ident + $b:literal) => { paste! {
        impl Div<[<$t$b>]> for [<$t$b>] {
            type Output = [<$t$b>];
            /// Performs the `/` operation, using truncated division.
            #[inline]
            fn div(self, rhs: [<$t$b>]) -> Self::Output {
                self.div_trunc(rhs)
            }
        }
        impl DivAssign for [<$t$b>] {
            /// Performs the `/=` operation.
            #[inline]
            fn div_assign(&mut self, rhs: [<$t$b>]) {
                self.0 /= rhs.0
            }
        }

        /// # Integer division
        impl [<$t$b>] {
            /// Truncated division.
            ///
            /// # Panics
            /// If `rhs` is 0 or if division results in overflow.
            ///
            /// # Examples
            /// ```
            /// use numera::all::Z8;
            ///
            /// assert_eq![Z8::new(7).div_trunc(Z8::new(3)), Z8::new(2)];
            /// assert_eq![Z8::new(7).div_trunc(Z8::new(-3)), Z8::new(-2)];
            /// assert_eq![Z8::new(-7).div_trunc(Z8::new(3)), Z8::new(-2)];
            /// assert_eq![Z8::new(-7).div_trunc(Z8::new(-3)), Z8::new(2)];
            /// ```
            #[inline]
            #[must_use]
            pub const fn div_trunc(self, rhs: [<$t$b>]) -> [<$t$b>] {
                Self(self.0 / rhs.0)
            }

            /// Checked truncated division.
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
            /// # Panics
            /// If `rhs` is 0 or if division results in overflow.
            ///
            /// # Examples
            /// ```
            /// use numera::all::Z8;
            ///
            /// assert_eq![Z8::new(7).div_trunc(Z8::new(3)), Z8::new(2)];
            /// assert_eq![Z8::new(7).div_trunc(Z8::new(-3)), Z8::new(-2)];
            /// assert_eq![Z8::new(-7).div_trunc(Z8::new(3)), Z8::new(-2)];
            /// assert_eq![Z8::new(-7).div_trunc(Z8::new(-3)), Z8::new(2)];
            /// ```
            #[inline]
            #[must_use]
            pub const fn div_euclid(self, rhs: [<$t$b>]) -> [<$t$b>] {
                Self(self.0.div_euclid(rhs.0))
            }

            /// Checked euclidean division.
            #[inline]
            #[must_use]
            pub const fn checked_div_euclid(self, rhs: [<$t$b>]) -> Option<[<$t$b>]> {
                if let Some(result) = self.0.checked_div_euclid(rhs.0) {
                    Some(Self(result))
                } else {
                    None
                }
            }

            /// Ceiled division.
            ///
            /// # Examples
            /// ```
            /// use numera::all::Z8;
            ///
            /// assert_eq![Z8::new(7).div_ceil(Z8::new(3)), Z8::new(3)];
            /// assert_eq![Z8::new(7).div_ceil(Z8::new(-3)), Z8::new(-2)];
            /// assert_eq![Z8::new(-7).div_ceil(Z8::new(3)), Z8::new(-2)];
            /// assert_eq![Z8::new(-7).div_ceil(Z8::new(-3)), Z8::new(3)];
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
            /// # Examples
            /// ```
            /// # use numera::all::*;
            ///
            /// assert_eq![Z8::new(7).checked_div_ceil(Z8::new(3)), Some(Z8::new(3))];
            /// assert_eq![Z8::new(7).checked_div_ceil(Z8::new(-3)), Some(Z8::new(-2))];
            /// assert_eq![Z8::new(-7).checked_div_ceil(Z8::new(3)), Some(Z8::new(-2))];
            /// assert_eq![Z8::new(-7).checked_div_ceil(Z8::new(-3)), Some(Z8::new(3))];
            ///
            /// assert_eq![Z8::new(7).checked_div_ceil(Z8::new(0)), None];
            /// assert_eq![Z8::MIN.checked_div_ceil(Z8::new(-1)), None];
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


            /// Floored division
            ///
            /// # Panics
            /// If `rhs` is 0 or if the division results in overflow.
            ///
            /// # Examples
            /// ```
            /// use numera::all::Z8;
            ///
            /// assert_eq![Z8::new(7).div_floor(Z8::new(3)), Z8::new(2)];
            /// assert_eq![Z8::new(7).div_floor(Z8::new(-3)), Z8::new(-3)];
            /// assert_eq![Z8::new(-7).div_floor(Z8::new(3)), Z8::new(-3)];
            /// assert_eq![Z8::new(-7).div_floor(Z8::new(-3)), Z8::new(2)];
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
            /// # Examples
            /// ```
            /// # use numera::all::*;
            ///
            /// assert_eq![Z8::new(7).checked_div_floor(Z8::new(3)), Some(Z8::new(2))];
            /// assert_eq![Z8::new(7).checked_div_floor(Z8::new(-3)), Some(Z8::new(-3))];
            /// assert_eq![Z8::new(-7).checked_div_floor(Z8::new(3)), Some(Z8::new(-3))];
            /// assert_eq![Z8::new(-7).checked_div_floor(Z8::new(-3)), Some(Z8::new(2))];
            ///
            /// assert_eq![Z8::new(7).checked_div_floor(Z8::new(0)), None];
            /// assert_eq![Z8::MIN.checked_div_floor(Z8::new(-1)), None];
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

            /// Rounded division.
            ///
            /// Rounds the quotient to the nearest integer.
            ///
            /// # Panics
            /// If `rhs` is 0 or if the division results in overflow.
            ///
            /// # Examples
            /// ```
            /// use numera::all::Z8;
            ///
            /// assert_eq![Z8::new(7).div_round(Z8::new(3)), Z8::new(2)];
            /// assert_eq![Z8::new(7).div_round(Z8::new(-3)), Z8::new(-2)];
            /// assert_eq![Z8::new(-7).div_round(Z8::new(3)), Z8::new(-2)];
            /// assert_eq![Z8::new(-7).div_round(Z8::new(-3)), Z8::new(2)];
            /// ```
            #[inline]
            #[must_use]
            pub const fn div_round(self, rhs: [<$t$b>]) -> [<$t$b>] {
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

            /// Checked rounded division.
            ///
            /// # Examples
            /// ```
            /// # use numera::all::*;
            ///
            /// assert_eq![Z8::new(7).checked_div_round(Z8::new(3)), Some(Z8::new(2))];
            /// assert_eq![Z8::new(7).checked_div_round(Z8::new(-3)), Some(Z8::new(-2))];
            /// assert_eq![Z8::new(-7).checked_div_round(Z8::new(3)), Some(Z8::new(-2))];
            /// assert_eq![Z8::new(-7).checked_div_round(Z8::new(-3)), Some(Z8::new(2))];
            ///
            /// assert_eq![Z8::new(7).checked_div_round(Z8::new(0)), None];
            /// assert_eq![Z8::MIN.checked_div_round(Z8::new(-1)), None];
            /// ```
            #[inline]
            #[must_use]
            pub const fn checked_div_round(self, rhs: [<$t$b>]) -> Option<[<$t$b>]> {
                if rhs.0 == 0 || ((self.0 == [<$p$b>]::MIN) & (rhs.0 == -1)) {
                    None
                } else {
                    Some(self.div_round(rhs))
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
    (op_rem: $t:ident + $p:ident + $b:literal) => { paste! {
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
        ///
        /// In the context of modular arithmetic, Euclidean division is often
        /// more appropriate for mathematicians because it guarantees a
        /// non-negative remainder, ensuring a unique representative for the
        /// modulo operation reddit.com. This property is essential in various
        /// mathematical and physical applications.
        ///
        /// On the other hand, truncated division might be more intuitive in
        /// some situations, such as expressing negative durations or
        /// timestamps, where the remainder should have the same sign as the
        /// dividend reddit.com.
        impl [<$t$b>] {
            /// Truncated remained operation.
            ///
            /// This is the default remainder operation in Rust, C and Java.
            ///
            /// It is based on truncated division, rounding the quotient towards
            /// zero, meaning the remainder has the same sign as the dividend.
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
    (op_div_rem: $t:ident + $p:ident + $b:literal) => { paste! {
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

    // impl the negation operations
    //
    // impl variants:
    // - op
    // - wrapping
    (op_neg: $t:ident + $p:ident + $b:literal) => { paste! {
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
impl_integer_ops![
    Integer + i + 8,
    Integer + i + 16,
    Integer + i + 32,
    Integer + i + 64,
    Integer + i + 128
];

#[cfg(test)]
mod tests {
    use crate::all::*;

    #[test]
    fn z_ops_add() -> NumeraResult<()> {
        // op
        assert_eq![Z8::new(120) + Z8::new(7), Z8::new(127)];

        // checked
        assert_eq![Z8::new(120).checked_add(Z8::new(7)), Some(Z8::new(127))];
        assert_eq![Z8::new(120).checked_add(Z8::new(8)), None];

        // saturating
        assert_eq![Z8::new(120).saturating_add(Z8::new(8)), Z8::new(127)];

        // wrapping
        assert_eq![Z8::new(120).wrapping_add(Z8::new(8)), Z8::MIN];

        // modular
        // assert_eq![Z8::new(120).modular_add(Z8::new(8), Z8::MAX), Z8::MIN];
        // assert_eq![Z8::new(5).modular_add(Z8::new(3), Z8::new(7)), Z8::new(1)];
        // assert_eq![Z8::new(-5).modular_add(Z8::new(3), Z8::new(7)), Z8::new(-2)];
        // assert_eq![Z8::new(127).modular_add(Z8::new(2), Z8::new(50)), Z8::new(-21)]; // BAD

        // CHECK negative numbers
        // assert_eq![Z8::new(-5).modular_add(Z8::new(-3), Z8::new(-7)), Z8::new(-1)];

        // modulo_count CHECK

        // overflowing

        // PANICS
        #[cfg(feature = "std")]
        {
            use std::panic::catch_unwind;
            // basic overflow
            assert![catch_unwind(|| Z8::new(120) + Z8::new(8)).is_err()];
        }

        Ok(())
    }

    #[test]
    fn z_ops_rem() -> NumeraResult<()> {
        // rem_trunc
        assert_eq![Z16::new(-347) % Z16::new(6), Z16::new(-5)];
        // rem_euclid
        assert_eq![Z16::new(-347).rem_euclid(Z16::new(6)), Z16::new(1)];

        Ok(())
    }

    // OLD DELETE
    #[test]
    fn z_ops() -> NumeraResult<()> {
        let _5 = Z8::new(5);
        let _7 = Z8::new(7);
        assert_eq![_7 + _5, Z8::new(12)];
        assert_eq![_7 - _5, Z8::new(2)];
        assert_eq![_5 - _7, Z8::new(-2)];
        assert_eq![_7 * _5, Z8::new(35)];
        assert_eq![_7 / _5, Z8::new(1)];
        assert_eq![_5 / _7, Z8::new(0)];
        assert_eq![-_7, Z8::new(-7)];

        #[cfg(feature = "std")]
        {
            use std::panic::catch_unwind;
            // overflow
            assert![catch_unwind(|| _7 * _7 * _7).is_err()];
            // underflow
            assert![catch_unwind(|| Z8::MIN - _5).is_err()];
        }
        Ok(())
    }
}
