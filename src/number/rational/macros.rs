// numera::number::rational::macros
//
//!
//
// TOC
//
// infallible conversions:
//   - from_rational!
//   - from_integer!
//
// fallible conversions:
//   - try_from_rational!
//   - try_from_integer!

/* infallible From conversions */

/// Implements `From`<`$from$from_b`> for `$for$for_b`.
///
/// # Args
/// - `$for`:       the base name of the target. e.g. `NonZeroRational`.
/// - `$for_b`:  the bit size of the target. e.g. `16`.
/// - `$num`:       the base type of the numerator. e.g. `Integer`.
/// - `$den`:       the base type of the denominator. e.g. `NonZeroInteger`.
/// - `$from`:    the base name of the origin. e.g. `u`.
/// - `$from_b`: a list of bit sizes of the origin. e.g. `8, 16`.
///
/// # Examples
/// ```ignore
/// from_rational![signed for:Q+16, num:Z16, den:N0z16, from:Q + 8, 16];
/// ```
macro_rules! from_rational {
    // default
    (
     for: $for:ident + $for_b:expr,
     num: $num:ident, den: $den:ident,
     from: $from:ident + $( $from_b:expr ),+
    ) => {
        $( from_rational![@ for: $for + $for_b, num: $num, den: $den, from: $from + $from_b]; )+
    };
    (@
     for: $for:ident + $for_b:expr,
     num: $num:ident, den: $den:ident,
     from: $from:ident + $from_b:expr
    ) => {
        $crate::all::impl_from!(for: $for + $for_b, from: @$from+$from_b, arg:f, body: {
            Self {
                num: f.num.into(),
                den: f.den.into(),
            }
        });
    };
}
pub(crate) use from_rational;

/// Implements `From`<`$from$from_b`> for `$for$for_b`.
///
/// # Args
/// - `$for`:       the base name of the target. e.g. `NonZeroRational`.
/// - `$for_b`:  the bit size of the target. e.g. `16`.
/// - `$num`:       the base type of the numerator. e.g. `Integer`.
/// - `$den`:       the base type of the denominator. e.g. `NonZeroInteger`.
/// - `$from`:    the base name of the origin. e.g. `u`.
/// - `$from_b`: a list of bit sizes of the origin. e.g. `8, 16`.
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
     for: $for:ident + $for_b:expr,
     num: $num:ident, den: $den:ident,
     from: $from:ident + $( $from_b:expr ),+
    ) => {
        $(
            from_integer![@primint for: $for + $for_b, num: $num, den: $den,
            from: $from + $from_b];
        )+
    };
    (@primint
     for: $for:ident + $for_b:expr,
     num: $num:ident, den: $den:ident,
     from: $from:ident + $from_b:expr
    ) => {
        devela::paste! {
            $crate::all::impl_from!(for: $for + $for_b, from: $from+$from_b, arg:f, body: {
                Self {
                    num: f.into(),
                    den: <[<$den$for_b>] as $crate::all::ConstOne>::ONE,
                }
            });
            $crate::all::impl_from!(for: $for + $for_b, from: &$from+$from_b, arg:f, body: {
                Self {
                    num: (*f).into(),
                    den: <[<$den$for_b>] as $crate::all::ConstOne>::ONE,
                }
            });
        }
    };

    // from NonZero* primitives
    (nonzero
     for: $for:ident + $for_b:expr,
     num: $num:ident, den: $den:ident,
     from: $from:ident + $( $from_b:expr ),+
    ) => {
        $(
            from_integer![@nonzero for: $for + $for_b, num: $num, den: $den,
            from: $from + $from_b];
        )+
    };
    (@nonzero
     for: $for:ident + $for_b:expr,
     num: $num:ident, den: $den:ident,
     from: $from:ident + $from_b:expr
    ) => {
        devela::paste! {
            $crate::all::impl_from!(for: $for + $for_b, from: @$from+$from_b, arg:f, body: {
                Self {
                    num: f.get().into(),
                    den: <[<$den$for_b>] as $crate::all::ConstOne>::ONE,
                }
            });
        }
    };
}
pub(crate) use from_integer;

/// Implements `TryFrom`<`$from$from_b`> for `$for$for_b`.
///
/// # Args
/// - `$for`:       the base name of the target. e.g. `NonZeroRational`.
/// - `$for_b`:  the bit size of the target. e.g. `16`.
/// - `$num`:       the base type of the numerator. e.g. `Integer`.
/// - `$den`:       the base type of the denominator. e.g. `NonZeroInteger`.
/// - `$from`:    the base name of the origin. e.g. `u`.
/// - `$from_b`: a list of bit sizes of the origin. e.g. `8, 16`.
///
/// # Examples
/// ```ignore
/// from_rational![signed for:Q+16, num:Z16, den:N0z16, from:Q + 8, 16];
/// ```
#[cfg(feature = "try_from")]
macro_rules! try_from_rational {
    // default
    (
     for: $for:ident + $for_b:expr,
     num: $num:ident, den: $den:ident,
     from: $from:ident + $( $from_b:expr ),+
    ) => {
        $(
            try_from_rational![@ for: $for + $for_b, num: $num, den: $den,
            from: $from + $from_b];
        )+
    };
    (@
     for: $for:ident + $for_b:expr,
     num: $num:ident, den: $den:ident,
     from: $from:ident + $from_b:expr
    ) => {
        $crate::all::impl_from!(try for: $for + $for_b, from: @$from+$from_b, arg:f, body: {
            Ok(Self {
                num: f.num.try_into()?,
                den: f.den.try_into()?,
            })
        });
    };
}
/// No-op alternative for disabling `TryFrom` impls.
#[cfg(not(feature = "try_from"))]
macro_rules! try_from_rational {
    ($($tt:tt)*) => {};
}
pub(crate) use try_from_rational;

/// Implements `TryFrom`<`$from$from_b`> for `$for$for_b`.
///
/// # Args
/// - `$for`:       the base name of the target. e.g. `NonZeroRational`.
/// - `$for_b`:  the bit size of the target. e.g. `16`.
/// - `$num`:       the base type of the numerator. e.g. `Integer`.
/// - `$den`:       the base type of the denominator. e.g. `NonZeroInteger`.
/// - `$from`:    the base name of the origin. e.g. `u`.
/// - `$from_b`: a list of bit sizes of the origin. e.g. `8, 16`.
///
/// # Examples
/// ```ignore
/// try_from_integer![prim for:Q+16, num:Z16, den:N0z16, from:i + 8, 16];
/// ```
#[cfg(feature = "try_from")]
macro_rules! try_from_integer {
    // from signed & unsigned primitives or integers
    (primint
     for: $for:ident + $for_b:expr,
     num: $num:ident, den: $den:ident,
     from: $from:ident + $( $from_b:expr ),+
    ) => {
        $(
            try_from_integer![@primint for: $for + $for_b, num: $num, den: $den,
            from: $from + $from_b];
        )+
    };
    (@primint
     for: $for:ident + $for_b:expr,
     num: $num:ident, den: $den:ident,
     from: $from:ident + $from_b:expr
    ) => {
        devela::paste! {
            $crate::all::impl_from!(try for: $for + $for_b, from: $from+$from_b, arg:f, body: {
                Ok(Self {
                    num: f.try_into()?,
                    den: <[<$den$for_b>] as $crate::all::ConstOne>::ONE,
                })
            });
            $crate::all::impl_from!(try for: $for + $for_b, from: &$from+$from_b, arg:f, body: {
                Ok(Self {
                    num: (*f).try_into()?,
                    den: <[<$den$for_b>] as $crate::all::ConstOne>::ONE,
                })
            });
        }
    };

    // from NonZero* primitives
    (nonzero
     for: $for:ident + $for_b:expr,
     num: $num:ident, den: $den:ident,
     from: $from:ident + $( $from_b:expr ),+
    ) => {
        $(
            try_from_integer![@nonzero for: $for + $for_b, num: $num, den: $den,
            from: $from + $from_b];
        )+
    };

    // having to convert from nonzero to core primitive
    (@nonzero
     for: $for:ident + $for_b:expr,
     num: $num:ident, den: $den:ident,
     from: $from:ident + $from_b:expr
    ) => {
        devela::paste! {
            $crate::all::impl_from!(try for: $for + $for_b, from: @$from+$from_b, arg:f, body: {
                Ok(Self {
                    num: f.get().try_into()?,
                    den: <[<$den$for_b>] as $crate::all::ConstOne>::ONE,
                })
            });
        }
    };
}
/// No-op alternative for disabling `TryFrom` impls.
#[cfg(not(feature = "try_from"))]
macro_rules! try_from_integer {
    ($($tt:tt)*) => {};
}
pub(crate) use try_from_integer;
