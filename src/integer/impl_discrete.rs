// numera::integer::impl_discrete
//
//! implements the `Discrete` trait for all integer types.
//

use crate::integer::{
    Integer, NegativeInteger, NonNegativeInteger, NonPositiveInteger, NonZeroInteger,
    PositiveInteger,
};
use crate::traits::{Discrete, NumberAble};

impl<I: NumberAble> Discrete for Integer<I> where I: Discrete {}
impl<I: NumberAble> Discrete for NonZeroInteger<I> where I: Discrete {}
impl<I: NumberAble> Discrete for PositiveInteger<I> where I: Discrete {}
impl<I: NumberAble> Discrete for NonNegativeInteger<I> where I: Discrete {}
impl<I: NumberAble> Discrete for NonPositiveInteger<I> where I: Discrete {}
impl<I: NumberAble> Discrete for NegativeInteger<I> where I: Discrete {}
