// numera::integer::impl_ops::mul::n0z
//
//! Implements the `Mul` operation on `NonZeroInteger`.
//!
//! Completed:
//! - NonZeroInteger<N: Signed> × *integer*<N> = NonZeroInteger<N>
//! - NonZeroInteger<N: Signed> × *integer*<M> = NonZeroInteger<N> (where M < N)
//! - NonZeroInteger<N: Signed> × N = NonZeroInteger<N>
//! - NonZeroInteger<N: Signed> × M = NonZeroInteger<N> (where M < N)
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
use core::ops::Mul;

// NonZeroInteger<N: Signed> × *integer*<N> = NonZeroInteger<N>
// -----------------------------------------------------------------------------

macro_rules! impl_mul_n0z_integer {
    (all: $($rhs:ident),+) => {
        $( impl_mul_n0z_integer![$rhs]; )+
    };
    ($rhs:ident) => {
        impl<N: Number + Signed + Mul<Output = N>> Mul<$rhs<N>> for NonZeroInteger<N> {
            type Output = NonZeroInteger<N>;
            fn mul(self, other: $rhs<N>) -> Self::Output {
                Self::Output::new(self.0.mul(other.0))
            }
        }
    };
}

#[rustfmt::skip]
impl_mul_n0z_integer![all:
    Integer, NegativeInteger, PositiveInteger,
    NonNegativeInteger, NonPositiveInteger, NonZeroInteger];

#[cfg(test)]
mod test_impl_mul_n0z_integer {
    use crate::{integer::a::*, traits::Number};

    #[test]
    fn impl_mul_n0z_rhs() {
        assert_eq![N0z::new(12), N0z::new(4) * Z::new(3)];
        assert_eq![N0z::new(12), N0z::new(4) * Pz::new(3)];
        assert_eq![N0z::new(12), N0z::new(4) * Nnz::new(3)];
        assert_eq![N0z::new(-12), N0z::new(4) * Npz::new(-3)];
        assert_eq![N0z::new(-12), N0z::new(4) * Nz::new(-3)];
    }
}

// NonZeroInteger<N: Signed> × *integer*<M> = NonZeroInteger<N> (where M < N)
// -----------------------------------------------------------------------------

macro_rules! impl_mul_n0z_smaller_integer {
    (for_all_integers: $doc:literal, $( ($n1:ty, $n2:ty ) ),+) => {
        $( impl_mul_n0z_smaller_integer![Integer, $doc, ($n1, $n2)]; )+
        $( impl_mul_n0z_smaller_integer![NonZeroInteger, $doc, ($n1, $n2)]; )+
        $( impl_mul_n0z_smaller_integer![NegativeInteger, $doc, ($n1, $n2)]; )+
        $( impl_mul_n0z_smaller_integer![PositiveInteger, $doc, ($n1, $n2)]; )+
        $( impl_mul_n0z_smaller_integer![NonNegativeInteger, $doc, ($n1, $n2)]; )+
        $( impl_mul_n0z_smaller_integer![NonPositiveInteger, $doc, ($n1, $n2)]; )+
    };
    ($rhs:ident, $doc:literal, ($n1:ty, $n2:ty) ) => {
        paste::paste! {
            #[doc = "`N0z<" $n1 "> × " $rhs "<" $n2 "> = N0z<" $n1 ">`" $doc ]
            impl Mul<$rhs<$n2>> for NonZeroInteger<$n1> {
                type Output = NonZeroInteger<$n1>;
                fn mul(self, other: $rhs<$n2>) -> Self::Output {
                    Self::Output::new(self.0.mul(other.0 as $n1))
                }
            }
        }
    };
}

#[rustfmt::skip]
impl_mul_n0z_smaller_integer![for_all_integers: "",
    (i16, i8),
    (i32, i16), (i32, i8),
    (i64, i32), (i64, i16), (i64, i8),
    (i128, i64), (i128, i32), (i128, i16), (i128, i8) ];

#[rustfmt::skip]
#[cfg(target_pointer_width = "128")]
impl_mul_n0z_smaller_integer![for_all_integers:
    "\n\nAssumes `target_pointer_width = \"128\"`",
    (isize, i8), (isize, i16), (isize, i32), (isize, i64), (isize, i128),
    (i128, isize) ];

#[rustfmt::skip]
#[cfg(target_pointer_width = "64")]
impl_mul_n0z_smaller_integer![for_all_integers:
    "\n\nAssumes `target_pointer_width = \"64\"`",
    (isize, i8), (isize, i16), (isize, i32), (isize, i64),
    (i64, isize), (i128, isize) ];

#[rustfmt::skip]
#[cfg(target_pointer_width = "32")]
impl_mul_n0z_smaller_integer![for_all_integers:
    "\n\nAssumes `target_pointer_width = \"32\"`",
    (isize, i8), (isize, i16), (isize, i32),
    (i32, isize), (i64, isize), (i128, isize) ];

#[rustfmt::skip]
#[cfg(target_pointer_width = "16")]
impl_mul_n0z_smaller_integer![for_all_integers:
    "\n\nAssumes `target_pointer_width = \"16\"`",
    (isize, i8), (isize, i16),
    (i16, isize), (i32, isize), (i64, isize), (i128, isize) ];

#[rustfmt::skip]
#[cfg(target_pointer_width = "8")]
impl_mul_n0z_smaller_integer![for_all_integers:
    "\n\nAssumes `target_pointer_width = \"8\"`",
    (isize, i8),
    (i8, isize), (i16, isize), (i32, isize), (i64, isize), (i128, isize) ];

#[cfg(test)]
mod test_impl_mul_n0z_smaller_integer {
    use crate::{integer::a::*, traits::Number};
    #[test]
    fn impl_mul_n0z_rhs() {
        assert_eq![N0z::new(12_i64), N0z::new(4_i64) * Z::new(3_i16)];
        assert_eq![N0z::new(-12), N0z::new(4) * Npz::new(-3_i8)];

        #[cfg(target_pointer_width = "64")]
        assert_eq![N0z::new(12_isize), N0z::new(4_isize) * N0z::new(3_i64)];
    }
}

// NonZeroInteger<N: Signed> × N = Integer<N>
// -----------------------------------------------------------------------------

macro_rules! impl_mul_same_prim {
    (all: $($n:ty),+) => {
        $( impl_mul_same_prim![$n]; )+
    };
    ($n:ty) => {
        paste::paste! {
           #[doc = "`N0z<" $n "> × " $n " = N0z<" $n ">`" ]
            impl Mul<$n> for NonZeroInteger<$n> {
                type Output = NonZeroInteger<$n>;
                fn mul(self, other: $n) -> Self::Output {
                    Self::Output::new(self.0.mul(other))
                }
            }
        }
    };
}
impl_mul_same_prim![all: i8, i16, i32, i64, i128, isize];

#[cfg(test)]
mod test_mul_same_prim {
    use crate::{integer::a::*, traits::Number};
    #[test]
    fn impl_mul_same_prim() {
        assert_eq![N0z::new(12_i8), N0z::new(4_i8) * 3];
        assert_eq![N0z::new(12), N0z::new(4) * 3]; // i32
    }
}

// NonZeroInteger<N: Signed> × M (where M < N) = NonZeroInteger<N>
// -----------------------------------------------------------------------------

/// implements `Mul` for an integer and a `< sized` primitive of the same sign,
/// and also from a integer and a `<= sized` pointer.
macro_rules! impl_mul_smaller_prim {
    (all: $doc:literal, $( ($n1:ty, $n2:ty ) ),+) => {
        $( impl_mul_smaller_prim![$doc, ( $n1, $n2 )]; )+
    };
    ($doc:literal, ($n1:ty, $n2:ty) ) => {
        paste::paste! {
            #[doc = "`N0z<" $n1 "> × " $n2 " = N0z<" $n1 ">`" $doc ]
            impl Mul<$n2> for NonZeroInteger<$n1> {
                type Output = NonZeroInteger<$n1>;
                fn mul(self, other: $n2) -> Self::Output {
                    Self::Output::new(self.0.mul(other as $n1))
                }
            }
        }
    };
}

#[rustfmt::skip]
impl_mul_smaller_prim![all: "",
    (i16, i8),
    (i32, i16), (i32, i8),
    (i64, i32), (i64, i16), (i64, i8),
    (i128, i64), (i128, i32), (i128, i16), (i128, i8) ];

#[rustfmt::skip]
#[cfg(target_pointer_width = "128")]
impl_mul_smaller_prim![all: 
    "\n\nAssumes `target_pointer_width = \"128\"`",
    (isize, i8), (isize, i16), (isize, i32), (isize, i64), (isize, i128),
    (i128, isize) ];

#[rustfmt::skip]
#[cfg(target_pointer_width = "64")]
impl_mul_smaller_prim![all:
    "\n\nAssumes `target_pointer_width = \"64\"`",
    (isize, i8), (isize, i16), (isize, i32), (isize, i64),
    (i64, isize), (i128, isize) ];

#[rustfmt::skip]
#[cfg(target_pointer_width = "32")]
impl_mul_smaller_prim![all:
    "\n\nAssumes `target_pointer_width = \"32\"`",
    (isize, i8), (isize, i16), (isize, i32),
    (i32, isize), (i64, isize), (i128, isize) ];

#[rustfmt::skip]
#[cfg(target_pointer_width = "16")]
impl_mul_smaller_prim![all:
    "\n\nAssumes `target_pointer_width = \"16\"`",
    (isize, i8), (isize, i16),
    (i16, isize), (i32, isize), (i64, isize), (i128, isize) ];

#[rustfmt::skip]
#[cfg(target_pointer_width = "8")]
impl_mul_smaller_prim![all:
    "\n\nAssumes `target_pointer_width = \"8\"`",
    (isize, i8),
    (i8, isize), (i16, isize), (i32, isize), (i64, isize), (i128, isize) ];

#[cfg(test)]
mod test_mul_smaller_prim {
    use crate::{integer::a::*, traits::Number};

    #[test]
    fn impl_mul_smaller_prim() {
        assert_eq![N0z::new(12_i16), N0z::new(4_i16) * 3_i8];
        assert_eq![N0z::new(12), N0z::new(4) * 3_i8]; // N0z<i32> * i8

        #[cfg(target_pointer_width = "64")]
        assert_eq![N0z::new(12_isize), N0z::new(4_isize) * 3_i64];
    }
}

// ## Integer<IBig> × IBig = Integer<IBig>
// ## Integer<IBig> × *primitive* = Integer<IBig>
// -----------------------------------------------------------------------------

#[cfg(feature = "ibig")]
mod mul_ibig {
    use super::*;
    use ibig::IBig;

    // NonZeroInteger<IBig> × IBig

    /// `N0z<IBig> × IBig = N0z<IBig>`
    impl Mul<IBig> for NonZeroInteger<IBig> {
        type Output = NonZeroInteger<IBig>;
        fn mul(self, other: IBig) -> Self::Output {
            Self::Output::new(self.0.mul(other))
        }
    }

    // Integer<IBig> × *any integer primitive*

    /// implements `Mul` for an IBig integer and a primitive.
    macro_rules! impl_mul_ibig_prim {
        (all: $tn:ident, $( $n:ty ),+) => {
            $( impl_mul_ibig_prim![$tn, $n]; )+
        };
        ($tn:ident, $n:ty) => {
            paste::paste! {
               #[doc = "`N0z<" $tn "> × " $n " = N0z<" $tn ">`" ]
                impl Mul<$n> for NonZeroInteger<$tn> {
                    type Output = NonZeroInteger<$tn>;
                    fn mul(self, other: $n) -> Self::Output {
                        Self::Output::new(self.0.mul(other))
                    }
                }
            }
        };
    }

    #[rustfmt::skip]
    impl_mul_ibig_prim![all: IBig,
        i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize ];

    #[cfg(test)]
    mod test_ibig {
        use super::IBig;
        use crate::{integer::a::*, traits::Number};

        #[test]
        fn ibig() {
            // N0z<IBig> × *integer*<IBig> = N0z<IBig>
            assert_eq![
                N0z::new(IBig::from(12)),
                N0z::new(IBig::from(4)) * Z::new(IBig::from(3))
            ];

            // N0z<IBig> × IBig = N0z<IBig>
            assert_eq![
                N0z::new(IBig::from(12)),
                N0z::new(IBig::from(4)) * IBig::from(3)
            ];

            // N0z<IBig> × *primitive* = N0z<IBig>
            assert_eq![
                N0z::new(IBig::from(12_i16)),
                N0z::new(IBig::from(4_i64)) * 3_u8
            ];
        }
    }
}
