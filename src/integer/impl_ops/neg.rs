// numera::integer::impl_ops::neg
//
//! implements the [`Neg`] operation for integer types.
//

use crate::traits::{NegSigned, Number, Signed};
use core::ops::Neg;

use crate::integer::{
    a::*, Integer, NegativeInteger, NonNegativeInteger, NonPositiveInteger, NonZeroInteger,
    PositiveInteger,
};

// use crate::traits::{CheckedAdd, CheckedDiv, CheckedMul, CheckedNeg, CheckedRem, CheckedSub};

// -Z = -Z, -N0z = -N0z
// -----------------------------------------------------------------------------

impl<N: Number + Signed + Neg<Output = N>> Neg for Integer<N> {
    type Output = Integer<N>;
    fn neg(self) -> Integer<N> {
        Integer::new(self.0.neg())
    }
}
impl<N: Number + Signed + Neg<Output = N>> Neg for NonZeroInteger<N> {
    type Output = N0z<N>;
    fn neg(self) -> N0z<N> {
        N0z::new(self.0.neg())
    }
}

// -Nz = Pz, -Pz = Nz, -Npz = Nnz, -Nnz = Npz
// -----------------------------------------------------------------------------

impl<N: Number + Signed + Neg<Output = N>> Neg for NegativeInteger<N> {
    type Output = PositiveInteger<N>;
    fn neg(self) -> Self::Output {
        Self::Output::new(self.0.neg())
    }
}
impl<N: Number + Signed + Neg<Output = N>> Neg for PositiveInteger<N> {
    type Output = NegativeInteger<N>;
    fn neg(self) -> Self::Output {
        Self::Output::new(self.0.neg())
    }
}
impl<N: Number + Signed + Neg<Output = N>> Neg for NonPositiveInteger<N> {
    type Output = NonNegativeInteger<N>;
    fn neg(self) -> Self::Output {
        Self::Output::new(self.0.neg())
    }
}
impl<N: Number + Signed + Neg<Output = N>> Neg for NonNegativeInteger<N> {
    type Output = NonPositiveInteger<N>;
    fn neg(self) -> Self::Output {
        Self::Output::new(self.0.neg())
    }
}

// WAIT:specialization, until then...
//
/// implements `Neg` for an integer<N: Unsigned>,
/// returning the sign complementary type.
macro_rules! impl_neg_unsigned {
    // negative to positive:
    (all_n2p: $t1:ident, $t2:ident; $($n:ty),+) => {
        $( impl_neg_unsigned![n2p: $t1, $t2, $n]; )+
    };
    (n2p: $t1:ident, $t2:ident, $n:ty) => {
        impl Neg for $t1<$n> {
            type Output = $t2<$n>;
            fn neg(self) -> Self::Output {
                Self::Output::new(self.0)
            }
        }
    };
    // positive to negative:
    (all_p2n: $t1:ident, $t2:ident; $($n:ty),+) => {
        $( impl_neg_unsigned![p2n: $t1, $t2, $n]; )+
    };
    (p2n: $t1:ident, $t2:ident, $n:ty) => {
        impl Neg for $t1<$n> {
            type Output = $t2<$n>;
            fn neg(self) -> Self::Output {
                NegSigned::new_neg(self.0)
            }
        }
    };
}
impl_neg_unsigned![all_n2p: NegativeInteger, PositiveInteger;
    u8, u16, u32, u64, u128, usize];
impl_neg_unsigned![all_p2n: PositiveInteger, NegativeInteger;
    u8, u16, u32, u64, u128, usize];
impl_neg_unsigned![all_n2p: NonPositiveInteger, NonNegativeInteger;
    u8, u16, u32, u64, u128, usize];
impl_neg_unsigned![all_p2n: NonNegativeInteger, NonPositiveInteger;
    u8, u16, u32, u64, u128, usize];

#[cfg(test)]
mod tests {
    use super::*;

    /// Checks
    #[test]
    fn check() {
        assert_eq![-Z::new(-100_i8), Z::new(100_i8)];
        assert_eq![-N0z::new(-100_i8), N0z::new(100_i8)];

        assert_eq![-Nz::new_neg(100_u8), Pz::new(100_u8)];
        assert_eq![-Pz::new(100_u8), Nz::new_neg(100_u8)];

        assert_eq![-Npz::new_neg(100_u8), Nnz::new(100_u8)];
        assert_eq![-Nnz::new(100_u8), Npz::new_neg(100_u8)];
    }

    /// when it can panic
    #[test]
    #[cfg(feature = "std")]
    fn check_panics() {
        use std::panic::catch_unwind;

        assert![catch_unwind(|| { Z::new(-1).neg() }).is_ok()];
    }
}
