// numera::number::real::float::f::convert
//
// TODO:
// - tests
// - from integer i,u, primitives
// - try_from integer i,u, primitives

// TODO
// #[cfg(test)]
// mod tests;

#[cfg(feature = "half")]
use half::{bf16, f16};
#[cfg(feature = "twofloat")]
use twofloat::TwoFloat as f128;

use crate::number::real::float::{
    macros::{from_primitive, try_from_primitive},
    *,
};
// #[cfg(feature = "try_from")]
// use core::num::NonZeroU128;
// use core::{
//     num::{
//         NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroU16, NonZeroU32,
//         NonZeroU64, NonZeroU8,
//     },
// };

/* for Float */

/* complementary primitive conversions */

// from smaller or equal sized f
#[cfg(feature = "half")]
from_primitive![float_f for:BFloat+16, from:bf+16];
#[cfg(feature = "half")]
from_primitive![float_f for:Float+16, from:f+16];
//
from_primitive![float_f for:Float+32, from:f+32];
#[cfg(feature = "half")]
from_primitive![float_half for:Float+32, from:bf+16];
#[cfg(feature = "half")]
from_primitive![float_half for:Float+32, from:f+16];
//
from_primitive![float_f for:Float+64, from:f+32,64];
#[cfg(feature = "half")]
from_primitive![float_half for:Float+64, from:bf+16];
#[cfg(feature = "half")]
from_primitive![float_half for:Float+64, from:f+16];
//
#[cfg(feature = "twofloat")]
from_primitive![float_f for:Float+128, from:f+128];
#[cfg(feature = "twofloat")]
from_primitive![float_tf for:Float+128, from:f+32,64];
#[cfg(all(feature = "twofloat", feature = "half"))]
from_primitive![float_tf_half for:Float+128, from:bf+16];
#[cfg(all(feature = "twofloat", feature = "half"))]
from_primitive![float_tf_half for:Float+128, from:f+16];
//
// try_from bigger or non-equal precision f
#[cfg(feature = "half")]
try_from_primitive![float_half_ne for:BFloat+bf+16, from:f+16];
#[cfg(feature = "half")]
try_from_primitive![float_half_ne for:Float+f+16, from:bf+16];
#[cfg(feature = "half")]
try_from_primitive![float_half for:BFloat+bf+16, from:f+32,64];
#[cfg(feature = "half")]
try_from_primitive![float_half for:Float+f+16, from:f+32,64];
// #[cfg(all(feature = "twofloat", feature = "half"))]
// try_from_primitive![float_f for:BFloat+16, from:f+128]; // TODO
// #[cfg(all(feature = "twofloat", feature = "half"))]
// try_from_primitive![float_f for:Float+16, from:f+128]; // TODO
//
try_from_primitive![float_f for:Float+32, from:f+64];
// #[cfg(feature = "twofloat")]
// try_from_primitive![float_f for:Float+32, from:f+128]; // TODO
//
// #[cfg(feature = "twofloat")]
// try_from_primitive![float_f for:Float+64, from:f+128]; // TODO
