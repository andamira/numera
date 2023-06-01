// numera::number::integer::z::ops::neg
//
//! Implement the negation operations.
//

use crate::number::integer::*;
use core::ops::Neg;
use devela::paste;

macro_rules! impl_integer_neg {
    // impl Neg ops for multiple integer types
    //
    // # Args
    // $t: integer base name. e.g. Integer
    // $p: inner primitive base name. e.g. i
    // $b: integer and primitive bitsize. e.g. 8
    ( $($t:ident + $p:ident + $b:literal, cast: $bcast:literal);+ ) => {
        $( impl_integer_neg![neg: $t + $p + $b]; )+
    };

    // impl the negation operations
    //
    // impl variants:
    // - neg
    // - checked_
    // - saturating_
    // - wrapping_
    (neg: $t:ident + $p:ident + $b:literal) => { paste! {
        impl Neg for [<$t$b>] {
            type Output = [<$t$b>];
            /// Performs the unary `-` operation.
            #[inline]
            #[must_use]
            fn neg(self) -> Self::Output {
                self.neg()
            }
        }
        /// # Negation
        impl [<$t$b>] {
            /// Basic negation. Computes `-self`.
            ///
            /// # Panics
            /// Panics in debug, if `self == MIN`.
            /// While in release, it returns `MIN`,
            #[doc = "same as [`wrapping_neg`][" [<$t$b>] "#method.wrapping_neg]"]
            #[inline]
            #[must_use]
            pub const fn neg(self) -> [<$t$b>] {
                [<$t$b>](-self.0)
            }

            /// Checked negation.
            /// Computes `-self`, returning `None` if `self == MIN` instead of overflowing.
            #[inline]
            #[must_use]
            pub const fn checked_neg(self) -> Option<[<$t$b>]> {
                if let Some(result) = self.0.checked_neg() {
                    Some(Self(result))
                } else {
                    None
                }
            }

            /// Saturating negation.
            /// Computes `-self`, returning `MAX` if `self == MIN` instead of overflowing.
            #[inline]
            #[must_use]
            pub const fn saturating_neg(self) -> [<$t$b>] {
                Self(self.0.saturating_neg())
            }

            /// Wrapping negation.
            /// Computes `-self`, returning `MIN` if `self == MIN` instead of overflowing.
            #[inline]
            #[must_use]
            pub const fn wrapping_neg(self) -> [<$t$b>] {
                Self(self.0.wrapping_neg())
            }

            /// Overflowing negation.
            /// Negates self, overflowing if this is equal to the minimum value.
            ///
            /// Returns a tuple of the negated version along with a boolean
            /// indicating whether an arithmetic overflow happened.
            /// If `self == MIN` then (`MIN`, `true`) is returned.
            #[inline]
            #[must_use]
            pub const fn overflowing_neg(self) -> ([<$t$b>], bool) {
                let (result, overflown) = self.0.overflowing_neg();
                (Self(result), overflown)
            }
        }
    }};
}

impl_integer_neg![
    Integer+i+8, cast:16;
    Integer+i+16, cast:32;
    Integer+i+32, cast:64;
    Integer+i+64, cast:128;
    Integer+i+128, cast:128
];

#[cfg(feature = "dashu-int")]
mod big {
    use super::*;

    impl Neg for IntegerBig {
        type Output = IntegerBig;
        /// Performs the unary `-` operation.
        #[inline]
        #[must_use]
        fn neg(self) -> Self::Output {
            Self(-self.0)
        }
    }
}
