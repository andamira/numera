// numera::number::integer::q::impl_rational
//
//!
//

use super::*;
use crate::number::{
    integer::{n0z::*, z::*, Integer},
    rational::Rational,
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
                    self.den = $den::new(self.den.0.get() / gcd_value.0).unwrap();
                    // BENCH:ALTERNATIVE:
                    // self.den = $den::new((Into::<$num>::into(self.den) / gcd_value).0).unwrap();
                }

                #[cfg(not(feature = "safe"))]
                // SAFETY: the value returned by gcd can't be zero.
                { unsafe { self.den = $den::new_unchecked(self.den.0.get() / gcd_value.0); } }
            }

            #[inline]
            fn reduced(&self) -> Self {
                let gcd_value = self.num.gcd(&Into::<$num>::into(self.den)).unwrap();

                $t {
                    num: self.num / gcd_value,

                    #[cfg(feature = "safe")]
                    den: $den::new(self.den.0.get() / gcd_value.0).unwrap(),

                    #[cfg(not(feature = "safe"))]
                    // SAFETY: the value returned by gcd can't be zero.
                    den: unsafe { $den::new_unchecked(self.den.0.get() / gcd_value.0) },
                }
            }

            #[inline]
            fn invert(&mut self) {
                if !self.num.is_zero() {
                    let old_num = self.num;
                    self.num = self.den.into();

                    #[cfg(feature = "safe")]
                    { self.den = $den::new(old_num.0).unwrap(); }

                    #[cfg(not(feature = "safe"))]
                    // SAFETY: we've just checked the value is not 0.
                    { self.den = unsafe { $den::new_unchecked(old_num.0) }; }
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
                        den: $den::new(self.num.0).unwrap(),

                        #[cfg(not(feature = "safe"))]
                        // SAFETY: we've just checked the value is not 0.
                        den: unsafe { $den::new_unchecked(self.num.0) },
                    }
                }
            }
        }
    };
}

impl_rational![
    many: (Rational8, Integer8, NonZeroInteger8),
    (Rational16, Integer16, NonZeroInteger16),
    (Rational32, Integer32, NonZeroInteger32),
    (Rational64, Integer64, NonZeroInteger64),
    (Rational128, Integer128, NonZeroInteger128)
];

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: test Rational
    // #[test]
    // fn q_rational() {
    // }
}
