// numera::number::rational::macros
//
//!
//
// TOC
//
// infallible conversions:
//   - from_rational! IMPROVE
//   - from_integer!
//
// fallible conversions:
//   - try_from_rational!
//   - try_from_integer!

/* infallible From conversions */

/// Implements `From`<`$from$from_size`> for `$for$for_size`.
///
/// # Args
/// - `$for`:       the base name of the target. e.g. `NonZeroRational`.
/// - `$for_size`:  the bit size of the target. e.g. `16`.
/// - `$num`:       the base type of the numerator. e.g. `Integer`.
/// - `$den`:       the base type of the denominator. e.g. `NonZeroInteger`.
///
/// - `$from_p`:    the base name of the origin. e.g. `u`.
/// - `$from_size`: a list of bit sizes of the origin. e.g. `8, 16`.
///
/// # Examples
/// ```ignore
/// from_rational![signed for:Q+16, num:Z16, den:N0z16, from:Q + 8, 16];
/// ```
macro_rules! from_rational {
    // default
    (
     for: $for:ident + $for_size:expr,
     num: $num:ident, den: $den:ident,
     from: $from_p:ident + $( $from_size:expr ),+
    ) => {
        $(
            from_rational![@ for: $for + $for_size, num: $num, den: $den,
            from: $from_p + $from_size];
        )+
    };

    // default
    (@
     for: $for:ident + $for_size:expr,
     num: $num:ident, den: $den:ident,
     from: $from_p:ident + $from_size:expr
    ) => {
        devela::paste! {
            impl From<[<$from_p$from_size>]> for [<$for$for_size>] {
                fn from(from: [<$from_p$from_size>]) -> Self {
                    return Self {
                        num: from.num.into(),
                        den: from.den.into(),
                    };
                }
            }
            impl From<&[<$from_p$from_size>]> for [<$for$for_size>] {
                fn from(from: &[<$from_p$from_size>]) -> Self {
                    return Self {
                        num: from.num.into(),
                        den: from.den.into(),
                    };
                }
            }
            impl From<&mut [<$from_p$from_size>]> for [<$for$for_size>] {
                fn from(from: &mut [<$from_p$from_size>]) -> Self {
                    return Self {
                        num: from.num.into(),
                        den: from.den.into(),
                    };
                }
            }
        }
    };

    // (nonzero
    //  for: $for:ident + $p:ident + $for_size:expr, from: $from:ident + $( $from_size:expr ),+) => {
    //     $(
    //         from_rational![@nonzero for: $for + $p + $for_size, from: $from + $from_size];
    //     )+
    // };
    // (@nonzero
    //  for: $for:ident + $p:ident + $for_size:expr, from: $from:ident + $from_size:expr) => {
    //     devela::paste! {
    //         impl From<[<$from$from_size>]> for [<$for$for_size>] {
    //             fn from(from: [<$from$from_size>]) -> Self {
    //                 // TODO
    //                 todo![]
    //             }
    //         }
    //     }
    // };
    //
    // (int_neg
    //  for: $for:ident + $p:ident + $for_size:expr, from: $from:ident + $( $from_size:expr ),+) => {
    //     $(
    //         from_rational![@int_neg for: $for + $p + $for_size, from: $from + $from_size];
    //     )+
    // };
    // (@int_neg
    //  for: $for:ident + $p:ident + $for_size:expr, from: $from:ident + $from_size:expr) => {
    //     devela::paste! {
    //         impl From<[<$from$from_size>]> for [<$for$for_size>] {
    //             fn from(from: [<$from$from_size>]) -> Self {
    //                 // TODO
    //                 todo![]
    //             }
    //         }
    //     }
    // };
    //
    // (nonzero_neg
    //  for: $for:ident + $p:ident + $for_size:expr, from: $from:ident + $( $from_size:expr ),+) => {
    //     $(
    //         from_rational![@nonzero_neg
    //         for: $for + $p + $for_size, from: $from + $from_size];
    //     )+
    // };
    // (@nonzero_neg
    //  for: $for:ident + $p:ident + $for_size:expr, from: $from:ident + $from_size:expr) => {
    //     devela::paste! {
    //         impl From<[<$from$from_size>]> for [<$for$for_size>] {
    //             fn from(from: [<$from$from_size>]) -> Self {
    //                 // TODO
    //                 todo![]
    //             }
    //         }
    //     }
    // };
}
pub(crate) use from_rational;

/// Implements `From`<`$from_p$from_size`> for `$for$for_size`.
///
/// # Args
/// - `$for`:       the base name of the target. e.g. `NonZeroRational`.
/// - `$for_size`:  the bit size of the target. e.g. `16`.
/// - `$num`:       the base type of the numerator. e.g. `Integer`.
/// - `$den`:       the base type of the denominator. e.g. `NonZeroInteger`.
///
/// - `$from_p`:    the base name of the origin. e.g. `u`.
/// - `$from_size`: a list of bit sizes of the origin. e.g. `8, 16`.
///
/// # Examples
/// ```ignore
/// from_integer![prim for:Q+16, num:Z16, den:N0z16, from:i + 8, 16];
/// from_integer![nonzero for:Q+16, num:Z16, den:N0z16, from:NonZeroI + 8, 16];
/// from_integer![integer for:Q+16, num:Z16, den:N0z16, from:Integer + 8, 16];
/// ```
macro_rules! from_integer {
    // from signed & unsigned primitives or integers
    (primint
     for: $for:ident + $for_size:expr,
     num: $num:ident, den: $den:ident,
     from: $from_p:ident + $( $from_size:expr ),+
    ) => {
        $(
            from_integer![@primint for: $for + $for_size, num: $num, den: $den,
            from: $from_p + $from_size];
        )+
    };

    (@primint
     for: $for:ident + $for_size:expr,
     num: $num:ident, den: $den:ident,
     from: $from_p:ident + $from_size:expr
    ) => {
        devela::paste! {
            impl From<[<$from_p$from_size>]> for [<$for$for_size>] {
                fn from(from: [<$from_p$from_size>]) -> Self {
                    return Self {
                        num: from.into(),
                        den: <[<$den$for_size>] as $crate::all::ConstOne>::ONE,
                    };
                }
            }
            impl From<&[<$from_p$from_size>]> for [<$for$for_size>] {
                fn from(from: &[<$from_p$from_size>]) -> Self {
                    return Self {
                        num: (*from).into(),
                        den: <[<$den$for_size>] as $crate::all::ConstOne>::ONE,
                    };
                }
            }
            impl From<&mut [<$from_p$from_size>]> for [<$for$for_size>] {
                fn from(from: &mut [<$from_p$from_size>]) -> Self {
                    return Self {
                        num: (*from).into(),
                        den: <[<$den$for_size>] as $crate::all::ConstOne>::ONE,
                    };
                }
            }
        }
    };

    // from NonZero* primitives
    (nonzero
     for: $for:ident + $for_size:expr,
     num: $num:ident, den: $den:ident,
     from: $from_p:ident + $( $from_size:expr ),+
    ) => {
        $(
            from_integer![@nonzero for: $for + $for_size, num: $num, den: $den,
            from: $from_p + $from_size];
        )+
    };

    (@nonzero
     for: $for:ident + $for_size:expr,
     num: $num:ident, den: $den:ident,
     from: $from_p:ident + $from_size:expr
    ) => {
        devela::paste! {
            impl From<[<$from_p$from_size>]> for [<$for$for_size>] {
                fn from(from: [<$from_p$from_size>]) -> Self {
                    return Self {
                        num: from.get().into(),
                        den: <[<$den$for_size>] as $crate::all::ConstOne>::ONE,
                    };
                }
            }
            impl From<&[<$from_p$from_size>]> for [<$for$for_size>] {
                fn from(from: &[<$from_p$from_size>]) -> Self {
                    return Self {
                        num: from.get().into(),
                        den: <[<$den$for_size>] as $crate::all::ConstOne>::ONE,
                    };
                }
            }
            impl From<&mut [<$from_p$from_size>]> for [<$for$for_size>] {
                fn from(from: &mut [<$from_p$from_size>]) -> Self {
                    return Self {
                        num: from.get().into(),
                        den: <[<$den$for_size>] as $crate::all::ConstOne>::ONE,
                    };
                }
            }
        }
    };
}
pub(crate) use from_integer;

/// Implements `TryFrom`<`$from$from_size`> for `$for$for_size`.
///
/// # Args
/// - `$for`:       the base name of the target. e.g. `NonZeroRational`.
/// - `$for_size`:  the bit size of the target. e.g. `16`.
/// - `$num`:       the base type of the numerator. e.g. `Integer`.
/// - `$den`:       the base type of the denominator. e.g. `NonZeroInteger`.
///
/// - `$from_p`:    the base name of the origin. e.g. `u`.
/// - `$from_size`: a list of bit sizes of the origin. e.g. `8, 16`.
///
/// # Examples
/// ```ignore
/// from_rational![signed for:Q+16, num:Z16, den:N0z16, from:Q + 8, 16];
/// ```
#[cfg(feature = "try_from")]
macro_rules! try_from_rational {
    // default
    (
     for: $for:ident + $for_size:expr,
     num: $num:ident, den: $den:ident,
     from: $from_p:ident + $( $from_size:expr ),+
    ) => {
        $(
            try_from_rational![@ for: $for + $for_size, num: $num, den: $den,
            from: $from_p + $from_size];
        )+
    };

    // default
    (@
     for: $for:ident + $for_size:expr,
     num: $num:ident, den: $den:ident,
     from: $from_p:ident + $from_size:expr
    ) => {
        devela::paste! {
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "try_from")))]
            impl TryFrom<[<$from_p$from_size>]> for [<$for$for_size>] {
                type Error = $crate::error::NumeraError;
                fn try_from(from: [<$from_p$from_size>])
                    -> $crate::error::NumeraResult<[<$for$for_size>]> {
                    return Ok(Self {
                        num: from.num.try_into()?,
                        den: from.den.try_into()?,
                    });
                }
            }
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "try_from")))]
            impl TryFrom<&[<$from_p$from_size>]> for [<$for$for_size>] {
                type Error = $crate::error::NumeraError;
                fn try_from(from: &[<$from_p$from_size>])
                    -> $crate::error::NumeraResult<[<$for$for_size>]> {
                    return Ok(Self {
                        num: from.num.try_into()?,
                        den: from.den.try_into()?,
                    });
                }
            }
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "try_from")))]
            impl TryFrom<&mut [<$from_p$from_size>]> for [<$for$for_size>] {
                type Error = $crate::error::NumeraError;
                fn try_from(from: &mut [<$from_p$from_size>])
                    -> $crate::error::NumeraResult<[<$for$for_size>]> {
                    return Ok(Self {
                        num: from.num.try_into()?,
                        den: from.den.try_into()?,
                    });
                }
            }
        }
    };
}
/// No-op alternative for disabling `TryFrom` impls.
#[cfg(not(feature = "try_from"))]
macro_rules! try_from_rational {
    ($($tt:tt)*) => {};
}
pub(crate) use try_from_rational;

/// Implements `TryFrom`<`$from_p$from_size`> for `$for$for_size`.
///
/// # Args
/// - `$for`:       the base name of the target. e.g. `NonZeroRational`.
/// - `$for_size`:  the bit size of the target. e.g. `16`.
/// - `$num`:       the base type of the numerator. e.g. `Integer`.
/// - `$den`:       the base type of the denominator. e.g. `NonZeroInteger`.

/// - `$from_p`:    the base name of the origin. e.g. `u`.
/// - `$from_size`: a list of bit sizes of the origin. e.g. `8, 16`.
///
/// # Examples
/// ```ignore
/// try_from_integer![prim for:Q+16, num:Z16, den:N0z16, from:i + 8, 16];
/// ```
#[cfg(feature = "try_from")]
macro_rules! try_from_integer {
    // from signed & unsigned primitives or integers
    (primint
     for: $for:ident + $for_size:expr,
     num: $num:ident, den: $den:ident,
     from: $from_p:ident + $( $from_size:expr ),+
    ) => {
        $(
            try_from_integer![@primint for: $for + $for_size, num: $num, den: $den,
            from: $from_p + $from_size];
        )+
    };

    (@primint
     for: $for:ident + $for_size:expr,
     num: $num:ident, den: $den:ident,
     from: $from_p:ident + $from_size:expr
    ) => {
        devela::paste! {
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "try_from")))]
            impl TryFrom<[<$from_p$from_size>]> for [<$for$for_size>] {
                type Error = $crate::error::NumeraError;
                fn try_from(from: [<$from_p$from_size>])
                    -> $crate::error::NumeraResult<[<$for$for_size>]> {
                    return Ok(Self {
                        num: from.try_into()?,
                        den: <[<$den$for_size>] as $crate::all::ConstOne>::ONE,
                    });
                }
            }
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "try_from")))]
            impl TryFrom<&[<$from_p$from_size>]> for [<$for$for_size>] {
                type Error = $crate::error::NumeraError;
                fn try_from(from: &[<$from_p$from_size>])
                    -> $crate::error::NumeraResult<[<$for$for_size>]> {
                    return Ok(Self {
                        num: (*from).try_into()?,
                        den: <[<$den$for_size>] as $crate::all::ConstOne>::ONE,
                    });
                }
            }
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "try_from")))]
            impl TryFrom<&mut [<$from_p$from_size>]> for [<$for$for_size>] {
                type Error = $crate::error::NumeraError;
                fn try_from(from: &mut [<$from_p$from_size>])
                    -> $crate::error::NumeraResult<[<$for$for_size>]> {
                    return Ok(Self {
                        num: (*from).try_into()?,
                        den: <[<$den$for_size>] as $crate::all::ConstOne>::ONE,
                    });
                }
            }
        }
    };

    // from NonZero* primitives
    (nonzero
     for: $for:ident + $for_size:expr,
     num: $num:ident, den: $den:ident,
     from: $from_p:ident + $( $from_size:expr ),+
    ) => {
        $(
            try_from_integer![@nonzero for: $for + $for_size, num: $num, den: $den,
            from: $from_p + $from_size];
        )+
    };

    // having to convert from nonzero to core primitive
    (@nonzero
     for: $for:ident + $for_size:expr,
     num: $num:ident, den: $den:ident,
     from: $from_p:ident + $from_size:expr
    ) => {
        devela::paste! {
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "try_from")))]
            impl TryFrom<[<$from_p$from_size>]> for [<$for$for_size>] {
                type Error = $crate::error::NumeraError;
                fn try_from(from: [<$from_p$from_size>])
                    -> $crate::error::NumeraResult<[<$for$for_size>]> {
                    return Ok(Self {
                        num: from.get().try_into()?,
                        den: <[<$den$for_size>] as $crate::all::ConstOne>::ONE,
                    });
                }
            }
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "try_from")))]
            impl TryFrom<&[<$from_p$from_size>]> for [<$for$for_size>] {
                type Error = $crate::error::NumeraError;
                fn try_from(from: &[<$from_p$from_size>])
                    -> $crate::error::NumeraResult<[<$for$for_size>]> {
                    return Ok(Self {
                        num: from.get().try_into()?,
                        den: <[<$den$for_size>] as $crate::all::ConstOne>::ONE,
                    });
                }
            }
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "try_from")))]
            impl TryFrom<&mut [<$from_p$from_size>]> for [<$for$for_size>] {
                type Error = $crate::error::NumeraError;
                fn try_from(from: &mut [<$from_p$from_size>])
                    -> $crate::error::NumeraResult<[<$for$for_size>]> {
                    return Ok(Self {
                        num: from.get().try_into()?,
                        den: <[<$den$for_size>] as $crate::all::ConstOne>::ONE,
                    });
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
