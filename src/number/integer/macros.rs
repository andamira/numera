// numera::number::integer::macros
//
//!
//
// TOC
//
// infallible conversions:
//   - from_integer!
//   - from_primitive!
//
// fallible conversions:
//   - try_from_integer!
//   - try_from_primitive!
//   - try_from_any!

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
/// from_integer![int for: Integer + i + 32, from: Integer + 8, 16];
/// ```
///
/// # Branches ids
/// - `int`
/// - `nonzero`
/// - `int_neg`
/// - `nonzero_neg`
macro_rules! from_integer {
    // when `from` has an inner integer primitive
    //
    // Used by:
    // - for: Z     from: Z, Nnz, Npz
    // - for: Nnz   from: Nnz
    // - for: Npz   from: Npz
    (int
     for: $for:ident + $for_size:expr,
     from: $from:ident + $( $from_size:expr ),+) => {
        $(
            from_integer![@int for: $for + $for_size, from: $from + $from_size];
        )+
    };
    (@int
     for: $for:ident + $for_size:expr, from: $from:ident + $from_size:expr) => {
        devela::paste! {
            impl From<[<$from$from_size>]> for [<$for$for_size>] {
                #[inline]
                fn from(from: [<$from$from_size>]) -> Self {
                    Self(from.0.into())
                }
            }
            impl From<&[<$from$from_size>]> for [<$for$for_size>] {
                #[inline]
                fn from(from: &[<$from$from_size>]) -> Self {
                    Self(from.0.into())
                }
            }
            impl From<&mut [<$from$from_size>]> for [<$for$for_size>] {
                #[inline]
                fn from(from: &mut [<$from$from_size>]) -> Self {
                    Self(from.0.into())
                }
            }
        }
    };

    // when `from` has an inner NonZero*
    //
    // Used by:
    // - for: Z     from: N0z, Pz
    // - for: N0z   from: N0z, Pz
    // - for: Nnz   from: Pz
    // - for: Pz    from: Pz
    // - for: Npz   from: Nz        (negative to negative is OK)
    // - for: Nz    from: Nz        (negative to negative is OK)
    (nonzero
     for: $for:ident + $for_size:expr,
     from: $from:ident + $( $from_size:expr ),+) => {
        $(
            from_integer![@nonzero for: $for + $for_size, from: $from + $from_size];
        )+
    };
    (@nonzero
     for: $for:ident + $for_size:expr, from: $from:ident + $from_size:expr) => {
        devela::paste! {
            impl From<[<$from$from_size>]> for [<$for$for_size>] {
                #[inline]
                fn from(from: [<$from$from_size>]) -> Self {
                    #[cfg(feature = "safe")]
                    return Self::from_parts(from.0.get().into()).unwrap();

                    #[cfg(not(feature = "safe"))]
                    // SAFETY: coming from a type that respects the invariant of not having 0
                    return unsafe { Self::from_parts_unchecked(from.0.get().into()) };
                }
            }
            impl From<&[<$from$from_size>]> for [<$for$for_size>] {
                #[inline]
                fn from(from: &[<$from$from_size>]) -> Self {
                    #[cfg(feature = "safe")]
                    return Self::from_parts(from.0.get().into()).unwrap();

                    #[cfg(not(feature = "safe"))]
                    // SAFETY: coming from a type that respects the invariant of not having 0
                    return unsafe { Self::from_parts_unchecked(from.0.get().into()) };
                }
            }
            impl From<&mut [<$from$from_size>]> for [<$for$for_size>] {
                #[inline]
                fn from(from: &mut [<$from$from_size>]) -> Self {
                    #[cfg(feature = "safe")]
                    return Self::from_parts(from.0.get().into()).unwrap();

                    #[cfg(not(feature = "safe"))]
                    // SAFETY: coming from a type that respects the invariant of not having 0
                    return unsafe { Self::from_parts_unchecked(from.0.get().into()) };
                }
            }
        }
    };

    // when `from` has an unsigned inner primitive that represents only negative values,
    // which has to be negated in the conversion
    //
    // - for: Z     from: Npz
    (int_neg
     for: $for:ident + $p:ident + $for_size:expr, from: $from:ident + $( $from_size:expr ),+) => {
        $(
            from_integer![@int_neg for: $for + $p + $for_size, from: $from + $from_size];
        )+
    };
    (@int_neg
     for: $for:ident + $p:ident + $for_size:expr, from: $from:ident + $from_size:expr) => {
        devela::paste! {
            impl From<[<$from$from_size>]> for [<$for$for_size>] {
                #[inline]
                fn from(from: [<$from$from_size>]) -> Self {
                    Self(Into::<[<$p$for_size>]>::into(from.0).neg())
                }
            }
            impl From<&[<$from$from_size>]> for [<$for$for_size>] {
                #[inline]
                fn from(from: &[<$from$from_size>]) -> Self {
                    Self(Into::<[<$p$for_size>]>::into(from.0).neg())
                }
            }
            impl From<&mut [<$from$from_size>]> for [<$for$for_size>] {
                #[inline]
                fn from(from: &mut [<$from$from_size>]) -> Self {
                    Self(Into::<[<$p$for_size>]>::into(from.0).neg())
                }
            }
        }
    };

    // when `from` has an unsigned inner NonZero* primitive that represents only negative values,
    // which has to be negated in the conversion
    //
    // - for: Z     from: Nz
    // - for: N0z   from: Nz
    (nonzero_neg
     for: $for:ident + $p:ident + $for_size:expr, from: $from:ident + $( $from_size:expr ),+) => {
        $(
            from_integer![@nonzero_neg
            for: $for + $p + $for_size, from: $from + $from_size];
        )+
    };
    (@nonzero_neg
     for: $for:ident + $p:ident + $for_size:expr, from: $from:ident + $from_size:expr) => {
        devela::paste! {
            impl From<[<$from$from_size>]> for [<$for$for_size>] {
                #[inline]
                fn from(from: [<$from$from_size>]) -> Self {
                    #[cfg(feature = "safe")]
                    return
                        Self::from_parts(Into::<[<$p$for_size>]>::into(from.0.get()).neg()).unwrap();

                    #[cfg(not(feature = "safe"))]
                    // SAFETY: coming from a type that respects the invariant of not having 0
                    return unsafe {
                        Self::from_parts_unchecked(Into::<[<$p$for_size>]>::into(from.0.get()).neg())
                    };
                }
            }
            impl From<&[<$from$from_size>]> for [<$for$for_size>] {
                #[inline]
                fn from(from: &[<$from$from_size>]) -> Self {
                    #[cfg(feature = "safe")]
                    return
                        Self::from_parts(Into::<[<$p$for_size>]>::into(from.0.get()).neg()).unwrap();

                    #[cfg(not(feature = "safe"))]
                    // SAFETY: coming from a type that respects the invariant of not having 0
                    return unsafe {
                        Self::from_parts_unchecked(Into::<[<$p$for_size>]>::into(from.0.get()).neg())
                    };
                }
            }
            impl From<&mut [<$from$from_size>]> for [<$for$for_size>] {
                #[inline]
                fn from(from: &mut [<$from$from_size>]) -> Self {
                    #[cfg(feature = "safe")]
                    return
                        Self::from_parts(Into::<[<$p$for_size>]>::into(from.0.get()).neg()).unwrap();

                    #[cfg(not(feature = "safe"))]
                    // SAFETY: coming from a type that respects the invariant of not having 0
                    return unsafe {
                        Self::from_parts_unchecked(Into::<[<$p$for_size>]>::into(from.0.get()).neg())
                    };
                }
            }
        }
    };
}
pub(crate) use from_integer;

/// Implements From<`$from_p$from_size`> for `$for$for_size`.
///
/// # Args
/// - `$for`:       the base name of the target. e.g. `NonZeroInteger`.
/// - `$for_size`:  the bit size of the target. e.g. `16`.
/// - `$from_p`:    the base name of the origin. e.g. `u`.
/// - `$from_size`: a list of bit sizes of the origin. e.g. `8, 16`.
///
/// # Examples
/// ```ignore
/// from_primitive![many for: Integer + 16, from: u + 8];
/// from_primitive![many for: Integer + 16, from: i + 8, 16];
/// ```
///
/// # Branches ids
/// - `int`
/// - `nonzero`
macro_rules! from_primitive {
    // when `from` is the same integer primitive than the inner part of `for`.
    //
    // - for: Z     from: u, i
    // - for: Nnz   from: u
    (int
     for: $for:ident + $for_size:expr,
     from: $from_p:ident + $( $from_size:expr ),+
    ) => {
        $(
            from_primitive![@int for: $for + $for_size, from: $from_p + $from_size];
        )+
    };
    (@int
     for: $for:ident + $for_size:expr,
     from: $from_p:ident + $from_size:expr
    ) => {
        devela::paste! {
            impl From<[<$from_p$from_size>]> for [<$for$for_size>] {
                #[inline]
                fn from(from: [<$from_p$from_size>]) -> Self {
                    #[cfg(feature = "safe")]
                    return Self::from_parts(from.into()).unwrap();

                    #[cfg(not(feature = "safe"))]
                    // SAFETY: all values should be valid
                    return unsafe { Self::from_parts_unchecked(from.into()) };
                }
            }
            impl From<&[<$from_p$from_size>]> for [<$for$for_size>] {
                #[inline]
                fn from(from: &[<$from_p$from_size>]) -> Self {
                    #[cfg(feature = "safe")]
                    return Self::from_parts((*from).into()).unwrap();

                    #[cfg(not(feature = "safe"))]
                    // SAFETY: all values should be valid
                    return unsafe { Self::from_parts_unchecked((*from).into()) };
                }
            }
            impl From<&mut [<$from_p$from_size>]> for [<$for$for_size>] {
                #[inline]
                fn from(from: &mut [<$from_p$from_size>]) -> Self {
                    #[cfg(feature = "safe")]
                    return Self::from_parts((*from).into()).unwrap();

                    #[cfg(not(feature = "safe"))]
                    // SAFETY: all values should be valid
                    return unsafe { Self::from_parts_unchecked((*from).into()) };
                }
            }
        }
    };

    // when `from` is a NonZero* primitive which has to be converted to primitive.
    //
    // - for: Z     from: NonZeroU, NonZeroI
    // - for: N0z   from: NonZeroU, NonZeroI
    // - for: Pz    from: NonZeroU
    // - for: Nnz   from: NonZeroU
    (nonzero
     for: $for:ident + $for_size:expr,
     from: $from_p:ident + $( $from_size:expr ),+
    ) => {
        $(
            from_primitive![@nonzero for: $for + $for_size, from: $from_p + $from_size];
        )+
    };
    (@nonzero
     for: $for:ident + $for_size:expr,
     from: $from_p:ident + $from_size:expr
    ) => {
        devela::paste! {
            impl From<[<$from_p$from_size>]> for [<$for$for_size>] {
                #[inline]
                fn from(from: [<$from_p$from_size>]) -> Self {
                    #[cfg(feature = "safe")]
                    return Self::from_parts(from.get().into()).unwrap();

                    #[cfg(not(feature = "safe"))]
                    // SAFETY: coming from a type that respects the invariant of not having 0
                    return unsafe { Self::from_parts_unchecked(from.get().into()) };
                }
            }
            impl From<&[<$from_p$from_size>]> for [<$for$for_size>] {
                #[inline]
                fn from(from: &[<$from_p$from_size>]) -> Self {
                    #[cfg(feature = "safe")]
                    return Self::from_parts((*from).get().into()).unwrap();

                    #[cfg(not(feature = "safe"))]
                    // SAFETY: coming from a type that respects the invariant of not having 0
                    return unsafe { Self::from_parts_unchecked((*from).get().into()) };
                }
            }
            impl From<&mut [<$from_p$from_size>]> for [<$for$for_size>] {
                #[inline]
                fn from(from: &mut [<$from_p$from_size>]) -> Self {
                    #[cfg(feature = "safe")]
                    return Self::from_parts((*from).get().into()).unwrap();

                    #[cfg(not(feature = "safe"))]
                    // SAFETY: coming from a type that respects the invariant of not having 0
                    return unsafe { Self::from_parts_unchecked((*from).get().into()) };
                }
            }
        }
    };
}
pub(crate) use from_primitive;

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
/// try_from_integer![int for: Integer + i + 8, from: Integer + 8, 16];
/// ```
///
/// # Branches ids
/// - `int`
/// - `int_neg`
/// - `int_new`
/// - `int_new_neg`
/// - `nonzero`
/// - `nonzero_neg`
/// - `neg_int`
/// - `neg_nonzero`
/// - `new_neg_int`
/// - `new_neg_nonzero`
macro_rules! try_from_integer {
    // when `from` has an inner integer primitive.
    //
    // Used by:
    // - for: Nnz   from: Z, Nnz,
    // - for: Z     from: Z, Nnz, Npz
    // - for: Npz   from: Npz
    // - for: Pz    from: N0z
    (int
     for: $for:ident + $for_size:expr,
     from: $from:ident + $( $from_size:expr ),+) => {
        $(
            try_from_integer![@int for: $for + $for_size, from: $from + $from_size];
        )+
    };
    (@int
     for: $for:ident + $for_size:expr, from: $from:ident + $from_size:expr) => {
        devela::paste! {
            impl TryFrom<[<$from$from_size>]> for [<$for$for_size>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: [<$from$from_size>])
                    -> $crate::error::NumeraResult<[<$for$for_size>]> {
                    Ok(Self(from.0.try_into()?))
                }
            }
            impl TryFrom<&[<$from$from_size>]> for [<$for$for_size>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: &[<$from$from_size>])
                    -> $crate::error::NumeraResult<[<$for$for_size>]> {
                    Ok(Self(from.0.try_into()?))
                }
            }
            impl TryFrom<&mut [<$from$from_size>]> for [<$for$for_size>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: &mut [<$from$from_size>])
                    -> $crate::error::NumeraResult<[<$for$for_size>]> {
                    Ok(Self(from.0.try_into()?))
                }
            }
        }
    };

    // when `from` has an unsigned inner primitive representing only negative values,
    // which has to be negated in the conversion.
    //
    // Used by:
    // - for: Z     from: Npz
    (int_neg
     for: $for:ident + $p:ident + $for_size:expr, from: $from:ident + $( $from_size:expr ),+) => {
        $(
            try_from_integer![@int_neg for: $for + $p + $for_size, from: $from + $from_size];
        )+
    };
    (@int_neg
     for: $for:ident + $p:ident + $for_size:expr, from: $from:ident + $from_size:expr) => {
        devela::paste! {
            impl TryFrom<[<$from$from_size>]> for [<$for$for_size>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: [<$from$from_size>])
                    -> $crate::error::NumeraResult<[<$for$for_size>]> {
                    Ok(Self(TryInto::<[<$p$for_size>]>::try_into(from.0)?.neg()))
                }
            }
            impl TryFrom<&[<$from$from_size>]> for [<$for$for_size>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: &[<$from$from_size>])
                    -> $crate::error::NumeraResult<[<$for$for_size>]> {
                    Ok(Self(TryInto::<[<$p$for_size>]>::try_into(from.0)?.neg()))
                }
            }
            impl TryFrom<&mut [<$from$from_size>]> for [<$for$for_size>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: &mut [<$from$from_size>])
                    -> $crate::error::NumeraResult<[<$for$for_size>]> {
                    Ok(Self(TryInto::<[<$p$for_size>]>::try_into(from.0)?.neg()))
                }
            }
        }
    };

    // when `from` has an inner integer primitive,
    // and we have to use the `for`::new constructor.
    //
    // Used by:
    // - for: N0z   from: Z, Nnz
    // - for: Pz    from: Z
    (int_new
     for: $for:ident + $for_size:expr,
     from: $from:ident + $( $from_size:expr ),+) => {
        $(
            try_from_integer![@int_new for: $for + $for_size, from: $from + $from_size];
        )+
    };
    (@int_new
     for: $for:ident + $for_size:expr, from: $from:ident + $from_size:expr) => {
        devela::paste! {
            impl TryFrom<[<$from$from_size>]> for [<$for$for_size>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: [<$from$from_size>])
                    -> $crate::error::NumeraResult<[<$for$for_size>]> {
                    Self::new(from.0.try_into()?)
                }
            }
            impl TryFrom<&[<$from$from_size>]> for [<$for$for_size>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: &[<$from$from_size>])
                    -> $crate::error::NumeraResult<[<$for$for_size>]> {
                    Self::new(from.0.try_into()?)
                }
            }
            impl TryFrom<&mut [<$from$from_size>]> for [<$for$for_size>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: &mut [<$from$from_size>])
                    -> $crate::error::NumeraResult<[<$for$for_size>]> {
                    Self::new(from.0.try_into()?)
                }
            }
        }
    };

    // when `from` has an unsigned inner primitive representing only negative values,
    // and we have to use the `for`::new constructor.
    //
    // Used by:
    // - for: N0z   from: Npz
    (int_new_neg
     for: $for:ident + $for_size:expr,
     from: $from:ident + $( $from_size:expr ),+) => {
        $(
            try_from_integer![@int_new_neg for: $for + $for_size, from: $from + $from_size];
        )+
    };
    (@int_new_neg
     for: $for:ident + $for_size:expr, from: $from:ident + $from_size:expr) => {
        devela::paste! {
            impl TryFrom<[<$from$from_size>]> for [<$for$for_size>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: [<$from$from_size>])
                    -> $crate::error::NumeraResult<[<$for$for_size>]> {
                    Ok(Self::new(from.0.try_into()?)?.neg())
                }
            }
            impl TryFrom<&[<$from$from_size>]> for [<$for$for_size>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: &[<$from$from_size>])
                    -> $crate::error::NumeraResult<[<$for$for_size>]> {
                    Ok(Self::new(from.0.try_into()?)?.neg())
                }
            }
            impl TryFrom<&mut [<$from$from_size>]> for [<$for$for_size>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: &mut [<$from$from_size>])
                    -> $crate::error::NumeraResult<[<$for$for_size>]> {
                    Ok(Self::new(from.0.try_into()?)?.neg())
                }
            }
        }
    };

    // when `from` has an inner NonZero*.
    //
    // Used by:
    // - for: Nnz   from: N0z, Pz
    // - for: Z     from: N0z, Pz
    // - for: Npz   from: Nz
    // - for: N0z   from: N0z, Pz
    // - for: Pz    from: Pz
    // - for: Nz    from: Nz
    (nonzero
     for: $for:ident + $for_size:expr,
     from: $from:ident + $( $from_size:expr ),+) => {
        $(
            try_from_integer![@nonzero for: $for + $for_size, from: $from + $from_size];
        )+
    };
    (@nonzero
     for: $for:ident + $for_size:expr, from: $from:ident + $from_size:expr) => {
        devela::paste! {
            impl TryFrom<[<$from$from_size>]> for [<$for$for_size>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: [<$from$from_size>])
                    -> $crate::error::NumeraResult<[<$for$for_size>]> {
                    #[cfg(feature = "safe")]
                    return Self::from_parts(from.0.get().try_into()?);

                    #[cfg(not(feature = "safe"))]
                    // SAFETY: all ok results of try_from should be valid
                    return Ok(unsafe { Self::from_parts_unchecked(from.0.get().try_into()?) });
                }
            }
            impl TryFrom<&[<$from$from_size>]> for [<$for$for_size>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: &[<$from$from_size>])
                    -> $crate::error::NumeraResult<[<$for$for_size>]> {
                    #[cfg(feature = "safe")]
                    return Self::from_parts(from.0.get().try_into()?);

                    #[cfg(not(feature = "safe"))]
                    // SAFETY: all ok results of try_from should be valid
                    return Ok(unsafe { Self::from_parts_unchecked(from.0.get().try_into()?) });
                }
            }
            impl TryFrom<&mut [<$from$from_size>]> for [<$for$for_size>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: &mut [<$from$from_size>])
                    -> $crate::error::NumeraResult<[<$for$for_size>]> {
                    #[cfg(feature = "safe")]
                    return Self::from_parts(from.0.get().try_into()?);

                    #[cfg(not(feature = "safe"))]
                    // SAFETY: all ok results of try_from should be valid
                    return Ok(unsafe { Self::from_parts_unchecked(from.0.get().try_into()?) });
                }
            }
        }
    };

    // when `from` has an unsigned inner NonZero* representing only negative values,
    // which has to be negated in the conversion.
    //
    // - for: Z     from: Nz
    // - for: N0z   from: Nz
    (nonzero_neg
     for: $for:ident + $p:ident + $for_size:expr, from: $from:ident + $( $from_size:expr ),+) => {
        $(
            try_from_integer![@nonzero_neg
            for: $for + $p + $for_size, from: $from + $from_size];
        )+
    };
    (@nonzero_neg
     for: $for:ident + $p:ident + $for_size:expr, from: $from:ident + $from_size:expr) => {
        devela::paste! {
            impl TryFrom<[<$from$from_size>]> for [<$for$for_size>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: [<$from$from_size>])
                    -> $crate::error::NumeraResult<[<$for$for_size>]> {
                    #[cfg(feature = "safe")]
                    return
                        Self::from_parts(TryInto::<[<$p$for_size>]>::try_into(from.0.get())?.neg());

                    #[cfg(not(feature = "safe"))]
                    // SAFETY: all ok results of try_from should be valid
                    return unsafe {
                        Ok(Self::from_parts_unchecked(
                                TryInto::<[<$p$for_size>]>::try_into(from.0.get())?.neg()
                        ))
                    };
                }
            }
            impl TryFrom<&[<$from$from_size>]> for [<$for$for_size>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: &[<$from$from_size>])
                    -> $crate::error::NumeraResult<[<$for$for_size>]> {
                    #[cfg(feature = "safe")]
                    return
                        Self::from_parts(TryInto::<[<$p$for_size>]>::try_into(from.0.get())?.neg());

                    #[cfg(not(feature = "safe"))]
                    // SAFETY: all ok results of try_from should be valid
                    return unsafe {
                        Ok(Self::from_parts_unchecked(
                                TryInto::<[<$p$for_size>]>::try_into(from.0.get())?.neg()
                        ))
                    };
                }
            }
            impl TryFrom<&mut [<$from$from_size>]> for [<$for$for_size>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: &mut [<$from$from_size>])
                    -> $crate::error::NumeraResult<[<$for$for_size>]> {
                    #[cfg(feature = "safe")]
                    return
                        Self::from_parts(TryInto::<[<$p$for_size>]>::try_into(from.0.get())?.neg());

                    #[cfg(not(feature = "safe"))]
                    // SAFETY: all ok results of try_from should be valid
                    return unsafe {
                        Ok(Self::from_parts_unchecked(
                                TryInto::<[<$p$for_size>]>::try_into(from.0.get())?.neg()
                        ))
                    };
                }
            }
        }
    };

    // when `for` can not represent negative values,
    // and `from` is an Integer.
    //
    // Used by:
    // - for: Npz   from: Z
    (neg_int
     for: $for:ident + $for_size:expr,
     from: $from:ident + $( $from_size:expr ),+) => {
        $(
            try_from_integer![@neg_int for: $for + $for_size, from: $from + $from_size];
        )+
    };
    (@neg_int
     for: $for:ident + $for_size:expr, from: $from:ident + $from_size:expr) => {
        devela::paste! {
            impl TryFrom<[<$from$from_size>]> for [<$for$for_size>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: [<$from$from_size>])
                    -> $crate::error::NumeraResult<[<$for$for_size>]> {
                    Ok(Self((-from.0).try_into()?))
                }
            }
            impl TryFrom<&[<$from$from_size>]> for [<$for$for_size>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: &[<$from$from_size>])
                    -> $crate::error::NumeraResult<[<$for$for_size>]> {
                    Ok(Self((-from.0).try_into()?))
                }
            }
            impl TryFrom<&mut [<$from$from_size>]> for [<$for$for_size>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: &mut [<$from$from_size>])
                    -> $crate::error::NumeraResult<[<$for$for_size>]> {
                    Ok(Self((-from.0).try_into()?))
                }
            }
        }
    };

    // when `for` can not represent negative values,
    // and `from` is a NonZeroInteger.
    //
    // Used by:
    // - for: Npz   from: N0z
    (neg_nonzero
     for: $for:ident + $for_size:expr,
     from: $from:ident + $( $from_size:expr ),+) => {
        $(
            try_from_integer![@neg_nonzero for: $for + $for_size, from: $from + $from_size];
        )+
    };
    (@neg_nonzero
     for: $for:ident + $for_size:expr, from: $from:ident + $from_size:expr) => {
        devela::paste! {
            impl TryFrom<[<$from$from_size>]> for [<$for$for_size>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: [<$from$from_size>])
                    -> $crate::error::NumeraResult<[<$for$for_size>]> {
                    #[cfg(feature = "safe")]
                    return Self::from_parts((-from.0.get()).try_into()?);

                    #[cfg(not(feature = "safe"))]
                    // SAFETY: all ok results of try_from should be valid
                    return Ok(unsafe { Self::from_parts_unchecked((-from.0.get()).try_into()?) });
                }
            }
            impl TryFrom<&[<$from$from_size>]> for [<$for$for_size>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: &[<$from$from_size>])
                    -> $crate::error::NumeraResult<[<$for$for_size>]> {
                    #[cfg(feature = "safe")]
                    return Self::from_parts((-from.0.get()).try_into()?);

                    #[cfg(not(feature = "safe"))]
                    // SAFETY: all ok results of try_from should be valid
                    return Ok(unsafe { Self::from_parts_unchecked((-from.0.get()).try_into()?) });
                }
            }
            impl TryFrom<&mut [<$from$from_size>]> for [<$for$for_size>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: &mut [<$from$from_size>])
                    -> $crate::error::NumeraResult<[<$for$for_size>]> {
                    #[cfg(feature = "safe")]
                    return Self::from_parts((-from.0.get()).try_into()?);

                    #[cfg(not(feature = "safe"))]
                    // SAFETY: all ok results of try_from should be valid
                    return Ok(unsafe { Self::from_parts_unchecked((-from.0.get()).try_into()?) });
                }
            }
        }
    };

    // when `for` can not represent negative values,
    // and we have to use its `new` constructor,
    // and `from` is an Integer.
    //
    // Used by:
    // - for: Nz   from: Z
    (new_neg_int
     for: $for:ident + $for_size:expr,
     from: $from:ident + $( $from_size:expr ),+) => {
        $(
            try_from_integer![@new_neg_int for: $for + $for_size, from: $from + $from_size];
        )+
    };
    (@new_neg_int
     for: $for:ident + $for_size:expr, from: $from:ident + $from_size:expr) => {
        devela::paste! {
            impl TryFrom<[<$from$from_size>]> for [<$for$for_size>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: [<$from$from_size>])
                    -> $crate::error::NumeraResult<[<$for$for_size>]> {
                    Self::new((-from.0).try_into()?)
                }
            }
            impl TryFrom<&[<$from$from_size>]> for [<$for$for_size>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: &[<$from$from_size>])
                    -> $crate::error::NumeraResult<[<$for$for_size>]> {
                    Self::new((-from.0).try_into()?)
                }
            }
            impl TryFrom<&mut [<$from$from_size>]> for [<$for$for_size>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: &mut [<$from$from_size>])
                    -> $crate::error::NumeraResult<[<$for$for_size>]> {
                    Self::new((-from.0).try_into()?)
                }
            }
        }
    };

    // when `for` can not represent negative values,
    // and we have to use its `new` constructor,
    // and `from` is a NonZeroInteger.
    //
    // Used by:
    // - for: Nz   from: N0z
    (new_neg_nonzero
     for: $for:ident + $for_size:expr,
     from: $from:ident + $( $from_size:expr ),+) => {
        $(
            try_from_integer![@new_neg_nonzero for: $for + $for_size, from: $from + $from_size];
        )+
    };
    (@new_neg_nonzero
     for: $for:ident + $for_size:expr, from: $from:ident + $from_size:expr) => {
        devela::paste! {
            impl TryFrom<[<$from$from_size>]> for [<$for$for_size>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: [<$from$from_size>])
                    -> $crate::error::NumeraResult<[<$for$for_size>]> {
                    Self::new((-from.0.get()).try_into()?)
                }
            }
            impl TryFrom<&[<$from$from_size>]> for [<$for$for_size>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: &[<$from$from_size>])
                    -> $crate::error::NumeraResult<[<$for$for_size>]> {
                    Self::from_parts((-from.0.get()).try_into()?)
                }
            }
            impl TryFrom<&mut [<$from$from_size>]> for [<$for$for_size>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: &mut [<$from$from_size>])
                    -> $crate::error::NumeraResult<[<$for$for_size>]> {
                    Self::new((-from.0.get()).try_into()?)
                }
            }
        }
    };
}
pub(crate) use try_from_integer;

/// Implements `TryFrom`<`$from$from_size`> for `$for$for_size`.
///
/// # Args
/// - `$for`:       the base name of the target. e.g. `NonZeroInteger`.
/// - `$for_size`:  the bit size of the target. e.g. `16`.
/// - `$from`:    the base name of the origin. e.g. `u`.
/// - `$from_size`: a list of bit sizes of the origin. e.g. `8, 16`.
///
/// # Examples
/// ```ignore
/// try_from_primitive![many for: Integer + 8, from: u + 8, 16, 32, 64, 128];
/// ```
///
/// # Branches ids
/// - `int`
/// - `nonzero`
/// - `neg_int`
/// - `neg_nonzero`
/// - `new_neg_int`
/// - `new_neg_nonzero`
macro_rules! try_from_primitive {
    // when `from` is an integer primitive.
    //
    // Used by:
    // - for: Z     from: u, i
    // - for: Nnz   from: u, i
    // - for: N0z   from: u, i
    // - for: Pz    from: u, i
    (int
     for: $for:ident + $for_size:expr,
     from: $from:ident + $( $from_size:expr ),+
    ) => {
        $(
            try_from_primitive![@int for: $for + $for_size, from: $from + $from_size];
        )+
    };
    (@int
     for: $for:ident + $for_size:expr,
     from: $from:ident + $from_size:expr
    ) => {
        devela::paste! {
            impl TryFrom<[<$from$from_size>]> for [<$for$for_size>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: [<$from$from_size>])
                    -> $crate::error::NumeraResult<[<$for$for_size>]> {
                    Self::from_parts(from.try_into()?)
                }
            }
            impl TryFrom<&[<$from$from_size>]> for [<$for$for_size>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: &[<$from$from_size>])
                    -> $crate::error::NumeraResult<[<$for$for_size>]> {
                    Self::from_parts((*from).try_into()?)
                }
            }
            impl TryFrom<&mut [<$from$from_size>]> for [<$for$for_size>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: &mut [<$from$from_size>])
                    -> $crate::error::NumeraResult<[<$for$for_size>]> {
                    Self::from_parts((*from).try_into()?)
                }
            }
        }
    };

    // when `from` is a non-zero integer primitive.
    //
    // Used by:
    // - for: Z     from: NonZeroU, NonZeroI
    // - for: Pz    from: NonZeroU, NonZeroI
    // - for: N0z   from: NonZeroU, NonZeroI
    // - for: Nnz   from: NonZeroU, NonZeroI
    (nonzero
     for: $for:ident + $for_size:expr,
     from: $from:ident + $( $from_size:expr ),+
    ) => {
        $(
            try_from_primitive![@nonzero for: $for + $for_size, from: $from + $from_size];
        )+
    };
    (@nonzero
     for: $for:ident + $for_size:expr,
     from: $from:ident + $from_size:expr
    ) => {
        devela::paste! {
            impl TryFrom<[<$from$from_size>]> for [<$for$for_size>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: [<$from$from_size>])
                    -> $crate::error::NumeraResult<[<$for$for_size>]> {
                    #[cfg(feature = "safe")]
                    return Self::from_parts(from.get().try_into()?);

                    #[cfg(not(feature = "safe"))]
                    // SAFETY: all ok results of try_from should be valid
                    return Ok(unsafe { Self::from_parts_unchecked(from.get().try_into()?) });
                }
            }
            impl TryFrom<&[<$from$from_size>]> for [<$for$for_size>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: &[<$from$from_size>])
                    -> $crate::error::NumeraResult<[<$for$for_size>]> {
                    #[cfg(feature = "safe")]
                    return Self::from_parts(from.get().try_into()?);

                    #[cfg(not(feature = "safe"))]
                    // SAFETY: all ok results of try_from should be valid
                    return Ok(unsafe { Self::from_parts_unchecked(from.get().try_into()?) });
                }
            }
            impl TryFrom<&mut [<$from$from_size>]> for [<$for$for_size>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: &mut [<$from$from_size>])
                    -> $crate::error::NumeraResult<[<$for$for_size>]> {
                    #[cfg(feature = "safe")]
                    return Self::from_parts(from.get().try_into()?);

                    #[cfg(not(feature = "safe"))]
                    // SAFETY: all ok results of try_from should be valid
                    return Ok(unsafe { Self::from_parts_unchecked(from.get().try_into()?) });
                }
            }
        }
    };

    // when `for` can not represent positive values,
    // and `from` is a primitive integer.
    //
    // Used by:
    // - for: Npz   from: i
    (neg_int
     for: $for:ident + $for_size:expr,
     from: $from:ident + $( $from_size:expr ),+
    ) => {
        $(
            try_from_primitive![@neg_int for: $for + $for_size, from: $from + $from_size];
        )+
    };
    (@neg_int
     for: $for:ident + $for_size:expr,
     from: $from:ident + $from_size:expr
    ) => {
        devela::paste! {
            impl TryFrom<[<$from$from_size>]> for [<$for$for_size>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: [<$from$from_size>])
                    -> $crate::error::NumeraResult<[<$for$for_size>]> {
                    Self::from_parts((-from).try_into()?)
                }
            }
            impl TryFrom<&[<$from$from_size>]> for [<$for$for_size>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: &[<$from$from_size>])
                    -> $crate::error::NumeraResult<[<$for$for_size>]> {
                    Self::from_parts((-*from).try_into()?)
                }
            }
            impl TryFrom<&mut [<$from$from_size>]> for [<$for$for_size>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: &mut [<$from$from_size>])
                    -> $crate::error::NumeraResult<[<$for$for_size>]> {
                    Self::from_parts((-*from).try_into()?)
                }
            }
        }
    };

    // when `for` can not represent positive values,
    // and `from` is a NonZeroI.
    //
    // Used by:
    // - for: Npz   from: NonZeroI
    (neg_nonzero
     for: $for:ident + $for_size:expr,
     from: $from:ident + $( $from_size:expr ),+
    ) => {
        $(
            try_from_primitive![@neg_nonzero for: $for + $for_size, from: $from + $from_size];
        )+
    };
    (@neg_nonzero
     for: $for:ident + $for_size:expr,
     from: $from:ident + $from_size:expr
    ) => {
        devela::paste! {
            impl TryFrom<[<$from$from_size>]> for [<$for$for_size>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: [<$from$from_size>])
                    -> $crate::error::NumeraResult<[<$for$for_size>]> {
                    #[cfg(feature = "safe")]
                    return Self::from_parts((-from.get()).try_into()?);

                    #[cfg(not(feature = "safe"))]
                    // SAFETY: all ok results of try_from should be valid
                    return Ok(unsafe { Self::from_parts_unchecked((-from.get()).try_into()?) });
                }
            }
            impl TryFrom<&[<$from$from_size>]> for [<$for$for_size>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: &[<$from$from_size>])
                    -> $crate::error::NumeraResult<[<$for$for_size>]> {
                    #[cfg(feature = "safe")]
                    return Self::from_parts((-from.get()).try_into()?);

                    #[cfg(not(feature = "safe"))]
                    // SAFETY: all ok results of try_from should be valid
                    return Ok(unsafe { Self::from_parts_unchecked((-from.get()).try_into()?) });
                }
            }
            impl TryFrom<&mut [<$from$from_size>]> for [<$for$for_size>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: &mut [<$from$from_size>])
                    -> $crate::error::NumeraResult<[<$for$for_size>]> {
                    #[cfg(feature = "safe")]
                    return Self::from_parts((-from.get()).try_into()?);

                    #[cfg(not(feature = "safe"))]
                    // SAFETY: all ok results of try_from should be valid
                    return Ok(unsafe { Self::from_parts_unchecked((-from.get()).try_into()?) });
                }
            }
        }
    };

    // when `for` can only represent negative values,
    // and we have to use its `new` constructor,
    // and `from` is a primitive integer.
    //
    // Used by:
    // - for: Nz    from: i
    (new_neg_int
     for: $for:ident + $for_size:expr,
     from: $from:ident + $( $from_size:expr ),+
    ) => {
        $(
            try_from_primitive![@new_neg_int for: $for + $for_size, from: $from + $from_size];
        )+
    };
    (@new_neg_int
     for: $for:ident + $for_size:expr,
     from: $from:ident + $from_size:expr
    ) => {
        devela::paste! {
            impl TryFrom<[<$from$from_size>]> for [<$for$for_size>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: [<$from$from_size>])
                    -> $crate::error::NumeraResult<[<$for$for_size>]> {
                    Self::new((-from).try_into()?)
                }
            }
            impl TryFrom<&[<$from$from_size>]> for [<$for$for_size>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: &[<$from$from_size>])
                    -> $crate::error::NumeraResult<[<$for$for_size>]> {
                    Self::new((-*from).try_into()?)
                }
            }
            impl TryFrom<&mut [<$from$from_size>]> for [<$for$for_size>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: &mut [<$from$from_size>])
                    -> $crate::error::NumeraResult<[<$for$for_size>]> {
                    Self::new((-*from).try_into()?)
                }
            }
        }
    };

    // when `for` can not represent positive values,
    // and we have to use its `new` constructor,
    // and `from` is a NonZeroI.
    //
    // Used by:
    // - for: Nz    from: NonZeroI
    (new_neg_nonzero
     for: $for:ident + $for_size:expr,
     from: $from:ident + $( $from_size:expr ),+
    ) => {
        $(
            try_from_primitive![@new_neg_nonzero for: $for + $for_size, from: $from + $from_size];
        )+
    };
    (@new_neg_nonzero
     for: $for:ident + $for_size:expr,
     from: $from:ident + $from_size:expr
    ) => {
        devela::paste! {
            impl TryFrom<[<$from$from_size>]> for [<$for$for_size>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: [<$from$from_size>])
                    -> $crate::error::NumeraResult<[<$for$for_size>]> {
                    return Self::new((-from.get()).try_into()?);
                }
            }
            impl TryFrom<&[<$from$from_size>]> for [<$for$for_size>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: &[<$from$from_size>])
                    -> $crate::error::NumeraResult<[<$for$for_size>]> {
                    return Self::new((-from.get()).try_into()?);
                }
            }
            impl TryFrom<&mut [<$from$from_size>]> for [<$for$for_size>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: &mut [<$from$from_size>])
                    -> $crate::error::NumeraResult<[<$for$for_size>]> {
                    return Self::new((-from.get()).try_into()?);
                }
            }
        }
    };

}
pub(crate) use try_from_primitive;

/// Implements `TryFrom`<`$from$from_size`> for `$for$for_size`.
///
/// # Args
/// - `$for`:       the base name of the target. e.g. `NonZeroInteger`.
/// - `$for_size`:  the bit size of the target. e.g. `16`.
/// - `$from`:    the base name of the origin. e.g. `u`.
/// - `$from_size`: a list of bit sizes of the origin. e.g. `8, 16`.
///
/// # Examples
/// ```ignore
/// try_from_any![zero for: NonPositiveInteger + 8, from: u + 8, 16, 32, 64, 128];
/// ```
///
/// # Branches ids
/// - `zero`
/// - `error`
macro_rules! try_from_any {
    // when the conversion is only valid when the `from` value is `zero`.
    //
    // Used by:
    // - for: Nnz   from: Npz
    // - for: Npz   from: Nnz, u
    (zero
     for: $for:ident + $for_size:expr,
     from: $from:ident + $( $from_size:expr ),+) => {
        $(
            try_from_any![@zero for: $for + $for_size, from: $from + $from_size];
        )+
    };
    (@zero
     for: $for:ident + $for_size:expr, from: $from:ident + $from_size:expr) => {
        devela::paste! {
            impl TryFrom<[<$from$from_size>]> for [<$for$for_size>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: [<$from$from_size>])
                    -> $crate::error::NumeraResult<[<$for$for_size>]> {
                    if from.is_zero() {
                        Ok([<$for$for_size>]::ZERO)
                    } else {
                        Err($crate::error::NumeraError::Conversion)
                    }
                }
            }
            impl TryFrom<&[<$from$from_size>]> for [<$for$for_size>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: &[<$from$from_size>])
                    -> $crate::error::NumeraResult<[<$for$for_size>]> {
                    if from.is_zero() {
                        Ok([<$for$for_size>]::ZERO)
                    } else {
                        Err($crate::error::NumeraError::Conversion)
                    }
                }
            }
            impl TryFrom<&mut [<$from$from_size>]> for [<$for$for_size>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: &mut [<$from$from_size>])
                    -> $crate::error::NumeraResult<[<$for$for_size>]> {
                    if from.is_zero() {
                        Ok([<$for$for_size>]::ZERO)
                    } else {
                        Err($crate::error::NumeraError::Conversion)
                    }
                }
            }
        }
    };

    // when the conversion attempt must always error.
    //
    // Used by:
    // - for: Pz    from: Nz, Npz
    // - for: Nz    from: Nnz, Pz
    // - for: Nnz   from: Nz
    // - for: Npz   from: Pz
    (error
     for: $for:ident + $for_size:expr,
     from: $from:ident + $( $from_size:expr ),+) => {
        $(
            try_from_any![@error for: $for + $for_size, from: $from + $from_size];
        )+
    };
    (@error
     for: $for:ident + $for_size:expr, from: $from:ident + $from_size:expr) => {
        devela::paste! {
            impl TryFrom<[<$from$from_size>]> for [<$for$for_size>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(_from: [<$from$from_size>])
                    -> $crate::error::NumeraResult<[<$for$for_size>]> {
                    Err($crate::error::NumeraError::Conversion)
                }
            }
            impl TryFrom<&[<$from$from_size>]> for [<$for$for_size>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(_from: &[<$from$from_size>])
                    -> $crate::error::NumeraResult<[<$for$for_size>]> {
                    Err($crate::error::NumeraError::Conversion)
                }
            }
            impl TryFrom<&mut [<$from$from_size>]> for [<$for$for_size>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(_from: &mut [<$from$from_size>])
                    -> $crate::error::NumeraResult<[<$for$for_size>]> {
                    Err($crate::error::NumeraError::Conversion)
                }
            }
        }
    };
}
pub(crate) use try_from_any;
