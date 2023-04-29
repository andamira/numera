// numera::number::integer::q::impl_rational
//
//!
//

use crate::number::{
    integer::{abbr::*, Integer},
    rational::{abbr::*, Rational},
    traits::{ConstOne, ConstZero, Ident, Number},
};

/// Implements the `Rational` trait on sized rationals.
macro_rules! impl_rational {
    // Args:
    // `$t`: the type of the rational
    // `$num`: the type of the numerator
    // `$den`: the type of the denominator
    (many: $(($t:ident, $num:ident, $den:ident)),+) => {
        $( impl_rational![single: $t, $num, $den]; )+
    };

    (single: $t:ident, $num:ident, $den:ident) => {
        impl Rational for $t {
            #[inline]
            fn is_integer(&self) -> bool {
                self.num % self.den.into() == $num::ZERO
            }
            #[inline]
            fn is_proper(&self) -> bool {
                // IMPROVE when impl abs for Signed
                self.num.0.abs() < self.den.0.get().abs()
            }
            #[inline]
            fn is_reduced(&self) -> bool {
                // IMPROVE when impl From for references.
                // self.num.gcd(self.den.into()).unwrap() == $num::ONE
                self.num.gcd(&Into::<$num>::into(self.den)).unwrap() == $num::ONE
            }
            #[inline]
            fn reduce(&mut self) {
                let gcd_value = self.num.gcd(&Into::<$num>::into(self.den)).unwrap();

                // self.num /= gcd_value; // IMPROVE: impl DivAssign
                self.num = self.num / gcd_value;

                #[cfg(feature = "safe")]
                {
                    self.den = $den::from_parts(self.den.0.get() / gcd_value.0).unwrap();
                    // BENCH:ALTERNATIVE:
                    // self.den = $den::from_parts((Into::<$num>::into(self.den) / gcd_value).0).unwrap();
                }

                #[cfg(not(feature = "safe"))]
                // SAFETY: the value returned by gcd can't be zero.
                { unsafe { self.den = $den::from_parts_unchecked(self.den.0.get() / gcd_value.0); } }
            }

            #[inline]
            fn reduced(&self) -> Self {
                let gcd_value = self.num.gcd(&Into::<$num>::into(self.den)).unwrap();

                $t {
                    num: self.num / gcd_value,

                    #[cfg(feature = "safe")]
                    den: $den::from_parts(self.den.0.get() / gcd_value.0).unwrap(),

                    #[cfg(not(feature = "safe"))]
                    // SAFETY: the value returned by gcd can't be zero.
                    den: unsafe { $den::from_parts_unchecked(self.den.0.get() / gcd_value.0) },
                }
            }

            #[inline]
            fn invert(&mut self) {
                if !self.num.is_zero() {
                    let old_num = self.num;
                    self.num = self.den.into();

                    #[cfg(feature = "safe")]
                    { self.den = $den::from_parts(old_num.0).unwrap(); }

                    #[cfg(not(feature = "safe"))]
                    // SAFETY: we've just checked the value is not 0.
                    { self.den = unsafe { $den::from_parts_unchecked(old_num.0) }; }
                }
            }
            #[inline]
            fn inverted(&self) -> Self {
                if self.num.is_zero() {
                    *self
                } else {
                    $t {
                        num: self.den.into(),

                        #[cfg(feature = "safe")]
                        den: $den::from_parts(self.num.0).unwrap(),

                        #[cfg(not(feature = "safe"))]
                        // SAFETY: we've just checked the value is not 0.
                        den: unsafe { $den::from_parts_unchecked(self.num.0) },
                    }
                }
            }
        }
    };
}

impl_rational![
    many: (Q8, Z8, N0z8),
    (Q16, Z16, N0z16),
    (Q32, Z32, N0z32),
    (Q64, Z64, N0z64),
    (Q128, Z128, N0z128)
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::all::{Number, NumeraResult};

    #[test]
    fn q_rational() -> NumeraResult<()> {
        // Integer
        //
        // true
        assert![Q8::from_parts((0, 7))?.is_integer()];
        assert![Q8::from_parts((1, 1))?.is_integer()];
        assert![Q8::from_parts((14, 1))?.is_integer()];
        assert![Q8::from_parts((32, 8))?.is_integer()];
        // false
        assert![!Q8::from_parts((1, 2))?.is_integer()];
        assert![!Q8::from_parts((32, 9))?.is_integer()];

        // Reduce
        //
        // true
        assert![Q8::from_parts((3, 14))?.is_reduced()];
        // false
        assert![!Q8::from_parts((21, 98))?.is_reduced()];
        //
        assert_eq![
            Q8::from_parts((21, 98))?.reduced(),
            Q8::from_parts((3, 14))?
        ];
        assert_eq![Q8::from_parts((0, 98))?.reduced(), Q8::from_parts((0, 1))?];

        // Invert
        //
        assert_eq![
            Q8::from_parts((21, 98))?.inverted(),
            Q8::from_parts((98, 21))?
        ];
        assert_eq![Q8::from_parts((0, 5))?.inverted(), Q8::from_parts((0, 5))?];

        Ok(())
    }
}
