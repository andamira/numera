// numera::number::integer::nnz::sized
//
//!
//
// TOC
//
// - macro
//   - define_non_negative_integer_sized
// - definitions
//   - NonNegativeInteger[8|16|32|64|128]

#[cfg(feature = "try_from")]
use crate::number::integer::NonNegativeIntegers;
use crate::{
    error::{IntegerErrors, NumeraResult},
    number::{
        macros::impl_larger_smaller,
        traits::{
            Bound, ConstLowerBounded, ConstOne, ConstUpperBounded, ConstZero, Count, Countable,
            Ident, LowerBounded, NonNegative, Number, One, Positive, Sign, UpperBounded, Zero,
        },
    },
};
use core::fmt;
use devela::paste;

/* macro */

/// # What it does
/// - defines an Integer of a concrete size.
/// - implements Number: Bound + Count + Ident + Sign
/// - implements Default → 0
///
/// # Args
/// - `$name`: the base name of the integer. E.g. `NonNegaiveInteger`.
/// - `$abbr`: the base abbreviated name, E.g. `Nnz`.
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
/// - `$b`: the size in bits of the primitive used.
macro_rules! define_nonnegative_integer_sized {
    // defines multiple integer types, with an inner primitive.
    (multi $name:ident, $abbr:ident, $p:ident,
     $doc_num:literal, $doc_type:literal, // $doc_new:literal,
     $doc_sign:literal, $doc_lower:expr, $doc_upper:expr,
        $(
            (
             $doc_det:literal, $b:expr,
             larger: $larger:literal, $larger_b:literal,
             smaller: $smaller:literal, $smaller_b:literal
            )
        ),+
     ) => {
        $(
            define_nonnegative_integer_sized![single $name, $abbr, $p,
               $doc_num, $doc_type, // $doc_new,
               $doc_sign, $doc_lower, $doc_upper,
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
     (
      $doc_det:literal, $b:expr,
      larger: $larger:literal, $larger_b:literal,
      smaller: $smaller:literal, $smaller_b:literal
     )
    ) => { paste! {
        #[doc = $doc_det " "$b "-bit " $doc_num $doc_type ","]
        #[doc = "also known as [`" [<$abbr$b>] "`][super::" [<$abbr$b>] "]."]
        #[doc = "\n\nThe range of valid numeric values is $\\lbrack"
        $doc_sign 0 " \\dots$ [`"
        $p$b "::" $doc_upper "`]$\\rbrack$."]
        #[doc = "\n\nIt is equivalent to the [`" [<u$b>] "`] primitive."]
        ///
        /// Also known as a [*natural number*][m0], you can also use the alias
        #[doc = "[`Natural" $b "`][super::Natural" $b "]."]
        ///
        /// [m0]: https://mathworld.wolfram.com/NaturalNumber.html
        #[derive(Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
        pub struct [<$name$b>](pub [<$p$b>]);

        impl fmt::Display for [<$name$b>]  {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }
        impl fmt::Debug for [<$name$b>]  {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}({})", stringify!([<$abbr$b>]), self.0)
            }
        }

        impl [<$name$b>]  {
            #[doc = "Returns a new `" [<$name$b>] "`."]
            #[inline]
            pub const fn new(value: [<$p$b>]) -> Self {
                Self(value)
            }
        }

        /* resizing */

        // uses "try_from"
        impl_larger_smaller![$name, $b, NonNegativeIntegers,
            larger: $larger, $larger_b, smaller: $smaller, $smaller_b
        ];

        /* sign */

        impl Sign for [<$name$b>] {
            #[inline]
            fn can_negative(&self) -> bool { false }
            #[inline]
            fn can_positive(&self) -> bool { true }
            #[inline]
            fn is_negative(&self) -> bool { false }
            #[inline]
            fn is_positive(&self) -> bool { self.0.is_positive() }
        }
        impl Positive for [<$name$b>] {}
        impl NonNegative for [<$name$b>] {}

        /* bound */

        impl Bound for [<$name$b>] {
            #[inline]
            fn is_lower_bounded(&self) -> bool { true }
            #[inline]
            fn is_upper_bounded(&self) -> bool { true }
            #[inline]
            fn lower_bound(&self) -> Option<Self> { Some([<$name$b>]::MIN) }
            #[inline]
            fn upper_bound(&self) -> Option<Self> { Some([<$name$b>]::MAX) }
        }
        impl LowerBounded for [<$name$b>] {
            #[inline]
            fn new_min() -> Self { [<$name$b>]::MIN }
        }
        impl UpperBounded for [<$name$b>] {
            #[inline]
            fn new_max() -> Self { [<$name$b>]::MAX }
        }
        impl ConstLowerBounded for [<$name$b>] {
            const MIN: Self = Self([<$p$b>]::MIN);
        }
        impl ConstUpperBounded for [<$name$b>] {
            const MAX: Self = Self([<$p$b>]::MAX);
        }

        /* count */

        impl Count for [<$name$b>] {
            #[inline]
            fn is_countable(&self) -> bool { true }
        }

        impl Countable for [<$name$b>] {
            #[inline]
            fn next(&self) -> NumeraResult<Self> {
                Ok(Self(self.0.checked_add(1).ok_or(IntegerErrors::Overflow)?))
            }
            #[inline]
            fn previous(&self) -> NumeraResult<Self> {
                Ok(Self(self.0.checked_sub(1).ok_or(IntegerErrors::Underflow)?))
            }
        }

        /* ident */

        impl Ident for [<$name$b>] {
            #[inline]
            fn can_zero(&self) -> bool { true }
            #[inline]
            fn can_one(&self) -> bool { true }
            #[inline]
            fn can_neg_one(&self) -> bool { false }

            #[inline]
            fn is_zero(&self) -> bool { self.0 == 0 }
            #[inline]
            fn is_one(&self) -> bool { self.0 == 1 }
            #[inline]
            fn is_neg_one(&self) -> bool { false }
        }
        impl ConstZero for [<$name$b>] { const ZERO: Self = Self(0); }
        impl Zero for [<$name$b>] {
            #[inline]
            fn new_zero() -> Self { Self(0) }
        }
        impl ConstOne for [<$name$b>] { const ONE: Self = Self(1); }
        impl One for [<$name$b>] {
            #[inline]
            fn new_one() -> Self { Self(1) }
        }

        /* number */

        impl Number for [<$name$b>] {
            type InnerRepr = [<$p$b>];
            type InnermostRepr = [<$p$b>];

            #[doc = "Returns a new `" [<$name$b>] "` from the inner representation."]
            ///
            /// # Errors
            /// This function can't fail.
            #[inline]
            fn from_inner_repr(value: Self::InnerRepr) -> NumeraResult<Self> { Ok(Self(value)) }

            #[doc = "Returns a new `" [<$name$b>] "` from the inner representation."]
            ///
            /// # Safety
            /// This function is safe.
            #[inline]
            #[cfg(not(feature = "safe"))]
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "not(safe)")))]
            unsafe fn from_inner_repr_unchecked(value: Self::InnerRepr) -> Self { Self(value) }

            #[doc = "Returns a new `" [<$name$b>] "` from the innermost representation."]
            ///
            /// # Errors
            /// This function can't fail.
            #[inline]
            fn from_innermost_repr(value: Self::InnermostRepr) -> NumeraResult<Self> {
                Ok(Self(value))
            }

            #[doc = "Returns a new `" [<$name$b>] "` from the innermost representation."]
            ///
            /// # Safety
            /// # This function is safe.
            #[inline]
            #[cfg(not(feature = "safe"))]
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "not(safe)")))]
            unsafe fn from_innermost_repr_unchecked(value: Self::InnermostRepr) -> Self {
                Self(value)
            }

            #[inline]
            fn into_inner_repr(self) -> Self::InnerRepr { self.0 }

            #[inline]
            fn into_innermost_repr(self) -> Self::InnermostRepr { self.0 }
        }
    }};
}

/* definitions */

define_nonnegative_integer_sized![multi NonNegativeInteger, Nnz, u,
    "non-negative integer number", ", from the set $\\Z^*$ ($\\N _0$)",
    // "",
    "", MIN, MAX,
    ("An", 8, larger: true, 16, smaller: false, 8),
    ("A", 16, larger: true, 32, smaller: true, 8),
    ("A", 32, larger: true, 64, smaller: true, 16),
    ("A", 64, larger: true, 128, smaller: true, 32),
    ("A", 128, larger: false, 128, smaller: true, 64)
];
