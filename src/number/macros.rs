// numera::number::macros
//
//!
//
// TOC
//
// - define-abbreviations!
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
                        $family::[<$name $smaller_b>](smaller)
                    } else {
                        $family::[<$name $b>](*self)
                    }
                }
                /// Returns the current number as the next smaller bit-size,
                /// or the same if the value can't fit in the smaller bit-size,
                /// or if there's no smaller bit-size available.
                #[devela::compile(not($smaller))]
                #[cfg(feature = "try_from")]
                #[cfg_attr(feature = "nightly", doc(cfg(feature = "try_from")))]
                pub fn as_smaller_or_same(&self) -> $family {
                    $family::[<$name $b>](*self)
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
