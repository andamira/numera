// numera::number::real::float::f::sized::decstr
//
//!
//
// TOC
//
// - macro
// - separate implementations
// - definitions

use crate::all::Uncountable;
#[cfg(not(feature = "std"))]
use crate::number::real::float::fns::{abs32, abs64};
use crate::{
    error::NumeraResult,
    number::traits::{
        Bound, ConstLowerBounded, ConstNegOne, ConstOne, ConstUpperBounded, ConstZero, Count,
        Ident, LowerBounded, NegOne, Negative, Number, One, Positive, Sign, UpperBounded, Zero,
    },
};
use core::fmt;
use decstr::{Bitstring128, Bitstring32, Bitstring64};
use devela::paste;

/* macro */

/// # What it does
/// - defines a Float of a concrete size.
/// - implements Number: Bound + Count + Ident + Sign
/// - implements Default → 0
///
/// # Args
/// - `$name`: the base name of the real. E.g. `Float`.
/// - `$abbr`: the base abbreviated name, E.g. `F`.
/// - `$pname`: the primitive name (f16, bf16, TwoFloat).
///
/// - `$doc_num`: the type of number.
/// - `$doc_type`: adds to the type doc-comment.
/// - `$doc_extra`: extra doc-comment paragraph.
///
/// - `$doc_sign`: an optional negative sign
/// - `$doc_lower`: the lower bound of the number type.
/// - `$doc_upper`: the upper bound of the number type.
///
/// - `$doc_det`: the determinant before the bit size. e.g. "An" (8-bit) or "A" 16-bit.
/// - `$b`: the size in bits of the primitive used.
macro_rules! define_float_sized {
    // defines a single float type, with an inner primitive.
    ($name:ident, $abbr:ident, $pname:ident,
     $doc_num:literal, $doc_type:literal, $doc_extra:literal,
     $doc_sign:literal, $doc_lower:expr, $doc_upper:expr,
     (
      $doc_det:literal, $b:expr,
      larger: $larger:literal, $larger_b:literal,
      smaller: $smaller:literal, $smaller_b:literal
     )
    ) => {
        paste! {
            #[doc = $doc_det " "$b "-bit " $doc_num $doc_type ","]
            #[doc = "also known as [`" [<$abbr $b>] "`][super::super::" [<$abbr $b>] "]."]
            #[doc = "\n\nThe range of valid numeric values is $\\lbrack"
            $doc_sign "$[`" $doc_lower "`][" [<$name $b>] "::" $doc_lower "] $\\dots$ [`"
                $doc_upper "`][" [<$name $b>] "::" $doc_upper "]$\\rbrack$."]
            ///
            #[doc = $doc_extra ]
            #[derive(Clone, Copy)] // No PartialEq, PartialOrd
            pub struct [<$name $b>](pub $pname);

            impl fmt::Display for [<$name $b>]  {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "{}", self.0)
                }
            }
            impl fmt::Debug for [<$name $b>]  {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "{}({})", stringify!([<$abbr $b>]), self.0)
                }
            }
            impl Default for [<$name $b>]  {
                fn default() -> Self {
                    Self::new_zero()
                }
            }
            impl PartialEq for [<$name $b>]  {
                fn eq(&self, other: &Self) -> bool {
                    // IMPROVE
                    self.0.to_f64().eq(&other.0.to_f64())
                }
            }
            impl PartialOrd for [<$name $b>]  {
                fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                    // IMPROVE
                    self.0.to_f64().partial_cmp(&other.0.to_f64())
                }
            }

            /// # Constructors
            impl [<$name $b>]  {
                #[doc = "Returns a new `" [<$name $b>] "`."]
                #[inline]
                pub const fn new(value: $pname) -> Self { Self(value) }
            }

            /* resizing */

            // TODO
            // impl_larger_smaller![$name, $b, Floats,
            //     larger: $larger, $larger_b, smaller: $smaller, $smaller_b
            // ];

            /* sign */

            impl Sign for [<$name $b>] {
                #[inline]
                fn can_negative(&self) -> bool { true }
                #[inline]
                fn can_positive(&self) -> bool { true }
                #[inline]
                fn is_negative(&self) -> bool { self.0.is_sign_negative() }
                #[inline]
                fn is_positive(&self) -> bool { !self.0.is_sign_negative() }
            }
            impl Positive for [<$name $b>] {}
            impl Negative for [<$name $b>] {}

            /* bound */

            impl Bound for [<$name $b>] {
                #[inline]
                fn is_lower_bounded(&self) -> bool { true }
                #[inline]
                fn is_upper_bounded(&self) -> bool { true }
                #[inline]
                fn lower_bound(&self) -> Option<Self> { Some([<$name $b>]::new_min()) }
                #[inline]
                fn upper_bound(&self) -> Option<Self> { Some([<$name $b>]::new_max()) }
            }
            impl LowerBounded for [<$name $b>] {
                #[inline]
                fn new_min() -> Self { Self::MIN }
            }
            impl UpperBounded for [<$name $b>] {
                #[inline]
                fn new_max() -> Self { Self::MAX }
            }
            impl ConstLowerBounded for [<$name $b>] {
                const MIN: Self = Self($pname::MIN);
            }
            impl ConstUpperBounded for [<$name $b>] {
                const MAX: Self = Self($pname::MAX);
            }

            /* count */

            impl Count for [<$name $b>] {
                #[inline]
                fn is_countable(&self) -> bool { false }
            }
            impl Uncountable for [<$name $b>] {}

            /* ident */

            impl Ident for [<$name $b>] {
                #[inline]
                fn can_zero(&self) -> bool { true }
                #[inline]
                fn can_one(&self) -> bool { true }
                #[inline]
                fn can_neg_one(&self) -> bool { true }

                #[inline]
                fn is_zero(&self) -> bool {
                    [<approx_eq_$pname>](self.0, $pname::ZERO, $pname::EPSILON)
                }
                #[inline]
                fn is_one(&self) -> bool {
                    [<approx_eq_$pname>](self.0, $pname::ONE, $pname::EPSILON)
                }
                #[inline]
                fn is_neg_one(&self) -> bool {
                    [<approx_eq_$pname>](self.0, $pname::NEG_ONE, $pname::EPSILON)
                }
            }

            impl ConstZero for [<$name $b>] { const ZERO: Self = Self($pname::ZERO); }
            impl ConstOne for [<$name $b>] { const ONE: Self = Self($pname::ONE); }
            impl ConstNegOne for [<$name $b>] { const NEG_ONE: Self = Self($pname::NEG_ONE); }
            impl Zero for [<$name $b>] {
                #[inline]
                fn new_zero() -> Self { Self::ZERO }
            }
            impl One for [<$name $b>] {
                #[inline]
                fn new_one() -> Self { Self::ONE }
            }
            impl NegOne for [<$name $b>] {
                #[inline]
                fn new_neg_one() -> Self { Self::NEG_ONE }
            }

            /* Number */

            impl Number for [<$name $b>] {
                type InnerRepr = $pname;
                type InnermostRepr = $pname; // IMPROVE: [u8; 4…16] | u32…u128?

                #[doc = "Returns a new `" [<$name $b>] " from the inner representation`."]
                ///
                /// # Errors
                /// This function can't fail.
                #[inline]
                fn from_inner_repr(value: Self::InnerRepr) -> NumeraResult<Self> { Ok(Self(value)) }

                #[doc = "Returns a new `" [<$name $b>] "` from the inner representation`."]
                ///
                /// # Safety
                /// This function is safe.
                #[inline]
                #[cfg(not(feature = "safe"))]
                #[cfg_attr(feature = "nightly", doc(cfg(feature = "not(safe)")))]
                unsafe fn from_inner_repr_unchecked(value: Self::InnerRepr) -> Self { Self(value) }

                #[doc = "Returns a new `" [<$name $b>] "` from the innermost representation."]
                ///
                /// # Errors
                /// This function can't fail.
                #[inline]
                fn from_innermost_repr(value: Self::InnermostRepr) -> NumeraResult<Self> {
                    Ok(Self(value))
                }

                #[doc = "Returns a new `" [<$name $b>] "` from the innermost representation."]
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
        }
    };
}

// Checks whether the inner primitive values are within a certain error margin.
// IMPROVE
#[inline]
#[allow(non_snake_case)]
pub(super) fn approx_eq_Bitstring32(a: Bitstring32, b: Bitstring32, epsilon: Bitstring32) -> bool {
    if let Some(a) = a.to_f32() {
        if let Some(b) = b.to_f32() {
            if let Some(epsilon) = epsilon.to_f32() {
                #[cfg(feature = "std")]
                return (a - b).abs() <= epsilon;

                #[cfg(not(feature = "std"))]
                return abs32(a - b) <= epsilon;
            }
        }
    }
    false
}
#[inline]
#[allow(non_snake_case)]
pub(super) fn approx_eq_Bitstring64(a: Bitstring64, b: Bitstring64, epsilon: Bitstring64) -> bool {
    if let Some(a) = a.to_f64() {
        if let Some(b) = b.to_f64() {
            if let Some(epsilon) = epsilon.to_f64() {
                #[cfg(feature = "std")]
                return (a - b).abs() <= epsilon;

                #[cfg(not(feature = "std"))]
                return abs64(a - b) <= epsilon;
            }
        }
    }
    false
}
#[inline]
#[allow(non_snake_case)]
pub(super) fn approx_eq_Bitstring128(
    a: Bitstring128,
    b: Bitstring128,
    epsilon: Bitstring128,
) -> bool {
    if let Some(a) = a.to_f64() {
        if let Some(b) = b.to_f64() {
            if let Some(epsilon) = epsilon.to_f64() {
                #[cfg(feature = "std")]
                return (a - b).abs() <= epsilon;

                #[cfg(not(feature = "std"))]
                return abs64(a - b) <= epsilon;
            }
        }
    }
    false
}

/* ieee 754 decimal */
#[cfg(feature = "decstr")]
define_float_sized![DecFloat, Df, Bitstring32,
    "ieee-754 single-precision *decimal* floating-point ([w][0w]) number", ",
    from the set $\\R$",
    "Supports 7 decimal digits of significand and a normalized exponent range of −101 to +90.

It doesn't implement any arithmetic operations and is mainly intended for storage and interchange.

[0w]: https://en.wikipedia.org/wiki/Decimal32_floating-point_format
",
    "", MIN, MAX,
    ("A", 32, larger: true, 64, smaller: false, 32)
];
#[cfg(feature = "decstr")]
define_float_sized![DecFloat, Df, Bitstring64,
    "ieee-754 double-precision *decimal* floating-point ([w][0w]) number",
    ", from the set $\\R$",
    "Supports 16 decimal digits of significand and a normalized exponent range of −398 to +369.

It doesn't implement any arithmetic operations and is mainly intended for storage and interchange.

[0w]: https://en.wikipedia.org/wiki/Decimal64_floating-point_format
",
    "", MIN, MAX,
    ("A", 64, larger: true, 128, smaller: true, 32)
];
#[cfg(feature = "decstr")]
define_float_sized![DecFloat, Df, Bitstring128,
    "ieee-754 quadruple-precision *decimal* floating-point ([w][0w]) number",
    ", from the set $\\R$",
    "Supports 34 decimal digits of significand and a normalized exponent range of −6176 to +6111.

It doesn't implement any arithmetic operations and is mainly intended for storage and interchange.

[0w]: https://en.wikipedia.org/wiki/Decimal128_floating-point_format
",
    "", MIN, MAX,
    ("A", 128, larger: false, 128, smaller: true, 64)
];
