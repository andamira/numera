// numera::number::integer::q::impl_ops
//
//!
//

use crate::all::{abbr::*, Rational};
use core::ops::{Add, Div, Mul, Neg, Rem, Sub};

// impl ops (which panic on overflow)
macro_rules! impl_rational_ops {
    // impl all ops for multiple integer types
    ($($t:ident),+) => {
        $(
            impl_rational_ops![Add: $t];
            impl_rational_ops![Sub: $t];
            impl_rational_ops![Mul: $t];
            // impl_rational_ops![Div: $t]; // TODO FIX
            impl_rational_ops![Neg: $t];
        )+
    };

    // impl Add for a single rational
    //
    // $t: rational type. e.g. Rational8
    (Add: $t:ident) => {
        impl Add for $t {
            type Output = Self;

            /// The addition operator `+`.
            ///
            /// Note that both operands are reduced before the operation,
            /// and the result is reduced afterwards.
            fn add(self, other: Self::Output) -> Self::Output {
                let rself = self.reduced();
                let rother = other.reduced();

                let num = rself.num * rother.den.into() + rother.num * rself.den.into();
                let den = rself.den * rother.den;
                Self { num, den }.reduced()
            }
        }
    };

    (Sub: $t:ident) => {
        impl Sub for $t {
            type Output = Self;

            /// The substraction operator `-`.
            ///
            /// Note that both operands are reduced before the operation,
            /// and the result is reduced afterwards.
            fn sub(self, other: Self) -> Self::Output {
                let rself = self.reduced();
                let rother = other.reduced();

                let num = rself.num * rother.den.into() - rother.num * rself.den.into();
                let den = rself.den * rother.den;
                Self { num, den }.reduced()
            }
        }
    };

    (Mul: $t:ident) => {
        impl Mul for $t {
            type Output = Self;

            /// The multiplication operator `*`.
            ///
            /// Note that both operands are reduced before the operation,
            /// and the result is reduced afterwards.
            fn mul(self, other: Self) -> Self::Output {
                let rself = self.reduced();
                let rother = other.reduced();

                let num = rself.num * rother.num;
                let den = rself.den * rother.den;
                Self { num, den }.reduced()
            }
        }
    };

    (Div: $t:ident) => {
        impl Div for $t {
            type Output = Self;

            /// The division operator `/`.
            ///
            /// Note that both operands are reduced before the operation,
            /// and the result is reduced afterwards.
            fn div(self, other: Self) -> Self::Output {
                let rself = self.reduced();
                let rother = other.reduced();

                let num = rself.num * rother.den.into();
                let den = rother.num * rself.den.into();
                if den.is_zero() {
                    panic![]
                } else {
                    // TODO FIX try_into
                    Self { num, den: den.try_into().unwrap() }.reduced()
                }
            }
        }
    };

    (Neg: $t:ident) => {
        impl Neg for $t {
            type Output = Self;

            /// Negates `self` and reduces the result.
            fn neg(self) -> Self::Output {
                Self {
                    num: self.num.neg(),
                    den: self.den,
                }.reduced()

            }
        }
    };
}
impl_rational_ops![Q8, Q16, Q32, Q64, Q128];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::all::{Number, NumeraResult};

    #[test]
    fn q_ops() -> NumeraResult<()> {
        let _5 = Q8::new((5, 1))?;
        let _7 = Q8::new((7, 1))?;

        // Neg
        assert_eq![-Q8::new((5, 1))?, Q8::new((-5, 1))?];
        assert_eq![-Q8::new((-5, 1))?, Q8::new((5, 1))?];
        assert_eq![-Q8::new((5, -1))?, Q8::new((5, 1))?];

        // Add
        assert_eq![Q8::new((5, 1))? + Q8::new((7, 1))?, Q8::new((12, 1))?];
        assert_eq![Q8::new((1, 5))? + Q8::new((1, 7))?, Q8::new((12, 35))?];
        assert_eq![Q8::new((2, 7))? + Q8::new((3, 8))?, Q8::new((37, 56))?];
        assert_eq![Q8::new((15, 32))? + Q8::new((27, 9))?, Q8::new((111, 32))?];

        // Sub
        assert_eq![Q8::new((12, 1))? - Q8::new((7, 1))?, Q8::new((5, 1))?];
        assert_eq![Q16::new((12, 35))? - Q16::new((1, 7))?, Q16::new((1, 5))?];
        assert_eq![Q16::new((37, 56))? - Q16::new((3, 8))?, Q16::new((2, 7))?];
        assert_eq![
            Q16::new((111, 32))? - Q16::new((27, 9))?,
            Q16::new((15, 32))?
        ];

        // Mul
        assert_eq![Q8::new((12, 1))? * Q8::new((7, 1))?, Q8::new((84, 1))?];
        assert_eq![Q16::new((2, 7))? * Q16::new((3, 8))?, Q16::new((3, 28))?];
        assert_eq![Q16::new((11, 5))? * Q16::new((4, 9))?, Q16::new((44, 45))?];

        // Div
        // TODO

        #[cfg(feature = "std")]
        {
            use std::panic::catch_unwind;
            let a = Q8::new((125, 13)).unwrap();
            let b = Q8::new((2, 26)).unwrap();

            // overflow
            assert![catch_unwind(|| a + b).is_err()];
        }
        Ok(())
    }
}
