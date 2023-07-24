// numera::number::family
//
//!
//

use super::{
    integer::AnyInteger,
    rational::AnyRational, // real::AnyReal, complex::AnyComplex,
    traits::{self, Numbers},
    NoNumber,
};

/// The family of *non-custom* numbers.
///
/// This is an alias of [`AnyNumber`] which allows to concisely use variants
/// other than `Any`, without having to specify a type.
pub type Number = AnyNumber<NoNumber>;

/// Abbreviation of [`AnyNumber`] family.
pub type AnyN<N> = AnyNumber<N>;

/// Defines the `AnyNumber` family and implements `Numbers` on it.
macro_rules! define_numbers {
    // applies a method to each variant
    (match_variants:
        $self:ident,
        $method:ident,
        no_std: $($v:ident),+
    ) => {
        match $self {
            $( AnyNumber::$v(n) => n.$method(), )+
            AnyNumber::Any(n) => n.$method(),
        }
    };

    // applies a method to each variant and re-wraps it
    (match_variants_rewrap:
        $self:ident,
        $method:ident,
        no_std: $($v:ident),+
    ) => {
        match $self {
            $( AnyNumber::$v(n) => n.$method().map(|n| AnyNumber::$v(n)), )+
            AnyNumber::Any(n) => n.$method().map(|n| AnyNumber::Any(n)),
        }
    };

    // $v: variant name
    // $t: variant data type
    (build_variants:
        no_std: $($v:ident, $t:ident),+
    ) => {
        /// The family of [any kind of number][super], also known as [`AnyN`].
        ///
        /// # Notes
        /// Note that it wont have several specific traits implemented, like for
        /// example [`Zero`][traits::Zero] or [`NonZero`][traits::NonZero],
        /// since they are mutually exclusive, and don't apply to all cases.
        ///
        /// The [`Number`] alias is more convenient to use unless you need to
        /// refer to custom numbers via the [`Any`][AnyNumber::Any] variant.
        #[derive(Clone, Debug, PartialEq)]
        #[non_exhaustive]
        #[allow(clippy::derive_partial_eq_without_eq)]
        pub enum AnyNumber<N: Numbers> {
            $( $v($t),)+

            /// Any kind of number.
            Any(N)
        }

        /* impl Numbers */

        /// This implementation is no-op.
        impl<N: Numbers> Numbers for AnyNumber<N> {
            type InnerRepr = Self;
            type InnermostRepr = Self;

            /// Returns `value` unchanged.
            #[inline]
            fn from_inner_repr(value: Self) -> $crate::all::NumeraResult<Self> { Ok(value) }

            /// Returns `value` unchanged.
            #[inline]
            #[cfg(not(feature = "safe"))]
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "not(safe)")))]
            unsafe fn from_inner_repr_unchecked(value: AnyNumber<N>) -> Self { value }

            /// Returns `value` unchanged.
            #[inline]
            fn from_innermost_repr(value: Self) -> $crate::all::NumeraResult<Self> { Ok(value) }

            /// Returns `value` unchanged.
            ///
            /// # Safety
            /// This is safe
            #[inline]
            #[cfg(not(feature = "safe"))]
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "not(safe)")))]
            unsafe fn from_innermost_repr_unchecked(value: Self) -> Self { value }

            /// Returns `self`.
            #[inline]
            fn into_inner_repr(self) -> Self::InnerRepr { self }
            /// Returns `self`.
            #[inline]
            fn into_innermost_repr(self) -> Self::InnermostRepr { self }
        }

        /// This implementation defers to the actual number variant.
        impl<N: Numbers> traits::Bound for AnyNumber<N> {
            fn is_lower_bounded(&self) -> bool {
                define_numbers! { match_variants: self, is_lower_bounded, no_std: $($v),+ }
            }
            fn is_upper_bounded(&self) -> bool {
                define_numbers! { match_variants: self, is_lower_bounded, no_std: $($v),+ }
            }
            fn lower_bound(&self) -> Option<Self> {
                define_numbers! { match_variants_rewrap: self, lower_bound, no_std: $($v),+ }
            }
            fn upper_bound(&self) -> Option<Self> {
                define_numbers! { match_variants_rewrap: self, upper_bound, no_std: $($v),+ }
            }
        }

        /// This implementation defers to the actual number variant.
        impl<N: Numbers> traits::Count for AnyNumber<N> {
            fn is_countable(&self) -> bool {
                define_numbers! { match_variants: self, is_countable, no_std: $($v),+ }
            }
        }

        /// This implementation defers to the actual number variant.
        impl<N: Numbers> traits::Sign for AnyNumber<N> {
            fn can_positive(&self) -> bool {
                define_numbers! { match_variants: self, can_positive, no_std: $($v),+ }
            }
            fn can_negative(&self) -> bool {
                define_numbers! { match_variants: self, can_negative, no_std: $($v),+ }
            }
            fn is_positive(&self) -> bool {
                define_numbers! { match_variants: self, is_positive, no_std: $($v),+ }
            }
            fn is_negative(&self) -> bool {
                define_numbers! { match_variants: self, is_negative, no_std: $($v),+ }
            }
        }

        /// This implementation defers to the actual number variant.
        impl<N: Numbers> traits::Ident for AnyNumber<N> {
            fn can_zero(&self) -> bool {
                define_numbers! { match_variants: self, can_zero, no_std: $($v),+ }
            }
            fn can_one(&self) -> bool {
                define_numbers! { match_variants: self, can_one, no_std: $($v),+ }
            }
            fn can_neg_one(&self) -> bool {
                define_numbers! { match_variants: self, can_neg_one, no_std: $($v),+ }
            }
            fn is_zero(&self) -> bool {
                define_numbers! { match_variants: self, is_zero, no_std: $($v),+ }
            }
            fn is_one(&self) -> bool {
                define_numbers! { match_variants: self, is_one, no_std: $($v),+ }
            }
            fn is_neg_one(&self) -> bool {
                define_numbers! { match_variants: self, is_neg_one, no_std: $($v),+ }
            }
        }

        /* impl From & TryFrom */

        $(
        impl<N: Numbers> From<$t> for AnyNumber<N> {
            #[inline]
            fn from(n: $t) -> AnyNumber<N> { AnyNumber::$v(n) }
        }
        )+

        $(
        impl<N: Numbers> TryFrom<AnyNumber<N>> for $t {
            type Error = crate::error::NumeraError;
            fn try_from(n: AnyNumber<N>) -> core::result::Result<$t, Self::Error> {
                match n {
                    AnyNumber::$v(n) => Ok(n),
                    _ => Err(Self::Error::Conversion)
                }
            }
        }
        )+
    };
}

#[rustfmt::skip]
define_numbers![build_variants:
    no_std:
    Integer, AnyInteger,
    Rational, AnyRational
    // Real, AnyReals,
    // Complex, AnyComplex,
];
