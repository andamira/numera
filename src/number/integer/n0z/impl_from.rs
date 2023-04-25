// numera::number::integer::n0z::impl_from
//
//!
//

use crate::number::integer::n0z::*;
use core::num::{
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroU16, NonZeroU32, NonZeroU64,
    NonZeroU8,
};

/// Implements From<`$from_p $from_size`> for `$for$for_size`.
///
/// # Args
/// - `$for`:       the base name of the target. e.g. `NonZeroInteger`.
/// - `$for_size`:  the bit size of the target. e.g. `16`.
/// - `$from_p`:    the base name of the origin. e.g. `u`.
/// - `$from_size`: a list of bit sizes of the origin. e.g. `8, 16`.
///
/// # Examples
/// ```ignore
/// impl_from_primitive![many for: Integer + 16, from: u + 8];
/// impl_from_primitive![many for: Integer + 16, from: i + 8, 16];
/// ```
macro_rules! impl_from_primitive {
    (many
     for: $for:ident + $for_size:expr,
     from: $from_p:ident + $( $from_size:expr ),+
    ) => {
        $(
            impl_from_primitive![for: $for + $for_size, from: $from_p + $from_size];
        )+
    };

    (
     for: $for:ident + $for_size:expr,
     from: $from_p:ident + $from_size:expr
    ) => {
        paste::paste! {
            impl From<[< $from_p $from_size >]> for [< $for $for_size >] {
                fn from(from: [< $from_p $from_size >]) -> Self {
                    Self(from.into())
                }
            }
        }
    };
}

/* from smaller unsigned primitive, and from smaller or equal signed primitive */

impl_from_primitive![many for: NonZeroInteger + 8, from: NonZeroI + 8];
impl_from_primitive![many for: NonZeroInteger + 16, from: NonZeroU + 8];
impl_from_primitive![many for: NonZeroInteger + 16, from: NonZeroI + 8, 16];
impl_from_primitive![many for: NonZeroInteger + 32, from: NonZeroU + 8, 16];
impl_from_primitive![many for: NonZeroInteger + 32, from: NonZeroI + 8, 16, 32];
impl_from_primitive![many for: NonZeroInteger + 64, from: NonZeroU + 8, 16, 32];
impl_from_primitive![many for: NonZeroInteger + 64, from: NonZeroI + 8, 16, 32, 64];
impl_from_primitive![many for: NonZeroInteger + 128, from: NonZeroU + 8, 16, 32, 64];
impl_from_primitive![many for: NonZeroInteger + 128, from: NonZeroI + 8, 16, 32, 64, 128];

/* from smaller sized Integer */

// TODO

// impl_from![int_nonzero for: NonZeroInteger + i + 16, from: PositiveInteger + 8];
// impl_from![int_nonzero for: NonZeroInteger + i + 32, from: PositiveInteger + 8, 16];
// impl_from![int_nonzero for: NonZeroInteger + i + 64, from: PositiveInteger + 8, 16, 32];
// impl_from![int_nonzero for: NonZeroInteger + i + 128, from: PositiveInteger + 8, 16, 32, 64];
//
// impl_from![int_nonzero_neg for: NonZeroInteger + i + 16, from: NegativeInteger + 8];
// impl_from![int_nonzero_neg for: NonZeroInteger + i + 32, from: NegativeInteger + 8, 16];
// impl_from![int_nonzero_neg for: NonZeroInteger + i + 64, from: NegativeInteger + 8, 16, 32];
// impl_from![int_nonzero_neg for: NonZeroInteger + i + 128, from: NegativeInteger + 8, 16, 32, 64];
