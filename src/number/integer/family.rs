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
    Integers, NegativeIntegers, NonNegativeIntegers, NonPositiveIntegers, NonZeroIntegers,
    PositiveIntegers, Primes,
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
        $fname:ident,
        $self:ident,
        $method:ident,
        no_std: $($vtype:ident),+ ;
        depending: $($vtype_dep:ident, $dep_name:literal)+
    ) => {
        match $self {
            $( $fname::$vtype(z) => z.$method(), )+
            $( #[cfg(feature = $dep_name)]
                $fname::$vtype_dep(z) => z.$method(), )*
        }
    };

    // applies a rewrap method to each variant
    (match_variants_0_rewrap:
        $fname:ident,
        $self:ident,
        $method:ident,
        no_std: $($vtype:ident),+ ;
        depending: $($vtype_dep:ident, $dep_name:literal)+
    ) => {
        match $self {
            $( $fname::$vtype(z) => z.$method().map(|z| $fname::$vtype(z)), )+
            $( #[cfg(feature = $dep_name)]
                $fname::$vtype_dep(z) => z.$method().map(|z| $fname::$vtype_dep(z)), )*
        }
    };

    //
    (build_variants:
        $fname:ident,
        $doc:literal,
        no_std: $($vtype:ident),+ ;
        depending: $($vtype_dep:ident, $dep_name:literal)+
    ) => {
        #[doc = $doc]
        #[derive(Clone, Debug, PartialEq, Eq)]
        #[non_exhaustive]
        pub enum $fname {
            $( $vtype($vtype),)+

            $( // feature-gated variants
                #[cfg(feature = $dep_name)]
                $vtype_dep($vtype_dep),
                // MAYBE multiple features:
                // #[cfg(all(feature = $dep1_name, feature = $dep2_name))]
                // MAYBE Box:
                // $vtype_dep(Box<$vtype_dep>), // reduces size from 32 to 24 Bytes
            )*
        }

        /// This implementation is no-op.
        impl crate::all::Number for $fname {
            type Inner = Self;

            #[inline]
            fn new(value: $fname) -> crate::all::NumeraResult<Self> { Ok(value) }

            #[inline]
            #[cfg(not(feature = "safe"))]
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "non-safe")))]
            unsafe fn new_unchecked(value: $fname) -> Self { value }
        }

        /// This implementation defers to the actual integer variant.
        impl crate::all::Bound for $fname {
            fn is_lower_bounded(&self) -> bool {
                define_integers_family! { match_variants_0:
                    $fname, self, is_lower_bounded,
                    no_std: $($vtype),+ ; depending: $($vtype_dep, $dep_name)+
                }
            }
            fn is_upper_bounded(&self) -> bool {
                define_integers_family! { match_variants_0:
                    $fname, self, is_lower_bounded,
                    no_std: $($vtype),+ ; depending: $($vtype_dep, $dep_name)+
                }
            }
            fn lower_bound(&self) -> Option<Self> {
                define_integers_family! { match_variants_0_rewrap:
                    $fname, self, lower_bound,
                    no_std: $($vtype),+ ; depending: $($vtype_dep, $dep_name)+
                }
            }
            fn upper_bound(&self) -> Option<Self> {
                define_integers_family! { match_variants_0_rewrap:
                    $fname, self, upper_bound,
                    no_std: $($vtype),+ ; depending: $($vtype_dep, $dep_name)+
                }
            }
        }
        impl crate::all::Count for $fname {
            /// All integers are countable.
            #[inline]
            fn is_countable(&self) -> bool { true }
        }
        /// This implementation defers to the actual integer variant.
        impl crate::all::Countable for $fname {
            fn next(&self) -> crate::all::NumeraResult<Self> {
                define_integers_family! { match_variants_0_rewrap:
                    $fname, self, next,
                    no_std: $($vtype),+ ; depending: $($vtype_dep, $dep_name)+
                }
            }
            fn previous(&self) -> crate::all::NumeraResult<Self> {
                define_integers_family! { match_variants_0_rewrap:
                    $fname, self, previous,
                    no_std: $($vtype),+ ; depending: $($vtype_dep, $dep_name)+
                }
            }
        }
        /// This implementation defers to the actual integer variant.
        impl crate::all::Sign for $fname {
            fn can_positive(&self) -> bool {
                define_integers_family! { match_variants_0:
                    $fname, self, can_positive,
                    no_std: $($vtype),+ ; depending: $($vtype_dep, $dep_name)+
                }
            }
            fn can_negative(&self) -> bool {
                define_integers_family! { match_variants_0:
                    $fname, self, can_negative,
                    no_std: $($vtype),+ ; depending: $($vtype_dep, $dep_name)+
                }
            }
            fn is_positive(&self) -> bool {
                define_integers_family! { match_variants_0:
                    $fname, self, is_positive,
                    no_std: $($vtype),+ ; depending: $($vtype_dep, $dep_name)+
                }
            }
            fn is_negative(&self) -> bool {
                define_integers_family! { match_variants_0:
                    $fname, self, is_negative,
                    no_std: $($vtype),+ ; depending: $($vtype_dep, $dep_name)+
                }
            }
        }
        /// This implementation defers to the actual integer variant.
        impl crate::all::Ident for $fname {
            fn can_zero(&self) -> bool {
                define_integers_family! { match_variants_0:
                    $fname, self, can_zero,
                    no_std: $($vtype),+ ; depending: $($vtype_dep, $dep_name)+
                }
            }
            fn can_one(&self) -> bool {
                define_integers_family! { match_variants_0:
                    $fname, self, can_one,
                    no_std: $($vtype),+ ; depending: $($vtype_dep, $dep_name)+
                }
            }
            fn can_neg_one(&self) -> bool {
                define_integers_family! { match_variants_0:
                    $fname, self, can_neg_one,
                    no_std: $($vtype),+ ; depending: $($vtype_dep, $dep_name)+
                }
            }
            fn is_zero(&self) -> bool {
                define_integers_family! { match_variants_0:
                    $fname, self, is_zero,
                    no_std: $($vtype),+ ; depending: $($vtype_dep, $dep_name)+
                }
            }
            fn is_one(&self) -> bool {
                define_integers_family! { match_variants_0:
                    $fname, self, is_one,
                    no_std: $($vtype),+ ; depending: $($vtype_dep, $dep_name)+
                }
            }
            fn is_neg_one(&self) -> bool {
                define_integers_family! { match_variants_0:
                    $fname, self, is_neg_one,
                    no_std: $($vtype),+ ; depending: $($vtype_dep, $dep_name)+
                }
            }
        }

        /* impl From & TryFrom */

        $(
        impl From<$vtype> for $fname {
            #[inline]
            fn from(z: $vtype) -> $fname {
                $fname::$vtype(z)
            }
        }
        )+
        $(
        #[cfg(feature = $dep_name)]
        impl From<$vtype_dep> for $fname {
            #[inline]
            fn from(z: $vtype_dep) -> $fname {
                $fname::$vtype_dep(z)
            }
        }
        )*

        $(
        impl TryFrom<$fname> for $vtype {
            type Error = crate::error::NumeraError;
            fn try_from(z: $fname) -> core::result::Result<$vtype, Self::Error> {
                match z {
                    $fname::$vtype(z) => Ok(z),
                    _ => Err(Self::Error::Conversion)
                }
            }
        }
        )+

        $( #[cfg(feature = $dep_name)]
        impl TryFrom<$fname> for $vtype_dep {
            type Error = crate::error::NumeraError;
            fn try_from(z: $fname) -> core::result::Result<$vtype_dep, Self::Error> {
                match z {
                    $fname::$vtype_dep(z) => Ok(z),
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
        $fname:ident,
        $self:ident,
        $method:ident,
        no_std: $($vname:ident),+
        // ; depending: $($vname_dep:ident, $dep_name:literal)+
    ) => {
        match $self {
            $( $fname::$vname(z) => z.$method(), )+
            // $( #[cfg(feature = $dep_name)]
            //     $fname::$vname_dep(z) => z.$method(), )*
        }
    };

    // applies a rewrap method to each variant
    (match_variants_0_rewrap:
        $fname:ident,
        $self:ident,
        $method:ident,
        no_std: $($vname:ident),+
        // ; depending: $($vname_dep:ident, $dep_name:literal)+
    ) => {
        match $self {
            $( $fname::$vname(z) => z.$method().map(|z| $fname::$vname(z)), )+
            // $( #[cfg(feature = $dep_name)]
            //     $fname::$vname_dep(z) => z.$method().map(|z| $fname::$vname_dep(z)), )*
        }
    };

    // # Args
    // - `$fname`: enum name
    // - `$doc`:         doc_comment
    // - `$vtype`: variant inner type
    // - `$vname`: variant name
    (build_variants:
        $fname:ident,
        $doc:literal,
        no_std: $($vtype:ident, $vname:ident),+
        // ; depending: $($vtype_dep:ident, $dep_name:literal)+
    ) => {
        #[doc = $doc]
        #[derive(Clone, Debug, PartialEq, Eq)]
        #[non_exhaustive]
        pub enum $fname {
            $( $vname($vtype),)+
        }

        /// This implementation is no-op.
        impl crate::all::Number for $fname {
            type Inner = Self;

            #[inline]
            fn new(value: $fname) -> crate::all::NumeraResult<Self> { Ok(value) }

            #[inline]
            #[cfg(not(feature = "safe"))]
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "non-safe")))]
            unsafe fn new_unchecked(value: $fname) -> Self { value }
        }

        /// This implementation defers to the actual integer variant.
        impl crate::all::Bound for $fname {
            fn is_lower_bounded(&self) -> bool {
                define_any_integers_family! { match_variants_0:
                    $fname, self, is_lower_bounded,
                    no_std: $($vname),+ // ; depending: $($vtype_dep, $dep_name)+
                }
            }
            fn is_upper_bounded(&self) -> bool {
                define_any_integers_family! { match_variants_0:
                    $fname, self, is_lower_bounded,
                    no_std: $($vname),+ // ; depending: $($vtype_dep, $dep_name)+
                }
            }
            fn lower_bound(&self) -> Option<Self> {
                define_any_integers_family! { match_variants_0_rewrap:
                    $fname, self, lower_bound,
                    no_std: $($vname),+ // ; depending: $($vtype_dep, $dep_name)+
                }
            }
            fn upper_bound(&self) -> Option<Self> {
                define_any_integers_family! { match_variants_0_rewrap:
                    $fname, self, upper_bound,
                    no_std: $($vname),+ // ; depending: $($vtype_dep, $dep_name)+
                }
            }
        }
        impl crate::all::Count for $fname {
            /// All integers are countable.
            #[inline]
            fn is_countable(&self) -> bool { true }
        }
        /// This implementation defers to the actual integer variant.
        impl crate::all::Countable for $fname {
            fn next(&self) -> crate::all::NumeraResult<Self> {
                define_any_integers_family! { match_variants_0_rewrap:
                    $fname, self, next,
                    no_std: $($vname),+ // ; depending: $($vtype_dep, $dep_name)+
                }
            }
            fn previous(&self) -> crate::all::NumeraResult<Self> {
                define_any_integers_family! { match_variants_0_rewrap:
                    $fname, self, previous,
                    no_std: $($vname),+ // ; depending: $($vtype_dep, $dep_name)+
                }
            }
        }
        /// This implementation defers to the actual integer variant.
        impl crate::all::Sign for $fname {
            fn can_positive(&self) -> bool {
                define_any_integers_family! { match_variants_0:
                    $fname, self, can_positive,
                    no_std: $($vname),+ // ; depending: $($vtype_dep, $dep_name)+
                }
            }
            fn can_negative(&self) -> bool {
                define_any_integers_family! { match_variants_0:
                    $fname, self, can_negative,
                    no_std: $($vname),+ // ; depending: $($vtype_dep, $dep_name)+
                }
            }
            fn is_positive(&self) -> bool {
                define_any_integers_family! { match_variants_0:
                    $fname, self, is_positive,
                    no_std: $($vname),+ // ; depending: $($vtype_dep, $dep_name)+
                }
            }
            fn is_negative(&self) -> bool {
                define_any_integers_family! { match_variants_0:
                    $fname, self, is_negative,
                    no_std: $($vname),+ // ; depending: $($vtype_dep, $dep_name)+
                }
            }
        }
        /// This implementation defers to the actual integer variant.
        impl crate::all::Ident for $fname {
            fn can_zero(&self) -> bool {
                define_any_integers_family! { match_variants_0:
                    $fname, self, can_zero,
                    no_std: $($vname),+ // ; depending: $($vtype_dep, $dep_name)+
                }
            }
            fn can_one(&self) -> bool {
                define_any_integers_family! { match_variants_0:
                    $fname, self, can_one,
                    no_std: $($vname),+ // ; depending: $($vtype_dep, $dep_name)+
                }
            }
            fn can_neg_one(&self) -> bool {
                define_any_integers_family! { match_variants_0:
                    $fname, self, can_neg_one,
                    no_std: $($vname),+ // ; depending: $($vtype_dep, $dep_name)+
                }
            }
            fn is_zero(&self) -> bool {
                define_any_integers_family! { match_variants_0:
                    $fname, self, is_zero,
                    no_std: $($vname),+ // ; depending: $($vtype_dep, $dep_name)+
                }
            }
            fn is_one(&self) -> bool {
                define_any_integers_family! { match_variants_0:
                    $fname, self, is_one,
                    no_std: $($vname),+ // ; depending: $($vtype_dep, $dep_name)+
                }
            }
            fn is_neg_one(&self) -> bool {
                define_any_integers_family! { match_variants_0:
                    $fname, self, is_neg_one,
                    no_std: $($vname),+ // ; depending: $($vtype_dep, $dep_name)+
                }
            }
        }

        /* impl From & TryFrom */

        $(
        impl From<$vtype> for $fname {
            #[inline]
            fn from(z: $vtype) -> $fname {
                $fname::$vname(z)
            }
        }
        )+
        // $(
        // #[cfg(feature = $dep_name)]
        // impl From<$vtype_dep> for $fname {
        //     fn from(z: $vtype_dep) -> $fname {
        //         $fname::$vname_dep(z)
        //     }
        // }
        // )*

        $(
        impl TryFrom<$fname> for $vtype {
            type Error = crate::error::NumeraError;
            fn try_from(z: $fname) -> core::result::Result<$vtype, Self::Error> {
                match z {
                    $fname::$vname(z) => Ok(z),
                    _ => Err(Self::Error::Conversion)
                }
            }
        }
        )+

        // $( #[cfg(feature = $dep_name)]
        // impl TryFrom<$fname> for $vtype_dep {
        //     type Error = crate::error::NumeraError;
        //     fn try_from(z: $fname) -> core::result::Result<$vtype_dep, Self::Error> {
        //         match z {
        //             $fname::$vname_dep(z) => Ok(z),
        //             _ => Err(Self::Error::Conversion)
        //         }
        //     }
        // }
        // )+
    };
}

#[rustfmt::skip]
define_any_integers_family![
    build_variants:
        AnyIntegers,
        "The family of any kind of integers.",
    no_std:
        Integers, Integers,
        NonZeroIntegers, NonZero,
        PositiveIntegers, Positive,
        NonNegativeIntegers, NonNegative,
        NegativeIntegers, Negative,
        NonPositiveIntegers, NonPositive,
        Primes, Primes
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
