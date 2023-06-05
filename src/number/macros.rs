// numera::number::macros
//
//!
//
// TOC
//
// - define-abbreviations!
// - impl_from!
// - impl_larger_smaller!

/// Creates abbreviation type aliases.
///
/// # Args
/// - `$a`: the abbreviation of the type.
/// - `$name`: the name of the type.
/// $(
/// - `$b`: the size in bits.
/// )*
macro_rules! define_abbreviations {
    // multiple sized
    ($a:ident, $name:ident, $( $b:expr ),+) => {
        $( define_abbreviations![sized $a, $name, $b]; )+

        define_abbreviations![family $a, $name];
    };

    // single sized
    (sized $a:ident, $name:ident, $b:expr) => {
        devela::paste!{
            #[doc = "Abbreviation of [`" $name $b "`]." ]
            pub type [<$a$b>] = [<$name $b>];
        }
    };

    // single family
    (family $a:ident, $name:ident) => {
        devela::paste!{
            #[doc = "Abbreviation of [`" $name "`] family." ]
            pub type $a = $name;
        }
    };
}
pub(crate) use define_abbreviations;

/// Creates type aliases.
///
/// # Args
/// - `$a`: the alias of the type.
/// - `$name`: the name of the type.
/// - `$b`: the size in bits.
macro_rules! define_aliases {
    ($a:ident, $name:ident, $( $b:expr ),+) => {
        $( define_aliases![sized $a, $name, $b]; )+
    };

    (sized $a:ident, $name:ident, $b:expr) => {
        devela::paste!{
            #[doc = "Alias of [`" $name $b "`]." ]
            pub type [<$a$b>] = [<$name $b>];
        }
    };
}
pub(crate) use define_aliases;

/// Implements `From` from: owned, `&` and `&mut`.
///
/// # Args
///
/// - try:       optional prefix for implementing TryFrom
///
/// - `$for`:    the base name of the target. e.g. `Integer`.
/// - `$for_b`:  the bit size of the target. e.g. `32`. (optional)
/// - `$from`:   the base name of the origin. e.g. `i`.
/// - `$from_b`: the bit size of the origin. e.g. `16`.
/// - `$arg`:    the name of the argument to the from function.
/// - `$body`:   the body of the from function.
macro_rules! impl_from {
    // implements `From` from: owned, `&` and `&mut`.
    //
    // Use this when all implementations share the same body.
    (
        for: $for:ident + $for_b:literal,
        from: @$from:ident + $from_b:literal,
        arg: $arg:ident, body: $body:block
     ) => {
        $crate::all::impl_from!(for: $for+$for_b, from: $from+$from_b, arg:$arg, body: $body);
        $crate::all::impl_from!(for: $for+$for_b, from: &$from+$from_b, arg:$arg, body: $body);
    };
    // implements `From` from: owned.
    (
        for: $for:ident + $for_b:literal,
        from: $from:ident + $from_b:literal,
        arg: $arg:ident, body: $body:block
     ) => { devela::paste! {
        impl From<[<$from$from_b>]> for [<$for $for_b>] {
            #[inline]
            fn from($arg: [<$from$from_b>]) -> Self { $body }
        }
    }};
    // implements `From` from: `&` and `&mut`.
    (
        for: $for:ident + $for_b:literal,
        from: &$from:ident + $from_b:literal,
        arg: $arg:ident, body: $body:block
    ) => { devela::paste! {
        impl From<&[<$from$from_b>]> for [<$for $for_b>] {
            #[inline]
            fn from($arg: &[<$from$from_b>]) -> Self { $body }
        }
        impl From<&mut [<$from$from_b>]> for [<$for $for_b>] {
            #[inline]
            fn from($arg: &mut [<$from$from_b>]) -> Self { $body }
        }
    }};

    // implements `From` from: owned, `&` and `&mut`, when `for` is not sized.
    //
    // Use this when all implementations share the same body.
    (
        for: $for:ident,
        from: @$from:ident + $from_b:literal,
        arg: $arg:ident, body: $body:block
     ) => {
        $crate::all::impl_from!(for: $for, from: $from+$from_b, arg:$arg, body: $body);
        $crate::all::impl_from!(for: $for, from: &$from+$from_b, arg:$arg, body: $body);
    };
    // implements `From` from: owned, when `for` is not sized.
    (
        for: $for:ident,
        from: $from:ident + $from_b:literal,
        arg: $arg:ident, body: $body:block
     ) => { devela::paste! {
        impl From<[<$from$from_b>]> for $for {
            #[inline]
            fn from($arg: [<$from$from_b>]) -> Self { $body }
        }
    }};
    // implements `From` from: `&` and `&mut`, when `for` is not sized.
    (
        for: $for:ident,
        from: &$from:ident + $from_b:literal,
        arg: $arg:ident, body: $body:block
    ) => { devela::paste! {
        impl From<&[<$from$from_b>]> for $for {
            #[inline]
            fn from($arg: &[<$from$from_b>]) -> Self { $body }
        }
        impl From<&mut [<$from$from_b>]> for $for {
            #[inline]
            fn from($arg: &mut [<$from$from_b>]) -> Self { $body }
        }
    }};

    /* TryFrom */

    // implements `TryFrom` from: owned, `&` and `&mut`.
    //
    // Use this when all implementations share the same body.
    (try
        for: $for:ident + $for_b:literal,
        from: @$from:ident + $from_b:literal,
        arg: $arg:ident, body: $body:block
     ) => {
        $crate::all::impl_from!(try for: $for+$for_b, from: $from+$from_b, arg:$arg, body: $body);
        $crate::all::impl_from!(try for: $for+$for_b, from: &$from+$from_b, arg:$arg, body: $body);
    };
    // implements `TryFrom` from: owned.
    (try
        for: $for:ident + $for_b:literal,
        from: $from:ident + $from_b:literal,
        arg: $arg:ident, body: $body:block
     ) => { devela::paste! {
        #[cfg_attr(feature = "nightly", doc(cfg(feature = "try_from")))]
        impl TryFrom<[<$from$from_b>]> for [<$for$for_b>] {
            type Error = $crate::error::NumeraError;
            #[inline]
            fn try_from($arg: [<$from$from_b>]) -> $crate::error::NumeraResult<[<$for$for_b>]> {
                $body
            }
        }
    }};
    // implements `TryFrom` from: `&` and `&mut`.
    (try
        for: $for:ident + $for_b:literal,
        from: &$from:ident + $from_b:literal,
        arg: $arg:ident, body: $body:block
    ) => { devela::paste! {
        #[cfg_attr(feature = "nightly", doc(cfg(feature = "try_from")))]
        impl TryFrom<&[<$from$from_b>]> for [<$for$for_b>] {
            type Error = $crate::error::NumeraError;
            #[inline]
            fn try_from($arg: &[<$from$from_b>]) -> $crate::error::NumeraResult<[<$for$for_b>]> {
                $body
            }
        }
        #[cfg_attr(feature = "nightly", doc(cfg(feature = "try_from")))]
        impl TryFrom<&mut [<$from$from_b>]> for [<$for$for_b>] {
            type Error = $crate::error::NumeraError;
            #[inline]
            fn try_from($arg: &mut [<$from$from_b>]) -> $crate::error::NumeraResult<[<$for$for_b>]> {
                $body
            }
        }
    }};

    // implements `TryFrom` from: owned, `&` and `&mut`, when `for` is not sized.
    //
    // Use this when all implementations share the same body.
    (try
        for: $for:ident,
        from: @$from:ident + $from_b:literal,
        arg: $arg:ident, body: $body:block
     ) => {
        $crate::all::impl_from!(try for: $for, from: $from+$from_b, arg:$arg, body: $body);
        $crate::all::impl_from!(try for: $for, from: &$from+$from_b, arg:$arg, body: $body);
    };
    // implements `TryFrom` from: owned.
    (try
        for: $for:ident,
        from: $from:ident + $from_b:literal,
        arg: $arg:ident, body: $body:block
     ) => { devela::paste! {
        #[cfg_attr(feature = "nightly", doc(cfg(feature = "try_from")))]
        impl TryFrom<[<$from$from_b>]> for $for {
            type Error = $crate::error::NumeraError;
            #[inline]
            fn try_from($arg: [<$from$from_b>]) -> $crate::error::NumeraResult<$for> {
                $body
            }
        }
    }};
    // implements `TryFrom` from: `&` and `&mut`, when `for` is not sized.
    (try
        for: $for:ident,
        from: &$from:ident + $from_b:literal,
        arg: $arg:ident, body: $body:block
    ) => { devela::paste! {
        #[cfg_attr(feature = "nightly", doc(cfg(feature = "try_from")))]
        impl TryFrom<&[<$from$from_b>]> for $for {
            type Error = $crate::error::NumeraError;
            #[inline]
            fn try_from($arg: &[<$from$from_b>]) -> $crate::error::NumeraResult<$for> {
                $body
            }
        }
        #[cfg_attr(feature = "nightly", doc(cfg(feature = "try_from")))]
        impl TryFrom<&mut [<$from$from_b>]> for $for {
            type Error = $crate::error::NumeraError;
            #[inline]
            fn try_from($arg: &mut [<$from$from_b>]) -> $crate::error::NumeraResult<$for> {
                $body
            }
        }
    }};

}
pub(crate) use impl_from;

/// Implements upcasting and downcasting methods.
macro_rules! impl_larger_smaller {
    (
     $name:ident, $b:expr, $family:ident,
      larger: $larger:literal, $larger_b:literal,
      smaller: $smaller:literal, $smaller_b:literal
    ) => {
        devela::paste! {

            /// # Methods for resizing
            impl [<$name $b>]  {
                /// Returns the current number as the next larger bit-size.
                ///
                /// This method is not implemented for the largest available bit-size.
                #[devela::compile($larger)]
                pub fn as_larger(&self) -> [<$name $larger_b>] {
                    [<$name $larger_b>]::from(self)
                }

                /// Returns the current number as the next larger bit-size,
                /// or the same if there's no larger one available.
                #[devela::compile($larger)]
                pub fn as_larger_or_same(&self) -> [<$name $larger_b>] {
                    [<$name $larger_b>]::from(self)
                }
                /// Returns the current number as the next larger bit-size,
                /// or the same if there's no larger one available.
                #[must_use]
                #[devela::compile(not($larger))]
                pub fn as_larger_or_same(&self) -> [<$name $b>] {
                    *self
                }

                /// Tries to return the current number as the next larger bit-size.
                /// # Errors
                /// If there's no larger bit-size available.
                #[devela::compile($larger)]
                pub fn try_as_larger(&self) -> NumeraResult<[<$name $larger_b>]> {
                    Ok([<$name $larger_b>]::from(self))
                }
                /// Tries to return the current number as the next larger bit-size.
                /// # Errors
                /// If there's no larger bit-size available.
                #[devela::compile(not($larger))]
                pub fn try_as_larger(&self) -> NumeraResult<[<$name $b>]> {
                    Err($crate::error::NumeraError::Conversion)
                }

                /* as_smaller */

                /// Returns the current number as the next smaller bit-size,
                /// or the same if the value can't fit in the smaller bit-size,
                /// or if there's no smaller bit-size available.
                #[devela::compile($smaller)]
                #[cfg(feature = "try_from")]
                #[cfg_attr(feature = "nightly", doc(cfg(feature = "try_from")))]
                pub fn as_smaller_or_same(&self) -> $family {
                    if let Ok(smaller) = [<$name $smaller_b>]::try_from(self) {
                        $family::[<_ $smaller_b>](smaller)
                    } else {
                        $family::[<_ $b>](*self)
                    }
                }
                /// Returns the current number as the next smaller bit-size,
                /// or the same if the value can't fit in the smaller bit-size,
                /// or if there's no smaller bit-size available.
                #[devela::compile(not($smaller))]
                #[cfg(feature = "try_from")]
                #[cfg_attr(feature = "nightly", doc(cfg(feature = "try_from")))]
                pub fn as_smaller_or_same(&self) -> $family {
                    $family::[<_ $b>](*self)
                }

                /// Tries to return the current number as the next smaller bit-size.
                /// # Errors
                /// If the value can't fit in the smaller bit-size,
                /// or if there's no smaller bit-size available.
                #[devela::compile($smaller)]
                #[cfg(feature = "try_from")]
                #[cfg_attr(feature = "nightly", doc(cfg(feature = "try_from")))]
                pub fn try_as_smaller(&self) -> NumeraResult<[<$name $smaller_b>]> {
                    [<$name $smaller_b>]::try_from(self)
                }
                /// Tries to return the current number as the next smaller bit-size.
                /// # Errors
                /// If the value can't fit in the smaller bit-size,
                /// or if there's no smaller bit-size available.
                #[devela::compile(not($smaller))]
                #[cfg(feature = "try_from")]
                #[cfg_attr(feature = "nightly", doc(cfg(feature = "try_from")))]
                pub fn try_as_smaller(&self) -> NumeraResult<[<$name $b>]> {
                    Err($crate::error::NumeraError::Conversion)
                }
            }
        }
    };
}
pub(crate) use impl_larger_smaller;
