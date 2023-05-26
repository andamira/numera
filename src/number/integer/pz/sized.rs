// numera::number::integer::pz::sized
//
//!
//
// TOC
//
// - macro
//   - define_positive_integer_sized
// - definitions
//   - PositiveInteger[8|16|32|64|128]

#[cfg(feature = "try_from")]
use crate::number::integer::PositiveIntegers;
use crate::{
    error::{IntegerError, NumeraError, NumeraResult},
    number::{
        macros::impl_larger_smaller,
        traits::{
            Bound, ConstLowerBounded, ConstOne, ConstUpperBounded, Count, Countable, Ident,
            LowerBounded, NonNegOne, NonZero, Number, One, Sign, Signed, UpperBounded,
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
/// - `$name`: the base name of the integer. E.g. `PositiveInteger`.
/// - `$abbr`: the base abbreviated name, E.g. `Pz`.
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
macro_rules! define_positive_integer_sized {
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
            define_positive_integer_sized![single $name, $abbr, $p,
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
        #[doc = "\n\nThe range of valid numeric values is $\\lbrack" $doc_sign
        "1 \\dots$ [`" u$b
        "::" $doc_upper "`]$\\rbrack$."]
        #[doc = "\n\nIt is equivalent to the [`" [<NonZeroU$b>] "`] primitive."]
        ///
        /// Also known as a [*counting number*][0m], you can also use the alias
        #[doc = "[`Counting" $b "`][super::Counting" $b "]."]
        ///
        /// [0m]: https://mathworld.wolfram.com/CountingNumber.html
        #[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
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
            ///
            /// # Errors
            /// If the given `value` is `0`.
            //
            // NOTE: accepting u* for converting to NonZeroU
            #[inline]
            pub const fn new(value: [<u$b>]) -> NumeraResult<Self> {
                if let Some(n) = [<$p$b>]::new(value) {
                    Ok(Self(n))
                } else {
                    Err(NumeraError::Integer(IntegerError::Zero))
                }
            }

            #[doc = "Returns a new `" [<$name$b>] "`."]
            ///
            /// # Safety
            /// The given `value` must not be 0.
            ///
            /// # Panics
            /// Panics in debug if the `value` is 0.
            #[inline]
            #[cfg(not(feature = "safe"))]
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "unsafe")))]
            pub const unsafe fn new_unchecked(value: [<u$b>]) -> Self {
                debug_assert![value != 0];
                Self([<$p$b>]::new_unchecked(value))
            }
        }

        /* resizing */

        impl_larger_smaller![$name, $b, PositiveIntegers,
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
            fn is_positive(&self) -> bool { true }
        }
        impl Signed for [<$name$b>] {}

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
            // IMPROVE WAIT for https://github.com/rust-lang/rust/pull/106633 1.70
            // const MIN: Self = Self([<$p$b>]::MIN);

            #[cfg(feature = "safe")]
            const MIN: Self = Self(
                if let Some(n) = [<$p$b>]::new(1)
                    { n } else { unreachable!() }
            );

            #[cfg(not(feature = "safe"))]
            // SAFETY: constant value
            const MIN: Self = Self(unsafe { [<$p$b>]::new_unchecked(1) });
        }
        impl ConstUpperBounded for [<$name$b>] {
            // IMPROVE WAIT for https://github.com/rust-lang/rust/pull/106633 1.70
            // const MAX: Self = Self([<$p$b>]::MAX);

            #[cfg(feature = "safe")]
            const MAX: Self = Self(
                if let Some(n) = [<$p$b>]::new([<u$b>]::MAX)
                    { n } else { unreachable!() }
            );

            #[cfg(not(feature = "safe"))]
            // SAFETY: constant value
            const MAX: Self = Self(unsafe {[<$p$b>]::new_unchecked([<u$b>]::MAX) });
        }

        /* count */

        impl Count for [<$name$b>] {
            #[inline]
            fn is_countable(&self) -> bool { true }
        }

        impl Countable for [<$name$b>] {
            /// Returns the next countable value, skipping 0.
            ///
            /// # Errors
            /// Errors if the operation results in overflow.
            #[inline]
            fn next(&self) -> NumeraResult<Self> {
                let next = self.0.get().checked_add(1).ok_or(IntegerError::Overflow)?;

                #[cfg(feature = "safe")]
                return Ok(Self([<$p$b>]::new(next).unwrap()));

                #[cfg(not(feature = "safe"))]
                // SAFETY: we've checked the value
                return Ok(Self(unsafe { [<$p$b>]::new_unchecked(next) }));
            }
            /// Returns the previous countable value, skipping 0.
            ///
            /// # Errors
            /// Errors if the operation results in underflow.
            #[inline]
            fn previous(&self) -> NumeraResult<Self> {
                let prev = self.0.get().checked_sub(1).ok_or(IntegerError::Underflow)?;

                #[cfg(feature = "safe")]
                return Ok(Self([<$p$b>]::new(prev).unwrap()));

                #[cfg(not(feature = "safe"))]
                // SAFETY: we've checked the value
                return Ok(Self(unsafe { [<$p$b>]::new_unchecked(prev) }));
            }
        }

        /* ident */

        impl Ident for [<$name$b>] {
            #[inline]
            fn can_zero(&self) -> bool { false }
            #[inline]
            fn can_one(&self) -> bool { true }
            #[inline]
            fn can_neg_one(&self) -> bool { false }

            #[inline]
            fn is_zero(&self) -> bool { false }
            #[inline]
            fn is_one(&self) -> bool { self.0.get() == 1 }
            #[inline]
            fn is_neg_one(&self) -> bool { false }
        }
        impl NonZero for [<$name$b>] {}
        impl ConstOne for [<$name$b>] {
            #[cfg(feature = "safe")]
            const ONE: Self = Self(
                if let Some(n) = [<$p$b>]::new(1)
                    { n } else { unreachable!() }
            );

            #[cfg(not(feature = "safe"))]
            // SAFETY: constant value
            const ONE: Self = Self(unsafe { [<$p$b>]::new_unchecked(1) });
        }
        impl One for [<$name$b>] {
            #[inline]
            fn new_one() -> Self {
                #[cfg(feature = "safe")]
                return Self([<$p$b>]::new(1).unwrap());

                #[cfg(not(feature = "safe"))]
                // SAFETY: constant value
                return Self(unsafe { [<$p$b>]::new_unchecked(1) });
            }
        }
        impl NonNegOne for [<$name$b>] {}

        /* number */

        impl Number for [<$name$b>] {
            type Parts = [<u$b>];

            #[doc = "Returns a new `" [<$name$b>] "` from the constituent parts."]
            ///
            /// # Errors
            /// If the given `value` is `0`.
            #[inline]
            fn from_parts(value: Self::Parts) -> NumeraResult<Self> {
                Ok(Self([<$p$b>]::new(value).ok_or(IntegerError::Zero)?))
            }

            #[doc = "Returns a new `" [<$name$b>] "` from the constituent parts."]
            ///
            /// # Panics
            /// In debug if the given `value` is `0`.
            ///
            /// # Safety
            /// The given `value` must not be `0`.
            #[inline]
            #[cfg(not(feature = "safe"))]
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "unsafe")))]
            unsafe fn from_parts_unchecked(value: Self::Parts) -> Self {
                debug_assert![value != 0];
                Self([<$p$b>]::new_unchecked(value))
            }
        }
    }};
}

/* definitions */

define_positive_integer_sized![multi PositiveInteger, Pz, NonZeroU,
    "positive integer number", ", from the set $\\Z^+$ ($\\N _1$)",
    // "",
    "", MIN, MAX,
    ("An", 8, larger: true, 16, smaller: false, 8),
    ("A", 16, larger: true, 32, smaller: true, 8),
    ("A", 32, larger: true, 64, smaller: true, 16),
    ("A", 64, larger: true, 128, smaller: true, 32),
    ("A", 128, larger: false, 128, smaller: true, 64)
];
