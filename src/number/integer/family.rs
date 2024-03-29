// numera::integer::family
//
//!
//
// TOC
// - define_integer_family!
// - define_any_integer_family!

use crate::number::integer::{
    Integers, NegativeIntegers, NonNegativeIntegers, NonPositiveIntegers, NonZeroIntegers,
    PositiveIntegers, Primes,
};

/// Defines a subfamily of integers and implements `Number` on it.
//
// For now it doesn't implement `Integers`.
macro_rules! define_integer_family {
    // applies a method to each variant (0 args)
    (match_variants_0:
        $tname:ident,
        $self:ident,
        $method:ident,
        common: $($vtype:ident+$vbit:literal),+ ;
        depending: $($dep_vname:ident, $dep_vtype:ident, $dependency:literal)+
    ) => { devela::paste! {
        match $self {
            $( $tname::[<_$vbit>](z) => z.$method(), )+
            $( #[cfg(feature = $dependency)]
                $tname::$dep_vname(z) => z.$method(), )*
        }
    }};

    // applies a rewrap method to each variant
    (match_variants_0_rewrap:
        $tname:ident,
        $self:ident,
        $method:ident,
        common: $($vtype:ident+$vbit:literal),+ ;
        depending: $($dep_vname:ident, $dep_vtype:ident, $dependency:literal)+
    ) => { devela::paste! {
        match $self {
            $( $tname::[<_$vbit>](z) => z.$method().map(|z| $tname::[<_$vbit>](z)), )+
            $( #[cfg(feature = $dependency)]
                $tname::$dep_vname(z) => z.$method().map(|z| $tname::$dep_vname(z)), )*
        }
    }};

    // Main entrypoint
    //
    // # Args
    // `$tname`: the enum name
    // common:
    //   `$vtype`: the base name of the variant type
    //   $(
    //   `$b`: the variant bit-size
    //   )+
    // depending:
    //   $(
    //     `$dep_vname`: the dependent variant name.
    //     `$dep_vtype`: the dependent variant type.
    //     `$dependency`: the dependency that enables the variant.
    //   )*
    //
    (build_variants:
        $tname:ident,
        $doc:literal,
        common: $($vtype:ident+$vbit:literal),+ ;
        depending: $($dep_vname:ident, $dep_vtype:ident, $dependency:literal)+
    ) => { devela::paste! {
        #[doc = $doc]
        #[derive(Clone, Debug, PartialEq, Eq)]
        #[non_exhaustive]
        pub enum $tname {
            $( [<_$vbit>]([<$vtype$vbit>]), )+

            $( // feature-gated variants
                #[cfg(feature = $dependency)]
                $dep_vname($dep_vtype),
                // MAYBE multiple features:
                // #[cfg(all(feature = $dep1_name, feature = $dep2_name))]
                // MAYBE Box:
                // $dep_vname(Box<$dep_vtype>), // reduces size from 32 to 24 Bytes
            )*
        }

        /// This implementation is no-op.
        impl $crate::all::Number for $tname {
            type InnerRepr = Self;
            type InnermostRepr = Self;

            /// Returns `value` unchanged.
            ///
            /// # Errors
            /// This function can't fail.
            #[inline]
            #[allow(clippy::missing_errors_doc)]
            fn from_inner_repr(value: $tname) -> $crate::all::NumeraResult<Self> { Ok(value) }

            /// Returns `value` unchanged.
            ///
            /// # Safety
            /// This is safe
            #[inline]
            #[cfg(not(feature = "safe"))]
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "not(safe)")))]
            #[allow(clippy::missing_errors_doc)]
            unsafe fn from_inner_repr_unchecked(value: $tname) -> Self { value }

            /// Returns `value` unchanged.
            ///
            /// # Errors
            /// This function can't fail.
            #[inline]
            fn from_innermost_repr(value: $tname) -> $crate::all::NumeraResult<Self> { Ok(value) }

            /// Returns `value` unchanged.
            ///
            /// # Safety
            /// This is safe
            #[inline]
            #[cfg(not(feature = "safe"))]
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "not(safe)")))]
            unsafe fn from_innermost_repr_unchecked(value: $tname) -> Self { value }

            /// Returns `self`.
            #[inline]
            fn into_inner_repr(self) -> Self::InnerRepr { self }
            /// Returns `self`.
            #[inline]
            fn into_innermost_repr(self) -> Self::InnermostRepr { self }
        }

        /// This implementation defers to the actual integer variant.
        impl $crate::all::Bound for $tname {
            fn is_lower_bounded(&self) -> bool {
                define_integer_family! { match_variants_0:
                    $tname, self, is_lower_bounded,
                    common: $($vtype+$vbit),+ ; depending: $($dep_vname, $dep_vtype, $dependency)+
                }
            }
            fn is_upper_bounded(&self) -> bool {
                define_integer_family! { match_variants_0:
                    $tname, self, is_lower_bounded,
                    common: $($vtype+$vbit),+ ; depending: $($dep_vname, $dep_vtype, $dependency)+
                }
            }
            fn lower_bound(&self) -> Option<Self> {
                define_integer_family! { match_variants_0_rewrap:
                    $tname, self, lower_bound,
                    common: $($vtype+$vbit),+ ; depending: $($dep_vname, $dep_vtype, $dependency)+
                }
            }
            fn upper_bound(&self) -> Option<Self> {
                define_integer_family! { match_variants_0_rewrap:
                    $tname, self, upper_bound,
                    common: $($vtype+$vbit),+ ; depending: $($dep_vname, $dep_vtype, $dependency)+
                }
            }
        }
        impl $crate::all::Count for $tname {
            /// All integers are countable.
            #[inline]
            fn is_countable(&self) -> bool { true }
        }
        /// This implementation defers to the actual integer variant.
        impl $crate::all::Countable for $tname {
            #[allow(clippy::missing_errors_doc)]
            fn next(&self) -> $crate::all::NumeraResult<Self> {
                define_integer_family! { match_variants_0_rewrap:
                    $tname, self, next,
                    common: $($vtype+$vbit),+ ; depending: $($dep_vname, $dep_vtype, $dependency)+
                }
            }
            #[allow(clippy::missing_errors_doc)]
            fn previous(&self) -> $crate::all::NumeraResult<Self> {
                define_integer_family! { match_variants_0_rewrap:
                    $tname, self, previous,
                    common: $($vtype+$vbit),+ ; depending: $($dep_vname, $dep_vtype, $dependency)+
                }
            }
        }
        /// This implementation defers to the actual integer variant.
        impl $crate::all::Sign for $tname {
            fn can_positive(&self) -> bool {
                define_integer_family! { match_variants_0:
                    $tname, self, can_positive,
                    common: $($vtype+$vbit),+ ; depending: $($dep_vname, $dep_vtype, $dependency)+
                }
            }
            fn can_negative(&self) -> bool {
                define_integer_family! { match_variants_0:
                    $tname, self, can_negative,
                    common: $($vtype+$vbit),+ ; depending: $($dep_vname, $dep_vtype, $dependency)+
                }
            }
            fn is_positive(&self) -> bool {
                define_integer_family! { match_variants_0:
                    $tname, self, is_positive,
                    common: $($vtype+$vbit),+ ; depending: $($dep_vname, $dep_vtype, $dependency)+
                }
            }
            fn is_negative(&self) -> bool {
                define_integer_family! { match_variants_0:
                    $tname, self, is_negative,
                    common: $($vtype+$vbit),+ ; depending: $($dep_vname, $dep_vtype, $dependency)+
                }
            }
        }
        /// This implementation defers to the actual integer variant.
        impl $crate::all::Ident for $tname {
            fn can_zero(&self) -> bool {
                define_integer_family! { match_variants_0:
                    $tname, self, can_zero,
                    common: $($vtype+$vbit),+ ; depending: $($dep_vname, $dep_vtype, $dependency)+
                }
            }
            fn can_one(&self) -> bool {
                define_integer_family! { match_variants_0:
                    $tname, self, can_one,
                    common: $($vtype+$vbit),+ ; depending: $($dep_vname, $dep_vtype, $dependency)+
                }
            }
            fn can_neg_one(&self) -> bool {
                define_integer_family! { match_variants_0:
                    $tname, self, can_neg_one,
                    common: $($vtype+$vbit),+ ; depending: $($dep_vname, $dep_vtype, $dependency)+
                }
            }
            fn is_zero(&self) -> bool {
                define_integer_family! { match_variants_0:
                    $tname, self, is_zero,
                    common: $($vtype+$vbit),+ ; depending: $($dep_vname, $dep_vtype, $dependency)+
                }
            }
            fn is_one(&self) -> bool {
                define_integer_family! { match_variants_0:
                    $tname, self, is_one,
                    common: $($vtype+$vbit),+ ; depending: $($dep_vname, $dep_vtype, $dependency)+
                }
            }
            fn is_neg_one(&self) -> bool {
                define_integer_family! { match_variants_0:
                    $tname, self, is_neg_one,
                    common: $($vtype+$vbit),+ ; depending: $($dep_vname, $dep_vtype, $dependency)+
                }
            }
        }

        /* impl From & TryFrom */

        $(
        impl From<[<$vtype$vbit>]> for $tname {
            #[inline]
            fn from(z: [<$vtype$vbit>]) -> $tname {
                $tname::[<_$vbit>](z)
            }
        }
        )+
        $(
        #[cfg(feature = $dependency)]
        impl From<$dep_vtype> for $tname {
            #[inline]
            fn from(z: $dep_vtype) -> $tname {
                $tname::$dep_vname(z)
            }
        }
        )*

        $(
        impl TryFrom<$tname> for [<$vtype$vbit>] {
            type Error = $crate::error::NumeraErrors;
            fn try_from(z: $tname) -> core::result::Result<[<$vtype$vbit>], Self::Error> {
                match z {
                    $tname::[<_$vbit>](z) => Ok(z),
                    _ => Err(Self::Error::Conversion)
                }
            }
        }
        )+

        $( #[cfg(feature = $dependency)]
        impl TryFrom<$tname> for $dep_vtype {
            type Error = $crate::error::NumeraErrors;
            fn try_from(z: $tname) -> core::result::Result<$dep_vtype, Self::Error> {
                match z {
                    $tname::$dep_vname(z) => Ok(z),
                    _ => Err(Self::Error::Conversion)
                }
            }
        }
        )+
    }};
}
pub(crate) use define_integer_family;

/// Defines the family of all integers and implements `Number` on it.
macro_rules! define_any_integer_family {
    // applies a method to each variant (0 args)
    (match_variants_0:
        $tname:ident,
        $self:ident,
        $method:ident,
        common: $($vname:ident),+
        // ; depending: $($vname_dep:ident, $dependency:literal)+
    ) => {
        match $self {
            $( $tname::$vname(z) => z.$method(), )+
            // $( #[cfg(feature = $dependency)]
            //     $tname::$vname_dep(z) => z.$method(), )*
        }
    };

    // applies a rewrap method to each variant
    (match_variants_0_rewrap:
        $tname:ident,
        $self:ident,
        $method:ident,
        common: $($vname:ident),+
        // ; depending: $($vname_dep:ident, $dependency:literal)+
    ) => {
        match $self {
            $( $tname::$vname(z) => z.$method().map(|z| $tname::$vname(z)), )+
            // $( #[cfg(feature = $dependency)]
            //     $tname::$vname_dep(z) => z.$method().map(|z| $tname::$vname_dep(z)), )*
        }
    };

    // Main entrypoint
    //
    // # Args
    //
    // `$tname`: enum name
    // `$doc`:   doc_comment
    // `$vtype`: variant inner type
    // `$vname`: variant name
    (build_variants:
        $tname:ident,
        $doc:literal,
        // common: $($vtype:ident+$vbit:literal, $vname:ident),+ // TODO
        common: $($vtype:ident, $vname:ident),+
        // ; depending: $($dep_vtype:ident, $dependency:literal)+
    ) => {
        #[doc = $doc]
        #[derive(Clone, Debug, PartialEq, Eq)]
        #[non_exhaustive]
        pub enum $tname {
            $( $vname($vtype),)+
        }

        /// This implementation is no-op.
        impl $crate::all::Number for $tname {
            type InnerRepr = Self;
            type InnermostRepr = Self;

            #[inline]
            fn from_inner_repr(value: $tname) -> $crate::all::NumeraResult<Self> { Ok(value) }
            #[inline]
            #[cfg(not(feature = "safe"))]
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "not(safe)")))]
            unsafe fn from_inner_repr_unchecked(value: $tname) -> Self { value }

            #[inline]
            fn from_innermost_repr(value: $tname) -> $crate::all::NumeraResult<Self> { Ok(value) }
            #[inline]
            #[cfg(not(feature = "safe"))]
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "not(safe)")))]
            unsafe fn from_innermost_repr_unchecked(value: $tname) -> Self { value }

            #[inline]
            fn into_inner_repr(self) -> Self::InnerRepr { self }
            #[inline]
            fn into_innermost_repr(self) -> Self::InnermostRepr { self }
        }

        /// This implementation defers to the actual integer variant.
        impl $crate::all::Bound for $tname {
            fn is_lower_bounded(&self) -> bool {
                define_any_integer_family! { match_variants_0:
                    $tname, self, is_lower_bounded,
                    common: $($vname),+ // ; depending: $($dep_vtype, $dependency)+
                }
            }
            fn is_upper_bounded(&self) -> bool {
                define_any_integer_family! { match_variants_0:
                    $tname, self, is_lower_bounded,
                    common: $($vname),+ // ; depending: $($dep_vtype, $dependency)+
                }
            }
            fn lower_bound(&self) -> Option<Self> {
                define_any_integer_family! { match_variants_0_rewrap:
                    $tname, self, lower_bound,
                    common: $($vname),+ // ; depending: $($dep_vtype, $dependency)+
                }
            }
            fn upper_bound(&self) -> Option<Self> {
                define_any_integer_family! { match_variants_0_rewrap:
                    $tname, self, upper_bound,
                    common: $($vname),+ // ; depending: $($dep_vtype, $dependency)+
                }
            }
        }
        impl $crate::all::Count for $tname {
            /// All integers are countable.
            #[inline]
            fn is_countable(&self) -> bool { true }
        }
        /// This implementation defers to the actual integer variant.
        impl $crate::all::Countable for $tname {
            fn next(&self) -> $crate::all::NumeraResult<Self> {
                define_any_integer_family! { match_variants_0_rewrap:
                    $tname, self, next,
                    common: $($vname),+ // ; depending: $($dep_vtype, $dependency)+
                }
            }
            fn previous(&self) -> $crate::all::NumeraResult<Self> {
                define_any_integer_family! { match_variants_0_rewrap:
                    $tname, self, previous,
                    common: $($vname),+ // ; depending: $($dep_vtype, $dependency)+
                }
            }
        }
        /// This implementation defers to the actual integer variant.
        impl $crate::all::Sign for $tname {
            fn can_positive(&self) -> bool {
                define_any_integer_family! { match_variants_0:
                    $tname, self, can_positive,
                    common: $($vname),+ // ; depending: $($dep_vtype, $dependency)+
                }
            }
            fn can_negative(&self) -> bool {
                define_any_integer_family! { match_variants_0:
                    $tname, self, can_negative,
                    common: $($vname),+ // ; depending: $($dep_vtype, $dependency)+
                }
            }
            fn is_positive(&self) -> bool {
                define_any_integer_family! { match_variants_0:
                    $tname, self, is_positive,
                    common: $($vname),+ // ; depending: $($dep_vtype, $dependency)+
                }
            }
            fn is_negative(&self) -> bool {
                define_any_integer_family! { match_variants_0:
                    $tname, self, is_negative,
                    common: $($vname),+ // ; depending: $($dep_vtype, $dependency)+
                }
            }
        }
        /// This implementation defers to the actual integer variant.
        impl $crate::all::Ident for $tname {
            fn can_zero(&self) -> bool {
                define_any_integer_family! { match_variants_0:
                    $tname, self, can_zero,
                    common: $($vname),+ // ; depending: $($dep_vtype, $dependency)+
                }
            }
            fn can_one(&self) -> bool {
                define_any_integer_family! { match_variants_0:
                    $tname, self, can_one,
                    common: $($vname),+ // ; depending: $($dep_vtype, $dependency)+
                }
            }
            fn can_neg_one(&self) -> bool {
                define_any_integer_family! { match_variants_0:
                    $tname, self, can_neg_one,
                    common: $($vname),+ // ; depending: $($dep_vtype, $dependency)+
                }
            }
            fn is_zero(&self) -> bool {
                define_any_integer_family! { match_variants_0:
                    $tname, self, is_zero,
                    common: $($vname),+ // ; depending: $($dep_vtype, $dependency)+
                }
            }
            fn is_one(&self) -> bool {
                define_any_integer_family! { match_variants_0:
                    $tname, self, is_one,
                    common: $($vname),+ // ; depending: $($dep_vtype, $dependency)+
                }
            }
            fn is_neg_one(&self) -> bool {
                define_any_integer_family! { match_variants_0:
                    $tname, self, is_neg_one,
                    common: $($vname),+ // ; depending: $($dep_vtype, $dependency)+
                }
            }
        }

        /* impl From & TryFrom */

        $(
        impl From<$vtype> for $tname {
            #[inline]
            fn from(z: $vtype) -> $tname {
                $tname::$vname(z)
            }
        }
        )+
        // $(
        // #[cfg(feature = $dependency)]
        // impl From<$dep_vtype> for $tname {
        //     fn from(z: $dep_vtype) -> $tname {
        //         $tname::$vname_dep(z)
        //     }
        // }
        // )*

        $(
        impl TryFrom<$tname> for $vtype {
            type Error = $crate::error::NumeraErrors;
            fn try_from(z: $tname) -> core::result::Result<$vtype, Self::Error> {
                match z {
                    $tname::$vname(z) => Ok(z),
                    _ => Err(Self::Error::Conversion)
                }
            }
        }
        )+

        // $( #[cfg(feature = $dependency)]
        // impl TryFrom<$tname> for $dep_vtype {
        //     type Error = $crate::error::NumeraErrors;
        //     fn try_from(z: $tname) -> core::result::Result<$dep_vtype, Self::Error> {
        //         match z {
        //             $tname::$vname_dep(z) => Ok(z),
        //             _ => Err(Self::Error::Conversion)
        //         }
        //     }
        // }
        // )+
    };
}

#[rustfmt::skip]
define_any_integer_family![
    build_variants:
        AllIntegers,
        "The family of [all kinds of integers][super], also known as [`AllZ`][super::AllZ].",
    common:
        Integers, Integer,
        NonZeroIntegers, NonZero,
        PositiveIntegers, Positive,
        NonNegativeIntegers, NonNegative,
        NegativeIntegers, Negative,
        NonPositiveIntegers, NonPositive,
        Primes, Prime
];

use crate::number::macros::define_abbreviations;
define_abbreviations![family AllZ, AllIntegers];
