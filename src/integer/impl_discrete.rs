// numera::integer::impl_discrete
//
//! implements the `Discrete` trait for all integer types.
//

use crate::integer::{
    Integer, NegativeInteger, NonNegativeInteger, NonPositiveInteger, NonZeroInteger,
    PositiveInteger, Signed,
};
use crate::traits::{Discrete, Number};

impl<N: Number + Signed> Discrete for Integer<N> where N: Discrete {}
impl<N: Number + Signed> Discrete for NonZeroInteger<N> where N: Discrete {}
impl<N: Number + Signed> Discrete for NonPositiveInteger<N> where N: Discrete {}
impl<N: Number + Signed> Discrete for NegativeInteger<N> where N: Discrete {}
impl<N: Number> Discrete for PositiveInteger<N> where N: Discrete {}
impl<N: Number> Discrete for NonNegativeInteger<N> where N: Discrete {}
