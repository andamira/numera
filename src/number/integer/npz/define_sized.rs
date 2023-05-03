// numera::number::integer::npz::define_sized
//
//!
//
// TOC
//
// - macro
//   - define_non_positive_integer_sized
// - definitions
//   - NonPositiveInteger[8|16|32|64|128]
//

use crate::{
    error::{IntegerError, NumeraResult},
    number::traits::{
        Bound, ConstLowerBounded, ConstNegOne, ConstUpperBounded, ConstZero, Count, Countable,
        Ident, LowerBounded, NegOne, NegSigned, NonOne, Number, Sign, UpperBounded, Zero,
    },
};
use core::fmt;

/* macro */

/// # What it does
/// - defines an Integer of a concrete size.
/// - implements Number: Bound + Count + Ident + Sign
/// - implements Default → 0
///
/// # Args
/// - `$name`: the base name of the integer e.g. `Integer`.
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
/// - `$doc_det`: the determinant before the bit size. e.g. "An" (8-bit) or "A" 16-bit.
/// - `$bsize`: the size in bits of the primitive used.
macro_rules! define_nonpositive_integer_sized {
    // defines multiple integer types, with an inner primitive.
    (multi $name:ident, $p:ident,
     $doc_num:literal, $doc_type:literal, // $doc_new:literal,
     $doc_sign:literal, $doc_lower:expr, $doc_upper:expr,
        $(
            ($doc_det:literal, $bsize:expr)
        ),+
    ) => {
        $(
            define_nonpositive_integer_sized![single $name, $p,
               $doc_num, $doc_type, // $doc_new,
               $doc_sign, $doc_lower, $doc_upper,
               ($doc_det, $bsize)];
        )+
    };
    // defines a single integer type, with an inner primitive.
    (single $name:ident, $p:ident,
     $doc_num:literal, $doc_type:literal, // $doc_new:literal,
     $doc_sign:literal, $doc_lower:expr, $doc_upper:expr,
     ($doc_det:literal,$bsize:expr)
    ) => {
        paste::paste! {
            #[doc = $doc_det " "$bsize "-bit " $doc_num $doc_type]
            #[doc = "\n\nThe range of valid numeric values is $\\lbrack$"
            "$" $doc_sign "$[`" $p$bsize "::" $doc_lower "`]"
            " $\\dots"  $doc_upper  "\\rbrack$."]

            #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
            pub struct [<$name$bsize>](pub(crate) [<$p$bsize>]);

            impl fmt::Display for [<$name$bsize>]  {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    // notice the negation
                    write!(f, "-{}", self.0)
                }
            }

            impl [<$name$bsize>]  {
                #[inline]
                #[doc = "Returns a new `" [<$name$bsize>] "`."]
                pub const fn new(value: [<$p$bsize>]) -> Self { Self(value) }
            }

            /* sign */

            impl Sign for [<$name$bsize>] {
                #[inline]
                fn can_negative(&self) -> bool { true }
                #[inline]
                fn can_positive(&self) -> bool { false }
                #[inline]
                fn is_negative(&self) -> bool { self.0.is_negative() }
                #[inline]
                fn is_positive(&self) -> bool { false }
            }
            impl NegSigned for [<$name$bsize>] {
                type Parts = [<$p$bsize>];
                #[inline]
                fn new_neg(value: Self::Parts) -> NumeraResult<Self> {
                    Ok(Self(value))
                }
            }

            /* bound */

            impl Bound for [<$name$bsize>] {
                #[inline]
                fn is_lower_bounded(&self) -> bool { true }
                #[inline]
                fn is_upper_bounded(&self) -> bool { true }
                #[inline]
                fn lower_bound(&self) -> Option<Self> where Self: Sized {
                    Some(Self([<$p$bsize>]::MAX))
                }
                #[inline]
                fn upper_bound(&self) -> Option<Self> where Self: Sized {
                    Some(Self(0))
                }
            }
            impl LowerBounded for [<$name$bsize>] {
                #[inline]
                fn new_min() -> Self { <Self as ConstLowerBounded>::MIN }
            }
            impl UpperBounded for [<$name$bsize>] {
                #[inline]
                fn new_max() -> Self { <Self as ConstUpperBounded>::MAX } // 0
            }
            impl ConstLowerBounded for [<$name$bsize>] {
                const MIN: Self = Self([<$p$bsize>]::MAX);
            }
            impl ConstUpperBounded for [<$name$bsize>] {
                const MAX: Self = Self(0);
            }

            /* count */

            impl Count for [<$name$bsize>] {
                #[inline]
                fn is_countable(&self) -> bool { true }
            }

            impl Countable for [<$name$bsize>] {
                #[inline]
                fn next(&self) -> NumeraResult<Self> {
                    Ok(Self(self.0.checked_sub(1).ok_or(IntegerError::Overflow)?))
                }
                #[inline]
                fn previous(&self) -> NumeraResult<Self> {
                    Ok(Self(self.0.checked_add(1).ok_or(IntegerError::Underflow)?))
                }
            }

            /* ident */

            impl Ident for [<$name$bsize>] {
                #[inline]
                fn can_zero(&self) -> bool { true }
                #[inline]
                fn can_one(&self) -> bool { false }
                #[inline]
                fn can_neg_one(&self) -> bool { true }

                #[inline]
                fn is_zero(&self) -> bool { self.0 == 0 }
                #[inline]
                fn is_one(&self) -> bool { false }
                #[inline]
                fn is_neg_one(&self) -> bool { self.0 == 1 }
            }
            impl ConstZero for [<$name$bsize>] { const ZERO: Self = Self(0); }
            impl Zero for [<$name$bsize>] {
                #[inline]
                fn new_zero() -> Self { Self(0) }
            }
            impl ConstNegOne for [<$name$bsize>] { const NEG_ONE: Self = Self(1); }
            impl NegOne for [<$name$bsize>] {
                #[inline]
                fn new_neg_one() -> Self { Self(1) }
            }
            impl NonOne for [<$name$bsize>] {}

            /* number */

            impl Number for [<$name$bsize>] {
                type Parts = [<$p$bsize>];

                #[doc = "Returns a new `" [<$name$bsize>] " from the constituent parts`."]
                ///
                /// Please note that the `value` provided will interpreted as negative.
                ///
                /// # Errors
                /// This function can't fail.
                //
                // ALTERNATIVE:
                // For `value`s other than 0, please use the
                // [`new_neg`][NegSigned#method.new_neg] method from the
                // [`NegSigned`] trait.
                #[inline]
                fn from_parts(value: Self::Parts) -> NumeraResult<Self> {
                    Ok(Self(value))

                    // IMPROVE number constructor
                    // ALTERNATIVE:
                    // if value == 0 {
                    //     Ok(Self(value))
                    // } else {
                    //     Err(IntegerError::MoreThanZero.into())
                    // }
                }
                #[doc = "Returns a new `" [<$name$bsize>] " from the constituent parts`."]
                ///
                /// Please note that the `value` provided will interpreted as negative.
                ///
                /// # Safety
                /// This function is safe.
                #[inline]
                #[cfg(not(feature = "safe"))]
                #[cfg_attr(feature = "nightly", doc(cfg(feature = "non-safe")))]
                unsafe fn from_parts_unchecked(value: Self::Parts) -> Self { Self(value) }
            }
        }
    };
}

/* definitions */

define_nonpositive_integer_sized![multi NonPositiveInteger, u,
    "non-positive integer number", ", from the set $\\Z^- \\cup {0}$",
    // "",
    "-", MAX, 0,
    ("An", 8), ("A", 16), ("A", 32), ("A", 64), ("A", 128)
];
