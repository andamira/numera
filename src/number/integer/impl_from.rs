// numera::number::integer::impl_from
//
//!
//

/// Implements From<`$from$from_size`> for `$for$for_size`.
///
/// # Args
/// - `$p`:
macro_rules! impl_from {
    // having an inner integer primitive
    (many_int for: $for:ident + $p:ident + $for_size:expr, from: $from:ident + $( $from_size:expr ),+) => {
        $(
            impl_from![int for: $for + $p + $for_size, from: $from + $from_size];
        )+
    };
    (int
     for: $for:ident + $p:ident + $for_size:expr, from: $from:ident + $from_size:expr) => {
        paste::paste! {
            impl From<[< $from $from_size >]> for [< $for $for_size >] {
                fn from(from: [< $from $from_size >]) -> Self {
                    // SAFETY
                    unsafe { Self::new_unchecked(from.0.into()) }
                }
            }
        }
    };

    // having an inner NonZero*
    (many_nonzero
     for: $for:ident + $p:ident + $for_size:expr, from: $from:ident + $( $from_size:expr ),+) => {
        $(
            impl_from![nonzero for: $for + $p + $for_size, from: $from + $from_size];
        )+
    };
    (nonzero
     for: $for:ident + $p:ident + $for_size:expr, from: $from:ident + $from_size:expr) => {
        paste::paste! {
            impl From<[< $from $from_size >]> for [< $for $for_size >] {
                fn from(from: [< $from $from_size >]) -> Self {
                    Self::new(from.0.get().into())
                }
            }
        }
    };

    // having an unsigned inner primitive, representing only negative values.
    (many_int_neg
     for: $for:ident + $p:ident + $for_size:expr, from: $from:ident + $( $from_size:expr ),+) => {
        $(
            impl_from![int_neg for: $for + $p + $for_size, from: $from + $from_size];
        )+
    };
    (int_neg
     for: $for:ident + $p:ident + $for_size:expr, from: $from:ident + $from_size:expr) => {
        paste::paste! {
            impl From<[< $from $from_size >]> for [< $for $for_size >] {
                fn from(from: [< $from $from_size >]) -> Self {
                    Self::new( Into::<[< $p $for_size >]>::into(from.0).neg())
                }
            }
        }
    };

    // having an unsigned inner NonZero*, representing only negative values.
    (many_nonzero_neg
     for: $for:ident + $p:ident + $for_size:expr, from: $from:ident + $( $from_size:expr ),+) => {
        $(
            impl_from![nonzero_neg
            for: $for + $p + $for_size, from: $from + $from_size];
        )+
    };
    (nonzero_neg
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
pub(crate) use impl_from;
