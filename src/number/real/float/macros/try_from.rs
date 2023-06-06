// numera::number::real::float::macros::try_from
//
//!
//
// TOC
//
// fallible conversions:
//   - try_from_float! TODO
//   - try_from_primitive! WIP
//   - try_for_primitive! TODO

/* fallible TryFrom conversions */

// /// Implements `TryFrom`: from floats, for floats.
// ///
// /// # Args
// /// - `$for`:    the base name of the target. e.g. `Float`.
// /// - `$for_b`:  the bit size of the target. e.g. `32`.
// /// - `$from`:   the base name of the origin. e.g. `Float`.
// /// - `$from_b`: a list of bit sizes of the origin. e.g. `32, 64`.
// ///
// /// # Examples
// /// ```ignore
// /// try_from_float![int for: Float + i + 8, from: Float + 8, 16];
// /// ```
// ///
// /// # Branches ids
// /// - ``
// #[cfg(feature = "try_from")]
// macro_rules! try_from_float {
// }
// /// No-op alternative for disabling `TryFrom` impls.
// #[cfg(not(feature = "try_from"))]
// macro_rules! try_from_float {
//     ($($tt:tt)*) => {};
// }
// pub(crate) use try_from_float;

/// Implements `TryFrom`: from primitives, for floats.
///
/// # Args
/// - `$for`:    the base name of the target. e.g. `Float`.
/// - `$for_i`:  the base name of the inner target. e.g. `f`. (only in `float_half`)
/// - `$for_b`:  the bit size of the target. e.g. `32`.
/// - `$from`:   the base name of the origin. e.g. `f`.
/// - `$from_b`: a list of bit sizes of the origin. e.g. `32,64`.
///
/// # Examples
/// ```ignore
/// try_from_primitive![for: Float + 32, from: f + 64];
/// ```
///
/// # Branches ids
/// - `float`
#[cfg(feature = "try_from")]
macro_rules! try_from_primitive {
    // try_from_primitive!
    // when `from` is a float primitive.
    // and `for` has an smaller inner float primitive.
    // (If casting overflows the converted result will be infinite)
    //
    // Used by:
    // - for: F32     from: f64
    (float_f for: $for:ident + $for_b:expr, from: $from:ident + $( $from_b:expr ),+) => {
        $( try_from_primitive![@float_f for: $for + $for_b, from: $from + $from_b]; )+
    };
    (@float_f for: $for:ident + $for_b:expr, from: $from:ident + $from_b:expr) => {
        devela::paste! {
            $crate::all::impl_from!(try for: $for+$for_b, from: $from+$from_b, arg:f, body: {
                let converted = f as [<$from$for_b>];
                if f.is_finite() && converted.is_infinite() {
                    Err(Self::Error::Conversion)
                } else {
                    Ok(Self(converted))
                }
            });
            $crate::all::impl_from!(try for: $for+$for_b, from: &$from+$from_b, arg:f, body: {
                let converted = *f as [<$from$for_b>];
                if f.is_finite() && converted.is_infinite() {
                    Err(Self::Error::Conversion)
                } else {
                    Ok(Self(converted))
                }
            });
        }
    };

    // try_from_primitive!
    // when `from` is a float primitive,
    // and `for` has an smaller external float primitive from the `half` crate.
    // (If casting overflows the converted result will be infinite)
    //
    // Used by:
    // - for: F16     from: f32,f64
    // - for: Bf16    from: f32,f64
    (float_half for: $for:ident + $for_i:ident + $for_b:expr,
     from: $from:ident + $( $from_b:expr ),+) => {
        $( try_from_primitive![@float_half for: $for + $for_i + $for_b, from: $from + $from_b]; )+
    };
    (@float_half for: $for:ident + $for_i:ident + $for_b:expr,
     from: $from:ident + $from_b:expr) => {
        devela::paste! {
            $crate::all::impl_from!(try for: $for+$for_b, from: $from+$from_b, arg:f, body: {
                let converted = [<$for_i$for_b>]::[<from_f$from_b>](f);
                if f.is_finite() && converted.is_infinite() {
                    Err(Self::Error::Conversion)
                } else {
                    Ok(Self(converted))
                }
            });
            $crate::all::impl_from!(try for: $for+$for_b, from: &$from+$from_b, arg:f, body: {
                let converted = [<$for_i$for_b>]::[<from_f$from_b>](*f);
                if f.is_finite() && converted.is_infinite() {
                    Err(Self::Error::Conversion)
                } else {
                    Ok(Self(converted))
                }
            });
        }
    };
    // try_from_primitive!
    // when `from` is a float external primitive from the `half` crate,
    // and `for` has an a different float external primitive from the `half` crate.
    // (If casting overflows the converted result will be infinite)
    //
    // Used by:
    // - for: F16     from: bf16
    // - for: Bf16    from: f16
    (float_half_ne for: $for:ident + $for_i:ident + $for_b:expr,
     from: $from:ident + $( $from_b:expr ),+) => {
        $( try_from_primitive![@float_half_ne for: $for + $for_i + $for_b, from: $from + $from_b]; )+
    };
    (@float_half_ne for: $for:ident + $for_i:ident + $for_b:expr,
     from: $from:ident + $from_b:expr) => {
        devela::paste! {
            $crate::all::impl_from!(try for: $for+$for_b, from: @$from+$from_b, arg:f, body: {
                let converted = [<$for_i$for_b>]::from_f32(f.to_f32());
                if f.is_finite() && converted.is_infinite() {
                    Err(Self::Error::Conversion)
                } else {
                    Ok(Self(converted))
                }
            });
        }
    };

}
/// No-op alternative for disabling `TryFrom` impls.
#[cfg(not(feature = "try_from"))]
macro_rules! try_from_primitive {
    ($($tt:tt)*) => {};
}
pub(crate) use try_from_primitive;

// /// Implements `TryFrom`: from floats, for primitives.
// ///
// /// # Args
// /// - `$for`:    the base name of the target. e.g. `NonZeroFloat`.
// /// - `$for_b`:  the bit size of the target. e.g. `16`.
// /// - `$from`:   the base name of the origin. e.g. `u`.
// /// - `$from_b`: a list of bit sizes of the origin. e.g. `8, 16`.
// ///
// /// # Examples
// /// ```ignore
// /// try_for_primitive![for: Float + 8, from: u + 8, 16, 32, 64, 128];
// /// ```
// ///
// /// # Branches ids
// /// - ``
// #[cfg(feature = "try_from")]
// macro_rules! try_for_primitive {
// }
// /// No-op alternative for disabling `TryFrom` impls.
// #[cfg(not(feature = "try_from"))]
// macro_rules! try_for_primitive {
//     ($($tt:tt)*) => {};
// }
// pub(crate) use try_for_primitive;
