// numera::number::rational::q::define_sized
//
//!
//
// TOC
//
// - macro
//   - define_rational_sized
// - definitions
//   - Rational[8|16|32|64|128]

use crate::{
    error::{NumeraResult, RationalError},
    number::{
        integer::n0z::*,
        integer::z::*,
        traits::{
            Bound, ConstLowerBounded, ConstNegOne, ConstOne, ConstUpperBounded, ConstZero, Count,
            Countable, Ident, LowerBounded, NegOne, Number, One, Sign, Signed, UpperBounded, Zero,
        },
    },
};
use core::{fmt, ops::Neg};

/* macro */

/// # What it does
/// - defines a Rational of a concrete size.
/// - implements Number: Bound + Count + Ident + Sign
/// - implements Default → 0
///
/// # Args
/// - `$name`: the base name of the rational. e.g. `Rational`.
/// - `$p`: the primitive prefix (i or u).
///
/// - `$doc_num`: the type of number.
/// - `$doc_type`: adds to the type doc-comment.
// - `$doc_new`: adds to the `new` constructor doc-comment.
///
/// - `$doc_sign`: an optional negative sign
/// - `$doc_lower`: the lower bound of the number type.
/// - `$doc_upper`: the upper bound of the number type.
///
/// - `$num`: the integer base type for the numerator. e.g. `Integer`.
/// - `$den`: the integer base type for the denominator. e.g. `NonZeroInteger`.
///
/// grouped:
/// - `$doc_det`: the determinant before the bit size. e.g. "An" (8-bit) or "A" 16-bit.
/// - `$bsize`: the size in bits of the primitive used.
macro_rules! define_rational_sized {
    // defines multiple integer types, with an inner primitive.
    (multi $name:ident, $p:ident,
     $doc_num:literal, $doc_type:literal, // $doc_new:literal,
     $sign:literal, $lower:expr, $upper:expr,
     $num:ident, $den:ident,
     $(($doc_det:literal,$bsize:expr)),+) => {
        $(
            define_rational_sized![single $name, $p,
               $doc_num, $doc_type, // $doc_new,
               $sign, $lower, $upper,
               $num, $den,
               ($doc_det,$bsize)];
        )+
    };
    // defines a single integer type, with an inner primitive.
    (single $name:ident, $p:ident,
     $doc_num:literal, $doc_type:literal, // $doc_new:literal,
     $doc_sign:literal, $doc_lower:expr, $doc_upper:expr,
     $num:ident, $den:ident,
     ($doc_det:literal,$bsize:expr)) => {

        paste::paste! {
            #[doc = $doc_det " "$bsize "-bit " $doc_num $doc_type]
            #[doc = "\n\nThe range of valid numeric values is $\\lbrack"
            $doc_sign "$[`" $p$bsize "::" $doc_lower "`] $\\dots$ [`"
            $p$bsize "::" $doc_upper "`]$\\rbrack$."]

            #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
            pub struct [<$name$bsize>] {
                pub(crate) num: [<$num$bsize>],
                pub(crate) den: [<$den$bsize>],
            }

            impl Default for [<$name$bsize>] {
                fn default() -> Self {
                    Self {
                        num: [<$num$bsize>]::ZERO,
                        den: [<$den$bsize>]::ONE,
                    }
                }
            }

            impl fmt::Display for [<$name$bsize>]  {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "{}/{}", self.num, self.den)
                }
            }

            /* sign */

            impl Sign for [<$name$bsize>] {
                #[inline]
                fn can_negative(&self) -> bool { true }
                #[inline]
                fn can_positive(&self) -> bool { true }
                #[inline]
                fn is_negative(&self) -> bool {
                    self.num.is_negative() && self.den.is_positive() ||
                    self.num.is_positive() && self.den.is_negative()
                }
                #[inline]
                fn is_positive(&self) -> bool {
                    self.num.is_negative() && self.den.is_negative() ||
                    self.num.is_positive() && self.den.is_positive()
                }
            }
            impl Signed for [<$name$bsize>] {}

            /* bound */

            impl Bound for [<$name$bsize>] {
                #[inline]
                fn is_lower_bounded(&self) -> bool { true }
                #[inline]
                fn is_upper_bounded(&self) -> bool { true }
                #[inline]
                fn lower_bound(&self) -> Option<Self> {
                    Some(<Self as ConstLowerBounded>::MIN)
                }
                #[inline]
                fn upper_bound(&self) -> Option<Self> {
                    Some(<Self as ConstUpperBounded>::MAX)
                }
            }
            impl LowerBounded for [<$name$bsize>] {
                #[inline]
                fn new_min() -> Self { <Self as ConstLowerBounded>::MIN }
            }
            impl UpperBounded for [<$name$bsize>] {
                #[inline]
                fn new_max() -> Self { <Self as ConstUpperBounded>::MAX }
            }
            impl ConstLowerBounded for [<$name$bsize>] {
                const MIN: Self = Self {
                        num: [<$num$bsize>]::MIN,
                        den: [<$den$bsize>]::ONE,
                    };
            }
            impl ConstUpperBounded for [<$name$bsize>] {
                const MAX: Self = Self {
                        num: [<$num$bsize>]::MAX,
                        den: [<$den$bsize>]::ONE,
                    };
            }

            /* count */

            impl Count for [<$name$bsize>] {
                #[inline]
                fn is_countable(&self) -> bool { true }
            }

            impl Countable for [<$name$bsize>] {
                /// Returns the next rational value maintaining the same
                /// denominator, increasing only the numerator.
                #[inline]
                fn next(&self) -> NumeraResult<Self> {
                    Ok(Self {
                        num: self.num.0.checked_add(1)
                            .ok_or(RationalError::NumeratorOverflow)?.into(),
                        den: self.den,
                    })
                }
                /// Returns the previous rational value maintaining the same
                /// denominator, increasing only the numerator.
                #[inline]
                fn previous(&self) -> NumeraResult<Self> {
                    Ok(Self {
                        num: self.num.0.checked_sub(1)
                            .ok_or(RationalError::NumeratorUnderflow)?.into(),
                        den: self.den,
                    })
                }
            }

            /* ident */

            impl Ident for [<$name$bsize>] {
                #[inline]
                fn can_zero(&self) -> bool { true }
                #[inline]
                fn can_one(&self) -> bool { true }
                #[inline]
                fn can_neg_one(&self) -> bool { true }

                #[inline]
                fn is_zero(&self) -> bool { self.num.is_zero() }
                #[inline]
                fn is_one(&self) -> bool { self.num == self.den.into() }
                #[inline]
                fn is_neg_one(&self) -> bool { self.num.neg() == self.den.into() }
            }
            impl ConstZero for [<$name$bsize>] {
                const ZERO: Self = Self {
                    num: [<$num$bsize>]::ZERO,
                    den: [<$den$bsize>]::ONE,
                };
            }
            impl Zero for [<$name$bsize>] {
                #[inline]
                fn new_zero() -> Self { <Self as ConstZero>::ZERO }
            }
            impl ConstOne for [<$name$bsize>] {
                const ONE: Self = Self {
                    num: [<$num$bsize>]::ONE,
                    den: [<$den$bsize>]::ONE,
                };
            }
            impl One for [<$name$bsize>] {
                #[inline]
                fn new_one() -> Self { <Self as ConstOne>::ONE }
            }
            impl ConstNegOne for [<$name$bsize>] {
                const NEG_ONE: Self = Self {
                    num: [<$num$bsize>]::NEG_ONE,
                    den: [<$den$bsize>]::ONE,
                };
            }
            impl NegOne for [<$name$bsize>] {
                #[inline]
                fn new_neg_one() -> Self { <Self as ConstNegOne>::NEG_ONE }
            }

            /* number */

            impl Number for [<$name$bsize>] {
                type Inner = ([<$p$bsize>], [<$p$bsize>]);

                /// Returns a new rational.
                ///
                /// # Errors
                /// If the denominator (`value.1`) is 0.
                #[inline]
                fn new(value: Self::Inner) -> NumeraResult<Self> {
                    Ok(
                        Self {
                            num: [<$num$bsize>]::new(value.0)?,
                            den: [<$den$bsize>]::new(value.1)
                                .map_err(|_| RationalError::ZeroDenominator)?,
                        }
                    )
                }

                #[inline]
                #[cfg(not(feature = "safe"))]
                #[cfg_attr(feature = "nightly", doc(cfg(feature = "non-safe")))]
                unsafe fn new_unchecked(value: Self::Inner) -> Self {
                    Self {
                        num: [<$num$bsize>]::new_unchecked(value.0),
                        den: [<$den$bsize>]::new_unchecked(value.1),
                    }
                }
            }
        }
    };
}

/* definitions */

define_rational_sized![multi Rational, i, // CHECK
    "rational number", ", from the set $\\Bbb{Q}$.",
    // "",
    "", MIN, MAX,
    Integer, NonZeroInteger,
    ("A 2×", 8), ("A 2×", 16), ("A 2×", 32), ("A 2×", 64), ("A 2x", 128)
];

#[cfg(test)]
mod tests {
    use crate::all::*;

    #[test]
    fn q_define_sized() -> NumeraResult<()> {
        assert_eq![
            Rational8::new((5, 0)),
            Err(RationalError::ZeroDenominator.into())
        ];

        let _q5 = Rational8::new((5, 1))?;

        // Display
        assert_eq![_q5.to_string(), "5/1"];

        // Bound
        // Count
        // Ident
        // Sign

        // #[cfg(feature = "std")]
        // {
        //     use std::panic::catch_unwind;
        //     // 0 denominator
        //     assert![catch_unwind(|| Rational8::new((5, 0)).unwrap()).is_err()];
        // }

        Ok(())
    }
}
