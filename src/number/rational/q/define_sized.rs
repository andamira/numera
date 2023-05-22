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

#[cfg(feature = "try_from")]
use crate::number::rational::Rationals;
use crate::{
    error::{NumeraError, NumeraResult, RationalError},
    number::{
        integer::*,
        macros::impl_larger_smaller,
        traits::{
            Bound, ConstLowerBounded, ConstNegOne, ConstOne, ConstUpperBounded, ConstZero, Count,
            Countable, Ident, LowerBounded, NegOne, Number, One, Sign, Signed, UpperBounded, Zero,
        },
    },
};
use core::{cmp::Ordering, fmt, ops::Neg, hash::{Hash, Hasher}};
use devela::paste;

/* macro */

/// # What it does
/// - defines a Rational of a concrete size.
/// - implements Number: Bound + Count + Ident + Sign
/// - implements Default → 0
///
/// # Args
/// - `$name`: the base name of the rational. e.g. `Rational`.
/// - `$abbr`: the base abbreviated name, E.g. `Q`.
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
/// - `$b`: the size in bits of the primitive used.
macro_rules! define_rational_sized {
    // defines multiple integer types, with an inner primitive.
    (multi $name:ident, $abbr:ident, $p:ident,
     $doc_num:literal, $doc_type:literal, // $doc_new:literal,
     $doc_sign:literal, $doc_lower:expr, $doc_upper:expr,
     $num:ident, $den:ident,
        $(
            (
             $doc_det:literal, $b:expr,
             larger: $larger:literal, $larger_b:literal,
             smaller: $smaller:literal, $smaller_b:literal
            )
        ),+
     ) => {
        $(
            define_rational_sized![single $name, $abbr, $p,
               $doc_num, $doc_type, // $doc_new,
               $doc_sign, $doc_lower, $doc_upper,
               $num, $den,
               ($doc_det, $b,
                larger: $larger, $larger_b,
                smaller: $smaller, $smaller_b
               )];
        )+
    };
    // defines a single integer type, with an inner primitive.
    (single $name:ident, $abbr:ident, $p:ident,
     $doc_num:literal, $doc_type:literal, // $doc_new:literal,
     $doc_sign:literal, $doc_lower:expr, $doc_upper:expr,
     $num:ident, $den:ident,
     (
      $doc_det:literal, $b:expr,
      larger: $larger:literal, $larger_b:literal,
      smaller: $smaller:literal, $smaller_b:literal
      )
    ) => { paste! {
        #[doc = $doc_det " "$b "-bit " $doc_num $doc_type ","]
        #[doc = "also known as [`" [<$abbr$b>] "`][super::" [<$abbr$b>] "]."]
        #[doc = "\n\nThe range of valid numeric values is $\\lbrack"
        $doc_sign "$[`" $p$b "::" $doc_lower "`] $\\dots$ [`"
        $p$b "::" $doc_upper "`]$\\rbrack$."]

        #[derive(Clone, Copy)]
        pub struct [<$name$b>] {
            pub num: [<$num$b>],
            pub den: [<$den$b>],
        }

        /// Returns $0/1$.
        impl Default for [<$name$b>] {
            fn default() -> Self {
                Self {
                    num: [<$num$b>]::ZERO,
                    den: [<$den$b>]::ONE,
                }
            }
        }

        impl Hash for [<$name$b>] {
            /// The hash of equivalent functions with different numerator and/or
            /// denominator is considered different, even if they are considered
            /// equal by the `PartialEq` and `Eq` implementations.
            fn hash<H: Hasher>(&self, state: &mut H) {
                self.num.hash(state);
                self.den.hash(state);
            }
        }

        impl PartialEq for [<$name$b>] {
            /// Equivalent functions are considered equal, even if they are
            /// considered different by the `Hash` implementation.
            fn eq(&self, other: &Self) -> bool {
                // upcast first
                let uself = self.as_larger_or_same();
                let uother = other.as_larger_or_same();

                // compare by cross-multiplying
                uself.num * uother.den.into() == uother.num * uself.den.into()

                // IMPROVE: if it overflows try reducing
                // let rself = uself.reduced();
                // let rother = uother.reduced();
                // rself.num == rother.num && rself.den == rother.den
            }
        }
        impl Eq for [<$name$b>] {}

        impl PartialOrd for [<$name$b>] {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                // upcast first
                let uself = self.as_larger_or_same();
                let uother = other.as_larger_or_same();

                // compare by cross-multiplying
                let lhs = uself.num * uother.den.into();
                let rhs = uother.num * uself.den.into();

                // IMPROVE: if it overflows try reducing

                lhs.partial_cmp(&rhs)
            }
        }
        impl Ord for [<$name$b>] {
            fn cmp(&self, other: &Self) -> Ordering {
                self.partial_cmp(&other).unwrap()
            }
        }

        impl fmt::Display for [<$name$b>]  {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}/{}", self.num, self.den)
            }
        }
        impl fmt::Debug for [<$name$b>]  {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}({}/{})", stringify!([<$abbr$b>]), self.num, self.den)
                // MAYBE?
                // write!(f, "{}({:?}/{:?})", stringify!([<$abbr$b>]), self.num, self.den)
            }
        }

        impl [<$name$b>]  {
            #[doc = "Returns a new `" [<$name$b>] "`."]
            ///
            /// # Errors
            /// If the `denominator` is `0`.
            #[inline]
            pub const fn new(numerator: [<i$b>], denominator: [<i$b>])
                -> NumeraResult<Self> {
                if let Ok(den) = [<$den$b>]::new(denominator) {
                    let num = [<$num$b>]::new(numerator);
                    Ok(Self{ num, den })
                } else {
                    Err(NumeraError::Rational(RationalError::ZeroDenominator))
                }
            }

            #[doc = "Returns a new `" [<$name$b>] "`."]
            ///
            /// # Safety
            /// The `denominator` must not be 0.
            ///
            /// # Panics
            /// Panics in debug if the `denominator` is 0.
            #[inline]
            #[cfg(not(feature = "safe"))]
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "unsafe")))]
            pub const unsafe fn new_unchecked(numerator: [<i$b>], denominator: [<i$b>])
                -> Self {
                debug_assert![denominator != 0];
                Self {
                    num: [<$num$b>]::new(numerator),
                    den: [<$den$b>]::new_unchecked(denominator),
                }
            }
        }

        /* resizing */

        impl_larger_smaller![$name, $b, Rationals,
            larger: $larger, $larger_b, smaller: $smaller, $smaller_b
        ];

        /* sign */

        impl Sign for [<$name$b>] {
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
        impl Signed for [<$name$b>] {}

        /* bound */

        impl Bound for [<$name$b>] {
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
        impl LowerBounded for [<$name$b>] {
            #[inline]
            fn new_min() -> Self { <Self as ConstLowerBounded>::MIN }
        }
        impl UpperBounded for [<$name$b>] {
            #[inline]
            fn new_max() -> Self { <Self as ConstUpperBounded>::MAX }
        }
        impl ConstLowerBounded for [<$name$b>] {
            const MIN: Self = Self {
                    num: [<$num$b>]::MIN,
                    den: [<$den$b>]::ONE,
                };
        }
        impl ConstUpperBounded for [<$name$b>] {
            const MAX: Self = Self {
                    num: [<$num$b>]::MAX,
                    den: [<$den$b>]::ONE,
                };
        }

        /* count */

        impl Count for [<$name$b>] {
            #[inline]
            fn is_countable(&self) -> bool { true }
        }

        impl Countable for [<$name$b>] {
            /// Returns the next rational value by increasing the
            /// numerator, while maintaining the same denominator.
            #[inline]
            fn next(&self) -> NumeraResult<Self> {
                Ok(Self {
                    num: self.num.0.checked_add(1)
                        .ok_or(RationalError::NumeratorOverflow)?.into(),
                    den: self.den,
                })
            }
            /// Returns the previous rational value by decreasing the
            /// numerator, while maintaining the same denominator.
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

        impl Ident for [<$name$b>] {
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
        impl ConstZero for [<$name$b>] {
            const ZERO: Self = Self {
                num: [<$num$b>]::ZERO,
                den: [<$den$b>]::ONE,
            };
        }
        impl Zero for [<$name$b>] {
            #[inline]
            fn new_zero() -> Self { <Self as ConstZero>::ZERO }
        }
        impl ConstOne for [<$name$b>] {
            const ONE: Self = Self {
                num: [<$num$b>]::ONE,
                den: [<$den$b>]::ONE,
            };
        }
        impl One for [<$name$b>] {
            #[inline]
            fn new_one() -> Self { <Self as ConstOne>::ONE }
        }
        impl ConstNegOne for [<$name$b>] {
            const NEG_ONE: Self = Self {
                num: [<$num$b>]::NEG_ONE,
                den: [<$den$b>]::ONE,
            };
        }
        impl NegOne for [<$name$b>] {
            #[inline]
            fn new_neg_one() -> Self { <Self as ConstNegOne>::NEG_ONE }
        }

        /* number */

        impl Number for [<$name$b>] {
            type Parts = ([<$p$b>], [<$p$b>]);

            /// Forms a new rational from a numerator and denominator.
            ///
            /// # Errors
            /// If the denominator (`value.1`) equals 0.
            #[inline]
            fn from_parts(value: Self::Parts) -> NumeraResult<Self> {
                Ok(
                    Self {
                        num: [<$num$b>]::from_parts(value.0)?,
                        den: [<$den$b>]::from_parts(value.1)
                            .map_err(|_| RationalError::ZeroDenominator)?,
                    }
                )
            }

            /// Forms a new rational from a numerator and denominator.
            #[inline]
            #[cfg(not(feature = "safe"))]
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "unsafe")))]
            unsafe fn from_parts_unchecked(value: Self::Parts) -> Self {
                debug_assert![value.1 != [<$p$b>]::ZERO];
                Self {
                    num: [<$num$b>]::from_parts_unchecked(value.0),
                    den: [<$den$b>]::from_parts_unchecked(value.1),
                }
            }
        }
    }};
}

/* definitions */

define_rational_sized![multi Rational, Q, i,
    "rational number", ", from the set $\\Bbb{Q}$",
    // "",
    "", MIN, MAX,
    Integer, NonZeroInteger,
    ("A 2×", 8, larger: true, 16, smaller: false, 4),
    ("A 2×", 16, larger: true, 32, smaller: true, 8),
    ("A 2×", 32, larger: true, 64, smaller: true, 16),
    ("A 2×", 64, larger: true, 128, smaller: true, 32),
    ("A 2×", 128, larger: false, 256, smaller: true, 64)
];

#[cfg(test)]
mod tests {
    use crate::all::*;

    #[test]
    fn q_define_sized() -> NumeraResult<()> {
        assert_eq![
            Rational8::from_parts((5, 0)),
            Err(RationalError::ZeroDenominator.into())
        ];

        let _q5 = Rational8::from_parts((5, 1))?;

        // Display
        #[cfg(feature = "std")]
        assert_eq![_q5.to_string(), "5/1"];

        // PartialEq
        assert![Q8::new(4, 2)? == Q8::new(4, 2)?]; // eq non-reduced
        assert![Q8::new(4, 2)? == Q8::new(2, 1)?]; // eq reduced
        assert![Q8::new(4, 2)? != Q8::new(3, 1)?]; // ne

        // PartialOrd
        assert![Q8::new(3, 2)? < Q8::new(4, 2)?];
        assert![Q8::new(3, 2)? > Q8::new(3, 5)?];

        // Bound
        // Count
        // Ident
        // Sign

        // #[cfg(feature = "std")]
        // {
        //     use std::panic::catch_unwind;
        //     // 0 denominator
        //     assert![catch_unwind(|| Rational8::from_parts((5, 0)).unwrap()).is_err()];
        // }

        Ok(())
    }
}
