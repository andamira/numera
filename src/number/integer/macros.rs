// numera::number::integer::macros
//
//!
//
// TOC
// - impl_from_integer
// - impl_from_primitive

/// Implements From<`$from$from_size`> for `$for$for_size`.
///
/// # Args
/// - `$p`:
///
/// # Examples
/// ```e.g.
/// impl_from_integer![many_int for: Integer + i + 32, from: Integer + 8, 16];
/// ```
macro_rules! impl_from_integer {
    // having an inner integer primitive
    (many_int for: $for:ident + $p:ident + $for_size:expr, from: $from:ident + $( $from_size:expr ),+) => {
        $(
            impl_from_integer![int for: $for + $p + $for_size, from: $from + $from_size];
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
            impl_from_integer![nonzero for: $for + $p + $for_size, from: $from + $from_size];
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
            impl_from_integer![int_neg for: $for + $p + $for_size, from: $from + $from_size];
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
            impl_from_integer![nonzero_neg
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
pub(crate) use impl_from_integer;

/// Implements From<`$from_p $from_size`> for `$for$for_size`.
///
/// # Args
/// - $for:
///
/// # Examples
/// ```e.g.
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
                    Self::new(from.into()).unwrap()
                }
            }
        }
    };
}
pub(crate) use impl_from_primitive;