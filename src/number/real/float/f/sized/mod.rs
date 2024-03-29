// numera::number::real::float::f::sized
//
//!
//
// TOC
//
// - macro
// - separate implementations
// - definitions

#[cfg(feature = "decstr")]
mod decstr;
#[cfg(feature = "decstr")]
pub use self::decstr::*;

#[cfg(feature = "half")]
use half::{bf16, f16};

#[cfg(feature = "twofloat")]
use twofloat::TwoFloat;

#[cfg(not(feature = "std"))]
use crate::number::real::float::fns::{abs32, abs64};
use crate::{
    error::{NumeraResult, RealErrors},
    number::traits::{
        Bound, ConstLowerBounded, ConstNegOne, ConstOne, ConstUpperBounded, ConstZero, Count,
        Countable, Ident, LowerBounded, NegOne, Negative, Number, One, Positive, Sign,
        UpperBounded, Zero,
    },
};
use core::fmt;
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
//
// TODO: IMPROVE: modularize, receive countable bool
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
            $doc_sign "$[`" $pname "::" $doc_lower "`] $\\dots$ [`"
            $pname "::" $doc_upper "`]$\\rbrack$."]
            ///
            #[doc = $doc_extra ]
            #[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
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
                fn is_positive(&self) -> bool { self.0.is_sign_positive() }
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
                fn lower_bound(&self) -> Option<Self> { Some([<$name $b>]::MIN) }
                #[inline]
                fn upper_bound(&self) -> Option<Self> { Some([<$name $b>]::MAX) }
            }
            impl LowerBounded for [<$name $b>] {
                #[inline]
                fn new_min() -> Self { [<$name $b>]::MIN }
            }
            impl UpperBounded for [<$name $b>] {
                #[inline]
                fn new_max() -> Self { [<$name $b>]::MAX }
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
                fn is_countable(&self) -> bool { true }
            }

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
                type InnermostRepr = $pname;

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
pub(crate) use define_float_sized;

/* separate implementations */

impl Countable for Float32 {
    // implementation based on:
    // https://doc.rust-lang.org/std/primitive.f32.html#method.next_up
    #[inline]
    fn next(&self) -> NumeraResult<Self> {
        const TINY_BITS: u32 = 0x1; // Smallest positive f32.
        const CLEAR_SIGN_MASK: u32 = 0x7fff_ffff;

        let bits = self.0.to_bits();
        if self.0.is_nan() || bits == f32::INFINITY.to_bits() {
            return Err(RealErrors::NaN.into());
        }

        let abs = bits & CLEAR_SIGN_MASK;
        let next_bits = if abs == 0 {
            TINY_BITS
        } else if bits == abs {
            bits + 1
        } else {
            bits - 1
        };
        Ok(Self(f32::from_bits(next_bits)))
    }

    // implementation based on:
    // https://doc.rust-lang.org/std/primitive.f32.html#method.next_down
    #[inline]
    fn previous(&self) -> NumeraResult<Self> {
        const NEG_TINY_BITS: u32 = 0x8000_0001; // Smallest (in magnitude) negative f32.
        const CLEAR_SIGN_MASK: u32 = 0x7fff_ffff;

        let bits = self.0.to_bits();
        if self.0.is_nan() || bits == f32::NEG_INFINITY.to_bits() {
            return Err(RealErrors::NaN.into());
        }

        let abs = bits & CLEAR_SIGN_MASK;
        let next_bits = if abs == 0 {
            NEG_TINY_BITS
        } else if bits == abs {
            bits - 1
        } else {
            bits + 1
        };
        Ok(Self(f32::from_bits(next_bits)))
    }
}

impl Countable for Float64 {
    // implementation based on:
    // https://doc.rust-lang.org/std/primitive.f64.html#method.next_up
    #[inline]
    fn next(&self) -> NumeraResult<Self> {
        const TINY_BITS: u64 = 0x1; // Smallest positive f64.
        const CLEAR_SIGN_MASK: u64 = 0x7fff_ffff_ffff_ffff;

        let bits = self.0.to_bits();
        if self.0.is_nan() || bits == f64::INFINITY.to_bits() {
            return Err(RealErrors::NaN.into());
        }

        let abs = bits & CLEAR_SIGN_MASK;
        let next_bits = if abs == 0 {
            TINY_BITS
        } else if bits == abs {
            bits + 1
        } else {
            bits - 1
        };
        Ok(Self(f64::from_bits(next_bits)))
    }

    // implementation based on:
    // https://doc.rust-lang.org/std/primitive.f64.html#method.next_down
    #[inline]
    fn previous(&self) -> NumeraResult<Self> {
        const NEG_TINY_BITS: u64 = 0x8000_0000_0000_0001; // Smallest (in magnitude) negative f64.
        const CLEAR_SIGN_MASK: u64 = 0x7fff_ffff_ffff_ffff;

        let bits = self.0.to_bits();
        if self.0.is_nan() || bits == f64::NEG_INFINITY.to_bits() {
            return Err(RealErrors::NaN.into());
        }

        let abs = bits & CLEAR_SIGN_MASK;
        let next_bits = if abs == 0 {
            NEG_TINY_BITS
        } else if bits == abs {
            bits - 1
        } else {
            bits + 1
        };
        Ok(Self(f64::from_bits(next_bits)))
    }
}

// Checks whether the inner primitive values are within a certain error margin.
#[inline]
fn approx_eq_f32(a: f32, b: f32, epsilon: f32) -> bool {
    #[cfg(feature = "std")]
    return (a - b).abs() <= epsilon;

    #[cfg(not(feature = "std"))]
    return abs32(a - b) <= epsilon;
}
// Checks whether the inner primitive values are within a certain error margin.
#[inline]
fn approx_eq_f64(a: f64, b: f64, epsilon: f64) -> bool {
    #[cfg(feature = "std")]
    return (a - b).abs() <= epsilon;

    #[cfg(not(feature = "std"))]
    return abs64(a - b) <= epsilon;
}
impl ConstZero for Float32 {
    const ZERO: Self = Self(f32::ZERO);
}
impl ConstOne for Float32 {
    const ONE: Self = Self(f32::ONE);
}
impl ConstNegOne for Float32 {
    const NEG_ONE: Self = Self(f32::NEG_ONE);
}
impl ConstZero for Float64 {
    const ZERO: Self = Self(f64::ZERO);
}
impl ConstOne for Float64 {
    const ONE: Self = Self(f64::ONE);
}
impl ConstNegOne for Float64 {
    const NEG_ONE: Self = Self(f64::NEG_ONE);
}

#[cfg(feature = "half")]
use impl_f16::{approx_eq_bf16, approx_eq_f16};
#[cfg(feature = "half")]
mod impl_f16 {
    use super::{
        bf16, f16, BrainFloat16, ConstNegOne, ConstOne, ConstZero, Countable, Float16, NumeraResult,
    };
    use crate::error::NumeraErrors;
    #[cfg(not(feature = "std"))]
    use crate::number::real::float::fns::abs32;

    impl ConstZero for Float16 {
        const ZERO: Self = Self(f16::ZERO);
    }
    impl ConstOne for Float16 {
        const ONE: Self = Self(f16::ONE);
    }
    impl ConstNegOne for Float16 {
        const NEG_ONE: Self = Self(f16::NEG_ONE);
    }

    impl ConstZero for BrainFloat16 {
        const ZERO: Self = Self(bf16::ZERO);
    }
    impl ConstOne for BrainFloat16 {
        const ONE: Self = Self(bf16::ONE);
    }
    impl ConstNegOne for BrainFloat16 {
        const NEG_ONE: Self = Self(bf16::NEG_ONE);
    }

    /// Unimplemented.
    impl Countable for Float16 {
        /// Not implemented.
        ///
        /// # Errors
        /// Returns [`NotImplemented`][NumeraErrors::NotImplemented].
        #[inline]
        fn next(&self) -> NumeraResult<Self> {
            Err(NumeraErrors::NotImplemented)
        }
        /// Not implemented.
        ///
        /// # Errors
        /// Returns [`NotImplemented`][NumeraErrors::NotImplemented].
        #[inline]
        fn previous(&self) -> NumeraResult<Self> {
            Err(NumeraErrors::NotImplemented)
        }
    }

    /// Unimplemented.
    impl Countable for BrainFloat16 {
        /// Not implemented.
        ///
        /// # Errors
        /// Returns [`NotImplemented`][NumeraErrors::NotImplemented].
        #[inline]
        fn next(&self) -> NumeraResult<Self> {
            Err(NumeraErrors::NotImplemented)
        }
        /// Not implemented.
        ///
        /// # Errors
        /// Returns [`NotImplemented`][NumeraErrors::NotImplemented].
        #[inline]
        fn previous(&self) -> NumeraResult<Self> {
            Err(NumeraErrors::NotImplemented)
        }
    }

    // Checks whether the inner primitive values are within a certain error margin.
    #[inline]
    pub(super) fn approx_eq_f16(a: f16, b: f16, epsilon: f16) -> bool {
        #[cfg(feature = "std")]
        return f16::from_f32((a - b).to_f32().abs()) <= epsilon;

        #[cfg(not(feature = "std"))]
        return f16::from_f32(abs32((a - b).to_f32())) <= epsilon;
    }
    // Checks whether the inner primitive values are within a certain error margin.
    #[inline]
    pub(super) fn approx_eq_bf16(a: bf16, b: bf16, epsilon: bf16) -> bool {
        #[cfg(feature = "std")]
        return bf16::from_f32((a - b).to_f32().abs()) <= epsilon;

        #[cfg(not(feature = "std"))]
        return bf16::from_f32(abs32((a - b).to_f32())) <= epsilon;
    }
}

#[cfg(feature = "twofloat")]
use impl_twofloat::approx_eq_TwoFloat;
#[cfg(feature = "twofloat")]
mod impl_twofloat {
    use super::{ConstNegOne, ConstOne, ConstZero, Countable, NumeraResult, TwoFloat, TwoFloat128};
    use crate::error::NumeraErrors;

    impl ConstZero for TwoFloat128 {
        const ZERO: Self = Self(TwoFloat::from_f64(0.0));
    }
    impl ConstOne for TwoFloat128 {
        const ONE: Self = Self(TwoFloat::from_f64(1.0));
    }
    impl ConstNegOne for TwoFloat128 {
        const NEG_ONE: Self = Self(TwoFloat::from_f64(-1.0));
    }

    /// Unimplemented.
    impl Countable for TwoFloat128 {
        /// Not implemented.
        ///
        /// # Errors
        /// Returns [`NotImplemented`][NumeraErrors::NotImplemented].
        #[inline]
        fn next(&self) -> NumeraResult<Self> {
            Err(NumeraErrors::NotImplemented)
        }
        /// Not implemented.
        ///
        /// # Errors
        /// Returns [`NotImplemented`][NumeraErrors::NotImplemented].
        #[inline]
        fn previous(&self) -> NumeraResult<Self> {
            Err(NumeraErrors::NotImplemented)
        }
    }

    // Checks whether the inner primitive values are within a certain error margin.
    #[inline]
    #[allow(non_snake_case)]
    pub(super) fn approx_eq_TwoFloat(a: TwoFloat, b: TwoFloat, epsilon: TwoFloat) -> bool {
        return (a - b).abs() <= epsilon;
    }
}

/* definitions */

// /* ieee 754 binary */
#[cfg(feature = "half")]
define_float_sized![Float, F, f16,
    "ieee-754 half-precision *binary* floating-point number ([w][0w])", ", from the set $\\R$",
    "It is comprised of 1 sign bit, 5 exponent bits, and 10 significand bits.

[0w]: https://en.wikipedia.org/wiki/Half-precision_floating-point_format
",
    "", MIN, MAX,
    ("A", 16, larger: true, 32, smaller: false, 16)
];
define_float_sized![Float, F, f32,
    "ieee-754 single-precision *binary* floating-point number ([w][0w])", ", from the set $\\R$",
    "It is comprised of 1 sign bit, 8 exponent bits, and 23 significand bits.

[0w]: https://en.wikipedia.org/wiki/Single-precision_floating-point_format
",
    "", MIN, MAX,
    ("A", 32, larger: true, 64, smaller: false, 32)
];
define_float_sized![Float, F, f64,
    "ieee-754 double-precision *binary* floating-point number ([w][0w])", ", from the set $\\R$",
    "It is comprised of 1 sign bit, 11 exponent bits, and 52 significand bits.

[0w]: https://en.wikipedia.org/wiki/Double-precision_floating-point_format
",
    "", MIN, MAX,
    ("A", 64, larger: false, 64, smaller: true, 32)
];

// /* other */
#[cfg(feature = "half")]
define_float_sized![BrainFloat, Bf, bf16,
    "*brain* floating-point ([w][0w]) number", ", from the set $\\R$",
    "It is comprised of 1 sign bit, 8 exponent bits, and 7 significand bits.

[0w]: https://en.wikipedia.org/wiki/Bfloat16_floating-point_format
",
    "", MIN, MAX,
    ("A", 16, larger: true, 32, smaller: false, 16)
];

#[cfg(feature = "twofloat")]
define_float_sized![TwoFloat, Tf, TwoFloat,
    "*two double-precision* binary floating-point number ([w][0w])", ", from the set $\\R$",
    "It is comprised of 1 sign bit, 11 exponent bits and 106 significand bits.

So its range essentially the same as [`Float64`], and its precision slightly
less than `Float128` *(not yet implemented)*.

[0w]: https://en.wikipedia.org/wiki/Quadruple-precision_floating-point_format#Double-double_arithmetic
",
    "", MIN, MAX,
    ("A", 128, larger: false, 128, smaller: true, 64)
];
