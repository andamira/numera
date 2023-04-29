// numera::number::integer::q::impl_from
//
//!
//

use crate::number::{
    integer::abbr::*,
    rational::{
        abbr::*,
        macros::impl_from_integer,
    },
};
use core::num::{
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroU16, NonZeroU32, NonZeroU64,
    NonZeroU8,
}; // NonZeroU128,

/* infallible conversions */

// from smaller unsigned primitives
impl_from_integer![prim for:Q+16, num:Z16, den:N0z16, from:u + 8];
impl_from_integer![prim for:Q+32, num:Z32, den:N0z32, from:u + 8, 16];
impl_from_integer![prim for:Q+64, num:Z64, den:N0z64, from:u + 8, 16, 32];
impl_from_integer![prim for:Q+128, num:Z128, den:N0z128, from:u + 8, 16, 32, 64];
// from smaller or equal sized signed primitives
impl_from_integer![prim for:Q+8, num:Z8, den:N0z8, from:i + 8];
impl_from_integer![prim for:Q+16, num:Z16, den:N0z16, from:i + 8, 16];
impl_from_integer![prim for:Q+32, num:Z32, den:N0z32, from:i + 8, 16, 32];
impl_from_integer![prim for:Q+64, num:Z64, den:N0z64, from:i + 8, 16, 32, 64];
impl_from_integer![prim for:Q+128, num:Z128, den:N0z128, from:i + 8, 16, 32, 64, 128];

// from smaller unsigned non-zero primitives
impl_from_integer![nonzero for:Q+16, num:Z16, den:N0z16, from:NonZeroU + 8];
impl_from_integer![nonzero for:Q+32, num:Z32, den:N0z32, from:NonZeroU + 8, 16];
impl_from_integer![nonzero for:Q+64, num:Z64, den:N0z64, from:NonZeroU + 8, 16, 32];
impl_from_integer![nonzero for:Q+128, num:Z128, den:N0z128, from:NonZeroU + 8, 16, 32, 64];
// from smaller or equal sized signed non-zero primitives
impl_from_integer![nonzero for:Q+8, num:Z8, den:N0z8, from:NonZeroI + 8];
impl_from_integer![nonzero for:Q+16, num:Z16, den:N0z16, from:NonZeroI + 8, 16];
impl_from_integer![nonzero for:Q+32, num:Z32, den:N0z32, from:NonZeroI + 8, 16, 32];
impl_from_integer![nonzero for:Q+64, num:Z64, den:N0z64, from:NonZeroI + 8, 16, 32, 64];
impl_from_integer![nonzero for:Q+128, num:Z128, den:N0z128, from:NonZeroI + 8, 16, 32, 64, 128];

// from smaller or equal sized Ingeger
impl_from_integer![integer for:Q+8, num:Z8, den:N0z8, from:Z + 8];
impl_from_integer![integer for:Q+16, num:Z16, den:N0z16, from:Z + 8, 16];
impl_from_integer![integer for:Q+32, num:Z32, den:N0z32, from:Z + 8, 16, 32];
impl_from_integer![integer for:Q+64, num:Z64, den:N0z64, from:Z + 8, 16, 32, 64];
impl_from_integer![integer for:Q+128, num:Z128, den:N0z128, from:Z + 8, 16, 32, 64, 128];
// from smaller or equal sized NonZeroInteger
impl_from_integer![integer for:Q+8, num:Z8, den:N0z8, from:N0z + 8];
impl_from_integer![integer for:Q+16, num:Z16, den:N0z16, from:N0z + 8, 16];
impl_from_integer![integer for:Q+32, num:Z32, den:N0z32, from:N0z + 8, 16, 32];
impl_from_integer![integer for:Q+64, num:Z64, den:N0z64, from:N0z + 8, 16, 32, 64];
impl_from_integer![integer for:Q+128, num:Z128, den:N0z128, from:N0z + 8, 16, 32, 64, 128];

// from smaller PositiveInteger
impl_from_integer![integer for:Q+16, num:Z16, den:N0z16, from:Pz + 8];
impl_from_integer![integer for:Q+32, num:Z32, den:N0z32, from:Pz + 8, 16];
impl_from_integer![integer for:Q+64, num:Z64, den:N0z64, from:Pz + 8, 16, 32];
impl_from_integer![integer for:Q+128, num:Z128, den:N0z128, from:Pz + 8, 16, 32, 64];
// from smaller NonNegativeInteger
impl_from_integer![integer for:Q+16, num:Z16, den:N0z16, from:Nnz + 8];
impl_from_integer![integer for:Q+32, num:Z32, den:N0z32, from:Nnz + 8, 16];
impl_from_integer![integer for:Q+64, num:Z64, den:N0z64, from:Nnz + 8, 16, 32];
impl_from_integer![integer for:Q+128, num:Z128, den:N0z128, from:Nnz + 8, 16, 32, 64];
// from smaller NegativeInteger
impl_from_integer![integer for:Q+16, num:Z16, den:N0z16, from:Nz + 8];
impl_from_integer![integer for:Q+32, num:Z32, den:N0z32, from:Nz + 8, 16];
impl_from_integer![integer for:Q+64, num:Z64, den:N0z64, from:Nz + 8, 16, 32];
impl_from_integer![integer for:Q+128, num:Z128, den:N0z128, from:Nz + 8, 16, 32, 64];
// from smaller NonPositiveInteger
impl_from_integer![integer for:Q+16, num:Z16, den:N0z16, from:Npz + 8];
impl_from_integer![integer for:Q+32, num:Z32, den:N0z32, from:Npz + 8, 16];
impl_from_integer![integer for:Q+64, num:Z64, den:N0z64, from:Npz + 8, 16, 32];
impl_from_integer![integer for:Q+128, num:Z128, den:N0z128, from:Npz + 8, 16, 32, 64];

// // from smaller sized Rational
// impl_from_rational![int for:Q + i + 16, from:Q + 8];
// impl_from_rational![int for:Q + i + 32, from:Q + 8, 16];
// impl_from_rational![int for:Q + i + 64, from:Q + 8, 16, 32];
// impl_from_rational![int for:Q + i + 128, from:Q + 8, 16, 32, 64];
//
// // from smaller or equal sized NonZeroRational
// impl_from_rational![nonzero for:Q + i + 8, from:N0q + 8];
// impl_from_rational![nonzero for:Q + i + 16, from:N0q + 8, 16];
// impl_from_rational![nonzero for:Q + i + 32, from:N0q + 8, 16, 32];
// impl_from_rational![nonzero for:Q + i + 128, from:N0q + 8, 16, 32, 64, 128];
//
// // from smaller sized NonNegativeRational
// impl_from_rational![int for:Q + i + 16, from:Nnq + 8];
// impl_from_rational![int for:Q + i + 32, from:Nnq + 8, 16];
// impl_from_rational![int for:Q + i + 64, from:Nnq + 8, 16, 32];
// impl_from_rational![int for:Q + i + 128, from:Nnq + 8, 16, 32, 64];
//
// // from smaller sized PositiveRational
// impl_from_rational![nonzero for:Q + i + 16, from:Pq + 8];
// impl_from_rational![nonzero for:Q + i + 32, from:Pq + 8, 16];
// impl_from_rational![nonzero for:Q + i + 64, from:Pq + 8, 16, 32];
// impl_from_rational![nonzero for:Q + i + 128, from:Pq + 8, 16, 32, 64];
//
// // from smaller sized NonPositiveRational
// impl_from_rational![int_neg for:Q + i + 16, from:Npq + 8];
// impl_from_rational![int_neg for:Q + i + 32, from:Npq + 8, 16];
// impl_from_rational![int_neg for:Q + i + 64, from:Npq + 8, 16, 32];
// impl_from_rational![int_neg for:Q + i + 128, from:Npq + 8, 16, 32, 64];
//
// // from smaller sized NegativeRational
// impl_from_rational![nonzero_neg for:Q + i + 16, from:Nnq + 8];
// impl_from_rational![nonzero_neg for:Q + i + 32, from:Nnq + 8, 16];
// impl_from_rational![nonzero_neg for:Q + i + 64, from:Nnq + 8, 16, 32];
// impl_from_rational![nonzero_neg for:Q + i + 128, from:Nnq + 8, 16, 32, 64];

// TODO
// #[cfg(test)]
// mod tests {
//     use crate::all::*;
//
//     #[test]
//     fn q_from() -> NumeraResult<()> {
//         // let _q5 = Rational8::new((5, 1))?;
//         Ok(())
//     }
// }
