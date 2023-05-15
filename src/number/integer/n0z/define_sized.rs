// numera::number::integer::n0z::define_sized
//
//!
//
// TOC
//
// - macro
//   - define_nonzero_integer_sized
// - definitions
//   - NonZeroInteger[8|16|32|64|128]

#[cfg(feature = "try_from")]
use crate::number::integer::NonZeroIntegers;
use crate::{
    error::{IntegerError, NumeraError, NumeraResult},
    number::{
        macros::impl_larger_smaller,
        traits::{
            Bound, ConstLowerBounded, ConstNegOne, ConstOne, ConstUpperBounded, Count, Countable,
            Ident, LowerBounded, NegOne, NonZero, Number, One, Sign, Signed, UpperBounded,
        },
    },
};
use core::{
    fmt,
    num::{NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8},
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
macro_rules! define_nonzero_integer_sized {
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
            define_nonzero_integer_sized![single $name, $p,
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
        #[doc = "\n\nThe range of valid numeric values is $\\lbrack" $doc_sign
        "$[`" i$bsize "::" $doc_lower "`] $\\dots -1, 1 \\dots$ [`" i$bsize
        "::" $doc_upper "`]$\\rbrack$."]

        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
        pub struct [<$name$bsize>](pub(crate) [<$p$bsize>]);

        impl fmt::Display for [<$name$bsize>]  {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl [<$name$bsize>]  {
            #[doc = "Returns a new `" [<$name$bsize>] "`."]
            ///
            /// # Errors
            /// If the provided `value` equals 0.
            //
            // NOTE: accepting i* for converting to NonZeroI
            #[inline]
            pub const fn new(value: [<i$bsize>]) -> NumeraResult<Self> {
                if let Some(n) = [<$p$bsize>]::new(value) {
                    Ok(Self(n))
                } else {
                    Err(NumeraError::Integer(IntegerError::Zero))
                }
            }

            #[doc = "Returns a new `" [<$name$bsize>] "`."]
            ///
            /// # Safety
            /// The provided `value` must not be 0.
            ///
            /// # Panics
            /// Panics in debug if the `value` is 0.
            #[inline]
            #[cfg(not(feature = "safe"))]
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "unsafe")))]
            pub const unsafe fn new_unchecked(value: [<i$bsize>]) -> Self {
                debug_assert![value != 0];
                Self([<$p$bsize>]::new_unchecked(value))
            }
        }

        /* resizing */

        impl_larger_smaller![$name, $bsize, NonZeroIntegers,
            larger: $larger, $larger_bsize, smaller: $smaller, $smaller_bsize
        ];

        /* sign */

        impl Sign for [<$name$bsize>] {
            #[inline]
            fn can_negative(&self) -> bool { true }
            #[inline]
            fn can_positive(&self) -> bool { true }
            #[inline]
            fn is_negative(&self) -> bool { self.0.get().is_negative() }
            #[inline]
            fn is_positive(&self) -> bool { self.0.get().is_positive() }
        }
        impl Signed for [<$name$bsize>] {}

        /* bound */

        impl Bound for [<$name$bsize>] {
            #[inline]
            fn is_lower_bounded(&self) -> bool { true }
            #[inline]
            fn is_upper_bounded(&self) -> bool { true }
            #[inline]
            fn lower_bound(&self) -> Option<Self> where Self: Sized {
                // IMPROVE WAIT for https://github.com/rust-lang/rust/pull/106633 1.70
                // Some(Self([<$p$bsize>]::MIN))

                #[cfg(feature = "safe")]
                return Some(Self([<$p$bsize>]::new([<i$bsize>]::MIN).unwrap()));

                #[cfg(not(feature = "safe"))]
                // SAFETY: constant value
                return Some(Self(unsafe {[<$p$bsize>]::new_unchecked([<i$bsize>]::MIN) }));
            }
            #[inline]
            fn upper_bound(&self) -> Option<Self> where Self: Sized {
                // IMPROVE WAIT for https://github.com/rust-lang/rust/pull/106633 1.70
                // Some(Self([<$p$bsize>]::MAX))

                #[cfg(feature = "safe")]
                return Some(Self([<$p$bsize>]::new([<i$bsize>]::MIN).unwrap()));

                #[cfg(not(feature = "safe"))]
                // SAFETY: constant value
                return Some(Self(unsafe {[<$p$bsize>]::new_unchecked([<i$bsize>]::MAX) }));
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
            // IMPROVE WAIT for https://github.com/rust-lang/rust/pull/106633 1.70
            // const MIN: Self = Self([<$p$bsize>]::MIN);

            #[cfg(feature = "safe")]
            const MIN: Self = Self(
                if let Some(n) = [<$p$bsize>]::new([<i$bsize>]::MIN)
                    { n } else { unreachable!() }
            );

            #[cfg(not(feature = "safe"))]
            // SAFETY: constant value
            const MIN: Self = Self(unsafe {[<$p$bsize>]::new_unchecked([<i$bsize>]::MIN) });
        }
        impl ConstUpperBounded for [<$name$bsize>] {
            // IMPROVE WAIT for https://github.com/rust-lang/rust/pull/106633 1.70
            // const MAX: Self = Self([<$p$bsize>]::MAX);

            #[cfg(feature = "safe")]
            const MAX: Self = Self(
                if let Some(n) = [<$p$bsize>]::new([<i$bsize>]::MAX)
                    { n } else { unreachable!() }
            );


            #[cfg(not(feature = "safe"))]
            // SAFETY: constant value
            const MAX: Self = Self(unsafe {[<$p$bsize>]::new_unchecked([<i$bsize>]::MAX) });
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
                let mut next = self.0.get().checked_add(1).ok_or(IntegerError::Overflow)?;
                if next == 0 {
                    next = 1;
                }

                #[cfg(feature = "safe")]
                return Ok(Self([<$p$bsize>]::new(next).unwrap()));

                #[cfg(not(feature = "safe"))]
                // SAFETY: constant value
                return Ok(Self(unsafe { [<$p$bsize>]::new_unchecked(next) }));
            }
            /// Returns the previous countable value, skipping 0.
            ///
            /// # Errors
            /// Errors if the operation results in underflow.
            #[inline]
            fn previous(&self) -> NumeraResult<Self> {
                let mut prev = self.0.get().checked_sub(1).ok_or(IntegerError::Underflow)?;
                if prev == 0 {
                    prev = -1;
                }

                #[cfg(feature = "safe")]
                return Ok(Self([<$p$bsize>]::new(prev).unwrap()));

                #[cfg(not(feature = "safe"))]
                // SAFETY: constant value
                return Ok(Self(unsafe { [<$p$bsize>]::new_unchecked(prev) }));
            }
        }

        /* ident */

        impl Ident for [<$name$bsize>] {
            #[inline]
            fn can_zero(&self) -> bool { false }
            #[inline]
            fn can_one(&self) -> bool { true }
            #[inline]
            fn can_neg_one(&self) -> bool { true }

            #[inline]
            fn is_zero(&self) -> bool { false }
            #[inline]
            fn is_one(&self) -> bool { self.0.get() == 1 }
            #[inline]
            fn is_neg_one(&self) -> bool { self.0.get() == -1 }
        }
        impl NonZero for [<$name$bsize>] {}
        impl ConstOne for [<$name$bsize>] {
            #[cfg(feature = "safe")]
            const ONE: Self = Self(
                if let Some(n) = [<$p$bsize>]::new(1) { n } else { unreachable!() }

            );

            #[cfg(not(feature = "safe"))]
            // SAFETY: constant value
            const ONE: Self = Self(unsafe { [<$p$bsize>]::new_unchecked(1) });
        }
        impl One for [<$name$bsize>] {
            #[inline]
            fn new_one() -> Self {
                #[cfg(feature = "safe")]
                return Self([<$p$bsize>]::new(1).unwrap());

                #[cfg(not(feature = "safe"))]
                // SAFETY: constant value
                return Self(unsafe { [<$p$bsize>]::new_unchecked(1) });
            }
        }
        impl ConstNegOne for [<$name$bsize>] {
            #[cfg(feature = "safe")]
            const NEG_ONE: Self = Self(
                if let Some(n) = [<$p$bsize>]::new(-1)
                    { n } else { unreachable!() }
            );

            #[cfg(not(feature = "safe"))]
            // SAFETY: constant value
            const NEG_ONE: Self = Self(unsafe { [<$p$bsize>]::new_unchecked(-1) });
        }
        impl NegOne for [<$name$bsize>] {
            #[inline]
            fn new_neg_one() -> Self {
                #[cfg(feature = "safe")]
                return Self([<$p$bsize>]::new(-1).unwrap());

                #[cfg(not(feature = "safe"))]
                // SAFETY: constant value
                return Self(unsafe { [<$p$bsize>]::new_unchecked(-1) });
            }
        }

        /* number */

        impl Number for [<$name$bsize>] {
            type Parts = [<i$bsize>];

            #[doc = "Returns a new `" [<$name$bsize>] " from the constituent parts`."]
            ///
            /// # Errors
            /// This function can't fail.
            #[inline]
            fn from_parts(value: Self::Parts) -> NumeraResult<Self> {
                Ok(Self([<$p$bsize>]::new(value).ok_or(IntegerError::Zero)?))
            }

            #[doc = "Returns a new `" [<$name$bsize>] " from the constituent parts`."]
            ///
            /// # Safety
            /// This function is safe.
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

define_nonzero_integer_sized![multi NonZeroInteger, NonZeroI,
    "non-zero integer number", ", from the set $\\Z \\setminus 0$.",
    // "",
    "", MIN, MAX,
    ("A", 8, larger: true, 16, smaller: false, 8),
    ("A", 16, larger: true, 32, smaller: true, 8),
    ("A", 32, larger: true, 64, smaller: true, 16),
    ("A", 64, larger: true, 128, smaller: true, 32),
    ("A", 128, larger: false, 128, smaller: true, 64)
];
