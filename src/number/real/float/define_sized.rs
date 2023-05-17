// numera::number::integer::z::define_sized
//
//!
//
// TOC
//
// - macro
//   - define_integer_sized
// - definitions
//   - Float[32|64]

// #[cfg(feature = "try_from")]
// use crate::number::real::Reals;
use crate::{
    error::{NumeraResult, RealError},
    number::traits::{
        Bound, ConstLowerBounded, ConstNegOne, ConstOne, ConstUpperBounded, ConstZero, Count,
        Countable, Ident, LowerBounded, NegOne, Number, One, Sign, Signed, UpperBounded, Zero,
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
/// - `$name`: the base name of the integer e.g. `Float`.
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
macro_rules! define_float_sized {
    // defines multiple flot types, with an inner primitive.
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
            define_float_sized![single $name, $p,
               $doc_num, $doc_type, // $doc_new,
               $doc_sign, $doc_lower, $doc_upper,
               ($doc_det, $bsize,
                larger: $larger, $larger_bsize,
                smaller: $smaller, $smaller_bsize
               )];
        )+
    };
    // defines a single float type, with an inner primitive.
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
        #[doc = "It is equivalent to the [`" [<f$bsize>] "`] primitive."]
        #[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
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

        // TODO
        // impl_larger_smaller![$name, $bsize, Floats,
        //     larger: $larger, $larger_bsize, smaller: $smaller, $smaller_bsize
        // ];

        /* sign */

        impl Sign for [<$name$bsize>] {
            #[inline]
            fn can_negative(&self) -> bool { true }
            #[inline]
            fn can_positive(&self) -> bool { true }
            #[inline]
            fn is_negative(&self) -> bool { self.0.is_sign_negative() }
            #[inline]
            fn is_positive(&self) -> bool { self.0.is_sign_positive() }
        }
        impl Signed for [<$name$bsize>] {}

        /* bound */

        impl Bound for [<$name$bsize>] {
            #[inline]
            fn is_lower_bounded(&self) -> bool { true }
            #[inline]
            fn is_upper_bounded(&self) -> bool { true }
            #[inline]
            fn lower_bound(&self) -> Option<Self> { Some([<$name$bsize>]::MAX) }
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
        // NOTE: `Countable` trait implemented separately.

        /* ident */

        // Checks whether the inner primitive values are within a certain error margin.
        #[inline]
        fn [<approx_eq_$bsize>](a: [<$p$bsize>], b: [<$p$bsize>], epsilon: [<$p$bsize>]) -> bool {
            #[cfg(feature = "std")]
            return (a - b).abs() <= epsilon;

            #[cfg(not(feature = "std"))]
            return super::super::[<abs$bsize>](a - b) <= epsilon;
        }

        impl Ident for [<$name$bsize>] {
            #[inline]
            fn can_zero(&self) -> bool { true }
            #[inline]
            fn can_one(&self) -> bool { true }
            #[inline]
            fn can_neg_one(&self) -> bool { true }

            #[inline]
            fn is_zero(&self) -> bool {
                [<approx_eq_$bsize>](self.0, 0.0, [<$p$bsize>]::EPSILON)
            }
            #[inline]
            fn is_one(&self) -> bool {
                [<approx_eq_$bsize>](self.0, 1.0, [<$p$bsize>]::EPSILON)
            }
            #[inline]
            fn is_neg_one(&self) -> bool {
                [<approx_eq_$bsize>](self.0, -1.0, [<$p$bsize>]::EPSILON)
            }
        }
        impl ConstZero for [<$name$bsize>] { const ZERO: Self = Self(0.0); }
        impl Zero for [<$name$bsize>] {
            #[inline]
            fn new_zero() -> Self { Self(0.0) }
        }
        impl ConstOne for [<$name$bsize>] { const ONE: Self = Self(1.0); }
        impl One for [<$name$bsize>] {
            #[inline]
            fn new_one() -> Self { Self(1.0) }
        }
        impl ConstNegOne for [<$name$bsize>] { const NEG_ONE: Self = Self(-1.0); }
        impl NegOne for [<$name$bsize>] {
            #[inline]
            fn new_neg_one() -> Self { Self(-1.0) }
        }

        /* number */

        impl Number for [<$name$bsize>] {
            type Parts = [<$p$bsize>];

            #[doc = "Returns a new `" [<$name$bsize>] " from the constituent parts`."]
            ///
            /// # Errors
            /// This function can't fail.
            #[inline]
            fn from_parts(value: Self::Parts) -> NumeraResult<Self> { Ok(Self(value)) }

            #[doc = "Returns a new `" [<$name$bsize>] "` from the constituent parts`."]
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

define_float_sized![multi Float, f,
    "floating-point number", ", from the set $\\R$.",
    // "",
    "", MIN, MAX,
    // ("An", 8, larger: true, 16, smaller: false, 8),
    // ("A", 16, larger: true, 32, smaller: false, 16),
    // ("A", 32, larger: true, 64, smaller: true, 16),
    // ("A", 64, larger: true, 128, smaller: true, 32),
    // ("A", 128, larger: false, 128, smaller: true, 64)

    ("A", 32, larger: true, 64, smaller: false, 32),
    ("A", 64, larger: false, 64, smaller: true, 32)
];
// TODO: f16, f128

/* specific separate implementations */

impl Countable for Float32 {
    // implementation based on:
    // https://doc.rust-lang.org/std/primitive.f32.html#method.next_up
    #[inline]
    fn next(&self) -> NumeraResult<Self> {
        const TINY_BITS: u32 = 0x1; // Smallest positive f32.
        const CLEAR_SIGN_MASK: u32 = 0x7fff_ffff;

        let bits = self.0.to_bits();
        if self.0.is_nan() || bits == f32::INFINITY.to_bits() {
            return Err(RealError::NaN.into());
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
            return Err(RealError::NaN.into());
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
            return Err(RealError::NaN.into());
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
            return Err(RealError::NaN.into());
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
