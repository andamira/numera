// numera::integer::impl_ops::sub::pz
//
//! Implements the `Sub` operation on `PositiveInteger`.
//!
//! Completed:
//! - PositiveInteger<N> - *integer*<N> = PositiveInteger<N>
//! - PositiveInteger<N> - *integer*<M> = PositiveInteger<N> (where M < N)
//! - PositiveInteger<N> - N = PositiveInteger<N>
//! - PositiveInteger<N> - M = PositiveInteger<N> (where M < N)
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
use core::ops::Sub;

// PositiveInteger<N> - *integer*<N> = PositiveInteger<N>
// -----------------------------------------------------------------------------

macro_rules! impl_sub_pz_integer {
    (all: $($rhs:ident),+) => {
        $( impl_sub_pz_integer![$rhs]; )+
    };
    ($rhs:ident) => {
        impl<N: Number + Sub<Output = N>> Sub<$rhs<N>> for PositiveInteger<N> {
            type Output = PositiveInteger<N>;
            fn sub(self, other: $rhs<N>) -> Self::Output {
                Self::Output::new(self.0.sub(other.0))
            }
        }
    };
    (all_signed: $($rhs:ident),+) => {
        $( impl_sub_pz_integer![signed: $rhs]; )+
    };
    (signed: $rhs:ident) => {
        impl<N: Number + Signed + Sub<Output = N>> Sub<$rhs<N>> for PositiveInteger<N> {
            type Output = PositiveInteger<N>;
            fn sub(self, other: $rhs<N>) -> Self::Output {
                Self::Output::new(self.0.sub(other.0))
            }
        }
    };
}

#[rustfmt::skip]
impl_sub_pz_integer![all:
    NegativeInteger, PositiveInteger, NonNegativeInteger, NonPositiveInteger];

impl_sub_pz_integer![all_signed: Integer, NonZeroInteger];

#[cfg(test)]
mod test_impl_sub_pz_integer {
    use crate::{integer::a::*, traits::Number};

    #[test]
    fn impl_sub_pz_rhs() {
        assert_eq![Pz::new(1), Pz::new(4) - Z::new(3)];
        assert_eq![Pz::new(1), Pz::new(4) - Pz::new(3)];
        assert_eq![Pz::new(1), Pz::new(4) - Nnz::new(3)];
        assert_eq![Pz::new(7), Pz::new(4) - Npz::new(-3)];
        assert_eq![Pz::new(7), Pz::new(4) - Nz::new(-3)];
    }
}

// PositiveInteger<N> - *integer*<M> = PositiveInteger<N> (where M < N)
// -----------------------------------------------------------------------------

macro_rules! impl_sub_pz_smaller_integer {
    (for_all_integers: $doc:literal, $( ($n1:ty, $n2:ty ) ),+) => {
        $( impl_sub_pz_smaller_integer![Integer, $doc, ($n1, $n2)]; )+
        $( impl_sub_pz_smaller_integer![NonZeroInteger, $doc, ($n1, $n2)]; )+
        $( impl_sub_pz_smaller_integer![NegativeInteger, $doc, ($n1, $n2)]; )+
        $( impl_sub_pz_smaller_integer![PositiveInteger, $doc, ($n1, $n2)]; )+
        $( impl_sub_pz_smaller_integer![NonNegativeInteger, $doc, ($n1, $n2)]; )+
        $( impl_sub_pz_smaller_integer![NonPositiveInteger, $doc, ($n1, $n2)]; )+
    };
    ($rhs:ident, $doc:literal, ($n1:ty, $n2:ty) ) => {
        paste::paste! {
            #[doc = "`Pz<" $n1 "> - " $rhs "<" $n2 "> = Pz<" $n1 ">`" $doc ]
            impl Sub<$rhs<$n2>> for PositiveInteger<$n1> {
                type Output = PositiveInteger<$n1>;
                fn sub(self, other: $rhs<$n2>) -> Self::Output {
                    Self::Output::new(self.0.sub(other.0 as $n1))
                }
            }
        }
    };
}

#[rustfmt::skip]
impl_sub_pz_smaller_integer![for_all_integers: "",
    (i16, i8),
    (i32, i16), (i32, i8),
    (i64, i32), (i64, i16), (i64, i8),
    (i128, i64), (i128, i32), (i128, i16), (i128, i8) ];

#[rustfmt::skip]
#[cfg(target_pointer_width = "128")]
impl_sub_pz_smaller_integer![for_all_integers:
    "\n\nAssumes `target_pointer_width = \"128\"`",
    (isize, i8), (isize, i16), (isize, i32), (isize, i64), (isize, i128),
    (i128, isize) ];

#[rustfmt::skip]
#[cfg(target_pointer_width = "64")]
impl_sub_pz_smaller_integer![for_all_integers:
    "\n\nAssumes `target_pointer_width = \"64\"`",
    (isize, i8), (isize, i16), (isize, i32), (isize, i64),
    (i64, isize), (i128, isize) ];

#[rustfmt::skip]
#[cfg(target_pointer_width = "32")]
impl_sub_pz_smaller_integer![for_all_integers:
    "\n\nAssumes `target_pointer_width = \"32\"`",
    (isize, i8), (isize, i16), (isize, i32),
    (i32, isize), (i64, isize), (i128, isize) ];

#[rustfmt::skip]
#[cfg(target_pointer_width = "16")]
impl_sub_pz_smaller_integer![for_all_integers:
    "\n\nAssumes `target_pointer_width = \"16\"`",
    (isize, i8), (isize, i16),
    (i16, isize), (i32, isize), (i64, isize), (i128, isize) ];

#[rustfmt::skip]
#[cfg(target_pointer_width = "8")]
impl_sub_pz_smaller_integer![for_all_integers:
    "\n\nAssumes `target_pointer_width = \"8\"`",
    (isize, i8),
    (i8, isize), (i16, isize), (i32, isize), (i64, isize), (i128, isize) ];

#[cfg(test)]
mod test_impl_sub_pz_smaller_integer {
    use crate::{integer::a::*, traits::Number};
    #[test]
    fn impl_sub_pz_rhs() {
        #[cfg(feature = "std")]
        use std::panic::catch_unwind;

        assert_eq![Pz::new(1_i64), Pz::new(4_i64) - Z::new(3_i16)];
        assert_eq![Pz::new(7), Pz::new(4) - Npz::new(-3_i8)];

        #[cfg(target_pointer_width = "64")]
        assert_eq![Pz::new(1_isize), Pz::new(4_isize) - Pz::new(3_i64)];

        #[cfg(feature = "std")]
        assert![catch_unwind(|| { Pz::new(3) - N0z::new(4) }).is_err()];
    }
}

// PositiveInteger<N> - N = Integer<N>
// -----------------------------------------------------------------------------

macro_rules! impl_sub_same_prim {
    (all: $($n:ty),+) => {
        $( impl_sub_same_prim![$n]; )+
    };
    ($n:ty) => {
        paste::paste! {
           #[doc = "`Pz<" $n "> - " $n " = Pz<" $n ">`" ]
            impl Sub<$n> for PositiveInteger<$n> {
                type Output = PositiveInteger<$n>;
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
        assert_eq![Pz::new(1_i8), Pz::new(4_i8) - 3];
        assert_eq![Pz::new(1), Pz::new(4) - 3]; // i32
    }
}

// PositiveInteger<N> - M (where M < N) = PositiveInteger<N>
// -----------------------------------------------------------------------------

/// implements `Sub` for an integer and a `< sized` primitive of the same sign,
/// and also from a integer and a `<= sized` pointer.
macro_rules! impl_sub_smaller_prim {
    (all: $doc:literal, $( ($n1:ty, $n2:ty ) ),+) => {
        $( impl_sub_smaller_prim![$doc, ( $n1, $n2 )]; )+
    };
    ($doc:literal, ($n1:ty, $n2:ty) ) => {
        paste::paste! {
            #[doc = "`Pz<" $n1 "> - " $n2 " = Pz<" $n1 ">`" $doc ]
            impl Sub<$n2> for PositiveInteger<$n1> {
                type Output = PositiveInteger<$n1>;
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
        assert_eq![Pz::new(1_i16), Pz::new(4_i16) - 3_i8];
        assert_eq![Pz::new(1), Pz::new(4) - 3_i8]; // Pz<i32> - i8

        #[cfg(target_pointer_width = "64")]
        assert_eq![Pz::new(1_isize), Pz::new(4_isize) - 3_i64];
    }
}

// ## Integer<IBig> - IBig = Integer<IBig>
// ## Integer<IBig> - *primitive* = Integer<IBig>
// -----------------------------------------------------------------------------

#[cfg(feature = "ibig")]
mod sub_ibig {
    use super::*;
    use ibig::IBig;

    // PositiveInteger<IBig> - IBig

    /// `Pz<IBig> - IBig = Pz<IBig>`
    impl Sub<IBig> for PositiveInteger<IBig> {
        type Output = PositiveInteger<IBig>;
        fn sub(self, other: IBig) -> Self::Output {
            Self::Output::new(self.0.sub(other))
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
               #[doc = "`Pz<" $tn "> - " $n " = Pz<" $tn ">`" ]
                impl Sub<$n> for PositiveInteger<$tn> {
                    type Output = PositiveInteger<$tn>;
                    fn sub(self, other: $n) -> Self::Output {
                        Self::Output::new(self.0.sub(other))
                    }
                }
            }
        };
    }

    #[rustfmt::skip]
    impl_sub_ibig_prim![all: IBig,
        i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize ];

    #[cfg(test)]
    mod test_ibig {
        use super::IBig;
        use crate::{integer::a::*, traits::Number};

        #[test]
        fn ibig() {
            // Pz<IBig> - *integer*<IBig> = Pz<IBig>
            assert_eq![
                Pz::new(IBig::from(1)),
                Pz::new(IBig::from(4)) - Z::new(IBig::from(3))
            ];

            // Pz<IBig> - IBig = Pz<IBig>
            assert_eq![
                Pz::new(IBig::from(1)),
                Pz::new(IBig::from(4)) - IBig::from(3)
            ];

            // Pz<IBig> - *primitive* = Pz<IBig>
            assert_eq![
                Pz::new(IBig::from(1_i16)),
                Pz::new(IBig::from(4_i64)) - 3_u8
            ];
        }
    }
}
