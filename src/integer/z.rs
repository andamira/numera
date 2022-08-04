// numera::integer::z
//
//! Integer
//

use crate::{
    integer::*,
    macros::define_integer_prim,
    traits::{Number, Signed},
};
use az::UnwrappedAs; // TEMP
use core::ops::{Add, Div, Mul, Neg, Rem, Sub};

/* definitions */

/// A generic integer number, from the set $\Z$.
pub struct Integer<N: Number + Signed>(N);

#[rustfmt::skip]
define_integer_prim![Integer, i,
    "integer number", ", from the set $\\Z$.",
    "",
    "", MIN, MAX,
    ("An", 8), ("A", 16), ("A", 32), ("A", 64), ("A", 128)];

// /* impl into_primitive */
//
// impl_into_primitive![unit Integer8, i8];
// // impl_into_primitive![all_unit Integer8, i8, Integer16, i16, Integer32, i32, Integer64, i64];

/* impl From */

/// Implements From `$from$from_size` for `$for$for_size`.
///
/// # Args
/// - `$p`:
//
// IDEA: common api for Integer<i8> | integer<NonZeroI8> | I8: (fn into_primitive|into_innermost)
//       retrieve .0.into() | 0.get().into() |
macro_rules! impl_from {
    // multiple
    (int for: $for:ident + $p:ident + $for_size:expr, from: $from:ident + $( $from_size:expr ),+) => {
        $(
            impl_from![int_single for: $for + $p + $for_size, from: $from + $from_size];
        )+
    };
    // having an inner primitive
    (int_single
     for: $for:ident + $p:ident + $for_size:expr, from: $from:ident + $from_size:expr) => {
        paste::paste! {
            impl From<[< $from $from_size >]> for [< $for $for_size >] {
                fn from(from: [< $from $from_size >]) -> Self {
                    Self::new(from.0.into())
                }
            }
        }
    };

    // multiple
    (int_nonzero
     for: $for:ident + $p:ident + $for_size:expr, from: $from:ident + $( $from_size:expr ),+) => {
        $(
            impl_from![int_single_nonzero for: $for + $p + $for_size, from: $from + $from_size];
        )+
    };
    // having an inner NonZero*
    (int_single_nonzero
     for: $for:ident + $p:ident + $for_size:expr, from: $from:ident + $from_size:expr) => {
        paste::paste! {
            impl From<[< $from $from_size >]> for [< $for $for_size >] {
                fn from(from: [< $from $from_size >]) -> Self {
                    Self::new(from.0.get().into())
                }
            }
        }
    };

    // multiple
    (int_neg
     for: $for:ident + $p:ident + $for_size:expr, from: $from:ident + $( $from_size:expr ),+) => {
        $(
            impl_from![int_single_neg for: $for + $p + $for_size, from: $from + $from_size];
        )+
    };
    // having an unsigned inner primitive, representing only negative values.
    (int_single_neg
     for: $for:ident + $p:ident + $for_size:expr, from: $from:ident + $from_size:expr) => {
        paste::paste! {
            impl From<[< $from $from_size >]> for [< $for $for_size >] {
                fn from(from: [< $from $from_size >]) -> Self {
                    Self::new( Into::<[< $p $for_size >]>::into(from.0).neg())
                }
            }
        }
    };

    // multiple
    (int_nonzero_neg
     for: $for:ident + $p:ident + $for_size:expr, from: $from:ident + $( $from_size:expr ),+) => {
        $(
            impl_from![int_single_nonzero_neg
            for: $for + $p + $for_size, from: $from + $from_size];
        )+
    };
    // having an unsigned inner NonZero*, representing only negative values.
    (int_single_nonzero_neg
     for: $for:ident + $p:ident + $for_size:expr, from: $from:ident + $from_size:expr) => {
        paste::paste! {
            impl From<[< $from $from_size >]> for [< $for $for_size >] {
                fn from(from: [< $from $from_size >]) -> Self {
                    Self::new( Into::<[< $p $for_size >]>::into(from.0.get()).neg())
                }
            }
        }
    };
}

#[rustfmt::skip]
mod impls_from {
    use super::*;

    impl_from![int for: Integer + i + 16, from: Integer + 8];
    impl_from![int for: Integer + i + 32, from: Integer + 8, 16];
    impl_from![int for: Integer + i + 64, from: Integer + 8, 16, 32];
    impl_from![int for: Integer + i + 128, from: Integer + 8, 16, 32, 64];

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
}

/* impl ops */

/// Implements operations between an integer and other integers.
///
// TODO WIP
// NOTE: needs several types of operations
//
// // Z & N0z
//
//
// + signed (Z, N0z) + negsigned (Nz, Npz) = signed - negsigned.0 (REVERSE)
//   Z(3) + Nz(4) = Z(3-4) = Z(-1)
//
// - signed (Z, N0z) - negsigned (Nz, Npz) = signed + negsigned.0 (REVERSE)
//   Z(3) - Nz(4) = Z(3+4) = Z(7)
//
// * signed (Z, N0z) * negsigned (Nz, Npz) = signed.neg() * negsigned.0 (NEG)
//   Z(3) * Nz(4) = Z((-3)*4) = Z(-12)
//
// / signed (Z, N0z) / negsigned (Nz, Npz) = signed.neg() / negsigned.0 (NEG)
//   Z(12) / Nz(4) = Z((-12)/4) = Z(-3)
//
// % signed (Z, N0z) % negsigned (Nz, Npz) = signed % negsigned.0 (NOTHING! sign is LHS')
//   Z(12) % Nz(5) = Z(12/4) = Z(2)
//   Z(-12) % Nz(5) = Z((-12)/4) = Z(-2)
//
// // Pz & Nnz (fails if result negative)
//
// + unsigned (Pz, Nnz) + negsigned (Nz, Npz) = unsigned - negsigned.0 (REVERSE)
//   Pz(4) + Nz(3) = Pz(4-3) = Pz(1)
// + unsigned (Pz, Nnz) + signed (Z, N0z)
//   THINK:
//   1. doing nothing: (casting RHS to LHS as always)
//      Pz(4) + Z(3) = Pz(4+3) = Pz(1) (OK)
//      Pz(4) + Z(-3) (BUG if RHS is negative) (I think this is acceptable)
//      This is the easiest and most consistent solution.
//      To solve this, you have to convert your type before operations.
//
//   2. casting LHS to RHS, and then back again.
//      Pz(4) + Z(-3) => Z(4) + Z(-3) = Z(1) => Pz(1) (OK)
//      Pz(200) + Z(-3) = Z(200) + Z(-3)
//   3. casting to the immediate superior iSIZE.
//
// - unsigned (Pz, Nnz) - negsigned (Nz, Npz) = unsigned + negsigned.0 (REVERSE)
//   Pz(4) - Nz(3) = Pz(4+3) = Pz(7)
//
// * unsigned (Pz, Nnz) * negsigned (Nz, Npz) = unsigned - negsigned.0 (SWITCHES TYPE!)
//   Pz(4) * Nz(3) = Nz(4*3) = Nz(12)
//
//
// // Nz & Npz (fails if result positive)
//
//
macro_rules! impl_integer_op_integer {
    () => {
        // impl Add<Integer8> for Integer8 {
        //     type Output = Integer8;
        //     fn add(self, other: Integer8) -> Self::Output {
        //         Integer8(self.0 + other.0)
        //     }
        // }
    };
    // implements a binary operation.
    (binary $t_self:ty => $t_out:ty, $inner:ty; $op:ident, $other:ty) => {
        paste::paste! {
            impl $op<$other> for $t_self {
                type Output = $t_out;
                fn [< $op:lower >] (self, other: $other) -> Self::Output {
                    Self::Output::new(self.0.[< $op:lower >](other.0.unwrapped_as::<$inner>()))
                }
            }
        }
    };
}
impl_integer_op_integer![binary Integer8 => Integer8, i8; Add, Integer16];

/// Implements operations between an integer and primitives.
///
/// # Args
/// - `$t_self`: the self type.
/// - `$t_out`: the output type.
/// - `$op`: the `core::ops` operation identifier.
/// - `$method`: the `core::ops` operation method identifier.
/// - `$inner`: inner primitive for the self type.
/// - `$other`: the other primitive used for the operation (RHS), casted to $inner.
///
/// # Panics
/// Panics if the RHS operator value doesn't fit into $inner.
macro_rules! impl_integer_op_prim {
    // implements all operations for all primitives
    (@all $t_self:ty => $t_out:ty, $inner:ty) => {
        impl_integer_op_prim![@all_binary $t_self => $t_out, $inner; Add, Sub, Mul, Div, Rem];
        impl_integer_op_prim![unary $t_self => $t_out, $inner; Neg];
    };
    // implements several binary operations for all primitives.
    (@all_binary $t_self:ty => $t_out:ty, $inner:ty; $($op:ident),+) => {
        $(
            impl_integer_op_prim![@multi_binary $t_self => $t_out, $inner; $op:
                i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, f32, f64];
        )+
    };
    // implements one operation for several primitives.
    (@multi_binary $t_self:ty => $t_out:ty, $inner:ty; $op:ident: $($other:ty),+) => {
        $(
            impl_integer_op_prim![binary $t_self => $t_out, $inner; $op, $other];
        )+
    };
    // implements a binary operation for one primitive.
    (binary $t_self:ty => $t_out:ty, $inner:ty; $op:ident, $other:ty) => {
        paste::paste!{
            impl $op<$other> for $t_self {
                type Output = $t_out;
                fn [< $op:lower >] (self, other: $other) -> Self::Output {
                    Self::Output::new(self.0.[< $op:lower >](other.unwrapped_as::<$inner>()))
                }
            }
        }
    };
    // implements an unary operation for one primitive.
    (unary $t_self:ty => $t_out:ty, $inner:ty; $op:ident) => {
        paste::paste!{
            impl $op<> for $t_self {
                type Output = $t_out;
                fn [< $op:lower >] (self) -> Self::Output {
                    Self::Output::new(self.0.[< $op:lower >]())
                }
            }
        }
    };
}
impl_integer_op_prim![@all Integer8 => Integer8, i8];
impl_integer_op_prim![@all Integer16 => Integer16, i16];
impl_integer_op_prim![@all Integer32 => Integer32, i32];
impl_integer_op_prim![@all Integer64 => Integer64, i64];
impl_integer_op_prim![@all Integer128 => Integer128, i128];

/* trait */

///
pub trait Integers {}

#[cfg(test)]
// -----------------------------------------------------------------------------
mod tests {
    use super::{a::*, *};
    // use crate::traits::NegSigned;

    // use core::mem::size_of;
    #[cfg(feature = "std")]
    use std::panic::catch_unwind;

    // FIX
    #[test]
    #[cfg(feature = "std")]
    fn integer_constructors() {
        assert![catch_unwind(|| { Integer8::new(i8::MIN) }).is_ok()];
        assert![catch_unwind(|| { Integer8::new(i8::MAX) }).is_ok()];

        // not possible since only accepting same inner primitive type.
        // assert![catch_unwind(|| { Integer8::new(0_u128) }).is_ok()];
        // assert![catch_unwind(|| { Integer8::new(-200) }).is_err()];
        // assert![catch_unwind(|| { Integer8::new(200) }).is_err()];
        // assert_eq![Integer8::new(100.0_f64), Integer8::new(100)];

        // move to N0z
        // assert![catch_unwind(|| { NonZeroInteger8::new(0) }).is_err()];
    }

    #[test]
    fn integer_from() {
        // FIX: TODO impl NegSigned for Nz & Npz
        // assert_eq![Z16::new(-5), Z16::from(Nz8::new(5))];
    }

    // #[test]
    // #[cfg(feature = "std")]
    // fn integer_ops() {
    //     // assert![catch_unwind(|| { Integer8::new(1) + 100 }).is_ok()];
    //     // assert_eq![Integer8::new(101), Integer8::new(1) + Integer16::new(100)]; // FIX
    //     assert![catch_unwind(|| { Integer8::new(1) + 200 }).is_err()];
    // }
}
