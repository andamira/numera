// numera::number::integer::q::rational
//
//!
//

use crate::number::{
    integer::*,
    rational::*,
    traits::{ConstOne, ConstZero, Ident},
};

/// Implements the `Rational` trait on sized rationals.
macro_rules! impl_rational {
    // Args:
    // $(
    //   `$t`: the type of the rational. E.g. Rational16.
    //   `$num`: the type of the numerator. E.g. Integer16.
    //   `$den`: the type of the denominator. E.g. NonZeroInteger16.
    // ),+
    (many: $(($t:ident, $num:ident, $den:ident)),+) => {
        $( impl_rational![single: $t, $num, $den]; )+
    };

    // Args:
    //   `$t`: the type of the rational. E.g. Rational16.
    //   `$num`: the type of the numerator. E.g. Integer16.
    //   `$den`: the type of the denominator. E.g. NonZeroInteger16.
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
                self.num.integer_gcd(&Into::<$num>::into(self.den)).unwrap() == $num::ONE
            }
            #[inline]
            fn reduce(&mut self) {
                let gcd_value = self.num.integer_gcd(&Into::<$num>::into(self.den)).unwrap();

                self.num /= gcd_value;

                // RETHINK safe/unsafe
                // #[cfg(feature = "safe")]
                {
                    self.den = $den::new(self.den.0.get() / gcd_value.0).unwrap();
                    // BENCH:ALTERNATIVE:
                    // self.den = $den::from_parts((Into::<$num>::into(self.den) / gcd_value).0).unwrap();
                }

                // RETHINK safe/unsafe
                // #[cfg(not(feature = "safe"))]
                // // SAFETY: the value returned by gcd can't be zero.
                // { unsafe { self.den = $den::from_parts_unchecked(self.den.0.get() / gcd_value.0); } }
            }

            #[inline]
            fn reduced(&self) -> Self {
                let gcd_value = self.num.integer_gcd(&Into::<$num>::into(self.den)).unwrap();

                $t {
                    num: self.num / gcd_value,

                    // RETHINK safe/unsafe
                    // #[cfg(feature = "safe")]
                    den: $den::new(self.den.0.get() / gcd_value.0).unwrap(),

                    // RETHINK safe/unsafe
                    // #[cfg(not(feature = "safe"))]
                    // // SAFETY: the value returned by gcd can't be zero.
                    // den: unsafe { $den::from_parts_unchecked(self.den.0.get() / gcd_value.0) },
                }
            }

            #[inline]
            fn invert(&mut self) {
                if !self.num.is_zero() {
                    let old_num = self.num;
                    self.num = self.den.into();

                    // RETHINK safe/unsafe
                    // #[cfg(feature = "safe")]
                    { self.den = $den::new(old_num.0).unwrap(); }

                    // RETHINK safe/unsafe
                    // #[cfg(not(feature = "safe"))]
                    // // SAFETY: we've just checked the value is not 0.
                    // { self.den = unsafe { $den::from_parts_unchecked(old_num.0) }; }
                }
            }
            #[inline]
            fn inverted(&self) -> Self {
                if self.num.is_zero() {
                    *self
                } else {
                    $t {
                        num: self.den.into(),

                        // RETHINK safe/unsafe
                        // #[cfg(feature = "safe")]
                        // den: $den::from_parts(self.num.0).unwrap(),
                        den: $den::new(self.num.0).unwrap(),

                        // RETHINK safe/unsafe
                        // #[cfg(not(feature = "safe"))]
                        // // SAFETY: we've just checked the value is not 0.
                        // den: unsafe { $den::from_parts_unchecked(self.num.0) },
                    }
                }
            }
        }
    };
}

impl_rational![
    many: (Rational8, Z8, N0z8),
    (Rational16, Z16, N0z16),
    (Rational32, Z32, N0z32),
    (Rational64, Z64, N0z64),
    (Rational128, Z128, N0z128)
];

#[cfg(test)]
mod tests {
    use crate::all::*;

    #[test]
    fn q_rational() -> NumeraResult<()> {
        // Integer
        //
        // true
        assert![Q8::new(0, 7)?.is_integer()];
        assert![Q8::new(1, 1)?.is_integer()];
        assert![Q8::new(14, 1)?.is_integer()];
        assert![Q8::new(32, 8)?.is_integer()];
        // false
        assert![!Q8::new(1, 2)?.is_integer()];
        assert![!Q8::new(32, 9)?.is_integer()];

        // Reduce
        //
        // true
        assert![Q8::new(3, 14)?.is_reduced()];
        // false
        assert![!Q8::new(21, 98)?.is_reduced()];
        //
        assert_eq![Q8::new(21, 98)?.reduced(), Q8::new(3, 14)?];
        assert_eq![Q8::new(0, 98)?.reduced(), Q8::new(0, 1)?];

        // Invert
        //
        assert_eq![Q8::new(21, 98)?.inverted(), Q8::new(98, 21)?];
        assert_eq![Q8::new(0, 5)?.inverted(), Q8::new(0, 5)?];

        Ok(())
    }
}
