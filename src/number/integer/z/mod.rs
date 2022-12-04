// numera::integer::z
//
//! $\Z$
//

mod define_sized;
pub use define_sized::{Integer128, Integer16, Integer32, Integer64, Integer8};

#[cfg(feature = "ibig")]
pub(super) mod define_big;

// mod impl_ops;

mod impl_integer;

/* impl From */

#[rustfmt::skip]
mod impls_from {
    use super::*;
    use super::super::impl_from;
    use crate::number::traits::Number;

    /* from smaller sized integers to larger sized integers */

    impl_from![many_int for: Integer + i + 16, from: Integer + 8];
    impl_from![many_int for: Integer + i + 32, from: Integer + 8, 16];
    impl_from![many_int for: Integer + i + 64, from: Integer + 8, 16, 32];
    impl_from![many_int for: Integer + i + 128, from: Integer + 8, 16, 32, 64];

    // TODO add other kinds of integers:
    //
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
