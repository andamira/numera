// numera::number::integer::z::ops::pow
//
//! Implement the exponent operations.
//

use crate::number::integer::*;
use devela::paste;

macro_rules! impl_integer_sub {
    // impl power ops for multiple integer types
    //
    // # Args
    // $t: integer base name. e.g. Integer
    // $p: inner primitive base name. e.g. i
    // $b: integer and primitive bitsize. e.g. 8
    ( $($t:ident + $p:ident + $b:literal, cast: $bcast:literal);+ ) => {
        $(
            impl_integer_sub![pow: $t + $p + $b];
        )+
    };

    // exponent operations
    //
    // impl variants:
    // - pow
    // - checked_
    // - saturating_
    // - wrapping_
    // - overflowing_
    (pow: $t:ident + $p:ident + $b:literal) => { paste! {
        /// # Exponentiation
        impl [<$t$b>] {

            /// Integer exponentiation.
            ///
            /// Raises self to the power of `exp`
            ///
            /// # Panics
            /// Panics in debug, on overflow.
            /// While in release, it performs two's complement wrapping.
            #[inline]
            #[must_use]
            pub const fn pow(self, exp: u32) -> [<$t$b>] {
                Self(self.0.pow(exp))
            }

            /// Checked integer exponentiation.
            ///
            /// Returns `None` on overflow.
            #[inline]
            #[must_use]
            pub const fn checked_pow(self, exp: u32) -> Option<[<$t$b>]> {
                if let Some(result) = self.0.checked_pow(exp) {
                    Some(Self(result))
                } else {
                    None
                }
            }

            /// Saturating integer exponentiation.
            ///
            /// Raises self to the power of `exp`,
            /// saturating at the numeric bounds instead of overflowing.
            #[inline]
            #[must_use]
            pub const fn saturating_pow(self, exp: u32) -> [<$t$b>] {
                Self(self.0.saturating_pow(exp))
            }

            /// Wrapping (modular) integer exponentiation.
            ///
            /// Raises self to the power of `exp`,
            /// wrapping around at the boundary of the type.
            #[inline]
            #[must_use]
            pub const fn wrapping_pow(self, exp: u32) -> [<$t$b>] {
                Self(self.0.wrapping_pow(exp))
            }

            /// Overflowing integer exponentiation.
            ///
            /// Raises self to the power of `exp`,
            ///
            /// Returns a tuple of the exponentiation along with a boolean
            /// indicating whether an overflow happened.
            #[inline]
            #[must_use]
            pub const fn overflowing_pow(self, exp: u32) -> ([<$t$b>], bool) {
                let (result, overflown) = self.0.overflowing_pow(exp);
                (Self(result), overflown)
            }
        }
    }};
}

impl_integer_sub![
    Integer+i+8, cast:16;
    Integer+i+16, cast:32;
    Integer+i+32, cast:64;
    Integer+i+64, cast:128;
    Integer+i+128, cast:128
];

#[cfg(feature = "dashu-int")]
mod big {
    use super::*;

    impl IntegerBig {
        /// Raises self to the power of `exp`.
        #[inline]
        #[must_use]
        pub fn pow(&self, exp: usize) -> IntegerBig {
            Self(self.0.pow(exp))
        }
    }
}
