// numera::number::rational::macros
//
//!
//
// TOC
// - from_rational! IMPROVE
// - from_integer!

/* conversions */

/// Implements From<`$from$from_size`> for `$for$for_size`.
///
/// # Args
/// - `$for`:       the base name of the target. e.g. `NonZeroRational`.
/// - `$for_size`:  the bit size of the target. e.g. `16`.
/// - `$num`:       the full type of the numerator. e.g. `Integer16`.
/// - `$den`:       the full type of the denominator. e.g. `NonZeroInteger16`.
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
        paste::paste! {
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
    //     paste::paste! {
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
    //     paste::paste! {
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
    //     paste::paste! {
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

/// Implements From<`$from_p$from_size`> for `$for$for_size`.
///
/// # Args
/// - `$for`:       the base name of the target. e.g. `NonZeroRational`.
/// - `$for_size`:  the bit size of the target. e.g. `16`.
/// - `$num`:       the full type of the numerator. e.g. `Integer16`.
/// - `$den`:       the full type of the denominator. e.g. `NonZeroInteger16`.
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
    // from signed & unsigned primitives
    (prim
     for: $for:ident + $for_size:expr,
     num: $num:ident, den: $den:ident,
     from: $from_p:ident + $( $from_size:expr ),+
    ) => {
        $(
            from_integer![@prim for: $for + $for_size, num: $num, den: $den,
            from: $from_p + $from_size];
        )+
    };

    (@prim
     for: $for:ident + $for_size:expr,
     num: $num:ident, den: $den:ident,
     from: $from_p:ident + $from_size:expr
    ) => {
        paste::paste! {
            impl From<[<$from_p$from_size>]> for [<$for$for_size>] {
                fn from(from: [<$from_p$from_size>]) -> Self {
                    return Self {
                        num: from.into(),
                        den: <$den as $crate::all::ConstOne>::ONE,
                    };
                }
            }
            impl From<&[<$from_p$from_size>]> for [<$for$for_size>] {
                fn from(from: &[<$from_p$from_size>]) -> Self {
                    return Self {
                        num: (*from).into(),
                        den: <$den as $crate::all::ConstOne>::ONE,
                    };
                }
            }
            impl From<&mut [<$from_p$from_size>]> for [<$for$for_size>] {
                fn from(from: &mut [<$from_p$from_size>]) -> Self {
                    return Self {
                        num: (*from).into(),
                        den: <$den as $crate::all::ConstOne>::ONE,
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

    // having to convert from nonzero to core primitive
    (@nonzero
     for: $for:ident + $for_size:expr,
     num: $num:ident, den: $den:ident,
     from: $from_p:ident + $from_size:expr
    ) => {
        paste::paste! {
            impl From<[<$from_p$from_size>]> for [<$for$for_size>] {
                fn from(from: [<$from_p$from_size>]) -> Self {
                    return Self {
                        num: from.get().into(),
                        den: <$den as $crate::all::ConstOne>::ONE,
                    };
                }
            }
            impl From<&[<$from_p$from_size>]> for [<$for$for_size>] {
                fn from(from: &[<$from_p$from_size>]) -> Self {
                    return Self {
                        num: from.get().into(),
                        den: <$den as $crate::all::ConstOne>::ONE,
                    };
                }
            }
            impl From<&mut [<$from_p$from_size>]> for [<$for$for_size>] {
                fn from(from: &mut [<$from_p$from_size>]) -> Self {
                    return Self {
                        num: from.get().into(),
                        den: <$den as $crate::all::ConstOne>::ONE,
                    };
                }
            }
        }
    };

    // from integers
    (integer
     for: $for:ident + $for_size:expr,
     num: $num:ident, den: $den:ident,
     from: $from_p:ident + $( $from_size:expr ),+
    ) => {
        $(
            from_integer![@integer for: $for + $for_size, num: $num, den: $den,
            from: $from_p + $from_size];
        )+
    };

    // having to convert from nonzero to core primitive
    (@integer
     for: $for:ident + $for_size:expr,
     num: $num:ident, den: $den:ident,
     from: $from_p:ident + $from_size:expr
    ) => {
        paste::paste! {
            impl From<[<$from_p$from_size>]> for [<$for$for_size>] {
                fn from(from: [<$from_p$from_size>]) -> Self {
                    return Self {
                        num: from.into(),
                        den: <$den as $crate::all::ConstOne>::ONE,
                    };
                }
            }
            impl From<&[<$from_p$from_size>]> for [<$for$for_size>] {
                fn from(from: &[<$from_p$from_size>]) -> Self {
                    return Self {
                        num: (*from).into(),
                        den: <$den as $crate::all::ConstOne>::ONE,
                    };
                }
            }
            impl From<&mut [<$from_p$from_size>]> for [<$for$for_size>] {
                fn from(from: &mut [<$from_p$from_size>]) -> Self {
                    return Self {
                        num: (*from).into(),
                        den: <$den as $crate::all::ConstOne>::ONE,
                    };
                }
            }
        }
    };
}
pub(crate) use from_integer;
