// numera::number::integer::z::define_sized
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
use crate::number::integer::Integers;
use crate::{
    error::{IntegerError, NumeraResult},
    number::{
        macros::impl_larger_smaller,
        traits::{
            Bound, ConstLowerBounded, ConstNegOne, ConstOne, ConstUpperBounded, ConstZero, Count,
            Countable, Ident, LowerBounded, NegOne, Number, One, Sign, Signed, UpperBounded, Zero,
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
macro_rules! define_integer_sized {
    // defines multiple integer types, with an inner primitive.
    (multi $name:ident, $p:ident,
     $doc_num:literal, $doc_type:literal, // $doc_new:literal,
     $doc_sign:literal, $doc_lower:expr, $doc_upper:expr,
        $(
             (
             $doc_det:literal, $bsize:expr,
             larger: $larger:literal, $larger_bsize:literal,
             smaller: $smaller:literal, $smaller_bsize:literal
            )
        ),+
     ) => {
        $(
            define_integer_sized![single $name, $p,
               $doc_num, $doc_type, // $doc_new,
               $doc_sign, $doc_lower, $doc_upper,
               ($doc_det, $bsize,
                larger: $larger, $larger_bsize,
                smaller: $smaller, $smaller_bsize
               )];
        )+
    };
    // defines a single integer type, with an inner primitive.
    (single $name:ident, $p:ident,
     $doc_num:literal, $doc_type:literal, // $doc_new:literal,
     $doc_sign:literal, $doc_lower:expr, $doc_upper:expr,
     (
      $doc_det:literal, $bsize:expr,
      larger: $larger:literal, $larger_bsize:literal,
      smaller: $smaller:literal, $smaller_bsize:literal
     )
    ) => { paste! {
        #[doc = $doc_det " "$bsize "-bit " $doc_num $doc_type]
        #[doc = "\n\nThe range of valid numeric values is $\\lbrack"
        $doc_sign "$[`" $p$bsize "::" $doc_lower "`] $\\dots$ [`"
        $p$bsize "::" $doc_upper "`]$\\rbrack$."]
        ///
        #[doc = "It is equivalent to the [`" [<i$bsize>] "`] primitive."]
        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
        pub struct [<$name$bsize>](pub(crate) [<$p$bsize>]);

        impl fmt::Display for [<$name$bsize>]  {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        /// # Constructors
        impl [<$name$bsize>]  {
            #[doc = "Returns a new `" [<$name$bsize>] "`."]
            #[inline]
            pub const fn new(value: [<$p$bsize>]) -> Self { Self(value) }
        }

        /* resizing */

        impl_larger_smaller![$name, $bsize, Integers,
            larger: $larger, $larger_bsize, smaller: $smaller, $smaller_bsize
        ];

        /* sign */

        impl Sign for [<$name$bsize>] {
            #[inline]
            fn can_negative(&self) -> bool { true }
            #[inline]
            fn can_positive(&self) -> bool { true }
            #[inline]
            fn is_negative(&self) -> bool { self.0.is_negative() }
            #[inline]
            fn is_positive(&self) -> bool { self.0.is_positive() }
        }
        impl Signed for [<$name$bsize>] {}

        /* bound */

        impl Bound for [<$name$bsize>] {
            #[inline]
            fn is_lower_bounded(&self) -> bool { true }
            #[inline]
            fn is_upper_bounded(&self) -> bool { true }
            #[inline]
            fn lower_bound(&self) -> Option<Self> { Some([<$name$bsize>]::MIN) }
            #[inline]
            fn upper_bound(&self) -> Option<Self> { Some([<$name$bsize>]::MAX) }
        }
        impl LowerBounded for [<$name$bsize>] {
            #[inline]
            fn new_min() -> Self { [<$name$bsize>]::MIN }
        }
        impl UpperBounded for [<$name$bsize>] {
            #[inline]
            fn new_max() -> Self { [<$name$bsize>]::MAX }
        }
        impl ConstLowerBounded for [<$name$bsize>] {
            const MIN: Self = Self([<$p$bsize>]::MIN);
        }
        impl ConstUpperBounded for [<$name$bsize>] {
            const MAX: Self = Self([<$p$bsize>]::MAX);
        }

        /* count */

        impl Count for [<$name$bsize>] {
            #[inline]
            fn is_countable(&self) -> bool { true }
        }

        impl Countable for [<$name$bsize>] {
            #[inline]
            fn next(&self) -> NumeraResult<Self> {
                Ok(Self(self.0.checked_add(1).ok_or(IntegerError::Overflow)?))
            }
            #[inline]
            fn previous(&self) -> NumeraResult<Self> {
                Ok(Self(self.0.checked_sub(1).ok_or(IntegerError::Underflow)?))
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
            fn is_zero(&self) -> bool { self.0 == 0 }
            #[inline]
            fn is_one(&self) -> bool { self.0 == 1 }
            #[inline]
            fn is_neg_one(&self) -> bool { self.0 == -1 }
        }
        impl ConstZero for [<$name$bsize>] { const ZERO: Self = Self(0); }
        impl Zero for [<$name$bsize>] {
            #[inline]
            fn new_zero() -> Self { Self(0) }
        }
        impl ConstOne for [<$name$bsize>] { const ONE: Self = Self(1); }
        impl One for [<$name$bsize>] {
            #[inline]
            fn new_one() -> Self { Self(1) }
        }
        impl ConstNegOne for [<$name$bsize>] { const NEG_ONE: Self = Self(-1); }
        impl NegOne for [<$name$bsize>] {
            #[inline]
            fn new_neg_one() -> Self { Self(-1) }
        }

        /* number */

        impl Number for [<$name$bsize>] {
            type Parts = [<$p$bsize>];

            #[doc = "Returns a new `" [<$name$bsize>] "` from the constituent parts."]
            ///
            /// # Errors
            /// This function can't fail.
            #[inline]
            fn from_parts(value: Self::Parts) -> NumeraResult<Self> { Ok(Self(value)) }

            #[doc = "Returns a new `" [<$name$bsize>] "` from the constituent parts."]
            ///
            /// # Safety
            /// This function is safe.
            #[inline]
            #[cfg(not(feature = "safe"))]
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "unsafe")))]
            unsafe fn from_parts_unchecked(value: Self::Parts) -> Self { Self(value) }
        }
    }};
}

/* definitions */

define_integer_sized![multi Integer, i,
    "integer number", ", from the set $\\Z$.",
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
