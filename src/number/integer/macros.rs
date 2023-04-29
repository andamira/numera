// numera::number::integer::macros
//
//!
//
// TOC
//
// infallible conversions:
//   - impl_from_integer!
//   - impl_from_primitive!
//
// fallible conversions:
//   - impl_try_from_integer!
//   - impl_try_from_primitive!

/* infallible From conversions */

/// Implements From<`$from$from_size`> for `$for$for_size`.
///
/// # Args
/// - `$for`:       the base name of the target. e.g. `Integer`.
/// - `$p`:         target's inner corresponding primitive prefix: `i` or `u`.
///                  (this is only used for `_neg` variants, MAYBE IMPROVE).
/// - `$for_size`:  the bit size of the target. e.g. `32`.
/// - `$from`:    the base name of the origin. e.g. `PositiveInteger`.
/// - `$from_size`: a list of bit sizes of the origin. e.g. `8, 16`.
///
/// # Examples
/// ```ignore
/// impl_from_integer![int for: Integer + i + 32, from: Integer + 8, 16];
/// ```
macro_rules! impl_from_integer {
    // having an inner integer primitive
    (int
     for: $for:ident + $p:ident + $for_size:expr,
     from: $from:ident + $( $from_size:expr ),+) => {
        $(
            impl_from_integer![@int for: $for + $p + $for_size, from: $from + $from_size];
        )+
    };
    (@int
     for: $for:ident + $p:ident + $for_size:expr, from: $from:ident + $from_size:expr) => {
        paste::paste! {
            impl From<[< $from $from_size >]> for [< $for $for_size >] {
                fn from(from: [< $from $from_size >]) -> Self {
                    Self(from.0.into())
                }
            }
        }
    };

    // having an inner NonZero*
    (nonzero
     for: $for:ident + $p:ident + $for_size:expr,
     from: $from:ident + $( $from_size:expr ),+) => {
        $(
            impl_from_integer![@nonzero for: $for + $p + $for_size, from: $from + $from_size];
        )+
    };
    (@nonzero
     for: $for:ident + $p:ident + $for_size:expr, from: $from:ident + $from_size:expr) => {
        paste::paste! {
            impl From<[< $from $from_size >]> for [< $for $for_size >] {
                fn from(from: [< $from $from_size >]) -> Self {
                    #[cfg(feature = "safe")]
                    return Self::new(from.0.get().into()).unwrap();

                    #[cfg(not(feature = "safe"))]
                    // SAFETY: coming from a type that respects the invariant of not having 0
                    return unsafe { Self::new_unchecked(from.0.get().into()) };
                }
            }
        }
    };

    // having an unsigned inner primitive, representing only negative values.
    (int_neg
     for: $for:ident + $p:ident + $for_size:expr, from: $from:ident + $( $from_size:expr ),+) => {
        $(
            impl_from_integer![@int_neg for: $for + $p + $for_size, from: $from + $from_size];
        )+
    };
    (@int_neg
     for: $for:ident + $p:ident + $for_size:expr, from: $from:ident + $from_size:expr) => {
        paste::paste! {
            impl From<[< $from $from_size >]> for [< $for $for_size >] {
                fn from(from: [< $from $from_size >]) -> Self {
                    Self(Into::<[< $p $for_size >]>::into(from.0).neg())
                }
            }
        }
    };

    // having an unsigned inner NonZero*, representing only negative values.
    (nonzero_neg
     for: $for:ident + $p:ident + $for_size:expr, from: $from:ident + $( $from_size:expr ),+) => {
        $(
            impl_from_integer![@nonzero_neg
            for: $for + $p + $for_size, from: $from + $from_size];
        )+
    };
    (@nonzero_neg
     for: $for:ident + $p:ident + $for_size:expr, from: $from:ident + $from_size:expr) => {
        paste::paste! {
            impl From<[< $from $from_size >]> for [< $for $for_size >] {
                fn from(from: [< $from $from_size >]) -> Self {
                    #[cfg(feature = "safe")]
                    return
                        Self::new(Into::<[< $p $for_size >]>::into(from.0.get()).neg()).unwrap();

                    #[cfg(not(feature = "safe"))]
                    // SAFETY: coming from a type that respects the invariant of not having 0
                    return unsafe {
                        Self::new_unchecked(Into::<[< $p $for_size >]>::into(from.0.get()).neg())
                    };
                }
            }
        }
    };
}
pub(crate) use impl_from_integer;

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
    // having the same inner integer primitive
    (int
     for: $for:ident + $for_size:expr,
     from: $from_p:ident + $( $from_size:expr ),+
    ) => {
        $(
            impl_from_primitive![@int for: $for + $for_size, from: $from_p + $from_size];
        )+
    };

    (@int
     for: $for:ident + $for_size:expr,
     from: $from_p:ident + $from_size:expr
    ) => {
        paste::paste! {
            impl From<[< $from_p $from_size >]> for [< $for $for_size >] {
                fn from(from: [< $from_p $from_size >]) -> Self {
                    #[cfg(feature = "safe")]
                    return Self::new(from.into()).unwrap();

                    #[cfg(not(feature = "safe"))]
                    // SAFETY: all values should be valid
                    return unsafe { Self::new_unchecked(from.into()) };
                }
            }
        }
    };

    (nonzero
     for: $for:ident + $for_size:expr,
     from: $from_p:ident + $( $from_size:expr ),+
    ) => {
        $(
            impl_from_primitive![@nonzero for: $for + $for_size, from: $from_p + $from_size];
        )+
    };

    // having to convert from nonzero to core primitive
    (@nonzero
     for: $for:ident + $for_size:expr,
     from: $from_p:ident + $from_size:expr
    ) => {
        paste::paste! {
            impl From<[< $from_p $from_size >]> for [< $for $for_size >] {
                fn from(from: [< $from_p $from_size >]) -> Self {
                    #[cfg(feature = "safe")]
                    return Self::new(from.get().into()).unwrap();

                    #[cfg(not(feature = "safe"))]
                    // SAFETY: coming from a type that respects the invariant of not having 0
                    return unsafe { Self::new_unchecked(from.get().into()) };
                }
            }
        }
    };
}
pub(crate) use impl_from_primitive;

/* fallible TryFrom conversions */

/// Implements `TryFrom`<`$from$from_size`> for `$for$for_size`.
///
/// # Args
/// - `$for`:       the base name of the target. e.g. `Integer`.
/// - `$p`:         target's inner corresponding primitive prefix: `i` or `u`.
///                  (this is only used for `_neg` variants, MAYBE IMPROVE).
/// - `$for_size`:  the bit size of the target. e.g. `8`.
/// - `$from`:    the base name of the origin. e.g. `PositiveInteger`.
/// - `$from_size`: a list of bit sizes of the origin. e.g. `8, 16`.
///
/// # Examples
/// ```ignore
/// impl_try_from_integer![int for: Integer + i + 8, from: Integer + 8, 16];
/// ```
macro_rules! impl_try_from_integer {
    // having an inner integer primitive
    (int
     for: $for:ident + $p:ident + $for_size:expr,
     from: $from:ident + $( $from_size:expr ),+) => {
        $(
            impl_try_from_integer![@int for: $for + $p + $for_size, from: $from + $from_size];
        )+
    };
    (@int
     for: $for:ident + $p:ident + $for_size:expr, from: $from:ident + $from_size:expr) => {
        paste::paste! {
            impl TryFrom<[< $from $from_size >]> for [< $for $for_size >] {
                type Error = $crate::error::NumeraError;
                fn try_from(from: [<$from$from_size>])
                    -> $crate::error::NumeraResult<[<$for$for_size>]> {

                    return Ok(Self(from.0.try_into()?));
                    // MAYBE DELETE
                    // #[cfg(feature = "safe")]
                    // return Self::new(from.0.try_into()?);
                    // #[cfg(not(feature = "safe"))]
                    // // SAFETY: all ok results of try_from should be valid
                    // return Ok(unsafe { Self::new_unchecked(from.0.try_into()?) });
                }
            }
        }
    };

    // having an inner NonZero*
    (nonzero
     for: $for:ident + $p:ident + $for_size:expr,
     from: $from:ident + $( $from_size:expr ),+) => {
        $(
            impl_try_from_integer![@nonzero for: $for + $p + $for_size, from: $from + $from_size];
        )+
    };
    (@nonzero
     for: $for:ident + $p:ident + $for_size:expr, from: $from:ident + $from_size:expr) => {
        paste::paste! {
            impl TryFrom<[< $from $from_size >]> for [< $for $for_size >] {
                type Error = $crate::error::NumeraError;
                fn try_from(from: [<$from$from_size>])
                    -> $crate::error::NumeraResult<[<$for$for_size>]> {

                    // return Ok(Self(from.0.get().try_into()?)); // TODO CHECK (I don't think'll work with n0z)

                    #[cfg(feature = "safe")]
                    return Self::new(from.0.get().try_into()?);

                    #[cfg(not(feature = "safe"))]
                    // SAFETY: all ok results of try_from should be valid
                    return Ok(unsafe { Self::new_unchecked(from.0.get().try_into()?) });
                }
            }
        }
    };

    // having an unsigned inner primitive, representing only negative values.
    (int_neg
     for: $for:ident + $p:ident + $for_size:expr, from: $from:ident + $( $from_size:expr ),+) => {
        $(
            impl_try_from_integer![@int_neg for: $for + $p + $for_size, from: $from + $from_size];
        )+
    };
    (@int_neg
     for: $for:ident + $p:ident + $for_size:expr, from: $from:ident + $from_size:expr) => {
        paste::paste! {
            impl TryFrom<[< $from $from_size >]> for [< $for $for_size >] {
                type Error = $crate::error::NumeraError;
                fn try_from(from: [<$from$from_size>])
                    -> $crate::error::NumeraResult<[<$for$for_size>]> {

                    Ok(Self(TryInto::<[<$p$for_size >]>::try_into(from.0)?.neg()))
                }
            }
        }
    };

    // having an unsigned inner NonZero*, representing only negative values.
    (nonzero_neg
     for: $for:ident + $p:ident + $for_size:expr, from: $from:ident + $( $from_size:expr ),+) => {
        $(
            impl_try_from_integer![@nonzero_neg
            for: $for + $p + $for_size, from: $from + $from_size];
        )+
    };
    (@nonzero_neg
     for: $for:ident + $p:ident + $for_size:expr, from: $from:ident + $from_size:expr) => {
        paste::paste! {
            impl TryFrom<[< $from $from_size >]> for [< $for $for_size >] {
                type Error = $crate::error::NumeraError;
                fn try_from(from: [<$from$from_size>])
                    -> $crate::error::NumeraResult<[<$for$for_size>]> {
                    #[cfg(feature = "safe")]
                    return
                        Self::new(TryInto::<[< $p $for_size >]>::try_into(from.0.get())?.neg());

                    #[cfg(not(feature = "safe"))]
                    // SAFETY: all ok results of try_from should be valid
                    return unsafe {
                        Ok(Self::new_unchecked(
                                TryInto::<[< $p $for_size >]>::try_into(from.0.get())?.neg()
                        ))
                    };
                }
            }
        }
    };
}
pub(crate) use impl_try_from_integer;

/// Implements `TryFrom`<`$from $from_size`> for `$for$for_size`.
///
/// # Args
/// - `$for`:       the base name of the target. e.g. `NonZeroInteger`.
/// - `$for_size`:  the bit size of the target. e.g. `16`.
/// - `$from`:    the base name of the origin. e.g. `u`.
/// - `$from_size`: a list of bit sizes of the origin. e.g. `8, 16`.
///
/// # Examples
/// ```ignore
/// impl_try_from_primitive![many for: Integer + 8, from: u + 8, 16, 32, 64, 128];
/// ```
macro_rules! impl_try_from_primitive {
    // having the same inner integer primitive
    (int
     for: $for:ident + $for_size:expr,
     from: $from:ident + $( $from_size:expr ),+
    ) => {
        $(
            impl_try_from_primitive![@int for: $for + $for_size, from: $from + $from_size];
        )+
    };

    (@int
     for: $for:ident + $for_size:expr,
     from: $from:ident + $from_size:expr
    ) => {
        paste::paste! {
            impl TryFrom<[< $from $from_size >]> for [< $for $for_size >] {
                type Error = $crate::error::NumeraError;
                fn try_from(from: [<$from$from_size>])
                    -> $crate::error::NumeraResult<[<$for$for_size>]> {

                    #[cfg(feature = "safe")]
                    return Self::new(from.try_into()?);

                    #[cfg(not(feature = "safe"))]
                    // SAFETY: all ok results of try_from should be valid
                    return Ok(unsafe { Self::new_unchecked(from.try_into()?) });
                }
            }
        }
    };

    (nonzero
     for: $for:ident + $for_size:expr,
     from: $from:ident + $( $from_size:expr ),+
    ) => {
        $(
            impl_try_from_primitive![@nonzero for: $for + $for_size, from: $from + $from_size];
        )+
    };

    // having to convert from nonzero to core primitive
    (@nonzero
     for: $for:ident + $for_size:expr,
     from: $from:ident + $from_size:expr
    ) => {
        paste::paste! {
            impl TryFrom<[< $from $from_size >]> for [< $for $for_size >] {
                type Error = $crate::error::NumeraError;
                fn try_from(from: [<$from$from_size>])
                    -> $crate::error::NumeraResult<[<$for$for_size>]> {

                    #[cfg(feature = "safe")]
                    return Self::new(from.get().try_into()?);

                    #[cfg(not(feature = "safe"))]
                    // SAFETY: all ok results of try_from should be valid
                    return Ok(unsafe { Self::new_unchecked(from.get().try_into()?) });
                }
            }
        }
    };
}
pub(crate) use impl_try_from_primitive;
