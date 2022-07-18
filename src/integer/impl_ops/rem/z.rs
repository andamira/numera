// numera::integer::impl_ops::rem::z
//
//! Implements the `Rem` operation on `Integer`.
//!
//! Completed:
//! - Integer<N: Signed> % *integer*<N> = Integer<N>
//! - Integer<N: Signed> % *integer*<M> = Integer<N> (where M < N)
//! - Integer<N: Signed> % N = Integer<N>
//! - Integer<N: Signed> % M = Integer<N> (where M < N)
//

use crate::{
    integer::{
        Integer, NegativeInteger, NonNegativeInteger, NonPositiveInteger, NonZeroInteger,
        PositiveInteger,
    },
    traits::{Number, Signed},
};
use core::ops::Rem;

// Integer<N: Signed> % *integer*<N> = Integer<N>
// -----------------------------------------------------------------------------

macro_rules! impl_rem_z_integer {
    (all: $($rhs:ident),+) => {
        $( impl_rem_z_integer![$rhs]; )+
    };
    ($rhs:ident) => {
        impl<N: Number + Signed + Rem<Output = N>> Rem<$rhs<N>> for Integer<N> {
            type Output = Integer<N>;
            fn rem(self, other: $rhs<N>) -> Self::Output {
                Self::Output::new(self.0.rem(other.0))
            }
        }
    };
}

#[rustfmt::skip]
impl_rem_z_integer![all:
    Integer, NegativeInteger, PositiveInteger,
    NonNegativeInteger, NonPositiveInteger, NonZeroInteger];

#[cfg(test)]
mod test_impl_rem_z_integer {
    use crate::{integer::a::*, traits::Number};
    #[test]
    fn impl_rem_z_rhs() {
        assert_eq![Z::new(0), Z::new(3) % N0z::new(3)];
        assert_eq![Z::new(1), Z::new(4) % Pz::new(3)];
        assert_eq![Z::new(2), Z::new(5) % Nnz::new(3)];
        assert_eq![Z::new(0), Z::new(6) % Npz::new(-3)];
        assert_eq![Z::new(1), Z::new(7) % Nz::new(-3)];
    }
}

// Integer<N: Signed> % *integer*<M> = Integer<N> (where M < N)
// -----------------------------------------------------------------------------

macro_rules! impl_rem_z_smaller_integer {
    (for_all_integers: $doc:literal, $( ($n1:ty, $n2:ty ) ),+) => {
        $( impl_rem_z_smaller_integer![Integer, $doc, ($n1, $n2)]; )+
        $( impl_rem_z_smaller_integer![NonZeroInteger, $doc, ($n1, $n2)]; )+
        $( impl_rem_z_smaller_integer![NegativeInteger, $doc, ($n1, $n2)]; )+
        $( impl_rem_z_smaller_integer![PositiveInteger, $doc, ($n1, $n2)]; )+
        $( impl_rem_z_smaller_integer![NonNegativeInteger, $doc, ($n1, $n2)]; )+
        $( impl_rem_z_smaller_integer![NonPositiveInteger, $doc, ($n1, $n2)]; )+
    };
    ($rhs:ident, $doc:literal, ($n1:ty, $n2:ty) ) => {
        paste::paste! {
            #[doc = "`Z<" $n1 "> % " $rhs "<" $n2 "> = Z<" $n1 ">`" $doc ]
            impl Rem<$rhs<$n2>> for Integer<$n1> {
                type Output = Integer<$n1>;
                fn rem(self, other: $rhs<$n2>) -> Self::Output {
                    Self::Output::new(self.0.rem(other.0 as $n1))
                }
            }
        }
    };
}

#[rustfmt::skip]
impl_rem_z_smaller_integer![for_all_integers: "",
    (i16, i8),
    (i32, i16), (i32, i8),
    (i64, i32), (i64, i16), (i64, i8),
    (i128, i64), (i128, i32), (i128, i16), (i128, i8) ];

#[rustfmt::skip]
#[cfg(target_pointer_width = "128")]
impl_rem_z_smaller_integer![for_all_integers:
    "\n\nAssumes `target_pointer_width = \"128\"`",
    (isize, i8), (isize, i16), (isize, i32), (isize, i64), (isize, i128),
    (i128, isize) ];

#[rustfmt::skip]
#[cfg(target_pointer_width = "64")]
impl_rem_z_smaller_integer![for_all_integers:
    "\n\nAssumes `target_pointer_width = \"64\"`",
    (isize, i8), (isize, i16), (isize, i32), (isize, i64),
    (i64, isize), (i128, isize) ];

#[rustfmt::skip]
#[cfg(target_pointer_width = "32")]
impl_rem_z_smaller_integer![for_all_integers:
    "\n\nAssumes `target_pointer_width = \"32\"`",
    (isize, i8), (isize, i16), (isize, i32),
    (i32, isize), (i64, isize), (i128, isize) ];

#[rustfmt::skip]
#[cfg(target_pointer_width = "16")]
impl_rem_z_smaller_integer![for_all_integers:
    "\n\nAssumes `target_pointer_width = \"16\"`",
    (isize, i8), (isize, i16),
    (i16, isize), (i32, isize), (i64, isize), (i128, isize) ];

#[rustfmt::skip]
#[cfg(target_pointer_width = "8")]
impl_rem_z_smaller_integer![for_all_integers:
    "\n\nAssumes `target_pointer_width = \"8\"`",
    (isize, i8),
    (i8, isize), (i16, isize), (i32, isize), (i64, isize), (i128, isize) ];

#[cfg(test)]
mod test_impl_rem_z_smaller_integer {
    use crate::{integer::a::*, traits::Number};
    #[test]
    fn impl_rem_z_rhs() {
        assert_eq![Z::new(1_i64), Z::new(4_i64) % N0z::new(3_i16)];
        assert_eq![Z::new(1), Z::new(4) % Npz::new(-3_i8)];

        #[cfg(target_pointer_width = "64")]
        assert_eq![Z::new(1_isize), Z::new(4_isize) % Z::new(3_i64)];
    }
}

// Integer<N: Signed> % N = Integer<N>
// -----------------------------------------------------------------------------

macro_rules! impl_rem_same_prim {
    (all: $t:ident, $($n:ty),+) => {
        $( impl_rem_same_prim![$t, $n]; )+
    };
    ($t:ident, $n:ty) => {
        paste::paste! {
           #[doc = "`Z<" $n "> % " $n " = Z<" $n ">`" ]
            impl Rem<$n> for $t<$n> {
                type Output = $t<$n>;
                fn rem(self, other: $n) -> Self::Output {
                    Self::Output::new(self.0.rem(other))
                }
            }
        }
    };
}
impl_rem_same_prim![all: Integer, i8, i16, i32, i64, i128, isize];

#[cfg(test)]
mod test_rem_same_prim {
    use crate::{integer::a::*, traits::Number};
    #[test]
    fn impl_rem_same_prim() {
        assert_eq![Z::new(1_i8), Z::new(4_i8) % 3];
        assert_eq![Z::new(1), Z::new(4) % 3]; // i32
    }
}

// Integer<N: Signed> % M (where M < N) = Integer<N>
// -----------------------------------------------------------------------------

/// implements `Rem` for an integer and a `< sized` primitive of the same sign,
/// and also from a integer and a `<= sized` pointer.
macro_rules! impl_rem_smaller_prim {
    (all: $doc:literal, $( ($n1:ty, $n2:ty ) ),+) => {
        $( impl_rem_smaller_prim![$doc, ( $n1, $n2 )]; )+
    };
    ($doc:literal, ($n1:ty, $n2:ty) ) => {
        paste::paste! {
            #[doc = "`Z<" $n1 "> % " $n2 " = Z<" $n1 ">`" $doc ]
            impl Rem<$n2> for Integer<$n1> {
                type Output = Integer<$n1>;
                fn rem(self, other: $n2) -> Self::Output {
                    Self::Output::new(self.0.rem(other as $n1))
                }
            }
        }
    };
}

#[rustfmt::skip]
impl_rem_smaller_prim![all: "",
    (i16, i8),
    (i32, i16), (i32, i8),
    (i64, i32), (i64, i16), (i64, i8),
    (i128, i64), (i128, i32), (i128, i16), (i128, i8) ];

#[rustfmt::skip]
#[cfg(target_pointer_width = "128")]
impl_rem_smaller_prim![all:
    "\n\nAssumes `target_pointer_width = \"128\"`",
    (isize, i8), (isize, i16), (isize, i32), (isize, i64), (isize, i128),
    (i128, isize) ];

#[rustfmt::skip]
#[cfg(target_pointer_width = "64")]
impl_rem_smaller_prim![all:
    "\n\nAssumes `target_pointer_width = \"64\"`",
    (isize, i8), (isize, i16), (isize, i32), (isize, i64),
    (i64, isize), (i128, isize) ];

#[rustfmt::skip]
#[cfg(target_pointer_width = "32")]
impl_rem_smaller_prim![all:
    "\n\nAssumes `target_pointer_width = \"32\"`",
    (isize, i8), (isize, i16), (isize, i32),
    (i32, isize), (i64, isize), (i128, isize) ];

#[rustfmt::skip]
#[cfg(target_pointer_width = "16")]
impl_rem_smaller_prim![all:
    "\n\nAssumes `target_pointer_width = \"16\"`",
    (isize, i8), (isize, i16),
    (i16, isize), (i32, isize), (i64, isize), (i128, isize) ];

#[rustfmt::skip]
#[cfg(target_pointer_width = "8")]
impl_rem_smaller_prim![all:
    "\n\nAssumes `target_pointer_width = \"8\"`",
    (isize, i8),
    (i8, isize), (i16, isize), (i32, isize), (i64, isize), (i128, isize) ];

#[cfg(test)]
mod test_rem_smaller_prim {
    use crate::{integer::a::*, traits::Number};

    #[test]
    fn impl_rem_smaller_prim() {
        assert_eq![Z::new(1_i16), Z::new(4_i16) % 3_i8];
        assert_eq![Z::new(1), Z::new(4) % 3_i8]; // Z<i32> % i8

        #[cfg(target_pointer_width = "64")]
        assert_eq![Z::new(1_isize), Z::new(4_isize) % 3_i64];
    }
}

// ## Integer<IBig> % IBig = Integer<IBig>
// ## Integer<IBig> % *primitive* = Integer<IBig>
// -----------------------------------------------------------------------------
#[cfg(feature = "ibig")]
mod rem_ibig {
    use super::*;
    use ibig::IBig;

    // Integer<IBig> % IBig

    /// `Z<IBig> % IBig = Z<IBig>`
    impl Rem<IBig> for Integer<IBig> {
        type Output = Integer<IBig>;
        fn rem(self, other: IBig) -> Self::Output {
            Self::Output::new(self.0.rem(other))
        }
    }

    // Integer<IBig> % *any integer primitive*

    /// implements `Rem` for an IBig integer and a primitive.
    macro_rules! impl_rem_ibig_prim {
        (all: $t:ident, $tn:ident, $( $n:ty ),+) => {
            $( impl_rem_ibig_prim![$t, $tn, $n]; )+
        };
        ($t:ident, $tn:ident, $n:ty) => {
            paste::paste! {
               #[doc = "`Z<" $tn "> % " $n " = Z<" $tn ">`" ]
                impl Rem<$n> for $t<$tn> {
                    type Output = $t<$tn>;
                    fn rem(self, other: $n) -> Self::Output {
                        Self::Output::new($tn::from(self.0.rem(other)))
                    }
                }
            }
        };
    }
    #[rustfmt::skip]
    impl_rem_ibig_prim![all: Integer, IBig,
        i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize ];

    #[cfg(test)]
    mod test_ibig {
        use super::IBig;
        use crate::{integer::a::*, traits::Number};

        #[test]
        fn ibig() {
            // Z<IBig> % *integer*<IBig> = Z<IBig>
            assert_eq![
                Z::new(IBig::from(1)),
                Z::new(IBig::from(4)) % N0z::new(IBig::from(3))
            ];

            // Z<IBig> % IBig = Z<IBig>
            assert_eq![Z::new(IBig::from(1)), Z::new(IBig::from(4)) % IBig::from(3)];

            // Z<IBig> % *primitive* = Z<IBig>
            assert_eq![Z::new(IBig::from(1_i16)), Z::new(IBig::from(4_i64)) % 3_u8];
        }
    }
}
