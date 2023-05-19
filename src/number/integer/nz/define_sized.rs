// numera::number::integer::nz::define_sized
//
//!
//
// TOC
//
// - macro
//   - define_negative_integer_sized
// - definitions
//   - NegativeInteger[8|16|32|64|128]

#[cfg(feature = "try_from")]
use crate::number::integer::NegativeIntegers;
use crate::{
    error::{IntegerError, NumeraError, NumeraResult},
    number::{
        macros::impl_larger_smaller,
        traits::{
            Bound, ConstLowerBounded, ConstNegOne, ConstUpperBounded, Count, Countable, Ident,
            LowerBounded, NegOne, NegSigned, NonOne, NonZero, Number, Sign, UpperBounded,
        },
    },
};
use core::{
    fmt,
    num::{NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8},
};
use devela::paste;

/* macro */

/// # What it does
/// - defines an Integer of a concrete size.
/// - implements Number: Bound + Count + Ident + Sign
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
macro_rules! define_negative_integer_sized {
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
            define_negative_integer_sized![single $name, $p,
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
        #[doc = "\n\nThe range of valid numeric values is $\\lbrack$"
        "$" $doc_sign "$[`" $p$bsize "::" $doc_lower "`]"
        " $\\dots"  $doc_sign $doc_upper  "\\rbrack$."]
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
        pub struct [<$name$bsize>](pub [<$p$bsize>]);
        impl fmt::Display for [<$name$bsize>]  {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                // notice the negation
                write!(f, "-{}", self.0)
            }
        }

        impl [<$name$bsize>]  {
            #[doc = "Returns a new `" [<$name$bsize>] "`."]
            ///
            /// Please note that the `value` will be interpreted as negative.
            ///
            /// # Errors
            /// If the `value` provided is `0`.
            //
            // NOTE: accepting u* for converting to NonZeroU
            #[inline]
            pub const fn new(value: [<u$bsize>]) -> NumeraResult<Self> {
                if let Some(n) = [<$p$bsize>]::new(value) {
                    Ok(Self(n))
                } else {
                    Err(NumeraError::Integer(IntegerError::Zero))
                }
            }
        }

        /* resizing */

        impl_larger_smaller![$name, $bsize, NegativeIntegers,
            larger: $larger, $larger_bsize, smaller: $smaller, $smaller_bsize
        ];

        /* sign */

        impl Sign for [<$name$bsize>] {
            #[inline]
            fn can_negative(&self) -> bool { true }
            #[inline]
            fn can_positive(&self) -> bool { false }
            #[inline]
            fn is_negative(&self) -> bool { true }
            #[inline]
            fn is_positive(&self) -> bool { false }
        }
        impl NegSigned for [<$name$bsize>] {
            type Parts = [<u$bsize>];
            #[inline]
            fn new_neg(value: Self::Parts) -> NumeraResult<Self> {
                Ok(Self([<$p$bsize>]::new(value).ok_or(IntegerError::Zero)?))
            }
        }

        /* bound */

        impl Bound for [<$name$bsize>] {
            fn is_lower_bounded(&self) -> bool { true }
            fn is_upper_bounded(&self) -> bool { true }
            fn lower_bound(&self) -> Option<Self> { Some([<$name$bsize>]::MIN) }
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
            // IMPROVE WAIT for https://github.com/rust-lang/rust/pull/106633 1.70
            // const MIN: Self = Self([<$p$bsize>]::MIN);

            #[cfg(feature = "safe")]
            const MIN: Self = Self(
                if let Some(n) = [<$p$bsize>]::new([<u$bsize>]::MAX)
                    { n } else { unreachable!() }
            );

            #[cfg(not(feature = "safe"))]
            // SAFETY: constant value
            const MIN: Self = Self(unsafe {[<$p$bsize>]::new_unchecked([<u$bsize>]::MAX) });
        }
        impl ConstUpperBounded for [<$name$bsize>] {
            // IMPROVE WAIT for https://github.com/rust-lang/rust/pull/106633 1.70
            // const MAX: Self = Self([<$p$bsize>]::MAX);

            #[cfg(feature = "safe")]
            const MAX: Self = Self(
                if let Some(n) = [<$p$bsize>]::new(1)
                    { n } else { unreachable!() }
            );

            #[cfg(not(feature = "safe"))]
            // SAFETY: constant value
            const MAX: Self = Self(unsafe { [<$p$bsize>]::new_unchecked(1) });
        }

        /* count */

        impl Count for [<$name$bsize>] {
            #[inline]
            fn is_countable(&self) -> bool { true }
        }

        impl Countable for [<$name$bsize>] {
            /// Returns the next countable value, skipping 0.
            ///
            /// # Errors
            /// Errors if the operation results in overflow.
            #[inline]
            fn next(&self) -> NumeraResult<Self> {
                let next = self.0.get().checked_add(1).ok_or(IntegerError::Overflow)?;

                #[cfg(feature = "safe")]
                return Ok(Self([<$p$bsize>]::new(next).unwrap()));

                #[cfg(not(feature = "safe"))]
                // SAFETY: we've checked the value
                return Ok(Self(unsafe { [<$p$bsize>]::new_unchecked(next) }));
            }
            /// Returns the previous countable value, skipping 0.
            ///
            /// # Errors
            /// Errors if the operation results in underflow.
            #[inline]
            fn previous(&self) -> NumeraResult<Self> {
                let prev = self.0.get().checked_sub(1).ok_or(IntegerError::Underflow)?;
                Ok(Self([<$p$bsize>]::new(prev).ok_or(IntegerError::Zero)?))
            }
        }

        /* ident */

        impl Ident for [<$name$bsize>] {
            #[inline]
            fn can_zero(&self) -> bool { false }
            #[inline]
            fn can_one(&self) -> bool { false }
            #[inline]
            fn can_neg_one(&self) -> bool { true }

            #[inline]
            fn is_zero(&self) -> bool { false }
            #[inline]
            fn is_one(&self) -> bool { false }
            #[inline]
            fn is_neg_one(&self) -> bool { self.0.get() == 1 }
        }
        impl NonZero for [<$name$bsize>] {}
        impl NonOne for [<$name$bsize>] {}
        impl ConstNegOne for [<$name$bsize>] {
            #[cfg(feature = "safe")]
            const NEG_ONE: Self = Self(
                if let Some(n) = [<$p$bsize>]::new(1) { n }
                else { unreachable!() }
            );

            #[cfg(not(feature = "safe"))]
            // SAFETY: constant value
            const NEG_ONE: Self = Self(unsafe { [<$p$bsize>]::new_unchecked(1) });
        }
        impl NegOne for [<$name$bsize>] {
            #[inline]
            fn new_neg_one() -> Self {
                #[cfg(feature = "safe")]
                return Self([<$p$bsize>]::new(1).unwrap());

                #[cfg(not(feature = "safe"))]
                // SAFETY: constant value
                return Self(unsafe { [<$p$bsize>]::new_unchecked(1) });
            }
        }

        /* number */

        impl Number for [<$name$bsize>] {
            type Parts = [<u$bsize>];

            #[doc = "Returns a new `" [<$name$bsize>] "` from the constituent parts."]
            ///
            /// Please note that the `value` provided will be interpreted as negative.
            ///
            /// # Errors
            /// If the `value` provided is `0`.
            //
            // ALTERNATIVE:
            // This constructur always return an error. Please use the
            // [`new_neg`][NegSigned#method.new_neg] method from the
            // [`NegSigned`] trait.
            #[inline]
            fn from_parts(value: Self::Parts) -> NumeraResult<Self> {
                Ok(Self([<$p$bsize>]::new(value).ok_or(IntegerError::Zero)?))

                // ALTERNATIVE:
                // Err(IntegerError::ZeroOrMore.into())
            }

            #[doc = "Returns a new `" [<$name$bsize>] "` from the constituent parts."]
            ///
            /// Please note that the `value` provided will interpreted as negative.
            ///
            /// # Panics
            /// In debug if the value is `0`.
            ///
            /// # Safety
            /// The `value` provided must not be `0`.
            #[inline]
            #[cfg(not(feature = "safe"))]
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "unsafe")))]
            unsafe fn from_parts_unchecked(value: Self::Parts) -> Self {
                debug_assert![value != 0];
                Self([<$p$bsize>]::new_unchecked(value))
            }
        }
    }};
}

/* definitions */

define_negative_integer_sized![multi NegativeInteger, NonZeroU,
    "negative integer number", ", from the set $\\Z^-$.",
    // "",
    "-", MAX, 1,
    ("An", 8, larger: true, 16, smaller: false, 8),
    ("A", 16, larger: true, 32, smaller: true, 8),
    ("A", 32, larger: true, 64, smaller: true, 16),
    ("A", 64, larger: true, 128, smaller: true, 32),
    ("A", 128, larger: false, 128, smaller: true, 64)
];
