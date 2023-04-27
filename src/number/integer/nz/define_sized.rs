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

use crate::{
    error::{IntegerError, NumeraResult},
    number::traits::{
        Bound, ConstLowerBounded, ConstNegOne, ConstUpperBounded, Count, Countable, Ident,
        LowerBounded, NegOne, NegSigned, NonOne, NonZero, Number, Sign, UpperBounded,
    },
};
use core::num::{NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8};

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
macro_rules! define_negative_integer_sized {
    // defines multiple integer types, with an inner primitive.
    (multi $name:ident, $p:ident,
     $doc_num:literal, $doc_type:literal, // $doc_new:literal,
     $sign:literal, $lower:expr, $upper:expr,
     $(($det:literal,$bsize:expr)),+) => {
        $(
            define_negative_integer_sized![single $name, $p,
               $doc_num, $doc_type, // $doc_new,
               $sign, $lower, $upper,
               ($det,$bsize)];
        )+
    };
    // defines a single integer type, with an inner primitive.
    (single $name:ident, $p:ident,
     $doc_num:literal, $doc_type:literal, // $doc_new:literal,
     $doc_sign:literal, $doc_lower:expr, $doc_upper:expr,
     ($doc_det:literal,$bsize:expr)) => {

        paste::paste! {
            #[doc = $doc_det " "$bsize "-bit " $doc_num $doc_type]
            #[doc = "\n\nThe range of valid numeric values is $\\lbrack$"
            "$" $doc_sign "$[`" $p$bsize "::" $doc_lower "`]"
            " $\\dots"  $doc_sign $doc_upper  "\\rbrack$."]

            #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
            pub struct [<$name$bsize>](pub(crate) [< $p$bsize >]);

            /* sign */

            impl Sign for [<$name$bsize>] {
                fn can_negative(&self) -> bool { true }
                fn can_positive(&self) -> bool { false }
                fn is_negative(&self) -> bool { true }
                fn is_positive(&self) -> bool { false }
            }
            impl NegSigned for [<$name$bsize>] {
                type Inner = [<u$bsize>];
                fn new_neg(value: Self::Inner) -> NumeraResult<Self> {
                    Ok(Self([<$p$bsize>]::new(value).ok_or(IntegerError::Zero)?))
                }
            }

            /* bound */

            impl Bound for [<$name$bsize>] {
                fn is_lower_bounded(&self) -> bool { true }
                fn is_upper_bounded(&self) -> bool { true }
                fn lower_bound(&self) -> Option<Self> where Self: Sized {
                    // IMPROVE WAIT for https://github.com/rust-lang/rust/pull/106633 1.70
                    // Some(Self([<$p$bsize>]::MAX))
                    // SAFETY: constant value
                    Some(Self(unsafe {[<$p$bsize>]::new_unchecked([<u$bsize>]::MAX) }))
                }
                fn upper_bound(&self) -> Option<Self> where Self: Sized {
                    // IMPROVE WAIT for https://github.com/rust-lang/rust/pull/106633 1.70
                    // Some(Self([<$p$bsize>]::MIN))
                    // SAFETY: constant value
                    Some(Self(unsafe {[<$p$bsize>]::new_unchecked(1) }))
                }
            }
            impl LowerBounded for [<$name$bsize>] {
                fn new_min() -> Self { <Self as ConstLowerBounded>::MIN }
            }
            impl UpperBounded for [<$name$bsize>] {
                fn new_max() -> Self { <Self as ConstUpperBounded>::MAX }
            }
            impl ConstLowerBounded for [<$name$bsize>] {
                // IMPROVE WAIT for https://github.com/rust-lang/rust/pull/106633 1.70
                // const MIN: Self = Self([<$p$bsize>]::MIN);
                //
                // SAFETY: constant value
                const MIN: Self = Self(unsafe {[<$p$bsize>]::new_unchecked([<u$bsize>]::MAX) });
            }
            impl ConstUpperBounded for [<$name$bsize>] {
                // IMPROVE WAIT for https://github.com/rust-lang/rust/pull/106633 1.70
                // const MAX: Self = Self([<$p$bsize>]::MAX);
                //
                // SAFETY: constant value
                const MAX: Self = Self(unsafe { [<$p$bsize>]::new_unchecked(1) });
            }

            /* count */

            impl Count for [<$name$bsize>] {
                fn is_countable(&self) -> bool { true }
            }

            impl Countable for [<$name$bsize>] {
                /// Returns the next countable value, skipping 0.
                ///
                /// # Errors
                /// Errors if the operation results in overflow.
                fn next(&self) -> NumeraResult<Self> {
                    let next = self.0.get().checked_add(1).ok_or(IntegerError::Overflow)?;
                    // SAFETY: we've checked the value
                    Ok(Self(unsafe { [<$p$bsize>]::new_unchecked(next) }))
                }
                /// Returns the previous countable value, skipping 0.
                ///
                /// # Errors
                /// Errors if the operation results in underflow.
                fn previous(&self) -> NumeraResult<Self> {
                    let prev = self.0.get().checked_sub(1).ok_or(IntegerError::Underflow)?;
                    Ok(Self([<$p$bsize>]::new(prev).ok_or(IntegerError::Zero)?))
                }
            }

            /* ident */

            impl Ident for [<$name$bsize>] {
                #[inline(always)]
                fn can_zero(&self) -> bool { false }
                #[inline(always)]
                fn can_one(&self) -> bool { false }
                #[inline(always)]
                fn can_neg_one(&self) -> bool { true }

                #[inline(always)]
                fn is_zero(&self) -> bool { false }
                #[inline(always)]
                fn is_one(&self) -> bool { false }
                #[inline(always)]
                fn is_neg_one(&self) -> bool { self.0.get() == 1 }
            }
            impl NonZero for [<$name$bsize>] {}
            impl NonOne for [<$name$bsize>] {}
            impl ConstNegOne for [<$name$bsize>] {
                // SAFETY: constant value
                const NEG_ONE: Self = Self(unsafe { [<$p$bsize>]::new_unchecked(1) });
            }
            impl NegOne for [<$name$bsize>] {
                #[inline(always)]
                fn new_neg_one() -> Self {
                    // SAFETY: constant value
                    Self(unsafe { [<$p$bsize>]::new_unchecked(1) })
                }
            }

            /* number */

            impl Number for [<$name$bsize>] {
                type Inner = [<u$bsize>];

                /// This constructur always return an error. Please use the
                /// [`new_neg`][NegSigned#method.new_neg] method from the
                /// [`NegSigned`] trait.
                #[inline(always)]
                fn new(_value: Self::Inner) -> NumeraResult<Self> {
                    Err(IntegerError::ZeroOrMore.into())

                    // ALTERNATIVE
                    // Ok(Self([<$p$bsize>]::new(value).ok_or(IntegerError::Zero)?))
                }
                /// Please note that the `value` provided will interpreted as negative.
                #[inline(always)]
                unsafe fn new_unchecked(value: Self::Inner) -> Self {
                    debug_assert![value != 0];
                    Self([<$p$bsize>]::new_unchecked(value))
                }
            }
        }
    };
}

/* definitions */

define_negative_integer_sized![multi NegativeInteger, NonZeroU,
    "integer number", ", from the set $\\Z^-$.",
    // "",
    "-", MAX, 1,
    ("An", 8), ("A", 16), ("A", 32), ("A", 64), ("A", 128)
];
