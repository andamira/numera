// numera::integer::family
//
//!
//
// NOTES:
// - doesn't implement Integers. brings too much complexity for very little gain

use super::{Integer128, Integer16, Integer32, Integer64, Integer8};
use crate::{error::NumeraResult as Result, number::traits};

#[cfg(feature = "ibig")]
use super::IntegerBig;

// Inner placeholder to ensure the macro call always works
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg(not(feature = "ibig"))]
#[allow(dead_code)]
struct IntegerBig;

/// Defines the family of `Integers` and implements `Number` & `Integer` on it.
//
// Sizes:
// - 24 bytes minimum (192bits)
// - 32 Bytes with bigint (24 if using Box)
macro_rules! define_integers {
    // applies a method to each variant (0 args)
    (match_variants_0:
        $self:ident,
        $method:ident,
        no_std: $($t:ident),+ ;
        depending: $($td:ident, $dep_name:literal)+
    ) => {
        match $self {
            $( Integers::$t(z) => z.$method(), )+
            $( #[cfg(feature = $dep_name)]
                Integers::$td(z) => z.$method(), )*
        }
    };

    // applies a rewrap method to each variant
    (match_variants_0_rewrap:
        $self:ident,
        $method:ident,
        no_std: $($t:ident),+ ;
        depending: $($td:ident, $dep_name:literal)+
    ) => {
        match $self {
            $( Integers::$t(z) => z.$method().map(|z| Integers::$t(z)), )+
            $( #[cfg(feature = $dep_name)]
                Integers::$td(z) => z.$method().map(|z| Integers::$td(z)), )*
        }
    };

    //
    (build_variants:
        no_std: $($t:ident),+ ;
        depending: $($td:ident, $dep_name:literal)+
    ) => {
        /// The family of integers.
        ///
        /// # Notes
        /// Note that it wont have several specific traits implemented, like for
        /// example [`Zero`][traits::Zero] or [`NonZero`][traits::NonZero],
        /// since they are mutually exclusive, and don't apply to all cases.
        ///
        /// [`Countable`][traits::Countable], on the other side, is implemented
        /// because it does apply to all integers.
        #[derive(Clone, Debug, PartialEq, Eq)]
        #[non_exhaustive]
        pub enum Integers {
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
        impl traits::Number for Integers {
            type Inner = Self;
            fn new(value: Integers) -> Result<Self> { Ok(value) }
            unsafe fn new_unchecked(value: Integers) -> Self { value }
            fn into_inner(self) -> Self::Inner { self }
            fn ref_inner(&self) -> &Self::Inner { &self }
        }

        /// This implementation defers to the actual integer variant.
        impl traits::Bound for Integers {
            fn is_lower_bounded(&self) -> bool {
                define_integers! { match_variants_0: self, is_lower_bounded,
                    no_std: $($t),+ ; depending: $($td, $dep_name)+
                }
            }
            fn is_upper_bounded(&self) -> bool {
                define_integers! { match_variants_0: self, is_lower_bounded,
                    no_std: $($t),+ ; depending: $($td, $dep_name)+
                }
            }
            fn lower_bound(&self) -> Option<Self> {
                define_integers! { match_variants_0_rewrap: self, lower_bound,
                    no_std: $($t),+ ; depending: $($td, $dep_name)+
                }
            }
            fn upper_bound(&self) -> Option<Self> {
                define_integers! { match_variants_0_rewrap: self, upper_bound,
                    no_std: $($t),+ ; depending: $($td, $dep_name)+
                }
            }
        }
        impl traits::Count for Integers {
            /// All integers are countable.
            fn is_countable(&self) -> bool { true }
        }
        /// This implementation defers to the actual integer variant.
        impl traits::Countable for Integers {
            fn next(&self) -> Result<Self> {
                define_integers! { match_variants_0_rewrap: self, next,
                    no_std: $($t),+ ; depending: $($td, $dep_name)+
                }
            }
            fn previous(&self) -> Result<Self> {
                define_integers! { match_variants_0_rewrap: self, previous,
                    no_std: $($t),+ ; depending: $($td, $dep_name)+
                }
            }
        }
        /// This implementation defers to the actual integer variant.
        impl traits::Sign for Integers {
            fn can_positive(&self) -> bool {
                define_integers! { match_variants_0: self, can_positive,
                    no_std: $($t),+ ; depending: $($td, $dep_name)+
                }
            }
            fn can_negative(&self) -> bool {
                define_integers! { match_variants_0: self, can_negative,
                    no_std: $($t),+ ; depending: $($td, $dep_name)+
                }
            }
            fn is_positive(&self) -> bool {
                define_integers! { match_variants_0: self, is_positive,
                    no_std: $($t),+ ; depending: $($td, $dep_name)+
                }
            }
            fn is_negative(&self) -> bool {
                define_integers! { match_variants_0: self, is_negative,
                    no_std: $($t),+ ; depending: $($td, $dep_name)+
                }
            }
        }
        /// This implementation defers to the actual integer variant.
        impl traits::Ident for Integers {
            fn can_zero(&self) -> bool {
                define_integers! { match_variants_0: self, can_zero,
                    no_std: $($t),+ ; depending: $($td, $dep_name)+
                }
            }
            fn can_one(&self) -> bool {
                define_integers! { match_variants_0: self, can_one,
                    no_std: $($t),+ ; depending: $($td, $dep_name)+
                }
            }
            fn can_neg_one(&self) -> bool {
                define_integers! { match_variants_0: self, can_neg_one,
                    no_std: $($t),+ ; depending: $($td, $dep_name)+
                }
            }
            fn is_zero(&self) -> bool {
                define_integers! { match_variants_0: self, is_zero,
                    no_std: $($t),+ ; depending: $($td, $dep_name)+
                }
            }
            fn is_one(&self) -> bool {
                define_integers! { match_variants_0: self, is_one,
                    no_std: $($t),+ ; depending: $($td, $dep_name)+
                }
            }
            fn is_neg_one(&self) -> bool {
                define_integers! { match_variants_0: self, is_neg_one,
                    no_std: $($t),+ ; depending: $($td, $dep_name)+
                }
            }
        }

        /* impl From & TryFrom */

        $(
        impl From<$t> for Integers {
            fn from(z: $t) -> Integers {
                Integers::$t(z)
            }
        }
        )+
        $(
        #[cfg(feature = $dep_name)]
        impl From<$td> for Integers {
            fn from(z: $td) -> Integers {
                Integers::$td(z)
            }
        }
        )*

        $(
        impl TryFrom<Integers> for $t {
            type Error = crate::error::NumeraError;
            fn try_from(z: Integers) -> core::result::Result<$t, Self::Error> {
                match z {
                    Integers::$t(z) => Ok(z),
                    _ => Err(Self::Error::Conversion)
                }
            }
        }
        )+

        $( #[cfg(feature = $dep_name)]
        impl TryFrom<Integers> for $td {
            type Error = crate::error::NumeraError;
            fn try_from(z: Integers) -> core::result::Result<$td, Self::Error> {
                match z {
                    Integers::$td(z) => Ok(z),
                    _ => Err(Self::Error::Conversion)
                }
            }
        }
        )+
    };
}

define_integers![build_variants:
    no_std:
        Integer8, Integer16, Integer32, Integer64, Integer128

        // NonZeroInteger8, NonZeroInteger16, NonZeroInteger32,
        // NonZeroInteger64, NonZeroInteger128,

        // PositiveInteger8, PositiveInteger16, PositiveInteger32,
        // PositiveInteger64, PositiveInteger128,
        //
        // NegativeInteger8, NegativeInteger16, NegativeInteger32,
        // NegativeInteger64, NegativeInteger128,
        //
        // NonPositiveInteger8, NonPositiveInteger16, NonPositiveInteger32,
        // NonPositiveInteger64, NonPositiveInteger128,
        //
        // NonNegativeInteger8, NonNegativeInteger16, NonNegativeInteger32,
        // NonNegativeInteger64, NonNegativeInteger128,

        // Prime8, Prime16,
        // Prime32, // MAYBE associated feature for big data-file?
        // Prime64, Prime128,
    ;

    // feature-gated variants
    depending:
        IntegerBig, "ibig"
        // ,
        // NonZeroIntegerBig, "ibig",
        // NegativeIntegerBig, "ibig",
        // PositiveIntegerBig, "ibig",
        // NonNegativeIntegerBig, "ibig",
        // NonPositiveIntegerBig, "ibig"
        // PrimeBig, "ibig",
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
        assert_eq![24, size_of::<super::Integers>()];
    }

    #[test]
    #[cfg(feature = "all")]
    fn size_all_features() {
        assert_eq![32, size_of::<super::Integers>()];
        // assert_eq![24, size_of::<super::Integers>()]; // MAYBE:Box bigint
    }
}
