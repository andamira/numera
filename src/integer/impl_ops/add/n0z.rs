// numera::integer::impl_ops::add::n0z
//
//! Implements the `Add` operation on `NonZeroInteger`.
//!
//! Completed:
//! - NonZeroInteger<N: Signed> + *integer*<N> = NonZeroInteger<N>
//! - NonZeroInteger<N: Signed> + *integer*<M> = NonZeroInteger<N> (where M:Signed < N)
//! - NonZeroInteger<N: Signed> + N = NonZeroInteger<N>
//! - NonZeroInteger<N: Signed> + M = NonZeroInteger<N> (where M < N)
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
use core::ops::{Add, Sub};

// NonZeroInteger<N: Signed> + *integer*<N> = NonZeroInteger<N>
// -----------------------------------------------------------------------------

macro_rules! impl_add_n0z_integer {
    (all: $($rhs:ident),+) => {
        $( impl_add_n0z_integer![$rhs]; )+
    };
    ($rhs:ident) => {
        impl<N: Number + Signed + Add<Output = N>> Add<$rhs<N>> for NonZeroInteger<N> {
            type Output = NonZeroInteger<N>;
            fn add(self, other: $rhs<N>) -> Self::Output {
                Self::Output::new(self.0.add(other.0))
            }
        }
    };
}

#[rustfmt::skip]
impl_add_n0z_integer![all:
    Integer, NegativeInteger, PositiveInteger,
    NonNegativeInteger, NonPositiveInteger, NonZeroInteger];

#[cfg(test)]
mod test_impl_add_n0z_integer {
    use crate::{integer::a::*, traits::Number};

    #[test]
    fn impl_add_n0z_rhs() {
        assert_eq![N0z::new(7), N0z::new(4) + Z::new(3)];
        assert_eq![N0z::new(7), N0z::new(4) + Pz::new(3)];
        assert_eq![N0z::new(7), N0z::new(4) + Nnz::new(3)];
        assert_eq![N0z::new(1), N0z::new(4) + Npz::new(-3)];
        assert_eq![N0z::new(1), N0z::new(4) + Nz::new(-3)];
    }
}

// NonZeroInteger<N: Signed> + *integer*<M> = NonZeroInteger<N> (where M < N)
// -----------------------------------------------------------------------------

macro_rules! impl_add_n0z_smaller_integer {
    // signed + signed. E.g.:
    // NonZeroInteger<-10_i32> + PositiveInteger<7_i16>  = NonZeroInteger<-3_i32>
    (all_signed: $doc:literal, $( ($n1:ty, $n2:ty ) ),+) => {
        $( impl_add_n0z_smaller_integer![Integer, $doc, ($n1, $n2)]; )+
        $( impl_add_n0z_smaller_integer![NonZeroInteger, $doc, ($n1, $n2)]; )+
        $( impl_add_n0z_smaller_integer![NegativeInteger, $doc, ($n1, $n2)]; )+
        $( impl_add_n0z_smaller_integer![PositiveInteger, $doc, ($n1, $n2)]; )+
        $( impl_add_n0z_smaller_integer![NonNegativeInteger, $doc, ($n1, $n2)]; )+
        $( impl_add_n0z_smaller_integer![NonPositiveInteger, $doc, ($n1, $n2)]; )+
    };
    // signed + unsigned positive. E.g.:
    // NonZeroInteger<-10_i32> + PositiveInteger<7_u8>  = NonZeroInteger<-3_i32>
    // signed + unsigned negative. E.g.:
    // NonZeroInteger<-10_i32> + NegativeInteger<7_u8>  = NonZeroInteger<-17_i32>
    (all_unsigned: $doc:literal, $( ($n1:ty, $n2:ty ) ),+) => {
        $( impl_add_n0z_smaller_integer![PositiveInteger, $doc, ($n1, $n2)]; )+
        $( impl_add_n0z_smaller_integer![NonNegativeInteger, $doc, ($n1, $n2)]; )+

        $( impl_add_n0z_smaller_integer![neg: NonPositiveInteger, $doc, ($n1, $n2)]; )+
        $( impl_add_n0z_smaller_integer![neg: NegativeInteger, $doc, ($n1, $n2)]; )+
    };
    // signed + signed
    // signed + unsigned positive
    ($rhs:ident, $doc:literal, ($n1:ty, $n2:ty) ) => {
        paste::paste! {
            #[doc = "`Z<" $n1 "> + " $rhs "<" $n2 "> = Z<" $n1 ">`" $doc ]
            impl Add<$rhs<$n2>> for NonZeroInteger<$n1> {
                type Output = NonZeroInteger<$n1>;
                fn add(self, other: $rhs<$n2>) -> Self::Output {
                    Self::Output::new(self.0.add(other.0 as $n1))
                }
            }
        }
    };
    // signed + unsigned negative
    (neg: $rhs:ident, $doc:literal, ($n1:ty, $n2:ty) ) => {
        paste::paste! {
            #[doc = "`Z<" $n1 "> + " $rhs "<" $n2 "> = Z<" $n1 ">`" $doc ]
            impl Add<$rhs<$n2>> for NonZeroInteger<$n1> {
                type Output = NonZeroInteger<$n1>;
                fn add(self, other: $rhs<$n2>) -> Self::Output {
                    Self::Output::new(self.0.sub(other.0 as $n1))
                }
            }
        }
    };
}

#[rustfmt::skip]
impl_add_n0z_smaller_integer![all_signed: "",
    (i16, i8),
    (i32, i16), (i32, i8),
    (i64, i32), (i64, i16), (i64, i8),
    (i128, i64), (i128, i32), (i128, i16), (i128, i8) ];

#[rustfmt::skip]
impl_add_n0z_smaller_integer![all_unsigned: "",
    (i16, u8),
    (i32, u16), (i32, u8),
    (i64, u32), (i64, u16), (i64, u8),
    (i128, u64), (i128, u32), (i128, u16), (i128, u8) ];

/* pointers */

#[rustfmt::skip]
#[cfg(target_pointer_width = "128")]
impl_add_n0z_smaller_integer![all_signed:
    "\n\nAssumes `target_pointer_width = \"128\"`",
    (isize, i8), (isize, i16), (isize, i32), (isize, i64), (isize, i128),
    (i128, isize) ];

#[rustfmt::skip]
#[cfg(target_pointer_width = "64")]
impl_add_n0z_smaller_integer![all_signed:
    "\n\nAssumes `target_pointer_width = \"64\"`",
    (isize, i8), (isize, i16), (isize, i32), (isize, i64),
    (i64, isize), (i128, isize) ];

#[rustfmt::skip]
#[cfg(target_pointer_width = "32")]
impl_add_n0z_smaller_integer![all_signed:
    "\n\nAssumes `target_pointer_width = \"32\"`",
    (isize, i8), (isize, i16), (isize, i32),
    (i32, isize), (i64, isize), (i128, isize) ];

#[rustfmt::skip]
#[cfg(target_pointer_width = "16")]
impl_add_n0z_smaller_integer![all_signed:
    "\n\nAssumes `target_pointer_width = \"16\"`",
    (isize, i8), (isize, i16),
    (i16, isize), (i32, isize), (i64, isize), (i128, isize) ];

#[rustfmt::skip]
#[cfg(target_pointer_width = "8")]
impl_add_n0z_smaller_integer![all_signed:
    "\n\nAssumes `target_pointer_width = \"8\"`",
    (isize, i8),
    (i8, isize), (i16, isize), (i32, isize), (i64, isize), (i128, isize) ];

//

#[rustfmt::skip]
#[cfg(target_pointer_width = "128")]
impl_add_n0z_smaller_integer![all_unsigned:
    "\n\nAssumes `target_pointer_width = \"128\"`",
    (isize, u8), (isize, u16), (isize, u32), (isize, u64) ];

#[rustfmt::skip]
#[cfg(target_pointer_width = "64")]
impl_add_n0z_smaller_integer![all_unsigned:
    "\n\nAssumes `target_pointer_width = \"64\"`",
    (isize, u8), (isize, u16), (isize, u32),
    (i128, usize) ];

#[rustfmt::skip]
#[cfg(target_pointer_width = "32")]
impl_add_n0z_smaller_integer![all_unsigned:
    "\n\nAssumes `target_pointer_width = \"32\"`",
    (isize, u8), (isize, u16),
    (i64, usize), (i128, usize) ];

#[rustfmt::skip]
#[cfg(target_pointer_width = "16")]
impl_add_n0z_smaller_integer![all_unsigned:
    "\n\nAssumes `target_pointer_width = \"16\"`",
    (isize, u8),
    (i32, usize), (i64, usize), (i128, usize) ];

#[rustfmt::skip]
#[cfg(target_pointer_width = "8")]
impl_add_n0z_smaller_integer![all_unsigned:
    "\n\nAssumes `target_pointer_width = \"8\"`",
    (i16, usize), (i32, usize), (i64, usize), (i128, usize) ];

#[cfg(test)]
mod test_impl_add_n0z_smaller_integer {
    use crate::{
        integer::a::*,
        traits::{NegSigned, Number},
    };

    #[test]
    fn impl_add_n0z_rhs() {
        // signed
        assert_eq![N0z::new(7_i64), N0z::new(4_i64) + Z::new(3_i16)];
        assert_eq![N0z::new(1), N0z::new(4) + Npz::new(-3_i8)];
        // unsigned
        assert_eq![N0z::new(7_i64), N0z::new(4_i64) + Nnz::new(3_u16)];
        // negsigned
        assert_eq![N0z::new(1_i64), N0z::new(4_i64) + Npz::new_neg(3_u16)];

        /* pointers */

        // signed
        #[cfg(target_pointer_width = "64")]
        assert_eq![N0z::new(7_isize), N0z::new(4_isize) + N0z::new(3_i64)];
        // unsigned
        #[cfg(target_pointer_width = "64")]
        assert_eq![N0z::new(7_isize), N0z::new(4_isize) + Pz::new(3_u32)];
        // negsigned
        #[cfg(target_pointer_width = "64")]
        assert_eq![N0z::new(1_isize), N0z::new(4_isize) + Nz::new_neg(3_u32)];
    }
}

// NonZeroInteger<N: Signed> + N = Integer<N>
// -----------------------------------------------------------------------------

macro_rules! impl_add_same_prim {
    (all: $($n:ty),+) => {
        $( impl_add_same_prim![$n]; )+
    };
    ($n:ty) => {
        paste::paste! {
           #[doc = "`N0z<" $n "> + " $n " = N0z<" $n ">`" ]
            impl Add<$n> for NonZeroInteger<$n> {
                type Output = NonZeroInteger<$n>;
                fn add(self, other: $n) -> Self::Output {
                    Self::Output::new(self.0.add(other))
                }
            }
        }
    };
}
impl_add_same_prim![all: i8, i16, i32, i64, i128, isize];

#[cfg(test)]
mod test_add_same_prim {
    use crate::{integer::a::*, traits::Number};
    #[test]
    fn impl_add_same_prim() {
        assert_eq![N0z::new(7_i8), N0z::new(4_i8) + 3];
        assert_eq![N0z::new(7), N0z::new(4) + 3]; // i32
    }
}

// NonZeroInteger<N: Signed> + M (where M < N) = NonZeroInteger<N>
// -----------------------------------------------------------------------------

/// implements `Add` for:
/// - an integer + `< sized` primitive (both signed and unsigned).
/// - an integer + `< sized` pointer (unsigned).
/// - an integer + `<= sized` pointer (signed).
macro_rules! impl_add_smaller_prim {
    (all: $doc:literal, $( ($n1:ty, $n2:ty ) ),+) => {
        $( impl_add_smaller_prim![$doc, ( $n1, $n2 )]; )+
    };
    ($doc:literal, ($n1:ty, $n2:ty) ) => {
        paste::paste! {
            #[doc = "`N0z<" $n1 "> + " $n2 " = N0z<" $n1 ">`" $doc ]
            impl Add<$n2> for NonZeroInteger<$n1> {
                type Output = NonZeroInteger<$n1>;
                fn add(self, other: $n2) -> Self::Output {
                    Self::Output::new(self.0.add(other as $n1))
                }
            }
        }
    };
}

#[rustfmt::skip]
impl_add_smaller_prim![all: "",
    (i16, i8),
    (i32, i16), (i32, i8),
    (i64, i32), (i64, i16), (i64, i8),
    (i128, i64), (i128, i32), (i128, i16), (i128, i8),
    (i16, u8),
    (i32, u16), (i32, u8),
    (i64, u32), (i64, u16), (i64, u8),
    (i128, u64), (i128, u32), (i128, u16), (i128, u8) ];

#[rustfmt::skip]
#[cfg(target_pointer_width = "128")]
impl_add_smaller_prim![all:
    "\n\nAssumes `target_pointer_width = \"128\"`",
    (isize, i8), (isize, i16), (isize, i32), (isize, i64), (isize, i128),
    (i128, isize),
    (isize, u8), (isize, u16), (isize, u32), (isize, u64) ];

#[rustfmt::skip]
#[cfg(target_pointer_width = "64")]
impl_add_smaller_prim![all:
    "\n\nAssumes `target_pointer_width = \"64\"`",
    (isize, i8), (isize, i16), (isize, i32), (isize, i64),
    (i64, isize), (i128, isize),
    (isize, u8), (isize, u16), (isize, u32),
    (i128, usize) ];

#[rustfmt::skip]
#[cfg(target_pointer_width = "32")]
impl_add_smaller_prim![all:
    "\n\nAssumes `target_pointer_width = \"32\"`",
    (isize, i8), (isize, i16), (isize, i32),
    (i32, isize), (i64, isize), (i128, isize),
    (isize, u8), (isize, u16),
    (i64, usize), (i128, usize) ];

#[rustfmt::skip]
#[cfg(target_pointer_width = "16")]
impl_add_smaller_prim![all:
    "\n\nAssumes `target_pointer_width = \"16\"`",
    (isize, i8), (isize, i16),
    (i16, isize), (i32, isize), (i64, isize), (i128, isize),
    (isize, u8),
    (i32, usize), (i64, usize), (i128, usize) ];

#[rustfmt::skip]
#[cfg(target_pointer_width = "8")]
impl_add_smaller_prim![all:
    "\n\nAssumes `target_pointer_width = \"8\"`",
    (isize, i8),
    (i8, isize), (i16, isize), (i32, isize), (i64, isize), (i128, isize),
    (i16, usize), (i32, usize), (i64, usize), (i128, usize) ];

#[cfg(test)]
mod test_add_smaller_prim {
    use crate::{integer::a::*, traits::Number};

    #[test]
    fn impl_add_smaller_prim() {
        // signed
        assert_eq![N0z::new(7_i16), N0z::new(4_i16) + 3_i8];
        assert_eq![N0z::new(7), N0z::new(4) + 3_i8]; // N0z<i32> + i8
                                                     // unsigned
        assert_eq![N0z::new(7_i64), N0z::new(4_i64) + 3_u32];

        /* pointers */

        // signed
        #[cfg(target_pointer_width = "64")]
        assert_eq![N0z::new(7_isize), N0z::new(4_isize) + 3_i64];
        // unsigned
        #[cfg(target_pointer_width = "64")]
        assert_eq![N0z::new(7_isize), N0z::new(4_isize) + 3_u32];
    }
}

// ## Integer<IBig> + IBig = Integer<IBig>
// ## Integer<IBig> + *primitive* = Integer<IBig>
// -----------------------------------------------------------------------------

#[cfg(feature = "ibig")]
mod add_ibig {
    use super::*;
    use ibig::IBig;

    // NonZeroInteger<IBig> + IBig

    /// `N0z<IBig> + IBig = N0z<IBig>`
    impl Add<IBig> for NonZeroInteger<IBig> {
        type Output = NonZeroInteger<IBig>;
        fn add(self, other: IBig) -> Self::Output {
            Self::Output::new(self.0.add(other))
        }
    }

    // Integer<IBig> + *any integer primitive*

    /// implements `Add` for an IBig integer and a primitive.
    macro_rules! impl_add_ibig_prim {
        (all: $tn:ident, $( $n:ty ),+) => {
            $( impl_add_ibig_prim![$tn, $n]; )+
        };
        ($tn:ident, $n:ty) => {
            paste::paste! {
               #[doc = "`N0z<" $tn "> + " $n " = N0z<" $tn ">`" ]
                impl Add<$n> for NonZeroInteger<$tn> {
                    type Output = NonZeroInteger<$tn>;
                    fn add(self, other: $n) -> Self::Output {
                        Self::Output::new(self.0.add(other))
                    }
                }
            }
        };
    }

    #[rustfmt::skip]
    impl_add_ibig_prim![all: IBig,
        i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize ];

    #[cfg(test)]
    mod test_ibig {
        use super::IBig;
        use crate::{integer::a::*, traits::Number};

        #[test]
        fn ibig() {
            // N0z<IBig> + *integer*<IBig> = N0z<IBig>
            assert_eq![
                N0z::new(IBig::from(7)),
                N0z::new(IBig::from(4)) + Z::new(IBig::from(3))
            ];

            // N0z<IBig> + IBig = N0z<IBig>
            assert_eq![
                N0z::new(IBig::from(7)),
                N0z::new(IBig::from(4)) + IBig::from(3)
            ];

            // N0z<IBig> + *primitive* = N0z<IBig>
            assert_eq![
                N0z::new(IBig::from(7_i16)),
                N0z::new(IBig::from(4_i64)) + 3_u8
            ];
        }
    }
}
