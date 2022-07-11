// numera::integer::impl_ops::add
//
//! implements the `Add` operation for integer types.
//!
//

use crate::{
    integer::{
        a::*, Integer, NegativeInteger, NonNegativeInteger, NonPositiveInteger, NonZeroInteger,
        PositiveInteger,
    },
    traits::{CheckedAdd, NegSigned, Number, Signed},
};
use core::ops::Add;

// Integer
// -----------------------------------------------------------------------------

/// `Z + Z = Z`
impl<N: Number + Signed + Add<Output = N>> Add for Integer<N> {
    type Output = Z<N>;
    fn add(self, other: Self) -> Z<N> {
        Z::new(self.0.add(other.0))
    }
}

/// implements `Add` for an integer + a primitive of the same inner type.
macro_rules! impl_add_same_prim {
    (all: $t:ident, $($n:ty),+) => {
        $( impl_add_same_prim![$t, $n]; )+
    };
    ($t:ident, $n:ty) => {
        paste::paste! {
           #[doc = "`Z<" $n "> + " $n " = Z<" $n ">`" ]
            impl Add<$n> for $t<$n> {
                type Output = $t<$n>;
                fn add(self, other: $n) -> Self::Output {
                    Self::Output::new(self.0.add(other))
                }
            }
        }
    };
}
impl_add_same_prim![all: Integer, i8, i16, i32, i64, i128, isize];

/// implements `Add` for an integer and a `< sized` primitive of the same sign,
/// and also from a integer and a `<= sized` pointer.
macro_rules! impl_add_smaller_prim {
    (all: $t:ident, $doc:literal, $( ($n1:ty, $n2:ty ) ),+) => {
        $( impl_add_smaller_prim![$t, $doc, ( $n1, $n2 )]; )+
    };
    ($t:ident, $doc:literal, ($n1:ty, $n2:ty) ) => {
        paste::paste! {
            #[doc = "`Z<" $n1 "> + " $n2 " = Z<" $n1 ">`" $doc ]
            impl Add<$n2> for $t<$n1> {
                type Output = $t<$n1>;
                fn add(self, other: $n2) -> Self::Output {
                    Self::Output::new(self.0.add(other as $n1))
                }
            }
        }
    };
}
#[rustfmt::skip]
impl_add_smaller_prim![ all: Integer, "",
    (i16, i8),
    (i32, i16), (i32, i8),
    (i64, i32), (i64, i16), (i64, i8),
    (i128, i64), (i128, i32), (i128, i16), (i128, i8) ];

// depending on pointer size

#[rustfmt::skip]
#[cfg(target_pointer_width = "12")]
impl_add_smaller_prim![ all: Integer,
    "\n\nAssumes `target_pointer_width = \"128\"`",
    (isize, i8), (isize, i16), (isize, i32), (isize, i64), (isize, i128),
    (i128, isize) ];

#[rustfmt::skip]
#[cfg(target_pointer_width = "64")]
impl_add_smaller_prim![ all: Integer,
    "\n\nAssumes `target_pointer_width = \"64\"`",
    (isize, i8), (isize, i16), (isize, i32), (isize, i64),
    (i64, isize), (i128, isize) ];

#[rustfmt::skip]
#[cfg(target_pointer_width = "32")]
impl_add_smaller_prim![ all: Integer,
    "\n\nAssumes `target_pointer_width = \"32\"`",
    (isize, i8), (isize, i16), (isize, i32),
    (i32, isize), (i64, isize), (i128, isize) ];

#[rustfmt::skip]
#[cfg(target_pointer_width = "16")]
impl_add_smaller_prim![ all: Integer,
    "\n\nAssumes `target_pointer_width = \"16\"`",
    (isize, i8), (isize, i16),
    (i16, isize), (i32, isize), (i64, isize), (i128, isize) ];

#[rustfmt::skip]
#[cfg(target_pointer_width = "8")]
impl_add_smaller_prim![ all: Integer,
    "\n\nAssumes `target_pointer_width = \"8\"`",
    (isize, i8),
    (i8, isize), (i16, isize), (i32, isize), (i64, isize), (i128, isize) ];

// -----------------------------------------------------------------------------

#[cfg(feature = "ibig")]
mod add_ibig {
    use super::*;
    use ibig::{IBig, UBig};

    // Integer<[IU]Big>

    /// `Z<IBig> + IBig = Z<IBig>`
    impl Add<IBig> for Integer<IBig> {
        type Output = Z<IBig>;
        fn add(self, other: IBig) -> Z<IBig> {
            Z::new(self.0.add(other))
        }
    }

    // NonZeroInteger<[IU]Big>

    /// `N0z<IBig> + IBig = N0z<IBig>`
    ///
    /// # Panic
    /// Panics if the result `== 0`.
    impl Add<IBig> for NonZeroInteger<IBig> {
        type Output = N0z<IBig>;
        fn add(self, other: IBig) -> N0z<IBig> {
            let res = self.0.add(other);
            assert![!res.is_zero()];
            N0z::new(res)
        }
    }

    // [IU]Big + primitives

    /// implements `Add` for an [IU]Big integer and a primitive.
    macro_rules! impl_add_ibig_prim {
        (all: $t:ident, $tn:ident, $( $n:ty ),+) => {
            $( impl_add_ibig_prim![$t, $tn, $n]; )+
        };
        ($t:ident, $tn:ident, $n:ty) => {
            paste::paste! {
               #[doc = "`Z<IBig> + " $n " = Z<IBig>`" ]
                impl Add<$n> for $t<$tn> {
                    type Output = $t<$tn>;
                    fn add(self, other: $n) -> Self::Output {
                        Self::Output::new(self.0.add(other))
                    }
                }
            }
        };
    }

    // IBig

    #[rustfmt::skip]
    impl_add_ibig_prim![ all: Integer, IBig,
        i8, u8, i16, u16, i32, u32, i64, u64, i128, isize, usize ];

}

#[cfg(test)]
mod tests {
    use super::*;

    /// Checks
    #[test]
    fn check() {
        // same type
        assert_eq![Z::new(5) + Z::new(3), Z::new(8)];

        // same inner primitive
        assert_eq![Z::new(5_i16) + 3_i16, Z::new(8_i16)];
        assert_eq![Z::new(5) + 3, Z::new(8)]; // i32

        //
        assert_eq![Z::new(5_i64) + 3_i32, Z::new(8_i64)];
    }

    /// big integers
    #[test]
    #[cfg(feature = "ibig")]
    fn add_ibig() {
        use ibig::{IBig, UBig};
        assert_eq![Z::new(IBig::from(5)) + 3_i32, Z::new(IBig::from(8))];
    }
}
