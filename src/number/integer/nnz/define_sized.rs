// numera::number::integer::nnz::define_sized
//
//!
//
// TOC
//
// - macro
//   - define_non_negative_integer_sized
// - definitions
//   - NonNegativeInteger[8|16|32|64|128]

use crate::{
    error::{IntegerError, NumeraResult as Result},
    number::traits::{
        Bound, ConstLowerBounded, ConstOne, ConstUpperBounded, ConstZero, Count, Countable, Ident,
        LowerBounded, NonNegOne, Number, One, Sign, Unsigned, UpperBounded, Zero,
    },
};

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
macro_rules! define_nonnegative_integer_sized {
    // defines multiple integer types, with an inner primitive.
    (multi $name:ident, $p:ident,
     $doc_num:literal, $doc_type:literal, // $doc_new:literal,
     $sign:literal, $lower:expr, $upper:expr,
     $(($det:literal,$bsize:expr)),+) => {
        $(
            define_nonnegative_integer_sized![single $name, $p,
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
            #[doc = "\n\nThe range of valid numeric values is $\\lbrack"
            $doc_sign "$" $doc_lower " $\\dots$ [`"
            $p$bsize "::" $doc_upper "`]$\\rbrack$."]

            #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
            pub struct [<$name$bsize>](pub(crate) [< $p$bsize >]);

            /* sign */

            impl Sign for [<$name$bsize>] {
                fn can_negative(&self) -> bool { false }
                fn can_positive(&self) -> bool { true }
                fn is_negative(&self) -> bool { false }
                fn is_positive(&self) -> bool { self.0.is_positive() }
            }
            impl Unsigned for [<$name$bsize>] {}

            /* bound */

            impl Bound for [<$name$bsize>] {
                fn is_lower_bounded(&self) -> bool { true }
                fn is_upper_bounded(&self) -> bool { true }
                fn lower_bound(&self) -> Option<Self> where Self: Sized {
                    Some(Self([<$p$bsize>]::MIN))
                }
                fn upper_bound(&self) -> Option<Self> where Self: Sized {
                    Some(Self([<$p$bsize>]::MAX))
                }
            }
            impl LowerBounded for [<$name$bsize>] {
                fn new_min() -> Self { <Self as ConstLowerBounded>::MIN }
            }
            impl UpperBounded for [<$name$bsize>] {
                fn new_max() -> Self { <Self as ConstUpperBounded>::MAX }
            }
            impl ConstLowerBounded for [<$name$bsize>] {
                const MIN: Self = Self([<$p$bsize>]::MIN);
            }
            impl ConstUpperBounded for [<$name$bsize>] {
                const MAX: Self = Self([<$p$bsize>]::MAX);
            }

            /* count */

            impl Count for [<$name$bsize>] {
                fn is_countable(&self) -> bool { true }
            }

            impl Countable for [<$name$bsize>] {
                fn next(&self) -> Result<Self> {
                    Ok(Self(self.0.checked_add(1).ok_or(IntegerError::Overflow)?))
                }
                fn previous(&self) -> Result<Self> {
                    Ok(Self(self.0.checked_sub(1).ok_or(IntegerError::Underflow)?))
                }
            }

            /* ident */

            impl Ident for [<$name$bsize>] {
                fn can_zero(&self) -> bool { true }
                fn can_one(&self) -> bool { true }
                fn can_neg_one(&self) -> bool { false }

                fn is_zero(&self) -> bool { self.0 == 0 }
                fn is_one(&self) -> bool { self.0 == 1 }
                fn is_neg_one(&self) -> bool { false }
            }
            impl ConstZero for [<$name$bsize>] { const ZERO: Self = Self(0); }
            impl Zero for [<$name$bsize>] { fn new_zero() -> Self { Self(0) } }
            impl ConstOne for [<$name$bsize>] { const ONE: Self = Self(1); }
            impl One for [<$name$bsize>] { fn new_one() -> Self { Self(1) } }
            impl NonNegOne for [<$name$bsize>] {}

            /* number */

            impl Number for [<$name$bsize>] {
                type Inner = [<$p$bsize>];

                fn new(value: Self::Inner) -> Result<Self> { Ok(Self(value)) }
                unsafe fn new_unchecked(value: Self::Inner) -> Self { Self(value) }

                // TODO: DELETE
                // fn into_inner(self) -> Self::Inner { self.0 }
                // fn ref_inner(&self) -> &Self::Inner { &self.0 }
            }
        }
    };
}

/* definitions */

define_nonnegative_integer_sized![multi NonNegativeInteger, u,
    "non-negative integer number", ", from the set $\\Z^*$",
    // "",
    "", 0, MAX,
    ("An", 8), ("A", 16), ("A", 32), ("A", 64), ("A", 128)
];
