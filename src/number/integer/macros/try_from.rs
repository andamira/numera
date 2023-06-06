// numera::number::integer::macros::try_from
//
//!
//
// TOC
//
// fallible conversions:
//   - try_from_integer!
//   - try_from_primitive!
//   - try_for_primitive!
//   - try_for_big! TODO
//   - try_from_any!

/* fallible TryFrom conversions */

/// Implements `TryFrom`: from integers, for integers.
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
    // - for: Nnz   from: Z, Nnz, P
    // - for: Z     from: Z, Nnz, Npz, P
    // - for: Npz   from: Npz
    // - for: Pz    from: N0z
    (int for: $for:ident + $for_b:expr, from: $from:ident + $( $from_b:expr ),+) => {
        $( try_from_integer![@int for: $for + $for_b, from: $from + $from_b]; )+
    };
    (@int for: $for:ident + $for_b:expr, from: $from:ident + $from_b:expr) => {
        $crate::all::impl_from!(try for: $for+$for_b, from: @$from+$from_b, arg:f, body: {
            Ok(Self(f.0.try_into()?))
        });
    };

    // try_from_integer!
    // when `from` has an inner integer primitive,
    // and `for` has an inner non-zero,
    // and we manually maintain the invariants.
    //
    // Used by:
    // - for: N0z   from: P
    // - for: Pz    from: P
    (int_non0 for: $for:ident + $for_b:expr, from: $from:ident + $( $from_b:expr ),+) => {
        $( try_from_integer![@int_non0 for: $for + $for_b, from: $from + $from_b]; )+
    };
    (@int_non0 for: $for:ident + $for_b:expr, from: $from:ident + $from_b:expr) => {
        $crate::all::impl_from!(try for: $for+$for_b, from: @$from+$from_b, arg:f, body: {
            Ok(Self::new(f.0.try_into()?).unwrap())
        });
    };

    // try_from_integer!
    // when `from` has an unsigned inner primitive representing only negative values,
    // which has to be negated in the conversion.
    //
    // Used by:
    // - for: Z     from: Npz
    (neg_signed
     for: $for:ident + $p:ident + $for_b:expr, from: $from:ident + $( $from_b:expr ),+) => {
        $( try_from_integer![@neg_signed for: $for + $p + $for_b, from: $from + $from_b]; )+
    };
    (@neg_signed for: $for:ident + $p:ident + $for_b:expr, from: $from:ident + $from_b:expr) => {
        devela::paste! {
            $crate::all::impl_from!(try for: $for+$for_b, from: @$from+$from_b, arg:f, body: {
                Ok(Self(TryInto::<[<$p$for_b>]>::try_into(f.0)?.neg()))
            });
        }
    };

    // try_from_integer!
    // when `from` has an inner integer primitive and `for` has an inner nonzero primitive
    //
    // Used by:
    // - for: N0z   from: Z, Nnz
    // - for: Pz    from: Z
    (non0_int for: $for:ident + $for_b:expr, from: $from:ident + $( $from_b:expr ),+) => {
        $( try_from_integer![@non0_int for: $for + $for_b, from: $from + $from_b]; )+
    };
    (@non0_int for: $for:ident + $for_b:expr, from: $from:ident + $from_b:expr) => {
        $crate::all::impl_from!(try for: $for+$for_b, from: @$from+$from_b, arg:f, body: {
            Self::new(f.0.try_into()?)
        });
    };

    // try_from_integer!
    // when `from` has an unsigned inner primitive representing only negative values,
    // and `for` has a non-zero inner primitive
    //
    // Used by:
    // - for: N0z   from: Npz
    (negnon0_non0 for: $for:ident + $for_b:expr, from: $from:ident + $( $from_b:expr ),+) => {
        $( try_from_integer![@negnon0_non0 for: $for + $for_b, from: $from + $from_b]; )+
    };
    (@negnon0_non0 for: $for:ident + $for_b:expr, from: $from:ident + $from_b:expr) => {
        $crate::all::impl_from!(try for: $for+$for_b, from: @$from+$from_b, arg:f, body: {
            Ok(Self::new(f.0.try_into()?)?.neg())
        });
    };

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
    (non0 for: $for:ident + $for_b:expr, from: $from:ident + $( $from_b:expr ),+) => {
        $( try_from_integer![@non0 for: $for + $for_b, from: $from + $from_b]; )+
    };
    (@non0 for: $for:ident + $for_b:expr, from: $from:ident + $from_b:expr) => {
        $crate::all::impl_from!(try for: $for+$for_b, from: @$from+$from_b, arg:f, body: {
            #[cfg(feature = "safe")]
            return Self::from_parts(f.0.get().try_into()?);

            #[cfg(not(feature = "safe"))]
            // SAFETY: all ok results of try_from should be valid
            return Ok(unsafe { Self::from_parts_unchecked(f.0.get().try_into()?) });
        });
    };

    // try_from_integer!
    // when `from` has an unsigned inner NonZero* representing only negative values,
    // which has to be negated in the conversion.
    //
    // - for: Z     from: Nz
    // - for: N0z   from: Nz
    (negnon0_signed
     for: $for:ident + $p:ident + $for_b:expr, from: $from:ident + $( $from_b:expr ),+) => {
        $( try_from_integer![@negnon0_signed for: $for + $p + $for_b, from: $from + $from_b]; )+
    };
    (@negnon0_signed
     for: $for:ident + $p:ident + $for_b:expr, from: $from:ident + $from_b:expr) => {
        devela::paste! {
            $crate::all::impl_from!(try for: $for+$for_b, from: @$from+$from_b, arg:f, body: {
                #[cfg(feature = "safe")]
                return Self::from_parts(TryInto::<[<$p$for_b>]>::try_into(f.0.get())?.neg());

                #[cfg(not(feature = "safe"))]
                // SAFETY: all ok results of try_from should be valid
                return unsafe {
                    Ok(Self::from_parts_unchecked(
                        TryInto::<[<$p$for_b>]>::try_into(f.0.get())?.neg()
                    ))
                };
            });
        }
    };

    // try_from_integer!
    // when `from` is an Integer and `for` can not represent negative values.
    //
    // Used by:
    // - for: Npz   from: Z
    (neg_int for: $for:ident + $for_b:expr, from: $from:ident + $( $from_b:expr ),+) => {
        $( try_from_integer![@neg_int for: $for + $for_b, from: $from + $from_b]; )+
    };
    (@neg_int for: $for:ident + $for_b:expr, from: $from:ident + $from_b:expr) => {
        $crate::all::impl_from!(try for: $for+$for_b, from: @$from+$from_b, arg:f, body: {
            Ok(Self((-f.0).try_into()?))
        });
    };

    // try_from_integer!
    // when `from` is an Integer and `for` can not represent negative values or 0.
    //
    // Used by:
    // - for: Nz   from: Z
    (negnon0_int for: $for:ident + $for_b:expr, from: $from:ident + $( $from_b:expr ),+) => {
        $( try_from_integer![@negnon0_int for: $for + $for_b, from: $from + $from_b]; )+
    };
    (@negnon0_int for: $for:ident + $for_b:expr, from: $from:ident + $from_b:expr) => {
        $crate::all::impl_from!(try for: $for+$for_b, from: @$from+$from_b, arg:f, body: {
            Self::new_neg((-f.0).try_into()?)
        });
    };

    // try_from_integer!
    // when `from` is a NonZeroInteger and `for` can only represent positive values
    //
    // Used by:
    // - for: Npz   from: N0z
    (non0_pos for: $for:ident + $for_b:expr, from: $from:ident + $( $from_b:expr ),+) => {
        $( try_from_integer![@non0_pos for: $for + $for_b, from: $from + $from_b]; )+
    };
    (@non0_pos for: $for:ident + $for_b:expr, from: $from:ident + $from_b:expr) => {
        $crate::all::impl_from!(try for: $for+$for_b, from: @$from+$from_b, arg:f, body: {
            #[cfg(feature = "safe")]
            return Self::from_parts((-f.0.get()).try_into()?);

            #[cfg(not(feature = "safe"))]
            // SAFETY: all ok results of try_from should be valid
            return Ok(unsafe { Self::from_parts_unchecked((-f.0.get()).try_into()?) });
        });
    };

    // try_from_integer!
    // when both `for` and `from` are non-positive Integers
    //
    // Used by:
    // - for: Nz   from: Npz
    (neg_non0neg for: $for:ident + $for_b:expr, from: $from:ident + $( $from_b:expr ),+) => {
        $( try_from_integer![@neg->non0_neg for: $for + $for_b, from: $from + $from_b]; )+
    };
    (@neg->non0_neg for: $for:ident + $for_b:expr, from: $from:ident + $from_b:expr) => {
        $crate::all::impl_from!(try for: $for+$for_b, from: @$from+$from_b, arg:f, body: {
            Self::new_neg((f.0).try_into()?)
        });
    };

    // try_from_integer!
    // when `from` is a non-zero integer and `for` can not represent positive values
    //
    // Used by:
    // - for: Nz    from: N0z
    (neg_non0 for: $for:ident + $for_b:expr, from: $from:ident + $( $from_b:expr ),+) => {
        $( try_from_integer![@neg_non0 for: $for + $for_b, from: $from + $from_b]; )+
    };
    (@neg_non0 for: $for:ident + $for_b:expr, from: $from:ident + $from_b:expr) => {
        $crate::all::impl_from!(try for: $for+$for_b, from: @$from+$from_b, arg:f, body: {
            Self::new_neg((-f.0.get()).try_into()?)
        });
    };
}
/// No-op alternative for disabling `TryFrom` impls.
#[cfg(not(feature = "try_from"))]
macro_rules! try_from_integer {
    ($($tt:tt)*) => {};
}
pub(crate) use try_from_integer;

/// Implements `TryFrom`: from primitives, for integers.
///
/// # Args
/// - `$for`:    the base name of the target. e.g. `NonZeroInteger`.
/// - `$for_b`:  the bit size of the target. e.g. `16`.
/// - `$from`:   the base name of the origin. e.g. `u`.
/// - `$from_b`: a list of bit sizes of the origin. e.g. `8, 16`.
///
/// # Examples
/// ```ignore
/// try_from_primitive![for: Integer + 8, from: u + 8, 16, 32, 64, 128];
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
    (int for: $for:ident + $for_b:expr, from: $from:ident + $( $from_b:expr ),+) => {
        $( try_from_primitive![@int for: $for + $for_b, from: $from + $from_b]; )+
    };
    (@int for: $for:ident + $for_b:expr, from: $from:ident + $from_b:expr) => {
        $crate::all::impl_from!(try for: $for+$for_b, from: $from+$from_b, arg:f, body: {
            Self::from_parts(f.try_into()?)
        });
        $crate::all::impl_from!(try for: $for+$for_b, from: &$from+$from_b, arg:f, body: {
            Self::from_parts((*f).try_into()?)
        });
    };

    // try_from_primitive!
    // when `from` is a non-zero integer primitive.
    //
    // Used by:
    // - for: Z     from: NonZeroU, NonZeroI
    // - for: Pz    from: NonZeroU, NonZeroI
    // - for: N0z   from: NonZeroU, NonZeroI
    // - for: Nnz   from: NonZeroU, NonZeroI
    (non0 for: $for:ident + $for_b:expr, from: $from:ident + $( $from_b:expr ),+) => {
        $( try_from_primitive![@non0 for: $for + $for_b, from: $from + $from_b]; )+
    };
    (@non0 for: $for:ident + $for_b:expr, from: $from:ident + $from_b:expr) => {
        $crate::all::impl_from!(try for: $for+$for_b, from: @$from+$from_b, arg:f, body: {
            #[cfg(feature = "safe")]
            return Self::from_parts(f.get().try_into()?);

            #[cfg(not(feature = "safe"))]
            // SAFETY: all ok results of try_from should be valid
            return Ok(unsafe { Self::from_parts_unchecked(f.get().try_into()?) });
        });
    };

    // try_from_primitive!
    // when `from` is a NonZeroI and `for` can not represent positive values
    //
    // Used by:
    // - for: Npz   from: NonZeroI
    (non0_pos for: $for:ident + $for_b:expr, from: $from:ident + $( $from_b:expr ),+) => {
        $( try_from_primitive![@non0_pos for: $for + $for_b, from: $from + $from_b]; )+
    };
    (@non0_pos for: $for:ident + $for_b:expr, from: $from:ident + $from_b:expr) => {
        $crate::all::impl_from!(try for: $for+$for_b, from: @$from+$from_b, arg:f, body: {
            #[cfg(feature = "safe")]
            return Self::from_parts((-f.get()).try_into()?);

            #[cfg(not(feature = "safe"))]
            // SAFETY: all ok results of try_from should be valid
            return Ok(unsafe { Self::from_parts_unchecked((-f.get()).try_into()?) });
        });
    };

    // try_from_primitive!
    // when `from` is a primitive integer and `for` can not represent positive values
    //
    // Used by:
    // - for: Npz   from: i
    (neg_int for: $for:ident + $for_b:expr, from: $from:ident + $( $from_b:expr ),+) => {
        $( try_from_primitive![@neg_int for: $for + $for_b, from: $from + $from_b]; )+
    };
    (@neg_int for: $for:ident + $for_b:expr, from: $from:ident + $from_b:expr) => {
        $crate::all::impl_from!(try for: $for+$for_b, from: $from+$from_b, arg:f, body: {
            Self::from_parts((-f).try_into()?)
        });
        $crate::all::impl_from!(try for: $for+$for_b, from: &$from+$from_b, arg:f, body: {
            Self::from_parts((-*f).try_into()?)
        });
    };

    // try_from_primitive!
    // when `from` is a primitive integer and `for` can only represent negative values.
    //
    // Used by:
    // - for: Nz    from: i
    (negnon0_int for: $for:ident + $for_b:expr, from: $from:ident + $( $from_b:expr ),+) => {
        $( try_from_primitive![@neg_int for: $for + $for_b, from: $from + $from_b]; )+
    };
    (@negnon0_int for: $for:ident + $for_b:expr, from: $from:ident + $from_b:expr) => {
        $crate::all::impl_from!(try for: $for+$for_b, from: $from+$from_b, arg:f, body: {
            Self::new_neg((-f).try_into()?)
        });
        $crate::all::impl_from!(try for: $for+$for_b, from: &$from+$from_b, arg:f, body: {
            Self::new_neg((-*f).try_into()?)
        });
    };

    // try_from_primitive!
    // when `from` is a NonZeroI and `for` can not represent positive values.
    //
    // Used by:
    // - for: Nz    from: NonZeroI
    (neg_non0 for: $for:ident + $for_b:expr, from: $from:ident + $( $from_b:expr ),+) => {
        $( try_from_primitive![@neg_non0 for: $for + $for_b, from: $from + $from_b]; )+
    };
    (@neg_non0 for: $for:ident + $for_b:expr, from: $from:ident + $from_b:expr) => {
        $crate::all::impl_from!(try for: $for+$for_b, from: @$from+$from_b, arg:f, body: {
            Self::new_neg((-f.get()).try_into()?)
        });
    };
}
/// No-op alternative for disabling `TryFrom` impls.
#[cfg(not(feature = "try_from"))]
macro_rules! try_from_primitive {
    ($($tt:tt)*) => {};
}
pub(crate) use try_from_primitive;

/// Implements `TryFrom`: from integers, for primitives.
///
/// # Args
/// - `$for`:    the base name of the target. e.g. `NonZeroInteger`.
/// - `$for_b`:  the bit size of the target. e.g. `16`.
/// - `$from`:   the base name of the origin. e.g. `u`.
/// - `$from_b`: a list of bit sizes of the origin. e.g. `8, 16`.
///
/// # Examples
/// ```ignore
/// try_for_primitive![for: Integer + 8, from: u + 8, 16, 32, 64, 128];
/// ```
///
/// # Branches ids
/// - `int`
/// - `non0`
/// - `neg_int`
/// - `non0_pos`
/// - `neg_int`
#[cfg(feature = "try_from")]
macro_rules! try_for_primitive {
    // try_for_primitive!
    // when `from` is an integer with the same Zeroness as `for`
    //
    // Used by:
    // - for: i        from: Z, Nnz
    // - for: u        from: Z, Nnz, Pz
    // - for: NonZeroI from: N0z, Pz
    // - for: NonZeroU from: N0z
    (int for: $for:ident + $for_b:expr, from: $from:ident + $( $from_b:expr ),+) => {
        $( try_for_primitive![@int for: $for + $for_b, from: $from + $from_b]; )+
    };
    (@int for: $for:ident + $for_b:expr, from: $from:ident + $from_b:expr) => {
        $crate::all::impl_from!(try for: $for+$for_b, from: @$from+$from_b, arg:f, body: {
            Ok(f.0.try_into()?)
        });
    };

    // try_for_primitive!
    // when `from` is a non-zero integer, while `for` does support zero.
    //
    // Used by:
    // - for: i     from: N0z, Pz
    // - for: u     from: N0z, Pz
    (non0 for: $for:ident + $for_b:expr, from: $from:ident + $( $from_b:expr ),+) => {
        $( try_for_primitive![@non0 for: $for + $for_b, from: $from + $from_b]; )+
    };
    (@non0 for: $for:ident + $for_b:expr, from: $from:ident + $from_b:expr) => {
        $crate::all::impl_from!(try for: $for+$for_b, from: @$from+$from_b, arg:f, body: {
            Ok(f.0.get().try_into()?)
        });
    };

    // try_for_primitive!
    // when `from` supports zero, but `for` does not.
    //
    // Used by:
    // - for: NonZeroI  from: Z, Nnz
    // - for: NonZeroU  from: Z, Nnz
    (int_non0 for: $for:ident + $for_b:expr, from: $from:ident + $( $from_b:expr ),+) => {
        $( try_for_primitive![@int_non0 for: $for + $for_b, from: $from + $from_b]; )+
    };
    (@int_non0 for: $for:ident + $for_b:expr, from: $from:ident + $from_b:expr) => {
        devela::paste! {
            $crate::all::impl_from!(try for: $for+$for_b, from: @$from+$from_b, arg:f, body: {
                [<$for$for_b>]::new(f.0.try_into()?).ok_or(Self::Error::Conversion)
            });
        }
    };

    // try_for_primitive!
    // when `from` is an non-positive integer and `for` is a signed primitive
    //
    // Used by:
    // - for: i        from: Npz
    (neg for: $for:ident + $for_b:expr, from: $from:ident + $( $from_b:expr ),+) => {
        $( try_for_primitive![@neg for: $for + $for_b, from: $from + $from_b]; )+
    };
    (@neg for: $for:ident + $for_b:expr, from: $from:ident + $from_b:expr) => {
        devela::paste! {
            $crate::all::impl_from!(try for: $for+$for_b, from: @$from+$from_b, arg:f, body: {
                Ok([<$for$for_b>]::try_from(f.0)?.neg())
            });
        }
    };

    // try_for_primitive!
    // when `from` is an non-positive integer and `for` is a non-zero signed primitive
    //
    // Used by:
    // - for: NonZeroI  from: Npz
    (neg_non0 for: $for:ident + $for_b:expr, from: $from:ident + $( $from_b:expr ),+) => {
        $( try_for_primitive![@neg_non0 for: $for + $for_b, from: $from + $from_b]; )+
    };
    (@neg_non0 for: $for:ident + $for_b:expr, from: $from:ident + $from_b:expr) => {
        devela::paste! {
            $crate::all::impl_from!(try for: $for+$for_b, from: @$from+$from_b, arg:f, body: {
                [<$for$for_b>]::new([<i$for_b>]::try_from(f.0)?.neg())
                    .ok_or(Self::Error::Conversion)
            });
        }
    };

    // try_for_primitive!
    // when `from` is a negative integer, and `for` is a signed primitive
    //
    // Used by:
    // - for: i     from: Nz
    (non0neg for: $for:ident + $for_b:expr, from: $from:ident + $( $from_b:expr ),+) => {
        $( try_for_primitive![@non0neg for: $for + $for_b, from: $from + $from_b]; )+
    };
    (@non0neg for: $for:ident + $for_b:expr, from: $from:ident + $from_b:expr) => {
        devela::paste! {
            $crate::all::impl_from!(try for: $for+$for_b, from: @$from+$from_b, arg:f, body: {
                Ok([<$for$for_b>]::try_from(f.0.get())?.neg())
            });
        }
    };

    // try_for_primitive!
    // when `from` is an negative integer and `for` is a non-zero signed primitive
    //
    // Used by:
    // - for: NonZeroI  from: Nz
    (non0neg_non0 for: $for:ident + $for_b:expr, from: $from:ident + $( $from_b:expr ),+) => {
        $( try_for_primitive![@non0neg_non0 for: $for + $for_b, from: $from + $from_b]; )+
    };
    (@non0neg_non0 for: $for:ident + $for_b:expr, from: $from:ident + $from_b:expr) => {
        devela::paste! {
            $crate::all::impl_from!(try for: $for+$for_b, from: @$from+$from_b, arg:f, body: {
                [<$for$for_b>]::new([<i$for_b>]::try_from(f.0.get())?.neg())
                    .ok_or(Self::Error::Conversion)
            });
        }
    };
}
/// No-op alternative for disabling `TryFrom` impls.
#[cfg(not(feature = "try_from"))]
macro_rules! try_for_primitive {
    ($($tt:tt)*) => {};
}
pub(crate) use try_for_primitive;

/// Implements `TryFrom`: from any, for any.
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
    // - for: Nnz       from: Npz
    // - for: Npz       from: Nnz, u
    // - for: u         from: Npz
    (zero for: $for:ident + $for_b:expr, from: $from:ident + $( $from_b:expr ),+) => {
        $( try_from_any![@zero for: $for + $for_b, from: $from + $from_b]; )+
    };
    (@zero
     for: $for:ident + $for_b:expr, from: $from:ident + $from_b:expr) => {
        devela::paste! {
            $crate::all::impl_from!(try for: $for+$for_b, from: @$from+$from_b, arg:f, body: {
                if f.is_zero() {
                    Ok([<$for$for_b>]::ZERO)
                } else {
                    Err($crate::error::NumeraError::Conversion)
                }
            });
        }
    };

    // try_from_any!
    // when the conversion attempt must always error.
    //
    // Used by:
    // - for: Pz        from: Nz, Npz
    // - for: Nz        from: Nnz, Pz, P
    // - for: Nnz       from: Nz
    // - for: Npz       from: Pz, P
    // - for: u         from: Nz
    // - for: NonZeroU  from: Npz, Nz
    (error for: $for:ident + $for_b:expr, from: $from:ident + $( $from_b:expr ),+) => {
        $( try_from_any![@error for: $for + $for_b, from: $from + $from_b]; )+
    };
    (@error for: $for:ident + $for_b:expr, from: $from:ident + $from_b:expr) => {
        $crate::all::impl_from!(try for: $for+$for_b, from: @$from+$from_b, arg:_f, body: {
            Err($crate::error::NumeraError::Conversion)
        });
    };
}
/// No-op alternative for disabling `TryFrom` impls.
#[cfg(not(feature = "try_from"))]
macro_rules! try_from_any {
    ($($tt:tt)*) => {};
}
pub(crate) use try_from_any;
