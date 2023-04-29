// numera::number::rational::macros
//
//!
//
// TOC
// - impl_from_rational! TODO
// - impl_from_integer!

/* conversions */

/// Implements From<`$from_p $from_size`> for `$for$for_size`.
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
/// impl_from_integer![prim for: Rational + 16, from: i + 8, 16];
/// impl_from_integer![nonzero for: Rational + 16, from: NonZeroI + 8, 16];
/// impl_from_integer![integer for: Rational + 16, from: Integer + 8, 16];
/// ```
macro_rules! impl_from_integer {
    // from signed & unsigned primitives
    (prim
     for: $for:ident + $for_size:expr,
     num: $num:ident, den: $den:ident,
     from: $from_p:ident + $( $from_size:expr ),+
    ) => {
        $(
            impl_from_integer![@prim for: $for + $for_size, num: $num, den: $den,
            from: $from_p + $from_size];
        )+
    };

    (@prim
     for: $for:ident + $for_size:expr,
     num: $num:ident, den: $den:ident,
     from: $from_p:ident + $from_size:expr
    ) => {
        paste::paste! {
            impl From<[< $from_p $from_size >]> for [< $for $for_size >] {
                fn from(from: [< $from_p $from_size >]) -> Self {
                    return Self {
                        num: from.into(),
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
            impl_from_integer![@nonzero for: $for + $for_size, num: $num, den: $den,
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
            impl From<[< $from_p $from_size >]> for [< $for $for_size >] {
                fn from(from: [< $from_p $from_size >]) -> Self {
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
            impl_from_integer![@integer for: $for + $for_size, num: $num, den: $den,
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
            impl From<[< $from_p $from_size >]> for [< $for $for_size >] {
                fn from(from: [< $from_p $from_size >]) -> Self {
                    return Self {
                        num: from.into(),
                        den: <$den as $crate::all::ConstOne>::ONE,
                    };
                }
            }
        }
    };

}
pub(crate) use impl_from_integer;
