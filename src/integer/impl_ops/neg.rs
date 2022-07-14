// numera::integer::impl_ops::neg
//
//! implements the [`Neg`] operation for integer types.
//

use crate::{
    integer::Integer,
    traits::{Number, Signed},
};
use core::ops::Neg;

// -Integer<N: Signed> = Integer<N>
// -----------------------------------------------------------------------------

impl<N: Number + Signed + Neg<Output = N>> Neg for Integer<N> {
    type Output = Integer<N>;
    fn neg(self) -> Integer<N> {
        Integer::new(self.0.neg())
    }
}

#[cfg(test)]
mod test_impl_neg_z {
    use crate::{integer::a::*, traits::Number};
    #[test]
    fn impl_neg_z() {
        assert_eq![Z::new(-7), -Z::new(7)];
    }

    #[test]
    fn impl_neg_z_ibig() {
        use ibig::IBig;
        assert_eq![Z::new(IBig::from(-7)), -Z::new(IBig::from(7))];
    }
}
