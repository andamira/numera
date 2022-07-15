// numera::integer::impl_ops::sub::n0z
//
//! Implements the `Sub` operation on `NonZeroInteger`.
//!
//! Completed:
//! - NonZeroInteger<N: Signed> - *integer*<N> = NonZeroInteger<N>
//! - NonZeroInteger<N: Signed> - *integer*<M> = NonZeroInteger<N> (where *prim* M < N)
//! - NonZeroInteger<N: Signed> - N = NonZeroInteger<N>
//! - NonZeroInteger<N: Signed> - M = NonZeroInteger<N> (where *prim* M < N)
//!
//! All panic if the result == 0.
//

use crate::{
    integer::{
        Integer, NegativeInteger, NonNegativeInteger, NonPositiveInteger, NonZeroInteger,
        PositiveInteger,
    },
    traits::{Number, Signed},
};
use core::ops::Sub;

// NonZeroInteger<N: Signed> - *integer*<N> = NonZeroInteger<N>
// -----------------------------------------------------------------------------

macro_rules! impl_sub_n0z_integer {
    (all: $($rhs:ident),+) => {
        $( impl_sub_n0z_integer![$rhs]; )+
    };
    ($rhs:ident) => {
        impl<N: Number + Signed + Sub<Output = N>> Sub<$rhs<N>> for NonZeroInteger<N> {
            type Output = NonZeroInteger<N>;
            fn sub(self, other: $rhs<N>) -> Self::Output {
                NonZeroInteger::new(self.0.sub(other.0)) // panics if == 0
            }
        }
    };
}

#[rustfmt::skip]
impl_sub_n0z_integer![all:
    Integer, NegativeInteger, PositiveInteger,
    NonNegativeInteger, NonPositiveInteger, NonZeroInteger];

#[cfg(test)]
mod test_impl_sub_n0z_integer {
    use crate::{integer::a::*, traits::Number};

    #[test]
    fn impl_sub_n0z_rhs() {
        assert_eq![N0z::new(1), N0z::new(4) - Z::new(3)];
        assert_eq![N0z::new(1), N0z::new(4) - Pz::new(3)];
        assert_eq![N0z::new(1), N0z::new(4) - Nnz::new(3)];
        assert_eq![N0z::new(7), N0z::new(4) - Npz::new(-3)];
        assert_eq![N0z::new(7), N0z::new(4) - Nz::new(-3)];
    }
}

// NonZeroInteger<N: Signed> - *integer*<M> = NonZeroInteger<N> (where primitive M < N)
// -----------------------------------------------------------------------------

macro_rules! impl_sub_n0z_smaller_integer {
    (for_all_integers: $doc:literal, $( ($n1:ty, $n2:ty ) ),+) => {
        $( impl_sub_n0z_smaller_integer![Integer, $doc, ($n1, $n2)]; )+
        $( impl_sub_n0z_smaller_integer![NonZeroInteger, $doc, ($n1, $n2)]; )+
        $( impl_sub_n0z_smaller_integer![NegativeInteger, $doc, ($n1, $n2)]; )+
        $( impl_sub_n0z_smaller_integer![PositiveInteger, $doc, ($n1, $n2)]; )+
        $( impl_sub_n0z_smaller_integer![NonNegativeInteger, $doc, ($n1, $n2)]; )+
        $( impl_sub_n0z_smaller_integer![NonPositiveInteger, $doc, ($n1, $n2)]; )+
    };
    ($rhs:ident, $doc:literal, ($n1:ty, $n2:ty) ) => {
        paste::paste! {
            #[doc = "`N0z<" $n1 "> - " $rhs "<" $n2 "> = N0z<" $n1 ">`" $doc ]
            impl Sub<$rhs<$n2>> for NonZeroInteger<$n1> {
                type Output = NonZeroInteger<$n1>;
                fn sub(self, other: $rhs<$n2>) -> Self::Output {
                    Self::Output::new(self.0.sub(other.0 as $n1))
                }
            }
        }
    };
}

#[rustfmt::skip]
impl_sub_n0z_smaller_integer![for_all_integers: "",
    (i16, i8),
    (i32, i16), (i32, i8),
    (i64, i32), (i64, i16), (i64, i8),
    (i128, i64), (i128, i32), (i128, i16), (i128, i8) ];

#[rustfmt::skip]
#[cfg(target_pointer_width = "128")]
impl_sub_n0z_smaller_integer![for_all_integers:
    "\n\nAssumes `target_pointer_width = \"128\"`",
    (isize, i8), (isize, i16), (isize, i32), (isize, i64), (isize, i128),
    (i128, isize) ];

#[rustfmt::skip]
#[cfg(target_pointer_width = "64")]
impl_sub_n0z_smaller_integer![for_all_integers:
    "\n\nAssumes `target_pointer_width = \"64\"`",
    (isize, i8), (isize, i16), (isize, i32), (isize, i64),
    (i64, isize), (i128, isize) ];

#[rustfmt::skip]
#[cfg(target_pointer_width = "32")]
impl_sub_n0z_smaller_integer![for_all_integers:
    "\n\nAssumes `target_pointer_width = \"32\"`",
    (isize, i8), (isize, i16), (isize, i32),
    (i32, isize), (i64, isize), (i128, isize) ];

#[rustfmt::skip]
#[cfg(target_pointer_width = "16")]
impl_sub_n0z_smaller_integer![for_all_integers:
    "\n\nAssumes `target_pointer_width = \"16\"`",
    (isize, i8), (isize, i16),
    (i16, isize), (i32, isize), (i64, isize), (i128, isize) ];

#[rustfmt::skip]
#[cfg(target_pointer_width = "8")]
impl_sub_n0z_smaller_integer![for_all_integers:
    "\n\nAssumes `target_pointer_width = \"8\"`",
    (isize, i8),
    (i8, isize), (i16, isize), (i32, isize), (i64, isize), (i128, isize) ];

#[cfg(test)]
mod test_impl_sub_n0z_smaller_integer {
    use crate::{integer::a::*, traits::Number};
    #[test]
    fn impl_sub_n0z_rhs() {
        assert_eq![N0z::new(1_i64), N0z::new(4_i64) - Z::new(3_i16)];
        assert_eq![N0z::new(7), N0z::new(4) - Npz::new(-3_i8)];

        #[cfg(target_pointer_width = "64")]
        assert_eq![N0z::new(1_isize), N0z::new(4_isize) - N0z::new(3_i64)];
    }
}

// NonZeroInteger<N: Signed> - N = Integer<N>
// -----------------------------------------------------------------------------

macro_rules! impl_sub_same_prim {
    (all: $($n:ty),+) => {
        $( impl_sub_same_prim![$n]; )+
    };
    ($n:ty) => {
        paste::paste! {
           #[doc = "`N0z<" $n "> - " $n " = N0z<" $n ">`" ]
            impl Sub<$n> for NonZeroInteger<$n> {
                type Output = NonZeroInteger<$n>;
                fn sub(self, other: $n) -> Self::Output {
                    Self::Output::new(self.0.sub(other))
                }
            }
        }
    };
}
impl_sub_same_prim![all: i8, i16, i32, i64, i128, isize];

#[cfg(test)]
mod test_sub_same_prim {
    use crate::{integer::a::*, traits::Number};
    #[test]
    fn impl_sub_same_prim() {
        assert_eq![N0z::new(1_i8), N0z::new(4_i8) - 3];
        assert_eq![N0z::new(1), N0z::new(4) - 3]; // i32
    }
}

// NonZeroInteger<N: Signed> - M (where primitive M < N) = NonZeroInteger<N>
// -----------------------------------------------------------------------------

/// implements `Sub` for an integer and a `< sized` primitive of the same sign,
/// and also from a integer and a `<= sized` pointer.
macro_rules! impl_sub_smaller_prim {
    (all: $doc:literal, $( ($n1:ty, $n2:ty ) ),+) => {
        $( impl_sub_smaller_prim![$doc, ( $n1, $n2 )]; )+
    };
    ($doc:literal, ($n1:ty, $n2:ty) ) => {
        paste::paste! {
            #[doc = "`N0z<" $n1 "> - " $n2 " = N0z<" $n1 ">`" $doc ]
            impl Sub<$n2> for NonZeroInteger<$n1> {
                type Output = NonZeroInteger<$n1>;
                fn sub(self, other: $n2) -> Self::Output {
                    Self::Output::new(self.0.sub(other as $n1))
                }
            }
        }
    };
}

#[rustfmt::skip]
impl_sub_smaller_prim![all: "",
    (i16, i8),
    (i32, i16), (i32, i8),
    (i64, i32), (i64, i16), (i64, i8),
    (i128, i64), (i128, i32), (i128, i16), (i128, i8) ];

#[rustfmt::skip]
#[cfg(target_pointer_width = "128")]
impl_sub_smaller_prim![all: 
    "\n\nAssumes `target_pointer_width = \"128\"`",
    (isize, i8), (isize, i16), (isize, i32), (isize, i64), (isize, i128),
    (i128, isize) ];

#[rustfmt::skip]
#[cfg(target_pointer_width = "64")]
impl_sub_smaller_prim![all:
    "\n\nAssumes `target_pointer_width = \"64\"`",
    (isize, i8), (isize, i16), (isize, i32), (isize, i64),
    (i64, isize), (i128, isize) ];

#[rustfmt::skip]
#[cfg(target_pointer_width = "32")]
impl_sub_smaller_prim![all:
    "\n\nAssumes `target_pointer_width = \"32\"`",
    (isize, i8), (isize, i16), (isize, i32),
    (i32, isize), (i64, isize), (i128, isize) ];

#[rustfmt::skip]
#[cfg(target_pointer_width = "16")]
impl_sub_smaller_prim![all:
    "\n\nAssumes `target_pointer_width = \"16\"`",
    (isize, i8), (isize, i16),
    (i16, isize), (i32, isize), (i64, isize), (i128, isize) ];

#[rustfmt::skip]
#[cfg(target_pointer_width = "8")]
impl_sub_smaller_prim![all:
    "\n\nAssumes `target_pointer_width = \"8\"`",
    (isize, i8),
    (i8, isize), (i16, isize), (i32, isize), (i64, isize), (i128, isize) ];

#[cfg(test)]
mod test_sub_smaller_prim {
    use crate::{integer::a::*, traits::Number};

    #[test]
    fn impl_sub_smaller_prim() {
        assert_eq![N0z::new(1_i16), N0z::new(4_i16) - 3_i8];
        assert_eq![N0z::new(1), N0z::new(4) - 3_i8]; // N0z<i32> - i8

        #[cfg(target_pointer_width = "64")]
        assert_eq![N0z::new(1_isize), N0z::new(4_isize) - 3_i64];
    }
}

// ## Integer<IBig> - IBig = Integer<IBig>
// ## Integer<IBig> - *primitive* = Integer<IBig>
// -----------------------------------------------------------------------------

#[cfg(feature = "ibig")]
mod sub_ibig {
    use super::*;
    use ibig::IBig;

    // NonZeroInteger<IBig> - IBig

    /// `N0z<IBig> - IBig = N0z<IBig>`
    impl Sub<IBig> for NonZeroInteger<IBig> {
        type Output = NonZeroInteger<IBig>;
        fn sub(self, other: IBig) -> NonZeroInteger<IBig> {
            NonZeroInteger::new(self.0.sub(other))
        }
    }

    // Integer<IBig> - *any integer primitive*

    /// implements `Sub` for an IBig integer and a primitive.
    macro_rules! impl_sub_ibig_prim {
        (all: $tn:ident, $( $n:ty ),+) => {
            $( impl_sub_ibig_prim![$tn, $n]; )+
        };
        ($tn:ident, $n:ty) => {
            paste::paste! {
               #[doc = "`N0z<" $tn "> - " $n " = N0z<" $tn ">`" ]
                impl Sub<$n> for NonZeroInteger<$tn> {
                    type Output = NonZeroInteger<$tn>;
                    fn sub(self, other: $n) -> Self::Output {
                        Self::Output::new(self.0.sub(other))
                    }
                }
            }
        };
    }

    #[rustfmt::skip]
    impl_sub_ibig_prim![all: IBig,
        i8, u8, i16, u16, i32, u32, i64, u64, i128, isize, usize ];

    #[cfg(test)]
    mod test_ibig {
        use super::IBig;
        use crate::{integer::a::*, traits::Number};

        #[test]
        fn ibig() {
            // N0z<IBig> - *integer*<IBig> = N0z<IBig>
            assert_eq![
                N0z::new(IBig::from(1)),
                N0z::new(IBig::from(4)) - Z::new(IBig::from(3))
            ];

            // N0z<IBig> - IBig = N0z<IBig>
            assert_eq![
                N0z::new(IBig::from(1)),
                N0z::new(IBig::from(4)) - IBig::from(3)
            ];

            // N0z<IBig> - *primitive* = N0z<IBig>
            assert_eq![
                N0z::new(IBig::from(1_i16)),
                N0z::new(IBig::from(4_i64)) - 3_u8
            ];
        }
    }
}
