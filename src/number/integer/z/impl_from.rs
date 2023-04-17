// numera::integer::z::impl_from
//
//!
//

use crate::number::{
    integer::{macros::{impl_from_integer, impl_from_primitive}, z::*},
    traits::Number,
};

/* from smaller unsigned primitive, and from smaller or equal signed primitive */
impl_from_primitive![many for: Integer + 8, from: i + 8];
impl_from_primitive![many for: Integer + 16, from: u + 8];
impl_from_primitive![many for: Integer + 16, from: i + 8, 16];
impl_from_primitive![many for: Integer + 32, from: u + 8, 16];
impl_from_primitive![many for: Integer + 32, from: i + 8, 16, 32];
impl_from_primitive![many for: Integer + 64, from: u + 8, 16, 32];
impl_from_primitive![many for: Integer + 64, from: i + 8, 16, 32, 64];
impl_from_primitive![many for: Integer + 128, from: u + 8, 16, 32, 64];
impl_from_primitive![many for: Integer + 128, from: i + 8, 16, 32, 64, 128];

/* from smaller sized Integer */
impl_from_integer![many_int for: Integer + i + 16, from: Integer + 8];
impl_from_integer![many_int for: Integer + i + 32, from: Integer + 8, 16];
impl_from_integer![many_int for: Integer + i + 64, from: Integer + 8, 16, 32];
impl_from_integer![many_int for: Integer + i + 128, from: Integer + 8, 16, 32, 64];

// TODO

// impl_from![int_nonzero for: Integer + i + 8, from: NonZeroInteger + 8];
// impl_from![int_nonzero for: Integer + i + 16, from: NonZeroInteger + 8, 16];
// impl_from![int_nonzero for: Integer + i + 32, from: NonZeroInteger + 8, 16, 32];
// impl_from![int_nonzero for: Integer + i + 128, from: NonZeroInteger + 8, 16, 32, 64, 128];
//
// impl_from![int for: Integer + i + 16, from: NonNegativeInteger + 8];
// impl_from![int for: Integer + i + 32, from: NonNegativeInteger + 8, 16];
// impl_from![int for: Integer + i + 64, from: NonNegativeInteger + 8, 16, 32];
// impl_from![int for: Integer + i + 128, from: NonNegativeInteger + 8, 16, 32, 64];
//
// impl_from![int_nonzero for: Integer + i + 16, from: PositiveInteger + 8];
// impl_from![int_nonzero for: Integer + i + 32, from: PositiveInteger + 8, 16];
// impl_from![int_nonzero for: Integer + i + 64, from: PositiveInteger + 8, 16, 32];
// impl_from![int_nonzero for: Integer + i + 128, from: PositiveInteger + 8, 16, 32, 64];
//
// impl_from![int_neg for: Integer + i + 16, from: NonPositiveInteger + 8];
// impl_from![int_neg for: Integer + i + 32, from: NonPositiveInteger + 8, 16];
// impl_from![int_neg for: Integer + i + 64, from: NonPositiveInteger + 8, 16, 32];
// impl_from![int_neg for: Integer + i + 128, from: NonPositiveInteger + 8, 16, 32, 64];
//
// impl_from![int_nonzero_neg for: Integer + i + 16, from: NegativeInteger + 8];
// impl_from![int_nonzero_neg for: Integer + i + 32, from: NegativeInteger + 8, 16];
// impl_from![int_nonzero_neg for: Integer + i + 64, from: NegativeInteger + 8, 16, 32];
// impl_from![int_nonzero_neg for: Integer + i + 128, from: NegativeInteger + 8, 16, 32, 64];
