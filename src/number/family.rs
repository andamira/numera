// numera::number::family
//
//!
//

use super::{
    integer::AnyIntegers, // rational::AnyRationals, real::AnyReals, complex::AnyComplex,
    traits::{self, Number},
};
use crate::error::NumeraResult as Result;

/// The family of all possible *non-custom* kinds of numbers.
///
/// This is an alias of [`AnyNumbers`] which allows to concisely use variants
/// other than `Any`, without having to specify a type.
pub type Numbers = AnyNumbers<()>;

/// Defines the family of `Numbers` and implements `Number` on it.
macro_rules! define_numbers {
    // applies a method to each variant
    (match_variants:
        $self:ident,
        $method:ident,
        no_std: $($v:ident),+
    ) => {
        match $self {
            $( AnyNumbers::$v(n) => n.$method(), )+
            AnyNumbers::Any(n) => n.$method(),
        }
    };

    // applies a method to each variant and re-wraps it
    (match_variants_rewrap:
        $self:ident,
        $method:ident,
        no_std: $($v:ident),+
    ) => {
        match $self {
            $( AnyNumbers::$v(n) => n.$method().map(|n| AnyNumbers::$v(n)), )+
            AnyNumbers::Any(n) => n.$method().map(|n| AnyNumbers::Any(n)),
        }
    };

    // $v: variant name
    // $t: variant data type
    (build_variants:
        no_std: $($v:ident, $t:ident),+
    ) => {
        /// The family of all possible kinds of numbers.
        ///
        /// # Notes
        /// Note that it wont have several specific traits implemented, like for
        /// example [`Zero`][traits::Zero] or [`NonZero`][traits::NonZero],
        /// since they are mutually exclusive, and don't apply to all cases.
        ///
        /// The [`Numbers`] alias is more convenient to use unless you need to
        /// refer to custom numbers via the [`Any`][AnyNumbers::Any] variant.
        #[derive(Clone, Debug, PartialEq)]
        #[non_exhaustive]
        #[allow(clippy::derive_partial_eq_without_eq)]
        pub enum AnyNumbers<N: Number> {
            $( $v($t),)+

            /// Any kind of number.
            Any(N)
        }

        /* impl Number */

        /// This implementation is no-op.
        impl<N: Number> traits::Number for AnyNumbers<N> {
            type Inner = Self;
            // fn new(value: Numbers<N>) -> Result<Self> { Ok(value) }
            // unsafe fn new_unchecked(value: Numbers<N>) -> Self { value }
            fn new(value: AnyNumbers<N>) -> Result<Self> { Ok(value) }
            unsafe fn new_unchecked(value: AnyNumbers<N>) -> Self { value }
        }

        /// This implementation defers to the actual number variant.
        impl<N: Number> traits::Bound for AnyNumbers<N> {
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
        impl<N: Number> traits::Count for AnyNumbers<N> {
            fn is_countable(&self) -> bool {
                define_numbers! { match_variants: self, is_countable, no_std: $($v),+ }
            }
        }

        /// This implementation defers to the actual number variant.
        impl<N: Number> traits::Sign for AnyNumbers<N> {
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
        impl<N: Number> traits::Ident for AnyNumbers<N> {
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
        impl<N: Number> From<$t> for AnyNumbers<N> {
            fn from(n: $t) -> AnyNumbers<N> { AnyNumbers::$v(n) }
        }
        )+

        $(
        impl<N: Number> TryFrom<AnyNumbers<N>> for $t {
            type Error = crate::error::NumeraError;
            fn try_from(n: AnyNumbers<N>) -> core::result::Result<$t, Self::Error> {
                match n {
                    AnyNumbers::$v(n) => Ok(n),
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
    Integer, AnyIntegers
    // Real, AnyReals,
    // Rational, AnyRationals,
    // Complex, AnyComplex,
];
