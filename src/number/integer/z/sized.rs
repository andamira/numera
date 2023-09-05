// numera::number::integer::z::sized
//
//!
//
// TOC
//
// - macro
//   - define_integer_sized
// - definitions
//   - Integer[8|16|32|64|128]

#[cfg(feature = "try_from")]
use crate::number::integer::Integer;
use crate::{
    error::{IntegerErrors, NumeraResult},
    number::{
        macros::impl_larger_smaller,
        traits::{
            Bound, ConstLowerBounded, ConstNegOne, ConstOne, ConstUpperBounded, ConstZero, Count,
            Countable, Ident, LowerBounded, NegOne, Negative, Numbers, One, Positive, Sign,
            UpperBounded, Zero,
        },
    },
};
use core::fmt;
use devela::paste;

/* macro */

/// # What it does
/// - defines an Integer of a concrete size.
/// - implements Numbers: Bound + Count + Ident + Sign
/// - implements Default → 0
///
/// # Args
/// - `$name`: the base name of the integer. E.g. `Integer`.
/// - `$abbr`: the base abbreviated name, E.g. `Z`.
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
macro_rules! define_integer_sized {
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
            define_integer_sized![single $name, $abbr, $p,
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
        $doc_sign "$[`" $p$b "::" $doc_lower "`] $\\dots$ [`"
        $p$b "::" $doc_upper "`]$\\rbrack$."]
        #[doc = "\nIt is equivalent to the [`" [<i$b>] "`] primitive."]
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

        /// # Constructors
        impl [<$name$b>]  {
            #[doc = "Returns a new `" [<$name$b>] "`."]
            #[inline]
            pub const fn new(value: [<$p$b>]) -> Self { Self(value) }
        }

        /* resizing */

        impl_larger_smaller![$name, $b, Integer,
            larger: $larger, $larger_b, smaller: $smaller, $smaller_b
        ];

        /* sign */

        impl Sign for [<$name$b>] {
            #[inline]
            fn can_negative(&self) -> bool { true }
            #[inline]
            fn can_positive(&self) -> bool { true }
            #[inline]
            fn is_negative(&self) -> bool { self.0.is_negative() }
            #[inline]
            fn is_positive(&self) -> bool { self.0.is_positive() }
        }
        impl Positive for [<$name$b>] {}
        impl Negative for [<$name$b>] {}

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
            fn can_neg_one(&self) -> bool { true }

            #[inline]
            fn is_zero(&self) -> bool { self.0 == 0 }
            #[inline]
            fn is_one(&self) -> bool { self.0 == 1 }
            #[inline]
            fn is_neg_one(&self) -> bool { self.0 == -1 }
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
        impl ConstNegOne for [<$name$b>] { const NEG_ONE: Self = Self(-1); }
        impl NegOne for [<$name$b>] {
            #[inline]
            fn new_neg_one() -> Self { Self(-1) }
        }

        /* number */

        impl Numbers for [<$name$b>] {
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

define_integer_sized![multi Integer, Z, i,
    "integer number", ", from the set $\\Z$",
    // "",
    "", MIN, MAX,
    ("An", 8, larger: true, 16, smaller: false, 8),
    ("A", 16, larger: true, 32, smaller: true, 8),
    ("A", 32, larger: true, 64, smaller: true, 16),
    ("A", 64, larger: true, 128, smaller: true, 32),
    ("A", 128, larger: false, 128, smaller: true, 64)
];

#[cfg(test)]
mod tests {
    use crate::all::*;

    #[test]
    fn z_define_sized() -> NumeraResult<()> {
        // Display
        #[cfg(feature = "std")]
        assert_eq![Z8::new(17).to_string(), "17"];

        Ok(())
    }

    #[test]
    fn z_define_sized_larger() -> NumeraResult<()> {
        // min
        assert_eq![Z8::new(100).as_larger_or_same(), Z16::new(100)];
        assert_eq![Z8::new(100).try_as_larger(), Ok(Z16::new(100))];

        // max
        assert_eq![Z128::new(100).as_larger_or_same(), Z128::new(100)];
        assert![Z128::new(100).try_as_larger().is_err()];

        Ok(())
    }

    #[test]
    #[cfg(feature = "try_from")]
    fn z_define_sized_smaller() -> NumeraResult<()> {
        // min
        assert_eq![
            Z8::new(100).as_smaller_or_same(),
            Integers::Integer8(Z8::new(100))
        ];
        assert![Z8::new(100).try_as_smaller().is_err()];

        // can't fit
        assert_eq![
            Z16::new(3_000).as_smaller_or_same(),
            Integers::Integer16(Z16::new(3_000))
        ];
        assert![Z16::new(3_000).try_as_smaller().is_err()];

        // max
        assert_eq![
            Z128::new(100).as_smaller_or_same(),
            Integers::Integer64(Z64::new(100))
        ];
        assert_eq![Z128::new(100).try_as_smaller(), Ok(Z64::new(100))];

        Ok(())
    }
}
