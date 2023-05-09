// numera::number::integer::q::impl_ops
//
//!
//

use crate::number::{
    rational::{Rational, *},
    traits::Ident,
};
use core::ops::{Add, Div, Mul, Neg, Rem, Sub};
use devela::paste;

// impl ops (which panic on overflow)
macro_rules! impl_rational_ops {
    // impl all ops for multiple integer types
    (
        $( $t:ident + $bsize:literal, cast: $castbsize:literal);+
    ) => {
        $(
            impl_rational_ops![Add: $t + $bsize, cast: $castbsize];
            impl_rational_ops![Sub: $t + $bsize, cast: $castbsize];
            impl_rational_ops![Mul: $t + $bsize, cast: $castbsize];
            impl_rational_ops![Div: $t + $bsize, cast: $castbsize];
            impl_rational_ops![Rem: $t + $bsize, cast: $castbsize];
            impl_rational_ops![Neg: $t + $bsize, cast: $castbsize];
        )+
    };

    // impl Add for a single rational
    //
    // $t: rational type. e.g. Rational8
    (Add: $t:ident + $bsize:literal, cast: $castbsize:literal) => { paste! {
        impl Add for [<$t$bsize>] {
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
                let cself = [<$t$castbsize>]::from(self);
                let cother = [<$t$castbsize>]::from(other);

                let num = cself.num * cother.den.into() + cother.num * cself.den.into();
                let den = cself.den * cother.den;

                let cresult = [<$t$castbsize>] { num, den }.reduced();
                cresult.try_into().unwrap()
            }
        }
    }};

    (Sub: $t:ident + $bsize:literal, cast: $castbsize:literal) => { paste! {
        impl Sub for [<$t$bsize>] {
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
                let cself = [<$t$castbsize>]::from(self);
                let cother = [<$t$castbsize>]::from(other);

                let num = cself.num * cother.den.into() - cother.num * cself.den.into();
                let den = cself.den * cother.den;

                let cresult = [<$t$castbsize>] { num, den }.reduced();
                cresult.try_into().unwrap()
            }
        }
    }};

    (Mul: $t:ident + $bsize:literal, cast: $castbsize:literal) => { paste! {
        impl Mul for [<$t$bsize>] {
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
                let cself = [<$t$castbsize>]::from(self);
                let cother = [<$t$castbsize>]::from(other);

                let num = cself.num * cother.num;
                let den = cself.den * cother.den;

                let cresult = [<$t$castbsize>] { num, den }.reduced();
                cresult.try_into().unwrap()
            }
        }
    }};

    (Div: $t:ident + $bsize:literal, cast: $castbsize:literal) => { paste! {
        impl Div for [<$t$bsize>] {
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
                let cself = [<$t$castbsize>]::from(self);
                let cother = [<$t$castbsize>]::from(other);

                let num = cself.num * cother.den.into();
                let den = cother.num * cself.den.into();

                if den.is_zero() {
                    unreachable![] // IMPROVE: use hint
                } else {
                    let cresult = [<$t$castbsize>] {
                        num,
                        den: den.try_into().unwrap()
                    }.reduced();

                    cresult.try_into().unwrap()
                }
            }
        }
    }};

    (Rem: $t:ident + $bsize:literal, cast: $castbsize:literal) => { paste! {
        impl Rem for [<$t$bsize>] {
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
                let cself = [<$t$castbsize>]::from(self);
                let cother = [<$t$castbsize>]::from(other);

                let lhs_num = cself.num * cother.den.into();
                let rhs_num = cother.num * cself.den.into();
                let num = (lhs_num % rhs_num);
                let den = cself.den * cother.den;

                let cresult = [<$t$castbsize>] { num, den }.reduced();
                cresult.try_into().unwrap()
            }
        }
    }};

    (Neg: $t:ident + $bsize:literal, cast: $castbsize:literal) => { paste! {
        impl Neg for [<$t$bsize>] {
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
    use crate::all::{abbr::*, NumeraResult};

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
