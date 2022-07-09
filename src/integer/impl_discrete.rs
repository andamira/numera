// numera::integer::impl_discrete
//
//! implements the `Discrete` trait for all integer types.
//

use crate::integer::{
    Integer, NegativeInteger, NonNegativeInteger, NonPositiveInteger, NonZeroInteger,
    PositiveInteger,
};
use crate::traits::{Discrete, Number};

impl<I: Number> Discrete for Integer<I> where I: Discrete {}
impl<I: Number> Discrete for NonZeroInteger<I> where I: Discrete {}
impl<I: Number> Discrete for PositiveInteger<I> where I: Discrete {}
impl<I: Number> Discrete for NonNegativeInteger<I> where I: Discrete {}
impl<I: Number> Discrete for NonPositiveInteger<I> where I: Discrete {}
impl<I: Number> Discrete for NegativeInteger<I> where I: Discrete {}
