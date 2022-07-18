// numera::integer::impl_ops::add::pz
//
//! Implements the `Add` operation on `NegativeInteger`.
//!
//! Completed:
//! - NegativeInteger<N> + *integer*<N> = NegativeInteger<N>
//! - NegativeInteger<N> + *integer*<M> = NegativeInteger<N> (where M < N)
//! - NegativeInteger<N> + N = NegativeInteger<N>
//! - NegativeInteger<N> + M = NegativeInteger<N> (where M < N)
//!
//! All panic if the result <= 0.
//

use crate::{
    integer::{
        Integer, NegativeInteger, NonNegativeInteger, NonPositiveInteger, NonZeroInteger,
        PositiveInteger,
    },
    traits::{Number, Signed},
};
use core::ops::{Add, Sub};

// NegativeInteger<N> + *integer*<N> = NegativeInteger<N>
// -----------------------------------------------------------------------------

macro_rules! impl_add_nz_integer {
    (all_signed: $($rhs:ident),+) => {
        $( impl_add_nz_integer![signed: $rhs]; )+
    };
    (signed: $rhs:ident) => {
        impl<N: Number + Signed + Add<Output = N>> Add<$rhs<N>> for NegativeInteger<N> {
            type Output = NegativeInteger<N>;
            fn add(self, other: $rhs<N>) -> Self::Output {
                Self::Output::new(self.0.add(other.0))
            }
        }
    };
}

#[rustfmt::skip]
impl_add_nz_integer![all_signed: Integer, NegativeInteger, PositiveInteger,
    NonNegativeInteger, NonPositiveInteger, NonZeroInteger ];

#[cfg(test)]
mod test_impl_add_nz_integer {
    use crate::{integer::a::*, traits::{NegSigned, Number}};

    #[test]
    fn impl_add_nz_rhs() {
        assert_eq![Nz::new(-7), Nz::new(-4) + Z::new(-3)];
        assert_eq![Nz::new(-7), Nz::new(-4) + Nz::new(-3)];
        assert_eq![Nz::new(-7), Nz::new(-4) + Npz::new(-3)];
        assert_eq![Nz::new(-1), Nz::new(-4) + Nnz::new(3)];
        assert_eq![Nz::new(-1), Nz::new(-4) + Pz::new(3)];
    }
}

// NegativeInteger<N> + *integer*<M> = NegativeInteger<N> (where M < N)
// -----------------------------------------------------------------------------
// Note: for unsigned numbers impl: M <= N

macro_rules! impl_add_nz_smaller_integer {
    // signed + signed. E.g.:
    // NegativeInteger<10_i32> + Integer<-7_i16>  = NegativeInteger<3_i32>
    (all_signed: $doc:literal, $( ($n1:ty, $n2:ty ) ),+) => {
        $( impl_add_nz_smaller_integer![Integer, $doc, ($n1, $n2)]; )+
        $( impl_add_nz_smaller_integer![NonZeroInteger, $doc, ($n1, $n2)]; )+
        $( impl_add_nz_smaller_integer![NegativeInteger, $doc, ($n1, $n2)]; )+
        $( impl_add_nz_smaller_integer![PositiveInteger, $doc, ($n1, $n2)]; )+
        $( impl_add_nz_smaller_integer![NonNegativeInteger, $doc, ($n1, $n2)]; )+
        $( impl_add_nz_smaller_integer![NonPositiveInteger, $doc, ($n1, $n2)]; )+
    };
    // (un)signed + unsigned positive. E.g.:
    // NegativeInteger<10_i32> + NegativeInteger<7_u16>  = Integer<17_i32>
    // NegativeInteger<10_u32> + NegativeInteger<7_u32>  = Integer<17_u32>
    // //
    // (un)signed + unsigned negative. E.g.:
    // NegativeInteger<10_i32> + NegativeInteger<7_u16> = NegativeInteger<3_i32>
    // NegativeInteger<10_u32> + NegativeInteger<7_u32> = NegativeInteger<3_u32>
    (all_unsigned: $doc:literal, $( ($n1:ty, $n2:ty ) ),+) => {
        // FIXME! (revert?) CHECK

        $( impl_add_nz_smaller_integer![neg: PositiveInteger, $doc, ($n1, $n2)]; )+
        $( impl_add_nz_smaller_integer![neg: NonNegativeInteger, $doc, ($n1, $n2)]; )+

        $( impl_add_nz_smaller_integer![NonPositiveInteger, $doc, ($n1, $n2)]; )+
        $( impl_add_nz_smaller_integer![NegativeInteger, $doc, ($n1, $n2)]; )+
    };

    // signed + signed
    // signed + unsigned positive
    // unsigned + unsigned positive
    ($rhs:ident, $doc:literal, ($n1:ty, $n2:ty) ) => {
        paste::paste! {
            #[doc = "`Z<" $n1 "> + " $rhs "<" $n2 "> = Z<" $n1 ">`" $doc ]
            impl Add<$rhs<$n2>> for NegativeInteger<$n1> {
                type Output = NegativeInteger<$n1>;
                fn add(self, other: $rhs<$n2>) -> Self::Output {
                    Self::Output::new(self.0.add(other.0 as $n1))
                }
            }
        }
    };
    // signed + unsigned negative
    // unsigned + unsigned negative
    (neg: $rhs:ident, $doc:literal, ($n1:ty, $n2:ty) ) => {
        paste::paste! {
            #[doc = "`Z<" $n1 "> + " $rhs "<" $n2 "> = Z<" $n1 ">`" $doc ]
            impl Add<$rhs<$n2>> for NegativeInteger<$n1> {
                type Output = NegativeInteger<$n1>;
                fn add(self, other: $rhs<$n2>) -> Self::Output {
                    Self::Output::new(self.0.sub(other.0 as $n1))
                }
            }
        }
    };
}

#[rustfmt::skip]
impl_add_nz_smaller_integer![all_signed: "",
    (i16, i8),
    (i32, i16), (i32, i8),
    (i64, i32), (i64, i16), (i64, i8),
    (i128, i64), (i128, i32), (i128, i16), (i128, i8) ];

#[rustfmt::skip]
impl_add_nz_smaller_integer![all_unsigned: "",
    (i16, u8),
    (i32, u16), (i32, u8),
    (i64, u32), (i64, u16), (i64, u8),
    (i128, u64), (i128, u32), (i128, u16), (i128, u8),
    (u16, u16), (u16, u8),
    (u32, u32), (u32, u16), (u32, u8),
    (u64, u64), (u64, u32), (u64, u16), (u64, u8),
    (u128, u128), (u128, u64), (u128, u32), (u128, u16), (u128, u8) ];

/* pointers */

#[rustfmt::skip]
#[cfg(target_pointer_width = "128")]
impl_add_nz_smaller_integer![all_signed:
    "\n\nAssumes `target_pointer_width = \"128\"`",
    (isize, i8), (isize, i16), (isize, i32), (isize, i64), (isize, i128),
    (i128, isize) ];

#[rustfmt::skip]
#[cfg(target_pointer_width = "64")]
impl_add_nz_smaller_integer![all_signed:
    "\n\nAssumes `target_pointer_width = \"64\"`",
    (isize, i8), (isize, i16), (isize, i32), (isize, i64),
    (i64, isize), (i128, isize) ];

#[rustfmt::skip]
#[cfg(target_pointer_width = "32")]
impl_add_nz_smaller_integer![all_signed:
    "\n\nAssumes `target_pointer_width = \"32\"`",
    (isize, i8), (isize, i16), (isize, i32),
    (i32, isize), (i64, isize), (i128, isize) ];

#[rustfmt::skip]
#[cfg(target_pointer_width = "16")]
impl_add_nz_smaller_integer![all_signed:
    "\n\nAssumes `target_pointer_width = \"16\"`",
    (isize, i8), (isize, i16),
    (i16, isize), (i32, isize), (i64, isize), (i128, isize) ];

#[rustfmt::skip]
#[cfg(target_pointer_width = "8")]
impl_add_nz_smaller_integer![all_signed:
    "\n\nAssumes `target_pointer_width = \"8\"`",
    (isize, i8),
    (i8, isize), (i16, isize), (i32, isize), (i64, isize), (i128, isize) ];

//

#[rustfmt::skip]
#[cfg(target_pointer_width = "128")]
impl_add_nz_smaller_integer![all_unsigned:
    "\n\nAssumes `target_pointer_width = \"128\"`",
    (isize, u8), (isize, u16), (isize, u32), (isize, u64),
    (usize, u8), (usize, u16), (usize, u32), (usize, u64), (usize, u128), (usize, usize),
    (u128, usize) ];

#[rustfmt::skip]
#[cfg(target_pointer_width = "64")]
impl_add_nz_smaller_integer![all_unsigned:
    "\n\nAssumes `target_pointer_width = \"64\"`",
    (isize, u8), (isize, u16), (isize, u32),
    (i128, usize),
    (usize, u8), (usize, u16), (usize, u32), (usize, u64), (usize, usize),
    (u64, usize), (u128, usize) ];

#[rustfmt::skip]
#[cfg(target_pointer_width = "32")]
impl_add_nz_smaller_integer![all_unsigned:
    "\n\nAssumes `target_pointer_width = \"32\"`",
    (isize, u8), (isize, u16),
    (i64, usize), (i128, usize),
    (usize, u8), (usize, u16), (usize, u32), (usize, usize),
    (u32, usize), (u64, usize), (u128, usize) ];

#[rustfmt::skip]
#[cfg(target_pointer_width = "16")]
impl_add_nz_smaller_integer![all_unsigned:
    "\n\nAssumes `target_pointer_width = \"16\"`",
    (isize, u8),
    (i32, usize), (i64, usize), (i128, usize),
    (usize, u8), (usize, u16), (usize, usize),
    (u16, usize), (u32, usize), (u64, usize), (u128, usize) ];

#[rustfmt::skip]
#[cfg(target_pointer_width = "8")]
impl_add_nz_smaller_integer![all_unsigned:
    "\n\nAssumes `target_pointer_width = \"8\"`",
    (i16, usize), (i32, usize), (i64, usize), (i128, usize),
    (usize, u8), (usize, usize),
    (u8, usize), (u16, usize), (u32, usize), (u64, usize), (u128, usize) ];

#[cfg(test)]
mod test_impl_add_nz_smaller_integer {
    use crate::{
        integer::a::*,
        traits::{NegSigned, Number},
    };
    #[test]
    fn impl_add_nz_rhs() {
        #[cfg(feature = "std")]
        use std::panic::catch_unwind;

        #[cfg(feature = "std")]
        assert![catch_unwind(|| { Nz::new(3) + N0z::new(-4) }).is_err()];

        // signed
        assert_eq![Nz::new(-7_i64), Nz::new(-4_i64) + Z::new(-3_i16)];
        assert_eq![Nz::new(-1), Nz::new(-4) + Nnz::new(3_i8)];
        // unsigned
        // assert_eq![Nz::new(-7_i64), Nz::new(-4_i64) + Npz::new_neg(3_u16)]; // FIXME
        // assert_eq![Nz::new_neg(7_u64), Nz::new_neg(4_u64) + Nnz::new(3_u16)];
        // negsigned
        // assert_eq![Nz::new(-1_i64), Nz::new(-4_i64) + Npz::new_neg(3_u16)];

        /* pointers */

        // // signed
        // #[cfg(target_pointer_width = "64")]
        // assert_eq![Nz::new(7_isize), Nz::new(4_isize) + Z::new(3_i64)];
        // // unsigned
        // #[cfg(target_pointer_width = "64")]
        // assert_eq![Nz::new(7_isize), Nz::new(4_isize) + Nnz::new(3_u32)];
        // // negsigned
        // #[cfg(target_pointer_width = "64")]
        // assert_eq![Nz::new(1_isize), Nz::new(4_isize) + Nz::new_neg(3_u32)];
    }
}

// NegativeInteger<N> + N = Integer<N>
// -----------------------------------------------------------------------------

macro_rules! impl_add_same_prim {
    (all: $($n:ty),+) => {
        $( impl_add_same_prim![$n]; )+
    };
    ($n:ty) => {
        paste::paste! {
           #[doc = "`Nz<" $n "> + " $n " = Nz<" $n ">`" ]
            impl Add<$n> for NegativeInteger<$n> {
                type Output = NegativeInteger<$n>;
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
        assert_eq![Nz::new(7_i8), Nz::new(4_i8) + 3];
        assert_eq![Nz::new(7), Nz::new(4) + 3]; // i32
    }
}

// NegativeInteger<N> + M (where M < N) = NegativeInteger<N>
// -----------------------------------------------------------------------------

/// implements `Add` for an integer and a `< sized` primitive of the same sign,
/// and also from a integer and a `<= sized` pointer.
macro_rules! impl_add_smaller_prim {
    (all: $doc:literal, $( ($n1:ty, $n2:ty ) ),+) => {
        $( impl_add_smaller_prim![$doc, ( $n1, $n2 )]; )+
    };
    ($doc:literal, ($n1:ty, $n2:ty) ) => {
        paste::paste! {
            #[doc = "`Nz<" $n1 "> + " $n2 " = Nz<" $n1 ">`" $doc ]
            impl Add<$n2> for NegativeInteger<$n1> {
                type Output = NegativeInteger<$n1>;
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
    (i128, i64), (i128, i32), (i128, i16), (i128, i8) ];

#[rustfmt::skip]
#[cfg(target_pointer_width = "128")]
impl_add_smaller_prim![all: 
    "\n\nAssumes `target_pointer_width = \"128\"`",
    (isize, i8), (isize, i16), (isize, i32), (isize, i64), (isize, i128),
    (i128, isize) ];

#[rustfmt::skip]
#[cfg(target_pointer_width = "64")]
impl_add_smaller_prim![all:
    "\n\nAssumes `target_pointer_width = \"64\"`",
    (isize, i8), (isize, i16), (isize, i32), (isize, i64),
    (i64, isize), (i128, isize) ];

#[rustfmt::skip]
#[cfg(target_pointer_width = "32")]
impl_add_smaller_prim![all:
    "\n\nAssumes `target_pointer_width = \"32\"`",
    (isize, i8), (isize, i16), (isize, i32),
    (i32, isize), (i64, isize), (i128, isize) ];

#[rustfmt::skip]
#[cfg(target_pointer_width = "16")]
impl_add_smaller_prim![all:
    "\n\nAssumes `target_pointer_width = \"16\"`",
    (isize, i8), (isize, i16),
    (i16, isize), (i32, isize), (i64, isize), (i128, isize) ];

#[rustfmt::skip]
#[cfg(target_pointer_width = "8")]
impl_add_smaller_prim![all:
    "\n\nAssumes `target_pointer_width = \"8\"`",
    (isize, i8),
    (i8, isize), (i16, isize), (i32, isize), (i64, isize), (i128, isize) ];

#[cfg(test)]
mod test_add_smaller_prim {
    use crate::{integer::a::*, traits::Number};

    #[test]
    fn impl_add_smaller_prim() {
        assert_eq![Nz::new(7_i16), Nz::new(4_i16) + 3_i8];
        assert_eq![Nz::new(7), Nz::new(4) + 3_i8]; // Nz<i32> + i8

        #[cfg(target_pointer_width = "64")]
        assert_eq![Nz::new(7_isize), Nz::new(4_isize) + 3_i64];
    }
}

// ## Integer<IBig> + IBig = Integer<IBig>
// ## Integer<IBig> + *primitive* = Integer<IBig>
// -----------------------------------------------------------------------------

#[cfg(feature = "ibig")]
mod add_ibig {
    use super::*;
    use ibig::IBig;

    // NegativeInteger<IBig> + IBig

    /// `Nz<IBig> + IBig = Nz<IBig>`
    impl Add<IBig> for NegativeInteger<IBig> {
        type Output = NegativeInteger<IBig>;
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
               #[doc = "`Nz<" $tn "> + " $n " = Nz<" $tn ">`" ]
                impl Add<$n> for NegativeInteger<$tn> {
                    type Output = NegativeInteger<$tn>;
                    fn add(self, other: $n) -> Self::Output {
                        Self::Output::new(self.0.add(other))
                    }
                }
            }
        };
    }

    #[rustfmt::skip]
    impl_add_ibig_prim![all: IBig,
        i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize];

    #[cfg(test)]
    mod test_ibig {
        use super::IBig;
        use crate::{integer::a::*, traits::Number};

        #[test]
        fn ibig() {
            // Nz<IBig> + *integer*<IBig> = Nz<IBig>
            assert_eq![
                Nz::new(IBig::from(7)),
                Nz::new(IBig::from(4)) + Z::new(IBig::from(3))
            ];

            // Nz<IBig> + IBig = Nz<IBig>
            assert_eq![
                Nz::new(IBig::from(7)),
                Nz::new(IBig::from(4)) + IBig::from(3)
            ];

            // Nz<IBig> + *primitive* = Nz<IBig>
            assert_eq![
                Nz::new(IBig::from(7_i16)),
                Nz::new(IBig::from(4_i64)) + 3_u8
            ];
        }
    }
}
