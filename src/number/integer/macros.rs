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

/// Implements From<`$from$from_b`> for `$for$for_b`.
///
/// # Args
/// - `$for`:    the base name of the target. e.g. `Integer`.
/// - `$p`:      target's inner corresponding primitive prefix: `i` or `u`.
///              (this is only used for `_neg` variants, MAYBE IMPROVE).
/// - `$for_b`:  the bit size of the target. e.g. `32`.
/// - `$from`:   the base name of the origin. e.g. `PositiveInteger`.
/// - `$from_b`: a list of bit sizes of the origin. e.g. `8, 16`.
///
/// # Examples
/// ```ignore
/// from_integer![int for: Integer + i + 32, from: Integer + 8, 16];
/// ```
///
/// # Branches ids
/// - `int`
/// - `nonzero`
/// - `neg_signed`
/// - `negnon0_signed`
macro_rules! from_integer {
    // from_integer!
    // when `from` has an inner integer primitive
    //
    // Used by:
    // - for: Z     from: Z, Nnz, Npz
    // - for: Nnz   from: Nnz
    // - for: Npz   from: Npz
    (int
     for: $for:ident + $for_b:expr,
     from: $from:ident + $( $from_b:expr ),+) => {
        $(
            from_integer![@int for: $for + $for_b, from: $from + $from_b];
        )+
    };
    (@int
     for: $for:ident + $for_b:expr, from: $from:ident + $from_b:expr) => {
        devela::paste! {
            impl From<[<$from$from_b>]> for [<$for$for_b>] {
                #[inline]
                fn from(from: [<$from$from_b>]) -> Self {
                    Self(from.0.into())
                }
            }
            impl From<&[<$from$from_b>]> for [<$for$for_b>] {
                #[inline]
                fn from(from: &[<$from$from_b>]) -> Self {
                    Self(from.0.into())
                }
            }
            impl From<&mut [<$from$from_b>]> for [<$for$for_b>] {
                #[inline]
                fn from(from: &mut [<$from$from_b>]) -> Self {
                    Self(from.0.into())
                }
            }
        }
    };

    // DONE
    // from_integer!
    // when `from` has an inner NonZero*
    //
    // Used by:
    // - for: Z     from: N0z, Pz
    // - for: N0z   from: N0z, Pz
    // - for: Nnz   from: Pz
    // - for: Pz    from: Pz
    // - for: Npz   from: Nz        (negative to negative is OK)
    // - for: Nz    from: Nz        (negative to negative is OK)
    (non0
     for: $for:ident + $for_b:expr,
     from: $from:ident + $( $from_b:expr ),+) => {
        $(
            from_integer![@non0 for: $for + $for_b, from: $from + $from_b];
        )+
    };
    (@non0
     for: $for:ident + $for_b:expr, from: $from:ident + $from_b:expr) => {
        devela::paste! {
            impl From<[<$from$from_b>]> for [<$for$for_b>] {
                #[inline]
                fn from(from: [<$from$from_b>]) -> Self {
                    #[cfg(feature = "safe")]
                    return Self::from_parts(from.0.get().into()).unwrap();

                    #[cfg(not(feature = "safe"))]
                    // SAFETY: coming from a type that respects the invariant of not having 0
                    return unsafe { Self::from_parts_unchecked(from.0.get().into()) };
                }
            }
            impl From<&[<$from$from_b>]> for [<$for$for_b>] {
                #[inline]
                fn from(from: &[<$from$from_b>]) -> Self {
                    #[cfg(feature = "safe")]
                    return Self::from_parts(from.0.get().into()).unwrap();

                    #[cfg(not(feature = "safe"))]
                    // SAFETY: coming from a type that respects the invariant of not having 0
                    return unsafe { Self::from_parts_unchecked(from.0.get().into()) };
                }
            }
            impl From<&mut [<$from$from_b>]> for [<$for$for_b>] {
                #[inline]
                fn from(from: &mut [<$from$from_b>]) -> Self {
                    #[cfg(feature = "safe")]
                    return Self::from_parts(from.0.get().into()).unwrap();

                    #[cfg(not(feature = "safe"))]
                    // SAFETY: coming from a type that respects the invariant of not having 0
                    return unsafe { Self::from_parts_unchecked(from.0.get().into()) };
                }
            }
        }
    };

    // from_integer!
    // when `from` has an unsigned inner primitive that represents only negative values,
    // which has to be negated in the conversion
    //
    // - for: Z     from: Npz
    (neg_signed
     for: $for:ident + $p:ident + $for_b:expr, from: $from:ident + $( $from_b:expr ),+) => {
        $(
            from_integer![@neg_signed for: $for + $p + $for_b, from: $from + $from_b];
        )+
    };
    (@neg_signed
     for: $for:ident + $p:ident + $for_b:expr, from: $from:ident + $from_b:expr) => {
        devela::paste! {
            impl From<[<$from$from_b>]> for [<$for$for_b>] {
                #[inline]
                fn from(from: [<$from$from_b>]) -> Self {
                    Self(Into::<[<$p$for_b>]>::into(from.0).neg())
                }
            }
            impl From<&[<$from$from_b>]> for [<$for$for_b>] {
                #[inline]
                fn from(from: &[<$from$from_b>]) -> Self {
                    Self(Into::<[<$p$for_b>]>::into(from.0).neg())
                }
            }
            impl From<&mut [<$from$from_b>]> for [<$for$for_b>] {
                #[inline]
                fn from(from: &mut [<$from$from_b>]) -> Self {
                    Self(Into::<[<$p$for_b>]>::into(from.0).neg())
                }
            }
        }
    };

    // DONE
    // from_integer!
    // when `from` has an unsigned inner NonZero* primitive that represents only negative values,
    // which has to be negated in the conversion
    //
    // - for: Z     from: Nz
    // - for: N0z   from: Nz
    (negnon0_signed
     for: $for:ident + $p:ident + $for_b:expr, from: $from:ident + $( $from_b:expr ),+) => {
        $(
            from_integer![@negnon0_signed
            for: $for + $p + $for_b, from: $from + $from_b];
        )+
    };
    (@negnon0_signed
     for: $for:ident + $p:ident + $for_b:expr, from: $from:ident + $from_b:expr) => {
        devela::paste! {
            impl From<[<$from$from_b>]> for [<$for$for_b>] {
                #[inline]
                fn from(from: [<$from$from_b>]) -> Self {
                    #[cfg(feature = "safe")]
                    return
                        Self::from_parts(Into::<[<$p$for_b>]>::into(from.0.get()).neg()).unwrap();

                    #[cfg(not(feature = "safe"))]
                    // SAFETY: coming from a type that respects the invariant of not having 0
                    return unsafe {
                        Self::from_parts_unchecked(Into::<[<$p$for_b>]>::into(from.0.get()).neg())
                    };
                }
            }
            impl From<&[<$from$from_b>]> for [<$for$for_b>] {
                #[inline]
                fn from(from: &[<$from$from_b>]) -> Self {
                    #[cfg(feature = "safe")]
                    return
                        Self::from_parts(Into::<[<$p$for_b>]>::into(from.0.get()).neg()).unwrap();

                    #[cfg(not(feature = "safe"))]
                    // SAFETY: coming from a type that respects the invariant of not having 0
                    return unsafe {
                        Self::from_parts_unchecked(Into::<[<$p$for_b>]>::into(from.0.get()).neg())
                    };
                }
            }
            impl From<&mut [<$from$from_b>]> for [<$for$for_b>] {
                #[inline]
                fn from(from: &mut [<$from$from_b>]) -> Self {
                    #[cfg(feature = "safe")]
                    return
                        Self::from_parts(Into::<[<$p$for_b>]>::into(from.0.get()).neg()).unwrap();

                    #[cfg(not(feature = "safe"))]
                    // SAFETY: coming from a type that respects the invariant of not having 0
                    return unsafe {
                        Self::from_parts_unchecked(Into::<[<$p$for_b>]>::into(from.0.get()).neg())
                    };
                }
            }
        }
    };
}
pub(crate) use from_integer;

/// Implements From<`$from_p$from_b`> for `$for$for_b`.
///
/// # Args
/// - `$for`:    the base name of the target. e.g. `NonZeroInteger`.
/// - `$for_b`:  the bit size of the target. e.g. `16`.
/// - `$from_p`: the base name of the origin. e.g. `u`.
/// - `$from_b`: a list of bit sizes of the origin. e.g. `8, 16`.
///
/// # Examples
/// ```ignore
/// from_primitive![many for: Integer + 16, from: u + 8];
/// from_primitive![many for: Integer + 16, from: i + 8, 16];
/// ```
///
/// # Branches ids
/// - `int`
/// - `non0`
macro_rules! from_primitive {
    // from_primitive!
    // when `from` is the same integer primitive than the inner part of `for`.
    //
    // - for: Z     from: u, i
    // - for: Nnz   from: u
    (int
     for: $for:ident + $for_b:expr,
     from: $from_p:ident + $( $from_b:expr ),+
    ) => {
        $(
            from_primitive![@int for: $for + $for_b, from: $from_p + $from_b];
        )+
    };
    (@int
     for: $for:ident + $for_b:expr,
     from: $from_p:ident + $from_b:expr
    ) => {
        devela::paste! {
            impl From<[<$from_p$from_b>]> for [<$for$for_b>] {
                #[inline]
                fn from(from: [<$from_p$from_b>]) -> Self {
                    #[cfg(feature = "safe")]
                    return Self::from_parts(from.into()).unwrap();

                    #[cfg(not(feature = "safe"))]
                    // SAFETY: all values should be valid
                    return unsafe { Self::from_parts_unchecked(from.into()) };
                }
            }
            impl From<&[<$from_p$from_b>]> for [<$for$for_b>] {
                #[inline]
                fn from(from: &[<$from_p$from_b>]) -> Self {
                    #[cfg(feature = "safe")]
                    return Self::from_parts((*from).into()).unwrap();

                    #[cfg(not(feature = "safe"))]
                    // SAFETY: all values should be valid
                    return unsafe { Self::from_parts_unchecked((*from).into()) };
                }
            }
            impl From<&mut [<$from_p$from_b>]> for [<$for$for_b>] {
                #[inline]
                fn from(from: &mut [<$from_p$from_b>]) -> Self {
                    #[cfg(feature = "safe")]
                    return Self::from_parts((*from).into()).unwrap();

                    #[cfg(not(feature = "safe"))]
                    // SAFETY: all values should be valid
                    return unsafe { Self::from_parts_unchecked((*from).into()) };
                }
            }
        }
    };

    // DONE
    // from_primitive!
    // when `from` is a NonZero* primitive
    //
    // - for: Z     from: NonZeroU, NonZeroI
    // - for: N0z   from: NonZeroU, NonZeroI
    // - for: Pz    from: NonZeroU
    // - for: Nnz   from: NonZeroU
    (non0
     for: $for:ident + $for_b:expr,
     from: $from_p:ident + $( $from_b:expr ),+
    ) => {
        $(
            from_primitive![@non0 for: $for + $for_b, from: $from_p + $from_b];
        )+
    };
    (@non0
     for: $for:ident + $for_b:expr,
     from: $from_p:ident + $from_b:expr
    ) => {
        devela::paste! {
            impl From<[<$from_p$from_b>]> for [<$for$for_b>] {
                #[inline]
                fn from(from: [<$from_p$from_b>]) -> Self {
                    #[cfg(feature = "safe")]
                    return Self::from_parts(from.get().into()).unwrap();

                    #[cfg(not(feature = "safe"))]
                    // SAFETY: coming from a type that respects the invariant of not having 0
                    return unsafe { Self::from_parts_unchecked(from.get().into()) };
                }
            }
            impl From<&[<$from_p$from_b>]> for [<$for$for_b>] {
                #[inline]
                fn from(from: &[<$from_p$from_b>]) -> Self {
                    #[cfg(feature = "safe")]
                    return Self::from_parts((*from).get().into()).unwrap();

                    #[cfg(not(feature = "safe"))]
                    // SAFETY: coming from a type that respects the invariant of not having 0
                    return unsafe { Self::from_parts_unchecked((*from).get().into()) };
                }
            }
            impl From<&mut [<$from_p$from_b>]> for [<$for$for_b>] {
                #[inline]
                fn from(from: &mut [<$from_p$from_b>]) -> Self {
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

/// Implements `TryFrom`<`$from$from_b`> for `$for$for_b`.
///
/// # Args
/// - `$for`:    the base name of the target. e.g. `Integer`.
/// - `$p`:      target's inner corresponding primitive prefix: `i` or `u`.
///              (this is only used for `_neg` variants, MAYBE IMPROVE).
/// - `$for_b`:  the bit size of the target. e.g. `8`.
/// - `$from`:   the base name of the origin. e.g. `PositiveInteger`.
/// - `$from_b`: a list of bit sizes of the origin. e.g. `8, 16`.
///
/// # Examples
/// ```ignore
/// try_from_integer![int for: Integer + i + 8, from: Integer + 8, 16];
/// ```
///
/// # Branches ids
/// - `int`
/// - `neg_signed`
/// - `non0_int`
/// - `negnon0_non0`
/// - `non0`
/// - `negnon0_signed`
/// - `neg_int`
/// - `neg_nonzero`
/// - `negnon0_int`
/// - `neg_nonzero`
#[cfg(feature = "try_from")]
macro_rules! try_from_integer {
    // try_from_integer!
    // when `from` has an inner integer primitive.
    //
    // Used by:
    // - for: Nnz   from: Z, Nnz,
    // - for: Z     from: Z, Nnz, Npz
    // - for: Npz   from: Npz
    // - for: Pz    from: N0z
    (int
     for: $for:ident + $for_b:expr,
     from: $from:ident + $( $from_b:expr ),+) => {
        $(
            try_from_integer![@int for: $for + $for_b, from: $from + $from_b];
        )+
    };
    (@int
     for: $for:ident + $for_b:expr, from: $from:ident + $from_b:expr) => {
        devela::paste! {
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "try_from")))]
            impl TryFrom<[<$from$from_b>]> for [<$for$for_b>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: [<$from$from_b>])
                    -> $crate::error::NumeraResult<[<$for$for_b>]> {
                    Ok(Self(from.0.try_into()?))
                }
            }
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "try_from")))]
            impl TryFrom<&[<$from$from_b>]> for [<$for$for_b>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: &[<$from$from_b>])
                    -> $crate::error::NumeraResult<[<$for$for_b>]> {
                    Ok(Self(from.0.try_into()?))
                }
            }
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "try_from")))]
            impl TryFrom<&mut [<$from$from_b>]> for [<$for$for_b>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: &mut [<$from$from_b>])
                    -> $crate::error::NumeraResult<[<$for$for_b>]> {
                    Ok(Self(from.0.try_into()?))
                }
            }
        }
    };

    // DONE
    // try_from_integer!
    // when `from` has an unsigned inner primitive representing only negative values,
    // which has to be negated in the conversion.
    //
    // Used by:
    // - for: Z     from: Npz
    (neg_signed
     for: $for:ident + $p:ident + $for_b:expr, from: $from:ident + $( $from_b:expr ),+) => {
        $(
            try_from_integer![@neg_signed for: $for + $p + $for_b, from: $from + $from_b];
        )+
    };
    (@neg_signed
     for: $for:ident + $p:ident + $for_b:expr, from: $from:ident + $from_b:expr) => {
        devela::paste! {
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "try_from")))]
            impl TryFrom<[<$from$from_b>]> for [<$for$for_b>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: [<$from$from_b>])
                    -> $crate::error::NumeraResult<[<$for$for_b>]> {
                    Ok(Self(TryInto::<[<$p$for_b>]>::try_into(from.0)?.neg()))
                }
            }
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "try_from")))]
            impl TryFrom<&[<$from$from_b>]> for [<$for$for_b>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: &[<$from$from_b>])
                    -> $crate::error::NumeraResult<[<$for$for_b>]> {
                    Ok(Self(TryInto::<[<$p$for_b>]>::try_into(from.0)?.neg()))
                }
            }
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "try_from")))]
            impl TryFrom<&mut [<$from$from_b>]> for [<$for$for_b>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: &mut [<$from$from_b>])
                    -> $crate::error::NumeraResult<[<$for$for_b>]> {
                    Ok(Self(TryInto::<[<$p$for_b>]>::try_into(from.0)?.neg()))
                }
            }
        }
    };

    // DONE
    // try_from_integer!
    // when `from` has an inner integer primitive and `for` has an inner nonzero primitive
    //
    // Used by:
    // - for: N0z   from: Z, Nnz
    // - for: Pz    from: Z
    (non0_int
     for: $for:ident + $for_b:expr,
     from: $from:ident + $( $from_b:expr ),+) => {
        $(
            try_from_integer![@non0_int for: $for + $for_b, from: $from + $from_b];
        )+
    };
    (@non0_int
     for: $for:ident + $for_b:expr, from: $from:ident + $from_b:expr) => {
        devela::paste! {
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "try_from")))]
            impl TryFrom<[<$from$from_b>]> for [<$for$for_b>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: [<$from$from_b>])
                    -> $crate::error::NumeraResult<[<$for$for_b>]> {
                    Self::new(from.0.try_into()?)
                }
            }
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "try_from")))]
            impl TryFrom<&[<$from$from_b>]> for [<$for$for_b>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: &[<$from$from_b>])
                    -> $crate::error::NumeraResult<[<$for$for_b>]> {
                    Self::new(from.0.try_into()?)
                }
            }
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "try_from")))]
            impl TryFrom<&mut [<$from$from_b>]> for [<$for$for_b>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: &mut [<$from$from_b>])
                    -> $crate::error::NumeraResult<[<$for$for_b>]> {
                    Self::new(from.0.try_into()?)
                }
            }
        }
    };

    // DONE
    // try_from_integer!
    // when `from` has an unsigned inner primitive representing only negative values,
    // and `for` has a non-zero inner primitive
    //
    // Used by:
    // - for: N0z   from: Npz
    (negnon0_non0
     for: $for:ident + $for_b:expr,
     from: $from:ident + $( $from_b:expr ),+) => {
        $(
            try_from_integer![@negnon0_non0 for: $for + $for_b, from: $from + $from_b];
        )+
    };
    (@negnon0_non0
     for: $for:ident + $for_b:expr, from: $from:ident + $from_b:expr) => {
        devela::paste! {
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "try_from")))]
            impl TryFrom<[<$from$from_b>]> for [<$for$for_b>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: [<$from$from_b>])
                    -> $crate::error::NumeraResult<[<$for$for_b>]> {
                    Ok(Self::new(from.0.try_into()?)?.neg())
                }
            }
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "try_from")))]
            impl TryFrom<&[<$from$from_b>]> for [<$for$for_b>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: &[<$from$from_b>])
                    -> $crate::error::NumeraResult<[<$for$for_b>]> {
                    Ok(Self::new(from.0.try_into()?)?.neg())
                }
            }
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "try_from")))]
            impl TryFrom<&mut [<$from$from_b>]> for [<$for$for_b>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: &mut [<$from$from_b>])
                    -> $crate::error::NumeraResult<[<$for$for_b>]> {
                    Ok(Self::new(from.0.try_into()?)?.neg())
                }
            }
        }
    };

    // DONE
    // try_from_integer!
    // when `from` has an inner NonZero*.
    //
    // Used by:
    // - for: Nnz   from: N0z, Pz
    // - for: Z     from: N0z, Pz
    // - for: Npz   from: Nz
    // - for: N0z   from: N0z, Pz
    // - for: Pz    from: Pz
    // - for: Nz    from: Nz
    (non0
     for: $for:ident + $for_b:expr,
     from: $from:ident + $( $from_b:expr ),+) => {
        $(
            try_from_integer![@non0 for: $for + $for_b, from: $from + $from_b];
        )+
    };
    (@non0
     for: $for:ident + $for_b:expr, from: $from:ident + $from_b:expr) => {
        devela::paste! {
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "try_from")))]
            impl TryFrom<[<$from$from_b>]> for [<$for$for_b>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: [<$from$from_b>])
                    -> $crate::error::NumeraResult<[<$for$for_b>]> {
                    #[cfg(feature = "safe")]
                    return Self::from_parts(from.0.get().try_into()?);

                    #[cfg(not(feature = "safe"))]
                    // SAFETY: all ok results of try_from should be valid
                    return Ok(unsafe { Self::from_parts_unchecked(from.0.get().try_into()?) });
                }
            }
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "try_from")))]
            impl TryFrom<&[<$from$from_b>]> for [<$for$for_b>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: &[<$from$from_b>])
                    -> $crate::error::NumeraResult<[<$for$for_b>]> {
                    #[cfg(feature = "safe")]
                    return Self::from_parts(from.0.get().try_into()?);

                    #[cfg(not(feature = "safe"))]
                    // SAFETY: all ok results of try_from should be valid
                    return Ok(unsafe { Self::from_parts_unchecked(from.0.get().try_into()?) });
                }
            }
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "try_from")))]
            impl TryFrom<&mut [<$from$from_b>]> for [<$for$for_b>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: &mut [<$from$from_b>])
                    -> $crate::error::NumeraResult<[<$for$for_b>]> {
                    #[cfg(feature = "safe")]
                    return Self::from_parts(from.0.get().try_into()?);

                    #[cfg(not(feature = "safe"))]
                    // SAFETY: all ok results of try_from should be valid
                    return Ok(unsafe { Self::from_parts_unchecked(from.0.get().try_into()?) });
                }
            }
        }
    };

    // DONE
    // try_from_integer!
    // when `from` has an unsigned inner NonZero* representing only negative values,
    // which has to be negated in the conversion.
    //
    // - for: Z     from: Nz
    // - for: N0z   from: Nz
    (negnon0_signed
     for: $for:ident + $p:ident + $for_b:expr, from: $from:ident + $( $from_b:expr ),+) => {
        $(
            try_from_integer![@negnon0_signed
            for: $for + $p + $for_b, from: $from + $from_b];
        )+
    };
    (@negnon0_signed
     for: $for:ident + $p:ident + $for_b:expr, from: $from:ident + $from_b:expr) => {
        devela::paste! {
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "try_from")))]
            impl TryFrom<[<$from$from_b>]> for [<$for$for_b>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: [<$from$from_b>])
                    -> $crate::error::NumeraResult<[<$for$for_b>]> {
                    #[cfg(feature = "safe")]
                    return
                        Self::from_parts(TryInto::<[<$p$for_b>]>::try_into(from.0.get())?.neg());

                    #[cfg(not(feature = "safe"))]
                    // SAFETY: all ok results of try_from should be valid
                    return unsafe {
                        Ok(Self::from_parts_unchecked(
                                TryInto::<[<$p$for_b>]>::try_into(from.0.get())?.neg()
                        ))
                    };
                }
            }
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "try_from")))]
            impl TryFrom<&[<$from$from_b>]> for [<$for$for_b>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: &[<$from$from_b>])
                    -> $crate::error::NumeraResult<[<$for$for_b>]> {
                    #[cfg(feature = "safe")]
                    return
                        Self::from_parts(TryInto::<[<$p$for_b>]>::try_into(from.0.get())?.neg());

                    #[cfg(not(feature = "safe"))]
                    // SAFETY: all ok results of try_from should be valid
                    return unsafe {
                        Ok(Self::from_parts_unchecked(
                                TryInto::<[<$p$for_b>]>::try_into(from.0.get())?.neg()
                        ))
                    };
                }
            }
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "try_from")))]
            impl TryFrom<&mut [<$from$from_b>]> for [<$for$for_b>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: &mut [<$from$from_b>])
                    -> $crate::error::NumeraResult<[<$for$for_b>]> {
                    #[cfg(feature = "safe")]
                    return
                        Self::from_parts(TryInto::<[<$p$for_b>]>::try_into(from.0.get())?.neg());

                    #[cfg(not(feature = "safe"))]
                    // SAFETY: all ok results of try_from should be valid
                    return unsafe {
                        Ok(Self::from_parts_unchecked(
                                TryInto::<[<$p$for_b>]>::try_into(from.0.get())?.neg()
                        ))
                    };
                }
            }
        }
    };

    // DONE
    // try_from_integer!
    // when `from` is an Integer and `for` can not represent negative values.
    //
    // Used by:
    // - for: Npz   from: Z
    (neg_int
     for: $for:ident + $for_b:expr,
     from: $from:ident + $( $from_b:expr ),+) => {
        $(
            try_from_integer![@neg_int for: $for + $for_b, from: $from + $from_b];
        )+
    };
    (@neg_int
     for: $for:ident + $for_b:expr, from: $from:ident + $from_b:expr) => {
        devela::paste! {
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "try_from")))]
            impl TryFrom<[<$from$from_b>]> for [<$for$for_b>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: [<$from$from_b>])
                    -> $crate::error::NumeraResult<[<$for$for_b>]> {
                    Ok(Self((-from.0).try_into()?))
                }
            }
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "try_from")))]
            impl TryFrom<&[<$from$from_b>]> for [<$for$for_b>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: &[<$from$from_b>])
                    -> $crate::error::NumeraResult<[<$for$for_b>]> {
                    Ok(Self((-from.0).try_into()?))
                }
            }
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "try_from")))]
            impl TryFrom<&mut [<$from$from_b>]> for [<$for$for_b>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: &mut [<$from$from_b>])
                    -> $crate::error::NumeraResult<[<$for$for_b>]> {
                    Ok(Self((-from.0).try_into()?))
                }
            }
        }
    };

    // DONE
    // try_from_integer!
    // when `from` is an Integer and `for` can not represent negative values or 0.
    //
    // Used by:
    // - for: Nz   from: Z
    (negnon0_int
     for: $for:ident + $for_b:expr,
     from: $from:ident + $( $from_b:expr ),+) => {
        $(
            try_from_integer![@negnon0_int for: $for + $for_b, from: $from + $from_b];
        )+
    };
    (@negnon0_int
     for: $for:ident + $for_b:expr, from: $from:ident + $from_b:expr) => {
        devela::paste! {
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "try_from")))]
            impl TryFrom<[<$from$from_b>]> for [<$for$for_b>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: [<$from$from_b>])
                    -> $crate::error::NumeraResult<[<$for$for_b>]> {
                    Self::new_neg((-from.0).try_into()?)
                }
            }
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "try_from")))]
            impl TryFrom<&[<$from$from_b>]> for [<$for$for_b>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: &[<$from$from_b>])
                    -> $crate::error::NumeraResult<[<$for$for_b>]> {
                    Self::new_neg((-from.0).try_into()?)
                }
            }
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "try_from")))]
            impl TryFrom<&mut [<$from$from_b>]> for [<$for$for_b>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: &mut [<$from$from_b>])
                    -> $crate::error::NumeraResult<[<$for$for_b>]> {
                    Self::new_neg((-from.0).try_into()?)
                }
            }
        }
    };

    // DONE
    // try_from_integer!
    // when `from` is a NonZeroInteger and `for` can only represent positive values
    //
    // Used by:
    // - for: Npz   from: N0z
    (non0_pos
     for: $for:ident + $for_b:expr,
     from: $from:ident + $( $from_b:expr ),+) => {
        $(
            try_from_integer![@non0_pos for: $for + $for_b, from: $from + $from_b];
        )+
    };
    (@non0_pos
     for: $for:ident + $for_b:expr, from: $from:ident + $from_b:expr) => {
        devela::paste! {
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "try_from")))]
            impl TryFrom<[<$from$from_b>]> for [<$for$for_b>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: [<$from$from_b>])
                    -> $crate::error::NumeraResult<[<$for$for_b>]> {
                    #[cfg(feature = "safe")]
                    return Self::from_parts((-from.0.get()).try_into()?);

                    #[cfg(not(feature = "safe"))]
                    // SAFETY: all ok results of try_from should be valid
                    return Ok(unsafe { Self::from_parts_unchecked((-from.0.get()).try_into()?) });
                }
            }
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "try_from")))]
            impl TryFrom<&[<$from$from_b>]> for [<$for$for_b>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: &[<$from$from_b>])
                    -> $crate::error::NumeraResult<[<$for$for_b>]> {
                    #[cfg(feature = "safe")]
                    return Self::from_parts((-from.0.get()).try_into()?);

                    #[cfg(not(feature = "safe"))]
                    // SAFETY: all ok results of try_from should be valid
                    return Ok(unsafe { Self::from_parts_unchecked((-from.0.get()).try_into()?) });
                }
            }
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "try_from")))]
            impl TryFrom<&mut [<$from$from_b>]> for [<$for$for_b>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: &mut [<$from$from_b>])
                    -> $crate::error::NumeraResult<[<$for$for_b>]> {
                    #[cfg(feature = "safe")]
                    return Self::from_parts((-from.0.get()).try_into()?);

                    #[cfg(not(feature = "safe"))]
                    // SAFETY: all ok results of try_from should be valid
                    return Ok(unsafe { Self::from_parts_unchecked((-from.0.get()).try_into()?) });
                }
            }
        }
    };

    // DONE
    // try_from_integer!
    // when both `for` and `from` are non-positive Integers
    //
    // Used by:
    // - for: Nz   from: Npz
    (neg_non0neg
     for: $for:ident + $for_b:expr,
     from: $from:ident + $( $from_b:expr ),+) => {
        $(
            try_from_integer![@neg->non0_neg for: $for + $for_b, from: $from + $from_b];
        )+
    };
    (@neg->non0_neg
     for: $for:ident + $for_b:expr, from: $from:ident + $from_b:expr) => {
        devela::paste! {
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "try_from")))]
            impl TryFrom<[<$from$from_b>]> for [<$for$for_b>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: [<$from$from_b>])
                    -> $crate::error::NumeraResult<[<$for$for_b>]> {
                    Self::new_neg((from.0).try_into()?)
                }
            }
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "try_from")))]
            impl TryFrom<&[<$from$from_b>]> for [<$for$for_b>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: &[<$from$from_b>])
                    -> $crate::error::NumeraResult<[<$for$for_b>]> {
                    Self::new_neg((from.0).try_into()?)
                }
            }
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "try_from")))]
            impl TryFrom<&mut [<$from$from_b>]> for [<$for$for_b>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: &mut [<$from$from_b>])
                    -> $crate::error::NumeraResult<[<$for$for_b>]> {
                    Self::new_neg((from.0).try_into()?)
                }
            }
        }
    };

    // DONE
    // try_from_integer!
    // when `from` is a non-zero integer and `for` can not represent positive values
    //
    // Used by:
    // - for: Nz    from: N0z
    (neg_non0
     for: $for:ident + $for_b:expr,
     from: $from:ident + $( $from_b:expr ),+) => {
        $(
            try_from_integer![@neg_non0 for: $for + $for_b, from: $from + $from_b];
        )+
    };
    (@neg_non0
     for: $for:ident + $for_b:expr, from: $from:ident + $from_b:expr) => {
        devela::paste! {
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "try_from")))]
            impl TryFrom<[<$from$from_b>]> for [<$for$for_b>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: [<$from$from_b>])
                    -> $crate::error::NumeraResult<[<$for$for_b>]> {
                    Self::new_neg((-from.0.get()).try_into()?)
                }
            }
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "try_from")))]
            impl TryFrom<&[<$from$from_b>]> for [<$for$for_b>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: &[<$from$from_b>])
                    -> $crate::error::NumeraResult<[<$for$for_b>]> {
                    Self::new_neg((-from.0.get()).try_into()?)
                }
            }
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "try_from")))]
            impl TryFrom<&mut [<$from$from_b>]> for [<$for$for_b>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: &mut [<$from$from_b>])
                    -> $crate::error::NumeraResult<[<$for$for_b>]> {
                    Self::new_neg((-from.0.get()).try_into()?)
                }
            }
        }
    };
}
/// No-op alternative for disabling `TryFrom` impls.
#[cfg(not(feature = "try_from"))]
macro_rules! try_from_integer {
    ($($tt:tt)*) => {};
}
pub(crate) use try_from_integer;

/// Implements `TryFrom`<`$from$from_b`> for `$for$for_b`.
///
/// # Args
/// - `$for`:    the base name of the target. e.g. `NonZeroInteger`.
/// - `$for_b`:  the bit size of the target. e.g. `16`.
/// - `$from`:   the base name of the origin. e.g. `u`.
/// - `$from_b`: a list of bit sizes of the origin. e.g. `8, 16`.
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
/// - `non0_pos`
/// - `neg_int`
#[cfg(feature = "try_from")]
macro_rules! try_from_primitive {
    // try_from_primitive!
    // when `from` is an integer primitive.
    //
    // Used by:
    // - for: Z     from: u, i
    // - for: Nnz   from: u, i
    // - for: N0z   from: u, i
    // - for: Pz    from: u, i
    (int
     for: $for:ident + $for_b:expr,
     from: $from:ident + $( $from_b:expr ),+
    ) => {
        $(
            try_from_primitive![@int for: $for + $for_b, from: $from + $from_b];
        )+
    };
    (@int
     for: $for:ident + $for_b:expr,
     from: $from:ident + $from_b:expr
    ) => {
        devela::paste! {
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "try_from")))]
            impl TryFrom<[<$from$from_b>]> for [<$for$for_b>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: [<$from$from_b>])
                    -> $crate::error::NumeraResult<[<$for$for_b>]> {
                    Self::from_parts(from.try_into()?)
                }
            }
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "try_from")))]
            impl TryFrom<&[<$from$from_b>]> for [<$for$for_b>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: &[<$from$from_b>])
                    -> $crate::error::NumeraResult<[<$for$for_b>]> {
                    Self::from_parts((*from).try_into()?)
                }
            }
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "try_from")))]
            impl TryFrom<&mut [<$from$from_b>]> for [<$for$for_b>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: &mut [<$from$from_b>])
                    -> $crate::error::NumeraResult<[<$for$for_b>]> {
                    Self::from_parts((*from).try_into()?)
                }
            }
        }
    };

    // DONE
    // try_from_primitive!
    // when `from` is a non-zero integer primitive.
    //
    // Used by:
    // - for: Z     from: NonZeroU, NonZeroI
    // - for: Pz    from: NonZeroU, NonZeroI
    // - for: N0z   from: NonZeroU, NonZeroI
    // - for: Nnz   from: NonZeroU, NonZeroI
    (non0
     for: $for:ident + $for_b:expr,
     from: $from:ident + $( $from_b:expr ),+
    ) => {
        $(
            try_from_primitive![@non0 for: $for + $for_b, from: $from + $from_b];
        )+
    };
    (@non0
     for: $for:ident + $for_b:expr,
     from: $from:ident + $from_b:expr
    ) => {
        devela::paste! {
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "try_from")))]
            impl TryFrom<[<$from$from_b>]> for [<$for$for_b>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: [<$from$from_b>])
                    -> $crate::error::NumeraResult<[<$for$for_b>]> {
                    #[cfg(feature = "safe")]
                    return Self::from_parts(from.get().try_into()?);

                    #[cfg(not(feature = "safe"))]
                    // SAFETY: all ok results of try_from should be valid
                    return Ok(unsafe { Self::from_parts_unchecked(from.get().try_into()?) });
                }
            }
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "try_from")))]
            impl TryFrom<&[<$from$from_b>]> for [<$for$for_b>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: &[<$from$from_b>])
                    -> $crate::error::NumeraResult<[<$for$for_b>]> {
                    #[cfg(feature = "safe")]
                    return Self::from_parts(from.get().try_into()?);

                    #[cfg(not(feature = "safe"))]
                    // SAFETY: all ok results of try_from should be valid
                    return Ok(unsafe { Self::from_parts_unchecked(from.get().try_into()?) });
                }
            }
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "try_from")))]
            impl TryFrom<&mut [<$from$from_b>]> for [<$for$for_b>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: &mut [<$from$from_b>])
                    -> $crate::error::NumeraResult<[<$for$for_b>]> {
                    #[cfg(feature = "safe")]
                    return Self::from_parts(from.get().try_into()?);

                    #[cfg(not(feature = "safe"))]
                    // SAFETY: all ok results of try_from should be valid
                    return Ok(unsafe { Self::from_parts_unchecked(from.get().try_into()?) });
                }
            }
        }
    };

    // DONE
    // try_from_primitive!
    // when `from` is a NonZeroI and `for` can not represent positive values
    //
    // Used by:
    // - for: Npz   from: NonZeroI
    (non0_pos
     for: $for:ident + $for_b:expr,
     from: $from:ident + $( $from_b:expr ),+
    ) => {
        $(
            try_from_primitive![@non0_pos for: $for + $for_b, from: $from + $from_b];
        )+
    };
    (@non0_pos
     for: $for:ident + $for_b:expr,
     from: $from:ident + $from_b:expr
    ) => {
        devela::paste! {
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "try_from")))]
            impl TryFrom<[<$from$from_b>]> for [<$for$for_b>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: [<$from$from_b>])
                    -> $crate::error::NumeraResult<[<$for$for_b>]> {
                    #[cfg(feature = "safe")]
                    return Self::from_parts((-from.get()).try_into()?);

                    #[cfg(not(feature = "safe"))]
                    // SAFETY: all ok results of try_from should be valid
                    return Ok(unsafe { Self::from_parts_unchecked((-from.get()).try_into()?) });
                }
            }
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "try_from")))]
            impl TryFrom<&[<$from$from_b>]> for [<$for$for_b>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: &[<$from$from_b>])
                    -> $crate::error::NumeraResult<[<$for$for_b>]> {
                    #[cfg(feature = "safe")]
                    return Self::from_parts((-from.get()).try_into()?);

                    #[cfg(not(feature = "safe"))]
                    // SAFETY: all ok results of try_from should be valid
                    return Ok(unsafe { Self::from_parts_unchecked((-from.get()).try_into()?) });
                }
            }
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "try_from")))]
            impl TryFrom<&mut [<$from$from_b>]> for [<$for$for_b>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: &mut [<$from$from_b>])
                    -> $crate::error::NumeraResult<[<$for$for_b>]> {
                    #[cfg(feature = "safe")]
                    return Self::from_parts((-from.get()).try_into()?);

                    #[cfg(not(feature = "safe"))]
                    // SAFETY: all ok results of try_from should be valid
                    return Ok(unsafe { Self::from_parts_unchecked((-from.get()).try_into()?) });
                }
            }
        }
    };

    // WIP CHECK duplicated??
    // try_from_primitive!
    // when `from` is a primitive integer and `for` can not represent positive values
    //
    // Used by:
    // - for: Npz   from: i
    (neg_int
     for: $for:ident + $for_b:expr,
     from: $from:ident + $( $from_b:expr ),+
    ) => {
        $(
            try_from_primitive![@neg_int for: $for + $for_b, from: $from + $from_b];
        )+
    };
    (@neg_int
     for: $for:ident + $for_b:expr,
     from: $from:ident + $from_b:expr
    ) => {
        devela::paste! {
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "try_from")))]
            impl TryFrom<[<$from$from_b>]> for [<$for$for_b>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: [<$from$from_b>])
                    -> $crate::error::NumeraResult<[<$for$for_b>]> {
                    Self::from_parts((-from).try_into()?)
                }
            }
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "try_from")))]
            impl TryFrom<&[<$from$from_b>]> for [<$for$for_b>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: &[<$from$from_b>])
                    -> $crate::error::NumeraResult<[<$for$for_b>]> {
                    Self::from_parts((-*from).try_into()?)
                }
            }
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "try_from")))]
            impl TryFrom<&mut [<$from$from_b>]> for [<$for$for_b>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: &mut [<$from$from_b>])
                    -> $crate::error::NumeraResult<[<$for$for_b>]> {
                    Self::from_parts((-*from).try_into()?)
                }
            }
        }
    };

    // DONE
    // try_from_primitive!
    // when `from` is a primitive integer and `for` can only represent negative values.
    //
    // Used by:
    // - for: Nz    from: i
    (negnon0_int
     for: $for:ident + $for_b:expr,
     from: $from:ident + $( $from_b:expr ),+
    ) => {
        $(
            try_from_primitive![@neg_int for: $for + $for_b, from: $from + $from_b];
        )+
    };
    (@negnon0_int
     for: $for:ident + $for_b:expr,
     from: $from:ident + $from_b:expr
    ) => {
        devela::paste! {
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "try_from")))]
            impl TryFrom<[<$from$from_b>]> for [<$for$for_b>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: [<$from$from_b>])
                    -> $crate::error::NumeraResult<[<$for$for_b>]> {
                    Self::new_neg((-from).try_into()?)
                }
            }
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "try_from")))]
            impl TryFrom<&[<$from$from_b>]> for [<$for$for_b>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: &[<$from$from_b>])
                    -> $crate::error::NumeraResult<[<$for$for_b>]> {
                    Self::new_neg((-*from).try_into()?)
                }
            }
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "try_from")))]
            impl TryFrom<&mut [<$from$from_b>]> for [<$for$for_b>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: &mut [<$from$from_b>])
                    -> $crate::error::NumeraResult<[<$for$for_b>]> {
                    Self::new_neg((-*from).try_into()?)
                }
            }
        }
    };

    // DONE
    // try_from_primitive!
    // when `from` is a NonZeroI and `for` can not represent positive values.
    //
    // Used by:
    // - for: Nz    from: NonZeroI
    (neg_non0
     for: $for:ident + $for_b:expr,
     from: $from:ident + $( $from_b:expr ),+
    ) => {
        $(
            try_from_primitive![@neg_non0 for: $for + $for_b, from: $from + $from_b];
        )+
    };
    (@neg_non0
     for: $for:ident + $for_b:expr,
     from: $from:ident + $from_b:expr
    ) => {
        devela::paste! {
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "try_from")))]
            impl TryFrom<[<$from$from_b>]> for [<$for$for_b>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: [<$from$from_b>])
                    -> $crate::error::NumeraResult<[<$for$for_b>]> {
                    Self::new_neg((-from.get()).try_into()?)
                }
            }
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "try_from")))]
            impl TryFrom<&[<$from$from_b>]> for [<$for$for_b>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: &[<$from$from_b>])
                    -> $crate::error::NumeraResult<[<$for$for_b>]> {
                    Self::new_neg((-from.get()).try_into()?)
                }
            }
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "try_from")))]
            impl TryFrom<&mut [<$from$from_b>]> for [<$for$for_b>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: &mut [<$from$from_b>])
                    -> $crate::error::NumeraResult<[<$for$for_b>]> {
                    Self::new_neg((-from.get()).try_into()?)
                }
            }
        }
    };

}
/// No-op alternative for disabling `TryFrom` impls.
#[cfg(not(feature = "try_from"))]
macro_rules! try_from_primitive {
    ($($tt:tt)*) => {};
}
pub(crate) use try_from_primitive;

/// Implements `TryFrom`<`$from$from_b`> for `$for$for_b`.
///
/// # Args
/// - `$for`:    the base name of the target. e.g. `NonZeroInteger`.
/// - `$for_b`:  the bit size of the target. e.g. `16`.
/// - `$from`:   the base name of the origin. e.g. `u`.
/// - `$from_b`: a list of bit sizes of the origin. e.g. `8, 16`.
///
/// # Examples
/// ```ignore
/// try_from_any![zero for: NonPositiveInteger + 8, from: u + 8, 16, 32, 64, 128];
/// ```
///
/// # Branches ids
/// - `zero`
/// - `error`
#[cfg(feature = "try_from")]
macro_rules! try_from_any {
    // try_from_any!
    // when the conversion is only valid when the `from` value is `zero`.
    //
    // Used by:
    // - for: Nnz   from: Npz
    // - for: Npz   from: Nnz, u
    (zero
     for: $for:ident + $for_b:expr,
     from: $from:ident + $( $from_b:expr ),+) => {
        $(
            try_from_any![@zero for: $for + $for_b, from: $from + $from_b];
        )+
    };
    (@zero
     for: $for:ident + $for_b:expr, from: $from:ident + $from_b:expr) => {
        devela::paste! {
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "try_from")))]
            impl TryFrom<[<$from$from_b>]> for [<$for$for_b>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: [<$from$from_b>])
                    -> $crate::error::NumeraResult<[<$for$for_b>]> {
                    if from.is_zero() {
                        Ok([<$for$for_b>]::ZERO)
                    } else {
                        Err($crate::error::NumeraError::Conversion)
                    }
                }
            }
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "try_from")))]
            impl TryFrom<&[<$from$from_b>]> for [<$for$for_b>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: &[<$from$from_b>])
                    -> $crate::error::NumeraResult<[<$for$for_b>]> {
                    if from.is_zero() {
                        Ok([<$for$for_b>]::ZERO)
                    } else {
                        Err($crate::error::NumeraError::Conversion)
                    }
                }
            }
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "try_from")))]
            impl TryFrom<&mut [<$from$from_b>]> for [<$for$for_b>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(from: &mut [<$from$from_b>])
                    -> $crate::error::NumeraResult<[<$for$for_b>]> {
                    if from.is_zero() {
                        Ok([<$for$for_b>]::ZERO)
                    } else {
                        Err($crate::error::NumeraError::Conversion)
                    }
                }
            }
        }
    };

    // try_from_any!
    // when the conversion attempt must always error.
    //
    // Used by:
    // - for: Pz    from: Nz, Npz
    // - for: Nz    from: Nnz, Pz
    // - for: Nnz   from: Nz
    // - for: Npz   from: Pz
    (error
     for: $for:ident + $for_b:expr,
     from: $from:ident + $( $from_b:expr ),+) => {
        $(
            try_from_any![@error for: $for + $for_b, from: $from + $from_b];
        )+
    };
    (@error
     for: $for:ident + $for_b:expr, from: $from:ident + $from_b:expr) => {
        devela::paste! {
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "try_from")))]
            impl TryFrom<[<$from$from_b>]> for [<$for$for_b>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(_from: [<$from$from_b>])
                    -> $crate::error::NumeraResult<[<$for$for_b>]> {
                    Err($crate::error::NumeraError::Conversion)
                }
            }
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "try_from")))]
            impl TryFrom<&[<$from$from_b>]> for [<$for$for_b>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(_from: &[<$from$from_b>])
                    -> $crate::error::NumeraResult<[<$for$for_b>]> {
                    Err($crate::error::NumeraError::Conversion)
                }
            }
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "try_from")))]
            impl TryFrom<&mut [<$from$from_b>]> for [<$for$for_b>] {
                type Error = $crate::error::NumeraError;
                #[inline]
                fn try_from(_from: &mut [<$from$from_b>])
                    -> $crate::error::NumeraResult<[<$for$for_b>]> {
                    Err($crate::error::NumeraError::Conversion)
                }
            }
        }
    };
}
/// No-op alternative for disabling `TryFrom` impls.
#[cfg(not(feature = "try_from"))]
macro_rules! try_from_any {
    ($($tt:tt)*) => {};
}
pub(crate) use try_from_any;
