// numera::integer::impl_ops::div::n0z
//
//! Implements the `div` operation on `NonZeroInteger`.
//!
//! Completed:
//! - NonZeroInteger<N: Signed> / *integer*<N> = NonZeroInteger<N>
//! - NonZeroInteger<N: Signed> / *integer*<M> = NonZeroInteger<N> (where *prim* M < N)
//! - NonZeroInteger<N: Signed> / N = NonZeroInteger<N>
//! - NonZeroInteger<N: Signed> / M = NonZeroInteger<N> (where *prim* M < N)
//

use crate::{
    integer::{
        Integer, NegativeInteger, NonNegativeInteger, NonPositiveInteger, NonZeroInteger,
        PositiveInteger,
    },
    traits::{Number, Signed},
};
use core::ops::Div;

// NonZeroInteger<N: Signed> / *integer*<N> = NonZeroInteger<N>
// -----------------------------------------------------------------------------

macro_rules! impl_div_z_integer {
    (all: $($rhs:ident),+) => {
        $( impl_div_z_integer![$rhs]; )+
    };
    ($rhs:ident) => {
        impl<N: Number + Signed + Div<Output = N>> Div<$rhs<N>> for NonZeroInteger<N> {
            type Output = NonZeroInteger<N>;
            fn div(self, other: $rhs<N>) -> Self::Output {
                Self::Output::new(self.0.div(other.0))
            }
        }
    };
}

#[rustfmt::skip]
impl_div_z_integer![all:
    Integer, NegativeInteger, PositiveInteger,
    NonNegativeInteger, NonPositiveInteger, NonZeroInteger];

#[cfg(test)]
mod test_impl_div_z_integer {
    use crate::{integer::a::*, traits::Number};
    #[test]
    fn impl_div_z_rhs() {
        assert_eq![N0z::new(3), N0z::new(12) / N0z::new(4)];
        assert_eq![N0z::new(3), N0z::new(12) / Pz::new(4)];
        assert_eq![N0z::new(3), N0z::new(12) / Nnz::new(4)];
        assert_eq![N0z::new(-3), N0z::new(12) / Npz::new(-4)];
        assert_eq![N0z::new(-3), N0z::new(12) / Nz::new(-4)];
    }
}

// NonZeroInteger<N: Signed> / *integer*<M> = NonZeroInteger<N> (where primitive M < N)
// -----------------------------------------------------------------------------

macro_rules! impl_div_z_smaller_integer {
    (for_all_integers: $doc:literal, $( ($n1:ty, $n2:ty ) ),+) => {
        $( impl_div_z_smaller_integer![Integer, $doc, ($n1, $n2)]; )+
        $( impl_div_z_smaller_integer![NonZeroInteger, $doc, ($n1, $n2)]; )+
        $( impl_div_z_smaller_integer![NegativeInteger, $doc, ($n1, $n2)]; )+
        $( impl_div_z_smaller_integer![PositiveInteger, $doc, ($n1, $n2)]; )+
        $( impl_div_z_smaller_integer![NonNegativeInteger, $doc, ($n1, $n2)]; )+
        $( impl_div_z_smaller_integer![NonPositiveInteger, $doc, ($n1, $n2)]; )+
    };
    ($rhs:ident, $doc:literal, ($n1:ty, $n2:ty) ) => {
        paste::paste! {
            #[doc = "`N0z<" $n1 "> / " $rhs "<" $n2 "> = N0z<" $n1 ">`" $doc ]
            impl Div<$rhs<$n2>> for NonZeroInteger<$n1> {
                type Output = NonZeroInteger<$n1>;
                fn div(self, other: $rhs<$n2>) -> Self::Output {
                    Self::Output::new(self.0.div(other.0 as $n1))
                }
            }
        }
    };
}

#[rustfmt::skip]
impl_div_z_smaller_integer![for_all_integers: "",
    (i16, i8),
    (i32, i16), (i32, i8),
    (i64, i32), (i64, i16), (i64, i8),
    (i128, i64), (i128, i32), (i128, i16), (i128, i8) ];

#[rustfmt::skip]
#[cfg(target_pointer_width = "128")]
impl_div_z_smaller_integer![for_all_integers:
    "\n\nAssumes `target_pointer_width = \"128\"`",
    (isize, i8), (isize, i16), (isize, i32), (isize, i64), (isize, i128),
    (i128, isize) ];

#[rustfmt::skip]
#[cfg(target_pointer_width = "64")]
impl_div_z_smaller_integer![for_all_integers:
    "\n\nAssumes `target_pointer_width = \"64\"`",
    (isize, i8), (isize, i16), (isize, i32), (isize, i64),
    (i64, isize), (i128, isize) ];

#[rustfmt::skip]
#[cfg(target_pointer_width = "32")]
impl_div_z_smaller_integer![for_all_integers:
    "\n\nAssumes `target_pointer_width = \"32\"`",
    (isize, i8), (isize, i16), (isize, i32),
    (i32, isize), (i64, isize), (i128, isize) ];

#[rustfmt::skip]
#[cfg(target_pointer_width = "16")]
impl_div_z_smaller_integer![for_all_integers:
    "\n\nAssumes `target_pointer_width = \"16\"`",
    (isize, i8), (isize, i16),
    (i16, isize), (i32, isize), (i64, isize), (i128, isize) ];

#[rustfmt::skip]
#[cfg(target_pointer_width = "8")]
impl_div_z_smaller_integer![for_all_integers:
    "\n\nAssumes `target_pointer_width = \"8\"`",
    (isize, i8),
    (i8, isize), (i16, isize), (i32, isize), (i64, isize), (i128, isize) ];

#[cfg(test)]
mod test_impl_div_z_smaller_integer {
    use crate::{integer::a::*, traits::Number};
    #[test]
    fn impl_div_z_rhs() {
        assert_eq![N0z::new(3_i64), N0z::new(12_i64) / N0z::new(4_i16)];
        assert_eq![N0z::new(-3), N0z::new(12) / Npz::new(-4_i8)];

        #[cfg(target_pointer_width = "64")]
        assert_eq![N0z::new(3_isize), N0z::new(12_isize) / N0z::new(4_i64)];
    }
}

// NonZeroInteger<N: Signed> / N = NonZeroInteger<N>
// -----------------------------------------------------------------------------

macro_rules! impl_div_same_prim {
    (all: $t:ident, $($n:ty),+) => {
        $( impl_div_same_prim![$t, $n]; )+
    };
    ($t:ident, $n:ty) => {
        paste::paste! {
           #[doc = "`N0z<" $n "> / " $n " = N0z<" $n ">`" ]
            impl Div<$n> for $t<$n> {
                type Output = $t<$n>;
                fn div(self, other: $n) -> Self::Output {
                    Self::Output::new(self.0.div(other))
                }
            }
        }
    };
}
impl_div_same_prim![all: NonZeroInteger, i8, i16, i32, i64, i128, isize];

#[cfg(test)]
mod test_div_same_prim {
    use crate::{integer::a::*, traits::Number};
    #[test]
    fn impl_div_same_prim() {
        assert_eq![N0z::new(3_i8), N0z::new(12_i8) / 4];
        assert_eq![N0z::new(3), N0z::new(12) / 4]; // i32
    }
}

// NonZeroInteger<N: Signed> / M (where primitive M < N) = NonZeroInteger<N>
// -----------------------------------------------------------------------------

/// implements `Div` for an integer and a `< sized` primitive of the same sign,
/// and also from a integer and a `<= sized` pointer.
macro_rules! impl_div_smaller_prim {
    (all: $doc:literal, $( ($n1:ty, $n2:ty ) ),+) => {
        $( impl_div_smaller_prim![$doc, ( $n1, $n2 )]; )+
    };
    ($doc:literal, ($n1:ty, $n2:ty) ) => {
        paste::paste! {
            #[doc = "`N0z<" $n1 "> / " $n2 " = N0z<" $n1 ">`" $doc ]
            impl Div<$n2> for NonZeroInteger<$n1> {
                type Output = NonZeroInteger<$n1>;
                fn div(self, other: $n2) -> Self::Output {
                    Self::Output::new(self.0.div(other as $n1))
                }
            }
        }
    };
}

#[rustfmt::skip]
impl_div_smaller_prim![all: "",
    (i16, i8),
    (i32, i16), (i32, i8),
    (i64, i32), (i64, i16), (i64, i8),
    (i128, i64), (i128, i32), (i128, i16), (i128, i8) ];

#[rustfmt::skip]
#[cfg(target_pointer_width = "128")]
impl_div_smaller_prim![all:
    "\n\nAssumes `target_pointer_width = \"128\"`",
    (isize, i8), (isize, i16), (isize, i32), (isize, i64), (isize, i128),
    (i128, isize) ];

#[rustfmt::skip]
#[cfg(target_pointer_width = "64")]
impl_div_smaller_prim![all:
    "\n\nAssumes `target_pointer_width = \"64\"`",
    (isize, i8), (isize, i16), (isize, i32), (isize, i64),
    (i64, isize), (i128, isize) ];

#[rustfmt::skip]
#[cfg(target_pointer_width = "32")]
impl_div_smaller_prim![all:
    "\n\nAssumes `target_pointer_width = \"32\"`",
    (isize, i8), (isize, i16), (isize, i32),
    (i32, isize), (i64, isize), (i128, isize) ];

#[rustfmt::skip]
#[cfg(target_pointer_width = "16")]
impl_div_smaller_prim![all:
    "\n\nAssumes `target_pointer_width = \"16\"`",
    (isize, i8), (isize, i16),
    (i16, isize), (i32, isize), (i64, isize), (i128, isize) ];

#[rustfmt::skip]
#[cfg(target_pointer_width = "8")]
impl_div_smaller_prim![all:
    "\n\nAssumes `target_pointer_width = \"8\"`",
    (isize, i8),
    (i8, isize), (i16, isize), (i32, isize), (i64, isize), (i128, isize) ];

#[cfg(test)]
mod test_div_smaller_prim {
    use crate::{integer::a::*, traits::Number};

    #[test]
    fn impl_div_smaller_prim() {
        assert_eq![N0z::new(3_i16), N0z::new(12_i16) / 4_i8];
        assert_eq![N0z::new(3), N0z::new(12) / 4_i8]; // N0z<i32> / i8

        #[cfg(target_pointer_width = "64")]
        assert_eq![N0z::new(3_isize), N0z::new(12_isize) / 4_i64];
    }
}

// ## NonZeroInteger<IBig> / IBig = NonZeroInteger<IBig>
// ## NonZeroInteger<IBig> / *primitive* = NonZeroInteger<IBig>
// -----------------------------------------------------------------------------

#[cfg(feature = "ibig")]
mod div_ibig {
    use super::*;
    use ibig::IBig;

    // NonZeroInteger<IBig> / IBig

    /// `N0z<IBig> / IBig = N0z<IBig>`
    impl Div<IBig> for NonZeroInteger<IBig> {
        type Output = NonZeroInteger<IBig>;
        fn div(self, other: IBig) -> NonZeroInteger<IBig> {
            Self::Output::new(self.0.div(other))
        }
    }

    // Self::Output<IBig> / *any integer primitive*

    /// implements `Div` for an IBig integer and a primitive.
    macro_rules! impl_div_ibig_prim {
        (all: $t:ident, $tn:ident, $( $n:ty ),+) => {
            $( impl_div_ibig_prim![$t, $tn, $n]; )+
        };
        ($t:ident, $tn:ident, $n:ty) => {
            paste::paste! {
               #[doc = "`N0z<" $tn "> / " $n " = N0z<" $tn ">`" ]
                impl Div<$n> for $t<$tn> {
                    type Output = $t<$tn>;
                    fn div(self, other: $n) -> Self::Output {
                        Self::Output::new(self.0.div(other))
                    }
                }
            }
        };
    }

    #[rustfmt::skip]
    impl_div_ibig_prim![all: NonZeroInteger, IBig,
        i8, u8, i16, u16, i32, u32, i64, u64, i128, isize, usize ];

    #[cfg(test)]
    mod tests {
        use super::IBig;
        use crate::{integer::a::*, traits::Number};

        #[test]
        fn ibig() {
            // N0z<IBig> / *integer*<IBig> = N0z<IBig>
            assert_eq![
                N0z::new(IBig::from(3)),
                N0z::new(IBig::from(12)) / N0z::new(IBig::from(4))
            ];

            // N0z<IBig> / IBig = N0z<IBig>
            assert_eq![
                N0z::new(IBig::from(3)),
                N0z::new(IBig::from(12)) / IBig::from(4)
            ];

            // N0z<IBig> / *primitive* = N0z<IBig>
            assert_eq![
                N0z::new(IBig::from(3_i16)),
                N0z::new(IBig::from(12_i64)) / 4_u8
            ];
        }
    }
}
