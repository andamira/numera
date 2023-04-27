// numera::integer::family
//
//!
//
// TOC
// - macro
//   - define_integers_family
//   - define_any_integers_family
// - tests

use crate::number::integer::{
    Integers, NonNegativeIntegers, NonZeroIntegers, PositiveIntegers, Primes,
};

/// Defines a subfamily of integers and implements `Number` on it.
//
// It doesn't implement `Integer`, brings too much complexity for little gain.
//
// Sizes:
// - 24 bytes minimum (192bits)
// - 32 Bytes with bigint (24 if using Box)
macro_rules! define_integers_family {
    // applies a method to each variant (0 args)
    (match_variants_0:
        $family_name:ident,
        $self:ident,
        $method:ident,
        no_std: $($t:ident),+ ;
        depending: $($td:ident, $dep_name:literal)+
    ) => {
        match $self {
            $( $family_name::$t(z) => z.$method(), )+
            $( #[cfg(feature = $dep_name)]
                $family_name::$td(z) => z.$method(), )*
        }
    };

    // applies a rewrap method to each variant
    (match_variants_0_rewrap:
        $family_name:ident,
        $self:ident,
        $method:ident,
        no_std: $($t:ident),+ ;
        depending: $($td:ident, $dep_name:literal)+
    ) => {
        match $self {
            $( $family_name::$t(z) => z.$method().map(|z| $family_name::$t(z)), )+
            $( #[cfg(feature = $dep_name)]
                $family_name::$td(z) => z.$method().map(|z| $family_name::$td(z)), )*
        }
    };

    //
    (build_variants:
        $family_name:ident,
        $doc:literal,
        no_std: $($t:ident),+ ;
        depending: $($td:ident, $dep_name:literal)+
    ) => {
        #[doc = $doc]
        #[derive(Clone, Debug, PartialEq, Eq)]
        #[non_exhaustive]
        pub enum $family_name {
            $( $t($t),)+

            $( // feature-gated variants
                #[cfg(feature = $dep_name)]
                $td($td),
                // MAYBE multiple features:
                // #[cfg(all(feature = $dep1_name, feature = $dep2_name))]
                // MAYBE Box:
                // $td(Box<$td>), // reduces size from 32 to 24 Bytes
            )*
        }

        /// This implementation is no-op.
        impl crate::all::Number for $family_name {
            type Inner = Self;
            fn new(value: $family_name) -> crate::all::NumeraResult<Self> { Ok(value) }
            unsafe fn new_unchecked(value: $family_name) -> Self { value }
        }

        /// This implementation defers to the actual integer variant.
        impl crate::all::Bound for $family_name {
            fn is_lower_bounded(&self) -> bool {
                define_integers_family! { match_variants_0:
                    $family_name, self, is_lower_bounded,
                    no_std: $($t),+ ; depending: $($td, $dep_name)+
                }
            }
            fn is_upper_bounded(&self) -> bool {
                define_integers_family! { match_variants_0:
                    $family_name, self, is_lower_bounded,
                    no_std: $($t),+ ; depending: $($td, $dep_name)+
                }
            }
            fn lower_bound(&self) -> Option<Self> {
                define_integers_family! { match_variants_0_rewrap:
                    $family_name, self, lower_bound,
                    no_std: $($t),+ ; depending: $($td, $dep_name)+
                }
            }
            fn upper_bound(&self) -> Option<Self> {
                define_integers_family! { match_variants_0_rewrap:
                    $family_name, self, upper_bound,
                    no_std: $($t),+ ; depending: $($td, $dep_name)+
                }
            }
        }
        impl crate::all::Count for $family_name {
            /// All integers are countable.
            fn is_countable(&self) -> bool { true }
        }
        /// This implementation defers to the actual integer variant.
        impl crate::all::Countable for $family_name {
            fn next(&self) -> crate::all::NumeraResult<Self> {
                define_integers_family! { match_variants_0_rewrap:
                    $family_name, self, next,
                    no_std: $($t),+ ; depending: $($td, $dep_name)+
                }
            }
            fn previous(&self) -> crate::all::NumeraResult<Self> {
                define_integers_family! { match_variants_0_rewrap:
                    $family_name, self, previous,
                    no_std: $($t),+ ; depending: $($td, $dep_name)+
                }
            }
        }
        /// This implementation defers to the actual integer variant.
        impl crate::all::Sign for $family_name {
            fn can_positive(&self) -> bool {
                define_integers_family! { match_variants_0:
                    $family_name, self, can_positive,
                    no_std: $($t),+ ; depending: $($td, $dep_name)+
                }
            }
            fn can_negative(&self) -> bool {
                define_integers_family! { match_variants_0:
                    $family_name, self, can_negative,
                    no_std: $($t),+ ; depending: $($td, $dep_name)+
                }
            }
            fn is_positive(&self) -> bool {
                define_integers_family! { match_variants_0:
                    $family_name, self, is_positive,
                    no_std: $($t),+ ; depending: $($td, $dep_name)+
                }
            }
            fn is_negative(&self) -> bool {
                define_integers_family! { match_variants_0:
                    $family_name, self, is_negative,
                    no_std: $($t),+ ; depending: $($td, $dep_name)+
                }
            }
        }
        /// This implementation defers to the actual integer variant.
        impl crate::all::Ident for $family_name {
            fn can_zero(&self) -> bool {
                define_integers_family! { match_variants_0:
                    $family_name, self, can_zero,
                    no_std: $($t),+ ; depending: $($td, $dep_name)+
                }
            }
            fn can_one(&self) -> bool {
                define_integers_family! { match_variants_0:
                    $family_name, self, can_one,
                    no_std: $($t),+ ; depending: $($td, $dep_name)+
                }
            }
            fn can_neg_one(&self) -> bool {
                define_integers_family! { match_variants_0:
                    $family_name, self, can_neg_one,
                    no_std: $($t),+ ; depending: $($td, $dep_name)+
                }
            }
            fn is_zero(&self) -> bool {
                define_integers_family! { match_variants_0:
                    $family_name, self, is_zero,
                    no_std: $($t),+ ; depending: $($td, $dep_name)+
                }
            }
            fn is_one(&self) -> bool {
                define_integers_family! { match_variants_0:
                    $family_name, self, is_one,
                    no_std: $($t),+ ; depending: $($td, $dep_name)+
                }
            }
            fn is_neg_one(&self) -> bool {
                define_integers_family! { match_variants_0:
                    $family_name, self, is_neg_one,
                    no_std: $($t),+ ; depending: $($td, $dep_name)+
                }
            }
        }

        /* impl From & TryFrom */

        $(
        impl From<$t> for $family_name {
            fn from(z: $t) -> $family_name {
                $family_name::$t(z)
            }
        }
        )+
        $(
        #[cfg(feature = $dep_name)]
        impl From<$td> for $family_name {
            fn from(z: $td) -> $family_name {
                $family_name::$td(z)
            }
        }
        )*

        $(
        impl TryFrom<$family_name> for $t {
            type Error = crate::error::NumeraError;
            fn try_from(z: $family_name) -> core::result::Result<$t, Self::Error> {
                match z {
                    $family_name::$t(z) => Ok(z),
                    _ => Err(Self::Error::Conversion)
                }
            }
        }
        )+

        $( #[cfg(feature = $dep_name)]
        impl TryFrom<$family_name> for $td {
            type Error = crate::error::NumeraError;
            fn try_from(z: $family_name) -> core::result::Result<$td, Self::Error> {
                match z {
                    $family_name::$td(z) => Ok(z),
                    _ => Err(Self::Error::Conversion)
                }
            }
        }
        )+
    };
}
pub(crate) use define_integers_family;

/// Defines the family of all integers and implements `Number` on it.
macro_rules! define_any_integers_family {
    // applies a method to each variant (0 args)
    (match_variants_0:
        $family_name:ident,
        $self:ident,
        $method:ident,
        no_std: $($t:ident),+
        // ; depending: $($td:ident, $dep_name:literal)+
    ) => {
        match $self {
            $( $family_name::$t(z) => z.$method(), )+
            // $( #[cfg(feature = $dep_name)]
            //     $family_name::$td(z) => z.$method(), )*
        }
    };

    // applies a rewrap method to each variant
    (match_variants_0_rewrap:
        $family_name:ident,
        $self:ident,
        $method:ident,
        no_std: $($t:ident),+
        // ; depending: $($td:ident, $dep_name:literal)+
    ) => {
        match $self {
            $( $family_name::$t(z) => z.$method().map(|z| $family_name::$t(z)), )+
            // $( #[cfg(feature = $dep_name)]
            //     $family_name::$td(z) => z.$method().map(|z| $family_name::$td(z)), )*
        }
    };

    (build_variants:
        $family_name:ident,
        $doc:literal,
        no_std: $($t:ident),+
        // ; depending: $($td:ident, $dep_name:literal)+
    ) => {
        #[doc = $doc]
        #[derive(Clone, Debug, PartialEq, Eq)]
        #[non_exhaustive]
        pub enum $family_name {
            $( $t($t),)+
        }

        /// This implementation is no-op.
        impl crate::all::Number for $family_name {
            type Inner = Self;
            fn new(value: $family_name) -> crate::all::NumeraResult<Self> { Ok(value) }
            unsafe fn new_unchecked(value: $family_name) -> Self { value }
        }

        /// This implementation defers to the actual integer variant.
        impl crate::all::Bound for $family_name {
            fn is_lower_bounded(&self) -> bool {
                define_any_integers_family! { match_variants_0:
                    $family_name, self, is_lower_bounded,
                    no_std: $($t),+ // ; depending: $($td, $dep_name)+
                }
            }
            fn is_upper_bounded(&self) -> bool {
                define_any_integers_family! { match_variants_0:
                    $family_name, self, is_lower_bounded,
                    no_std: $($t),+ // ; depending: $($td, $dep_name)+
                }
            }
            fn lower_bound(&self) -> Option<Self> {
                define_any_integers_family! { match_variants_0_rewrap:
                    $family_name, self, lower_bound,
                    no_std: $($t),+ // ; depending: $($td, $dep_name)+
                }
            }
            fn upper_bound(&self) -> Option<Self> {
                define_any_integers_family! { match_variants_0_rewrap:
                    $family_name, self, upper_bound,
                    no_std: $($t),+ // ; depending: $($td, $dep_name)+
                }
            }
        }
        impl crate::all::Count for $family_name {
            /// All integers are countable.
            fn is_countable(&self) -> bool { true }
        }
        /// This implementation defers to the actual integer variant.
        impl crate::all::Countable for $family_name {
            fn next(&self) -> crate::all::NumeraResult<Self> {
                define_any_integers_family! { match_variants_0_rewrap:
                    $family_name, self, next,
                    no_std: $($t),+ // ; depending: $($td, $dep_name)+
                }
            }
            fn previous(&self) -> crate::all::NumeraResult<Self> {
                define_any_integers_family! { match_variants_0_rewrap:
                    $family_name, self, previous,
                    no_std: $($t),+ // ; depending: $($td, $dep_name)+
                }
            }
        }
        /// This implementation defers to the actual integer variant.
        impl crate::all::Sign for $family_name {
            fn can_positive(&self) -> bool {
                define_any_integers_family! { match_variants_0:
                    $family_name, self, can_positive,
                    no_std: $($t),+ // ; depending: $($td, $dep_name)+
                }
            }
            fn can_negative(&self) -> bool {
                define_any_integers_family! { match_variants_0:
                    $family_name, self, can_negative,
                    no_std: $($t),+ // ; depending: $($td, $dep_name)+
                }
            }
            fn is_positive(&self) -> bool {
                define_any_integers_family! { match_variants_0:
                    $family_name, self, is_positive,
                    no_std: $($t),+ // ; depending: $($td, $dep_name)+
                }
            }
            fn is_negative(&self) -> bool {
                define_any_integers_family! { match_variants_0:
                    $family_name, self, is_negative,
                    no_std: $($t),+ // ; depending: $($td, $dep_name)+
                }
            }
        }
        /// This implementation defers to the actual integer variant.
        impl crate::all::Ident for $family_name {
            fn can_zero(&self) -> bool {
                define_any_integers_family! { match_variants_0:
                    $family_name, self, can_zero,
                    no_std: $($t),+ // ; depending: $($td, $dep_name)+
                }
            }
            fn can_one(&self) -> bool {
                define_any_integers_family! { match_variants_0:
                    $family_name, self, can_one,
                    no_std: $($t),+ // ; depending: $($td, $dep_name)+
                }
            }
            fn can_neg_one(&self) -> bool {
                define_any_integers_family! { match_variants_0:
                    $family_name, self, can_neg_one,
                    no_std: $($t),+ // ; depending: $($td, $dep_name)+
                }
            }
            fn is_zero(&self) -> bool {
                define_any_integers_family! { match_variants_0:
                    $family_name, self, is_zero,
                    no_std: $($t),+ // ; depending: $($td, $dep_name)+
                }
            }
            fn is_one(&self) -> bool {
                define_any_integers_family! { match_variants_0:
                    $family_name, self, is_one,
                    no_std: $($t),+ // ; depending: $($td, $dep_name)+
                }
            }
            fn is_neg_one(&self) -> bool {
                define_any_integers_family! { match_variants_0:
                    $family_name, self, is_neg_one,
                    no_std: $($t),+ // ; depending: $($td, $dep_name)+
                }
            }
        }

        /* impl From & TryFrom */

        $(
        impl From<$t> for $family_name {
            fn from(z: $t) -> $family_name {
                $family_name::$t(z)
            }
        }
        )+
        // $(
        // #[cfg(feature = $dep_name)]
        // impl From<$td> for $family_name {
        //     fn from(z: $td) -> $family_name {
        //         $family_name::$td(z)
        //     }
        // }
        // )*

        $(
        impl TryFrom<$family_name> for $t {
            type Error = crate::error::NumeraError;
            fn try_from(z: $family_name) -> core::result::Result<$t, Self::Error> {
                match z {
                    $family_name::$t(z) => Ok(z),
                    _ => Err(Self::Error::Conversion)
                }
            }
        }
        )+

        // $( #[cfg(feature = $dep_name)]
        // impl TryFrom<$family_name> for $td {
        //     type Error = crate::error::NumeraError;
        //     fn try_from(z: $family_name) -> core::result::Result<$td, Self::Error> {
        //         match z {
        //             $family_name::$td(z) => Ok(z),
        //             _ => Err(Self::Error::Conversion)
        //         }
        //     }
        // }
        // )+
    };
}

define_any_integers_family![
    build_variants: AnyIntegers,
    "The family of any kind of integers.",
    no_std: Integers,
    NonZeroIntegers,
    PositiveIntegers,
    NonNegativeIntegers,
    // NegativeIntegers, NonPositiveIntegers,
    Primes
];

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use core::mem::size_of;

    // #[cfg(not(any(feature="twofloat", feature="half", feature="ibig", feature="ruint")))]
    #[cfg(not(feature = "std"))]
    #[test]
    fn sizes() {
        // 24 because of the enum discriminant
        assert_eq![24, size_of::<super::AnyIntegers>()];
    }

    #[test]
    #[cfg(feature = "deps_all")]
    fn size_all_features() {
        assert_eq![32, size_of::<super::AnyIntegers>()];
        // assert_eq![24, size_of::<super::AnyIntegers>()]; // MAYBE:Box bigint
    }
}
