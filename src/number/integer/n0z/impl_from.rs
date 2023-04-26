// numera::number::integer::n0z::impl_from
//
//!
//

use crate::number::{
    integer::{
        macros::{impl_from_integer, impl_from_primitive},
        n0z::*,
        pz::*,
    },
    traits::Number,
};
use core::num::{
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroU16, NonZeroU32, NonZeroU64,
    NonZeroU8,
};

/* from smaller unsigned primitive, and from smaller or equal signed primitive */

impl_from_primitive![many_nonzero for: NonZeroInteger + 8, from: NonZeroI + 8];
impl_from_primitive![many_nonzero for: NonZeroInteger + 16, from: NonZeroU + 8];
impl_from_primitive![many_nonzero for: NonZeroInteger + 16, from: NonZeroI + 8, 16];
impl_from_primitive![many_nonzero for: NonZeroInteger + 32, from: NonZeroU + 8, 16];
impl_from_primitive![many_nonzero for: NonZeroInteger + 32, from: NonZeroI + 8, 16, 32];
impl_from_primitive![many_nonzero for: NonZeroInteger + 64, from: NonZeroU + 8, 16, 32];
impl_from_primitive![many_nonzero for: NonZeroInteger + 64, from: NonZeroI + 8, 16, 32, 64];
impl_from_primitive![many_nonzero for: NonZeroInteger + 128, from: NonZeroU + 8, 16, 32, 64];
impl_from_primitive![many_nonzero for: NonZeroInteger + 128, from: NonZeroI + 8, 16, 32, 64, 128];

/* from smaller sized Integer */

impl_from_integer![many_nonzero for: NonZeroInteger + i + 16, from: PositiveInteger + 8];
impl_from_integer![many_nonzero for: NonZeroInteger + i + 32, from: PositiveInteger + 8, 16];
impl_from_integer![many_nonzero for: NonZeroInteger + i + 64, from: PositiveInteger + 8, 16, 32];
impl_from_integer![many_nonzero for: NonZeroInteger + i + 128, from: PositiveInteger + 8, 16, 32, 64];

// impl_from![int_nonzero_neg for: NonZeroInteger + i + 16, from: NegativeInteger + 8];
// impl_from![int_nonzero_neg for: NonZeroInteger + i + 32, from: NegativeInteger + 8, 16];
// impl_from![int_nonzero_neg for: NonZeroInteger + i + 64, from: NegativeInteger + 8, 16, 32];
// impl_from![int_nonzero_neg for: NonZeroInteger + i + 128, from: NegativeInteger + 8, 16, 32, 64];
