// numera::number::real::float::macros::from
//
//!
//
// TOC
//
// infallible conversions:
//   - from_integer! TODO
//   - from_primitive!
//   - for_primitive! TODO

/* infallible From conversions */

// /// Implements From: from integers, for integers.
// ///
// /// # Args
// /// - `$for`:    the base name of the target. e.g. `Integer`.
// /// - `$for_b`:  the bit size of the target. e.g. `32`.
// /// - `$from`:   the base name of the origin. e.g. `PositiveInteger`.
// /// - `$from_b`: a list of bit sizes of the origin. e.g. `8, 16`.
// ///
// /// # Examples
// /// ```ignore
// /// from_integer![int for: Integer + i + 32, from: Integer + 8, 16];
// /// ```
// ///
// /// # Branches ids
// /// - ``
// macro_rules! from_integer {
// }
// pub(crate) use from_integer;

/// Implements From: from primitives, for floats.
///
/// # Args
/// - `$for`:    the base name of the target. e.g. `NonZeroInteger`.
/// - `$for_b`:  the bit size of the target. e.g. `16`.
/// - `$from`: the base name of the origin. e.g. `u`.
/// - `$from_b`: a list of bit sizes of the origin. e.g. `8, 16`.
///
/// # Examples
/// ```ignore
/// from_primitive![for: Float + 64, from: f + 32];
/// from_primitive![for: Float + 64, from: f + 32, 64];
/// ```
///
/// # Branches ids
/// - `float_f`
/// - `float_half`
/// - `float_tf`
/// - `float_tf_half`
macro_rules! from_primitive {
    // from_primitive!
    // when `from` is the same integer primitive than the inner part of `for`.
    //
    // - for: Bf    from: bf
    // - for: F     from: f
    // - for: F128  from: TwoFloat
    (float_f for: $for:ident + $for_b:expr, from: $from:ident + $( $from_b:expr ),+) => {
        $( from_primitive![@float_f for: $for + $for_b, from: $from + $from_b]; )+
    };
    (@float_f for: $for:ident + $for_b:expr, from: $from:ident + $from_b:expr) => {
        $crate::all::impl_from!(for: $for+$for_b, from: $from+$from_b, arg:f, body: {
            Self(f.into())
        });
        $crate::all::impl_from!(for: $for+$for_b, from: &$from+$from_b, arg:f, body: {
            Self((*f).into())
        });
    };

    /* external primitives */

    // from_primitive!
    // when `from` is an external primitive from the `half` crate.
    //
    // - for: F     from: bf16, f16
    (float_half for: $for:ident + $for_b:expr, from: $from:ident + $( $from_b:expr ),+) => {
        $( from_primitive![@float_half for: $for + $for_b, from: $from + $from_b]; )+
    };
    (@float_half for: $for:ident + $for_b:expr, from: $from:ident + $from_b:expr) => {
        $crate::all::impl_from!(for: $for+$for_b, from: @$from+$from_b, arg:f, body: {
            f.to_f32().into()
        });
    };

    // from_primitive!
    // when `for` has an inner `twofloat::TwoFloat`.
    //
    // - for: F128   from: f, TwoFloat
    (float_tf for: $for:ident + $for_b:expr, from: $from:ident + $( $from_b:expr ),+) => {
        $( from_primitive![@float_tf for: $for + $for_b, from: $from + $from_b]; )+
    };
    (@float_tf for: $for:ident + $for_b:expr, from: $from:ident + $from_b:expr) => {
        $crate::all::impl_from!(for: $for+$for_b, from: $from+$from_b, arg:f, body: {
            Self(f.into())
        });
        $crate::all::impl_from!(for: $for+$for_b, from: &$from+$from_b, arg:f, body: {
            Self((*f).into())
        });
    };

    // from_primitive!
    // when `from` is an external primitive from the `half` crate.
    // and `for` has an inner TwoFloat.
    //
    // - for: F128   from: bf16, f16
    (float_tf_half for: $for:ident + $for_b:expr, from: $from:ident + $( $from_b:expr ),+) => {
        $( from_primitive![@float_tf_half for: $for + $for_b, from: $from + $from_b]; )+
    };
    (@float_tf_half for: $for:ident + $for_b:expr, from: $from:ident + $from_b:expr) => {
        $crate::all::impl_from!(for: $for+$for_b, from: @$from+$from_b, arg:f, body: {
            Self(f.to_f32().into())
        });
    };

}
pub(crate) use from_primitive;

// /// Implements From: from integers, for primitives.
// ///
// /// # Args
// /// - `$for`:    the base name of the target. e.g. `NonZeroInteger`.
// /// - `$for_b`:  the bit size of the target. e.g. `16`.
// /// - `$from`: the base name of the origin. e.g. `u`.
// /// - `$from_b`: a list of bit sizes of the origin. e.g. `8, 16`.
// ///
// /// # Examples
// /// ```ignore
// /// ```
// ///
// /// # Branches ids
// /// - ``
// macro_rules! for_primitive {
// }
// pub(crate) use for_primitive;
