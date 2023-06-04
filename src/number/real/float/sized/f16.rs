// numera::number::real::float::f::sized::external_16
//
//!
//
// TODO

use half::{bf16, f16};

use crate::{
    error::{NumeraError, NumeraResult, RealError},
    number::{
        real::float::fns::abs32,
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
/// - defines a Float of a concrete size.
/// - implements Numbers: Bound + Count + Ident + Sign
/// - implements Default → 0
///
/// # Args
/// - `$name`: the base name of the real. E.g. `Float`.
/// - `$abbr`: the base abbreviated name, E.g. `F`.
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
macro_rules! define_float_sized {
    // defines multiple flot types, with an inner primitive.
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
            define_float_sized![single $name, $abbr, $p,
               $doc_num, $doc_type, // $doc_new,
               $doc_sign, $doc_lower, $doc_upper,
               ($doc_det, $b,
                larger: $larger, $larger_b,
                smaller: $smaller, $smaller_b
               )];
        )+
    };
    // defines a single float type, with an inner primitive.
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
        #[doc = "also known as [`" [<$abbr$b>] "`][super::super::" [<$abbr$b>] "]."]
        #[doc = "\n\nThe range of valid numeric values is $\\lbrack"
        $doc_sign "$[`" $p$b "::" $doc_lower "`] $\\dots$ [`"
        $p$b "::" $doc_upper "`]$\\rbrack$."]
        #[doc = "\n\nIt is equivalent to the [`" [<$p$b>] "`] external primitive."]
        #[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
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

        // TODO
        // impl_larger_smaller![$name, $b, Floats,
        //     larger: $larger, $larger_b, smaller: $smaller, $smaller_b
        // ];

        /* sign */

        impl Sign for [<$name$b>] {
            #[inline]
            fn can_negative(&self) -> bool { true }
            #[inline]
            fn can_positive(&self) -> bool { true }
            #[inline]
            fn is_negative(&self) -> bool { self.0.is_sign_negative() }
            #[inline]
            fn is_positive(&self) -> bool { self.0.is_sign_positive() }
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
            fn lower_bound(&self) -> Option<Self> { Some([<$name$b>]::MAX) }
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
        /// Unimplemented.
        impl Countable for [<$name$b>] {
            /// Not implemented.
            ///
            /// # Errors
            /// Returns [`NotImplemented`][NumeraError::NotImplemented].
            #[inline]
            fn next(&self) -> NumeraResult<Self> { Err(NumeraError::NotImplemented) }
            /// Not implemented.
            ///
            /// # Errors
            /// Returns [`NotImplemented`][NumeraError::NotImplemented].
            #[inline]
            fn previous(&self) -> NumeraResult<Self> { Err(NumeraError::NotImplemented) }
        }

        /* ident */

        // Checks whether the inner primitive values are within a certain error margin.
        #[inline]
        fn [<approx_eq_$p$b>](a: [<$p$b>], b: [<$p$b>], epsilon: [<$p$b>]) -> bool {
            #[cfg(feature = "std")]
            return [<$p$b>]::from_f32((a - b).to_f32().abs()) <= epsilon;

            #[cfg(not(feature = "std"))]
            return [<$p$b>]::from_f32(abs32((a - b).to_f32())) <= epsilon;
        }

        impl Ident for [<$name$b>] {
            #[inline]
            fn can_zero(&self) -> bool { true }
            #[inline]
            fn can_one(&self) -> bool { true }
            #[inline]
            fn can_neg_one(&self) -> bool { true }

            #[inline]
            fn is_zero(&self) -> bool {
                [<approx_eq_$p$b>](self.0, [<$p$b>]::ZERO, [<$p$b>]::EPSILON)
            }
            #[inline]
            fn is_one(&self) -> bool {
                [<approx_eq_$p$b>](self.0, [<$p$b>]::ONE, [<$p$b>]::EPSILON)
            }
            #[inline]
            fn is_neg_one(&self) -> bool {
                [<approx_eq_$p$b>](self.0, [<$p$b>]::NEG_ONE, [<$p$b>]::EPSILON)
            }
        }
        impl ConstZero for [<$name$b>] { const ZERO: Self = Self([<$p$b>]::ZERO); }
        impl Zero for [<$name$b>] {
            #[inline]
            fn new_zero() -> Self { Self::ZERO }
        }
        impl ConstOne for [<$name$b>] { const ONE: Self = Self([<$p$b>]::ONE); }
        impl One for [<$name$b>] {
            #[inline]
            fn new_one() -> Self { Self::ONE }
        }
        impl ConstNegOne for [<$name$b>] { const NEG_ONE: Self = Self([<$p$b>]::NEG_ONE); }
        impl NegOne for [<$name$b>] {
            #[inline]
            fn new_neg_one() -> Self { Self::NEG_ONE }
        }

        /* Numbers */

        impl Numbers for [<$name$b>] {
            type Parts = [<$p$b>];

            #[doc = "Returns a new `" [<$name$b>] " from the constituent parts`."]
            ///
            /// # Errors
            /// This function can't fail.
            #[inline]
            fn from_parts(value: Self::Parts) -> NumeraResult<Self> { Ok(Self(value)) }

            #[doc = "Returns a new `" [<$name$b>] "` from the constituent parts`."]
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

define_float_sized![multi Float, F, f,
    "floating-point number", ", from the set $\\R$",
    // "",
    "", MIN, MAX,
    ("A", 16, larger: true, 32, smaller: false, 16)
];
define_float_sized![multi BFloat, Bf, bf,
    "floating-point number", ", from the set $\\R$",
    // "",
    "", MIN, MAX,
    ("A", 16, larger: true, 32, smaller: false, 16)
];
