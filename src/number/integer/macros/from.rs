// numera::number::integer::macros::from
//
//!
//
// TOC
//
// infallible conversions:
//   - from_integer!
//   - from_primitive!
//   - for_primitive!
//   - for_big

/* infallible From conversions */

/// Implements From: from integers, for integers.
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
    // - for: Z     from: Z, Nnz, Npz, P
    // - for: Nnz   from: Nnz, P
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

    // from_integer!
    // when `from` has an inner integer primitive,
    // and `for` has an inner non-zero,
    // and we manually maintain the invariants.
    //
    // Used by:
    // - for: N0z   from: P
    // - for: Pz    from: P
    (int_non0
     for: $for:ident + $for_b:expr,
     from: $from:ident + $( $from_b:expr ),+) => {
        $(
            from_integer![@int_non0 for: $for + $for_b, from: $from + $from_b];
        )+
    };
    (@int_non0
     for: $for:ident + $for_b:expr, from: $from:ident + $from_b:expr) => {
        devela::paste! {
            impl From<[<$from$from_b>]> for [<$for$for_b>] {
                #[inline]
                fn from(from: [<$from$from_b>]) -> Self {
                    Self::new(from.0.into()).unwrap()
                }
            }
            impl From<&[<$from$from_b>]> for [<$for$for_b>] {
                #[inline]
                fn from(from: &[<$from$from_b>]) -> Self {
                    Self::new(from.0.into()).unwrap()
                }
            }
            impl From<&mut [<$from$from_b>]> for [<$for$for_b>] {
                #[inline]
                fn from(from: &mut [<$from$from_b>]) -> Self {
                    Self::new(from.0.into()).unwrap()
                }
            }
        }
    };

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

/// Implements From: from primitives, for integers.
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

/// Implements From: from integers, for primitives.
///
/// # Args
/// - `$for`:    the base name of the target. e.g. `NonZeroInteger`.
/// - `$for_b`:  the bit size of the target. e.g. `16`.
/// - `$from_p`: the base name of the origin. e.g. `u`.
/// - `$from_b`: a list of bit sizes of the origin. e.g. `8, 16`.
///
/// # Examples
/// ```ignore
/// for_primitive![many for: i+16, from: Integer+8];
/// for_primitive![many for: i+16, from: Integer+8,16];
/// ```
///
/// # Branches ids
/// - `int`
/// - `non0`
macro_rules! for_primitive {
    // for_primitive!
    // when `from` is an integer
    //
    // - for: i        from: Z
    // - for: u        from: Z, Nnz
    // - for: NonZeroI from: N0z
    // - for: NonZeroU from: Pz
    (int
     for: $for:ident + $for_b:expr,
     from: $from_p:ident + $( $from_b:expr ),+
    ) => {
        $(
            for_primitive![@int for: $for + $for_b, from: $from_p + $from_b];
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
                    from.0.into()
                }
            }
            impl From<&[<$from_p$from_b>]> for [<$for$for_b>] {
                #[inline]
                fn from(from: &[<$from_p$from_b>]) -> Self {
                    from.0.into()
                }
            }
            impl From<&mut [<$from_p$from_b>]> for [<$for$for_b>] {
                #[inline]
                fn from(from: &mut [<$from_p$from_b>]) -> Self {
                    from.0.into()
                }
            }
        }
    };

    // for_primitive!
    // when `from` is a non-zero integer
    //
    // - for: i        from: N0z
    // - for: u        from: Pz
    // - for: NonZeroI from: N0z
    (non0
     for: $for:ident + $for_b:expr,
     from: $from_p:ident + $( $from_b:expr ),+
    ) => {
        $(
            for_primitive![@non0 for: $for + $for_b, from: $from_p + $from_b];
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
                    from.0.get().into()
                }
            }
            impl From<&[<$from_p$from_b>]> for [<$for$for_b>] {
                #[inline]
                fn from(from: &[<$from_p$from_b>]) -> Self {
                    from.0.get().into()
                }
            }
            impl From<&mut [<$from_p$from_b>]> for [<$for$for_b>] {
                #[inline]
                fn from(from: &mut [<$from_p$from_b>]) -> Self {
                    from.0.get().into()
                }
            }
        }
    };
}
pub(crate) use for_primitive;

// WIP
/// Implements From: from integers, for big integers.
///
/// # Args
/// - `$for`:    the base name of the target. e.g. `NonZeroInteger`.
/// - `$for_b`:  the bit size of the target. e.g. `16`.
/// - `$from_p`: the base name of the origin. e.g. `u`.
/// - `$from_b`: a list of bit sizes of the origin. e.g. `8, 16`.
///
/// # Examples
/// ```ignore
/// TODO
/// for_big![many for: IntegerBig, from: Integer+8];
/// for_big![many for: IntegerBig, from: Integer+8,16];
/// ```
///
/// # Branches ids
/// - `int`
/// - `non0`
macro_rules! for_big {
    // for_big!
    // when `from` is an integer
    //
    // - from: Z, Nnz
    (int
     for: $for:ident,
     from: $from_p:ident + $( $from_b:expr ),+
    ) => {
        $(
            for_big![@int for: $for, from: $from_p + $from_b];
        )+
    };
    (@int
     for: $for:ident,
     from: $from_p:ident + $from_b:expr
    ) => {
        devela::paste! {
            impl From<[<$from_p$from_b>]> for $for {
                #[inline]
                fn from(from: [<$from_p$from_b>]) -> Self {
                    Self(from.0.into())
                }
            }
            impl From<&[<$from_p$from_b>]> for $for {
                #[inline]
                fn from(from: &[<$from_p$from_b>]) -> Self {
                    Self(from.0.into())
                }
            }
            impl From<&mut [<$from_p$from_b>]> for $for {
                #[inline]
                fn from(from: &mut [<$from_p$from_b>]) -> Self {
                    Self(from.0.into())
                }
            }
        }
    };

    // for_big!
    // when `from` is a negative integer
    //
    // - from: Nz
    (intneg
     for: $for:ident,
     from: $from_p:ident + $( $from_b:expr ),+
    ) => {
        $(
            for_big![@intneg for: $for, from: $from_p + $from_b];
        )+
    };
    (@intneg
     for: $for:ident,
     from: $from_p:ident + $from_b:expr
    ) => {
        devela::paste! {
            impl From<[<$from_p$from_b>]> for $for {
                #[inline]
                fn from(from: [<$from_p$from_b>]) -> Self {
                    Self(<ibig::IBig>::from(from.0).neg())
                }
            }
            impl From<&[<$from_p$from_b>]> for $for {
                #[inline]
                fn from(from: &[<$from_p$from_b>]) -> Self {
                    Self(<ibig::IBig>::from(from.0).neg())
                }
            }
            impl From<&mut [<$from_p$from_b>]> for $for {
                #[inline]
                fn from(from: &mut [<$from_p$from_b>]) -> Self {
                    Self(<ibig::IBig>::from(from.0).neg())
                }
            }
        }
    };

    // for_big!
    // when `from` is a non-zero integer
    //
    // - from: N0z, Pz
    (non0int
     for: $for:ident,
     from: $from_p:ident + $( $from_b:expr ),+
    ) => {
        $(
            for_big![@non0int for: $for, from: $from_p + $from_b];
        )+
    };
    (@non0int
     for: $for:ident,
     from: $from_p:ident + $from_b:expr
    ) => {
        devela::paste! {
            impl From<[<$from_p$from_b>]> for $for {
                #[inline]
                fn from(from: [<$from_p$from_b>]) -> Self {
                    Self(from.0.get().into())
                }
            }
            impl From<&[<$from_p$from_b>]> for $for {
                #[inline]
                fn from(from: &[<$from_p$from_b>]) -> Self {
                    Self(from.0.get().into())
                }
            }
            impl From<&mut [<$from_p$from_b>]> for $for {
                #[inline]
                fn from(from: &mut [<$from_p$from_b>]) -> Self {
                    Self(from.0.get().into())
                }
            }
        }
    };

    // for_big!
    // when `from` is a non-zero negative integer
    //
    // - from: Nz
    (non0intneg
     for: $for:ident,
     from: $from_p:ident + $( $from_b:expr ),+
    ) => {
        $(
            for_big![@non0intneg for: $for, from: $from_p + $from_b];
        )+
    };
    (@non0intneg
     for: $for:ident,
     from: $from_p:ident + $from_b:expr
    ) => {
        devela::paste! {
            impl From<[<$from_p$from_b>]> for $for {
                #[inline]
                fn from(from: [<$from_p$from_b>]) -> Self {
                    Self(<ibig::IBig>::from(from.0.get()).neg())
                }
            }
            impl From<&[<$from_p$from_b>]> for $for {
                #[inline]
                fn from(from: &[<$from_p$from_b>]) -> Self {
                    Self(<ibig::IBig>::from(from.0.get()).neg())
                }
            }
            impl From<&mut [<$from_p$from_b>]> for $for {
                #[inline]
                fn from(from: &mut [<$from_p$from_b>]) -> Self {
                    Self(<ibig::IBig>::from(from.0.get()).neg())
                }
            }
        }
    };

    // for_big!
    // when `from` is a primitive
    //
    // - from: i, u
    (prim
     for: $for:ident,
     from: $from_p:ident + $( $from_b:expr ),+
    ) => {
        $(
            for_big![@prim for: $for, from: $from_p + $from_b];
        )+
    };
    (@prim
     for: $for:ident,
     from: $from_p:ident + $from_b:expr
    ) => {
        devela::paste! {
            impl From<[<$from_p$from_b>]> for $for {
                #[inline]
                fn from(from: [<$from_p$from_b>]) -> Self {
                    Self(from.into())
                }
            }
            impl From<&[<$from_p$from_b>]> for $for {
                #[inline]
                fn from(from: &[<$from_p$from_b>]) -> Self {
                    Self(<ibig::IBig>::from(*from))
                }
            }
            impl From<&mut [<$from_p$from_b>]> for $for {
                #[inline]
                fn from(from: &mut [<$from_p$from_b>]) -> Self {
                    Self(<ibig::IBig>::from(*from))
                }
            }
        }
    };

    // for_big!
    // when `from` is a non-zero primitive
    //
    // - from: NonZeroI, NonZeroU
    (non0prim
     for: $for:ident,
     from: $from_p:ident + $( $from_b:expr ),+
    ) => {
        $(
            for_big![@non0prim for: $for, from: $from_p + $from_b];
        )+
    };
    (@non0prim
     for: $for:ident,
     from: $from_p:ident + $from_b:expr
    ) => {
        devela::paste! {
            impl From<core::num::[<$from_p$from_b>]> for $for {
                #[inline]
                fn from(from: core::num::[<$from_p$from_b>]) -> Self {
                    Self(from.get().into())
                }
            }
            impl From<&core::num::[<$from_p$from_b>]> for $for {
                #[inline]
                fn from(from: &core::num::[<$from_p$from_b>]) -> Self {
                    Self(<ibig::IBig>::from(from.get()))
                }
            }
            impl From<&mut core::num::[<$from_p$from_b>]> for $for {
                #[inline]
                fn from(from: &mut core::num::[<$from_p$from_b>]) -> Self {
                    Self(<ibig::IBig>::from(from.get()))
                }
            }
        }
    };
}
pub(crate) use for_big;
