// numera::integer::impl_ops::neg
//
//! implements the [`Neg`] operation for integer types.
//!
//! - -Integer -> Integer
//! - -NonZeroInteger -> NonZeroInteger
//! - -PositiveInteger -> NegativeInteger
//! - -NegativeInteger -> PositiveInteger
//! - -NonPositiveInteger -> NonNegativeInteger
//! - -NonNegativeInteger -> NonPositiveInteger
//

use crate::{
    integer::{
        Integer, NegativeInteger, NonNegativeInteger, NonPositiveInteger, NonZeroInteger,
        PositiveInteger,
    },
    traits::{NegSigned, Number, Signed},
};
use core::ops::Neg;

// -Integer<N: Signed> = Integer<N>
// -----------------------------------------------------------------------------

impl<N: Number + Signed + Neg<Output = N>> Neg for Integer<N> {
    type Output = Integer<N>;
    fn neg(self) -> Integer<N> {
        Self::Output::new(self.0.neg())
    }
}

#[cfg(test)]
mod test_impl_neg_z {
    use crate::{integer::a::*, traits::Number};
    #[test]
    fn impl_neg_z() {
        assert_eq![Z::new(-7), -Z::new(7)];
    }

    #[cfg(feature = "ibig")]
    #[test]
    fn impl_neg_z_ibig() {
        use ibig::IBig;
        assert_eq![Z::new(IBig::from(-7)), -Z::new(IBig::from(7))];
    }
}

// -NonZeroInteger<N: Signed> = NonZeroInteger<N>
// -----------------------------------------------------------------------------

impl<N: Number + Signed + Neg<Output = N>> Neg for NonZeroInteger<N> {
    type Output = NonZeroInteger<N>;
    fn neg(self) -> NonZeroInteger<N> {
        Self::Output::new(self.0.neg())
    }
}

#[cfg(test)]
mod test_impl_neg_n0z {
    use crate::{integer::a::*, traits::Number};
    #[test]
    fn impl_neg_z() {
        assert_eq![N0z::new(-7), -N0z::new(7)];
    }

    #[cfg(feature = "ibig")]
    #[test]
    fn impl_neg_z_ibig() {
        use ibig::IBig;
        assert_eq![N0z::new(IBig::from(-7)), -N0z::new(IBig::from(7))];
    }
}

/* using NegSigned/Unsigned inner types */

/// implements `Neg` for an integer with an inner unsigned primitive, that
/// returns another kind of integer with the same inner unsigned primitive.
macro_rules! impl_neg_z1_z2 {
    ($t1:ident -> $t2:ident, $method:ident: $($prim:ty),+) => {
        $( impl_neg_z1_z2![single $t1 -> $t2, $method: $prim]; )+
    };
    (single $t1:ident -> $t2:ident, $method:ident : $prim:ty) => {
        impl Neg for $t1<$prim> {
            type Output = $t2<$prim>;
            fn neg(self) -> Self::Output {
                Self::Output::$method(self.0)
            }
        }
    };
}

// -PositiveInteger<N> = NegativeInteger<N>
// -----------------------------------------------------------------------------

impl<N: Number + Signed + Neg<Output = N>> Neg for PositiveInteger<N> {
    type Output = NegativeInteger<N>;
    fn neg(self) -> Self::Output {
        Self::Output::new(self.0.neg())
    }
}

// WAITFOR:specialization
// Can't do this. It's necessary to implement these manually to avoid conflicts.
//
// impl<N: Number + crate::traits::Unsigned + Neg<Output = N>> Neg for PositiveInteger<N> {
//     type Output = NegativeInteger<N>;
//     fn neg(self) -> NegativeInteger<N> {
//         Self::Output::new(self.0)
//     }
// }

#[rustfmt::skip]
impl_neg_z1_z2![PositiveInteger -> NegativeInteger, new_neg: u8, u16, u32, u64, usize];

#[cfg(feature = "ibig")]
mod neg_pz_ibig {
    use super::*;
    use ibig::UBig;
    impl_neg_z1_z2![PositiveInteger -> NegativeInteger, new_neg: UBig];
}

#[cfg(test)]
mod test_impl_neg_pz {
    use crate::{
        integer::a::*,
        traits::{NegSigned, Number},
    };
    #[test]
    fn impl_neg_pz() {
        assert_eq![-Pz::new(7), Nz::new(-7)];
        assert_eq![-Pz::new(7_u8), Nz::new_neg(7_u8)];
    }

    #[cfg(feature = "ibig")]
    #[test]
    fn impl_neg_pz_ibig() {
        use ibig::{IBig, UBig};
        assert_eq![-Pz::new(IBig::from(7)), Nz::new(IBig::from(-7))];
        assert_eq![-Pz::new(UBig::from(7_u32)), Nz::new_neg(UBig::from(7_u32))];
    }
}

// -NegativeInteger<N> = PositiveInteger<N>
// -----------------------------------------------------------------------------

impl<N: Number + Signed + Neg<Output = N>> Neg for NegativeInteger<N> {
    type Output = PositiveInteger<N>;
    fn neg(self) -> Self::Output {
        Self::Output::new(self.0.neg())
    }
}

#[rustfmt::skip]
impl_neg_z1_z2![NegativeInteger -> PositiveInteger, new: u8, u16, u32, u64, usize];

#[cfg(feature = "ibig")]
mod neg_nz_ibig {
    use super::*;
    use ibig::UBig;
    impl_neg_z1_z2![NegativeInteger -> PositiveInteger, new: UBig];
}

#[cfg(test)]
mod test_impl_neg_nz {
    use crate::{
        integer::a::*,
        traits::{NegSigned, Number},
    };
    #[test]
    fn impl_neg_pz() {
        assert_eq![-Nz::new(-7), Pz::new(7)];
        assert_eq![-Nz::new_neg(7_u8), Pz::new(7_u8)];
    }

    #[cfg(feature = "ibig")]
    #[test]
    fn impl_neg_pz_ibig() {
        use ibig::{IBig, UBig};
        assert_eq![-Nz::new(IBig::from(-7)), Pz::new(IBig::from(7))];
        assert_eq![-Nz::new_neg(UBig::from(7_u32)), Pz::new(UBig::from(7_u32))];
    }
}

// -NonNegativeInteger<N> = NonPositiveInteger<N>
// -----------------------------------------------------------------------------

impl<N: Number + Signed + Neg<Output = N>> Neg for NonNegativeInteger<N> {
    type Output = NonPositiveInteger<N>;
    fn neg(self) -> Self::Output {
        Self::Output::new(self.0.neg())
    }
}

#[rustfmt::skip]
impl_neg_z1_z2![NonNegativeInteger -> NonPositiveInteger, new_neg: u8, u16, u32, u64, usize];

#[cfg(feature = "ibig")]
mod neg_nnz_ibig {
    use super::*;
    use ibig::UBig;
    impl_neg_z1_z2![NonNegativeInteger -> NonPositiveInteger, new_neg: UBig];
}

#[cfg(test)]
mod test_impl_neg_nnz {
    use crate::{
        integer::a::*,
        traits::{NegSigned, Number},
    };
    #[test]
    fn impl_neg_pz() {
        assert_eq![-Nnz::new(7), Npz::new(-7)];
        assert_eq![-Nnz::new(7_u8), Npz::new_neg(7_u8)];
    }

    #[cfg(feature = "ibig")]
    #[test]
    fn impl_neg_pz_ibig() {
        use ibig::{IBig, UBig};
        assert_eq![-Nnz::new(IBig::from(7)), Npz::new(IBig::from(-7))];
        assert_eq![
            -Nnz::new(UBig::from(7_u32)),
            Npz::new_neg(UBig::from(7_u32))
        ];
    }
}

// -NonPositiveInteger<N> = NonNegativeInteger<N>
// -----------------------------------------------------------------------------

impl<N: Number + Signed + Neg<Output = N>> Neg for NonPositiveInteger<N> {
    type Output = NonNegativeInteger<N>;
    fn neg(self) -> Self::Output {
        Self::Output::new(self.0.neg())
    }
}

#[rustfmt::skip]
impl_neg_z1_z2![NonPositiveInteger -> NonNegativeInteger, new: u8, u16, u32, u64, usize];

#[cfg(feature = "ibig")]
mod neg_npz_ibig {
    use super::*;
    use ibig::UBig;
    impl_neg_z1_z2![NonPositiveInteger -> NonNegativeInteger, new: UBig];
}

#[cfg(test)]
mod test_impl_neg_npz {
    use crate::{
        integer::a::*,
        traits::{NegSigned, Number},
    };
    #[test]
    fn impl_neg_pz() {
        assert_eq![-Npz::new(-7), Nnz::new(7)];
        assert_eq![-Npz::new_neg(7_u8), Nnz::new(7_u8)];
    }

    #[cfg(feature = "ibig")]
    #[test]
    fn impl_neg_pz_ibig() {
        use ibig::{IBig, UBig};
        assert_eq![-Npz::new(IBig::from(-7)), Nnz::new(IBig::from(7))];
        assert_eq![
            -Npz::new_neg(UBig::from(7_u32)),
            Nnz::new(UBig::from(7_u32))
        ];
    }
}
