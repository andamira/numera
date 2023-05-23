// numera::number::integer::q::ops
//
//!
//

use crate::number::{rational::*, traits::Ident};
use core::ops::{Add, Div, Mul, Neg, Rem, Sub};
use devela::paste;

// impl ops (which panic on overflow)
macro_rules! impl_rational_ops {
    // impl all ops for multiple integer types
    (
        $( $t:ident + $b:literal, cast: $bcast:literal);+
    ) => {
        $(
            impl_rational_ops![Add: $t + $b, cast: $bcast];
            impl_rational_ops![Sub: $t + $b, cast: $bcast];
            impl_rational_ops![Mul: $t + $b, cast: $bcast];
            impl_rational_ops![Div: $t + $b, cast: $bcast];
            impl_rational_ops![Rem: $t + $b, cast: $bcast];
            impl_rational_ops![Neg: $t + $b, cast: $bcast];
        )+
    };

    // impl Add for a single rational
    //
    // $t: rational type. e.g. Rational8
    (Add: $t:ident + $b:literal, cast: $bcast:literal) => { paste! {
        impl Add for [<$t$b>] {
            type Output = Self;

            /// The addition operator `+`.
            ///
            /// The operands are upcasted for this operation to the next larger
            /// bit-size (except for 128-bit), and the result is reduced before
            /// trying to downcast it to the original bit-size.
            ///
            /// # Panics
            /// If the result can't fit the current bit-size.
            fn add(self, other: Self::Output) -> Self::Output {
                let cself = [<$t$bcast>]::from(self);
                let cother = [<$t$bcast>]::from(other);

                let num = cself.num * cother.den.into() + cother.num * cself.den.into();
                let den = cself.den * cother.den;

                let creduced = [<$t$bcast>] { num, den }.reduced();

                #[cfg(feature = "try_from")]
                return creduced.try_into().unwrap();

                #[cfg(not(feature = "try_from"))]
                return [<$t$b>]::new(
                    creduced.num.0.try_into().unwrap(),
                    creduced.den.0.get().try_into().unwrap()
                ).unwrap();
            }
        }
    }};

    (Sub: $t:ident + $b:literal, cast: $bcast:literal) => { paste! {
        impl Sub for [<$t$b>] {
            type Output = Self;

            /// The substraction operator `-`.
            ///
            /// The operands are upcasted for this operation to the next larger
            /// bit-size (except for 128-bit), and the result is reduced before
            /// trying to downcast it to the original bit-size.
            ///
            /// # Panics
            /// If the result can't fit the current bit-size.
            fn sub(self, other: Self) -> Self::Output {
                let cself = [<$t$bcast>]::from(self);
                let cother = [<$t$bcast>]::from(other);

                let num = cself.num * cother.den.into() - cother.num * cself.den.into();
                let den = cself.den * cother.den;

                let creduced = [<$t$bcast>] { num, den }.reduced();

                #[cfg(feature = "try_from")]
                return creduced.try_into().unwrap();

                #[cfg(not(feature = "try_from"))]
                return [<$t$b>]::new(
                    creduced.num.0.try_into().unwrap(),
                    creduced.den.0.get().try_into().unwrap()
                ).unwrap();
            }
        }
    }};

    (Mul: $t:ident + $b:literal, cast: $bcast:literal) => { paste! {
        impl Mul for [<$t$b>] {
            type Output = Self;

            /// The multiplication operator `*`.
            ///
            /// The operands are upcasted for this operation to the next larger
            /// bit-size (except for 128-bit), and the result is reduced before
            /// trying to downcast it to the original bit-size.
            ///
            /// # Panics
            /// If the result can't fit the current bit-size.
            fn mul(self, other: Self) -> Self::Output {
                let cself = [<$t$bcast>]::from(self);
                let cother = [<$t$bcast>]::from(other);

                let num = cself.num * cother.num;
                let den = cself.den * cother.den;

                let creduced = [<$t$bcast>] { num, den }.reduced();

                #[cfg(feature = "try_from")]
                return creduced.try_into().unwrap();

                #[cfg(not(feature = "try_from"))]
                return [<$t$b>]::new(
                    creduced.num.0.try_into().unwrap(),
                    creduced.den.0.get().try_into().unwrap()
                ).unwrap();
            }
        }
    }};

    (Div: $t:ident + $b:literal, cast: $bcast:literal) => { paste! {
        impl Div for [<$t$b>] {
            type Output = Self;

            /// The division operator `/`.
            ///
            /// The operands are upcasted for this operation to the next larger
            /// bit-size (except for 128-bit), and the result is reduced before
            /// trying to downcast it to the original bit-size.
            ///
            /// # Panics
            /// If the result can't fit the current bit-size.
            fn div(self, other: Self) -> Self::Output {
                let cself = [<$t$bcast>]::from(self);
                let cother = [<$t$bcast>]::from(other);

                let num = cself.num * cother.den.into();
                let den = cother.num * cself.den.into();

                if den.is_zero() {
                    unreachable![] // IMPROVE: use hint
                } else {
                    #[cfg(feature = "try_from")]
                    let den = den.try_into().unwrap();

                    #[cfg(not(feature = "try_from"))]
                    let den = core::num::[<NonZeroI$bcast>]::new(den.0).unwrap().into();

                    let creduced = [<$t$bcast>] { num, den }.reduced();

                    #[cfg(feature = "try_from")]
                    return creduced.try_into().unwrap();

                    #[cfg(not(feature = "try_from"))]
                    return [<$t$b>]::new(
                        creduced.num.0.try_into().unwrap(),
                        creduced.den.0.get().try_into().unwrap()
                    ).unwrap();
                }
            }
        }
    }};

    (Rem: $t:ident + $b:literal, cast: $bcast:literal) => { paste! {
        impl Rem for [<$t$b>] {
            type Output = Self;

            /// The remainder operator `%` (using truncated division)
            ///
            /// The operands are upcasted for this operation to the next larger
            /// bit-size (except for 128-bit), and the result is reduced before
            /// trying to downcast it to the original bit-size.
            ///
            /// # Panics
            /// If the result can't fit the current bit-size.
            fn rem(self, other: Self) -> Self::Output {
                let cself = [<$t$bcast>]::from(self);
                let cother = [<$t$bcast>]::from(other);

                let lhs_num = cself.num * cother.den.into();
                let rhs_num = cother.num * cself.den.into();
                let num = (lhs_num % rhs_num);
                let den = cself.den * cother.den;

                #[cfg(feature = "try_from")]
                return [<$t$bcast>] { num, den }.reduced().try_into().unwrap();

                #[cfg(not(feature = "try_from"))]
                return [<$t$b>]::new(
                    num.0.try_into().unwrap(),
                    den.0.get().try_into().unwrap()
                ).unwrap().reduced();
            }
        }
    }};

    (Neg: $t:ident + $b:literal, cast: $bcast:literal) => { paste! {
        impl Neg for [<$t$b>] {
            type Output = Self;

            /// The negation operator `-`.
            ///
            /// The result is reduced.
            fn neg(self) -> Self::Output {
                Self {
                    num: self.num.neg(),
                    den: self.den,
                }
                .reduced()
            }
        }
    }};
}
impl_rational_ops![
    Rational+8, cast: 16;
    Rational+16, cast: 32;
    Rational+32, cast: 64;
    Rational+64, cast: 128;
    Rational+128, cast: 128
];

#[cfg(test)]
mod tests {
    use crate::all::*;

    #[test]
    fn q_ops() -> NumeraResult<()> {
        let _5 = Q8::new(5, 1)?;
        let _7 = Q8::new(7, 1)?;

        // Neg
        assert_eq![-Q8::new(5, 1)?, Q8::new(-5, 1)?];
        assert_eq![-Q8::new(-5, 1)?, Q8::new(5, 1)?];
        assert_eq![-Q8::new(5, -1)?, Q8::new(5, 1)?];

        // Add
        assert_eq![Q8::new(5, 1)? + Q8::new(7, 1)?, Q8::new(12, 1)?];
        assert_eq![Q8::new(1, 5)? + Q8::new(1, 7)?, Q8::new(12, 35)?];
        assert_eq![Q8::new(2, 7)? + Q8::new(3, 8)?, Q8::new(37, 56)?];
        assert_eq![Q8::new(15, 32)? + Q8::new(27, 9)?, Q8::new(111, 32)?];

        // Sub
        assert_eq![Q8::new(12, 1)? - Q8::new(7, 1)?, Q8::new(5, 1)?];
        assert_eq![Q16::new(12, 35)? - Q16::new(1, 7)?, Q16::new(1, 5)?];
        assert_eq![Q16::new(37, 56)? - Q16::new(3, 8)?, Q16::new(2, 7)?];
        assert_eq![Q16::new(111, 32)? - Q16::new(27, 9)?, Q16::new(15, 32)?];

        // Mul
        assert_eq![Q8::new(12, 1)? * Q8::new(7, 1)?, Q8::new(84, 1)?];
        assert_eq![Q16::new(2, 7)? * Q16::new(3, 8)?, Q16::new(3, 28)?];
        assert_eq![Q16::new(11, 5)? * Q16::new(4, 9)?, Q16::new(44, 45)?];

        // Div
        assert_eq![Q8::new(84, 1)? / Q8::new(7, 1)?, Q8::new(12, 1)?];
        assert_eq![Q16::new(3, 28)? / Q16::new(3, 8)?, Q16::new(2, 7)?];
        assert_eq![Q16::new(44, 45)? / Q16::new(4, 9)?, Q16::new(11, 5)?];

        // Rem
        assert_eq![Q8::new(12, 1)? % Q8::new(7, 1)?, Q8::new(5, 1)?];
        assert_eq![Q16::new(12, 35)? % Q16::new(1, 7)?, Q16::new(2, 35)?];
        assert_eq![Q16::new(44, 45)? % Q16::new(4, 9)?, Q16::new(4, 45)?];

        // #[cfg(feature = "std")]
        // {
        //     use std::panic::catch_unwind;
        //     let a = Q8::new(125, 13).unwrap();
        //     let b = Q8::new(2, 26).unwrap();
        //
        //     // overflow
        //     assert![catch_unwind(|| a + b).is_err()];
        // }
        Ok(())
    }
}
