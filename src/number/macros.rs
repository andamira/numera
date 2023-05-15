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
/// - `$bsize`: the size in bits.
macro_rules! define_abbreviations {
    (many $a:ident, $name:ident, $( $bsize:expr ),+) => {
        $( define_abbreviations![$a, $name, $bsize]; )+
    };

    ($a:ident, $name:ident, $bsize:expr) => {
        devela::paste!{
            #[doc = "Abbreviation of [`" $name $bsize "`]." ]
            pub type [<$a$bsize>] = [<$name $bsize>];
        }
    };
}
pub(crate) use define_abbreviations;

/// Creates type aliases.
///
/// # Args
/// - `$a`: the alias of the type.
/// - `$name`: the name of the type.
/// - `$bsize`: the size in bits.
macro_rules! define_aliases {
    (many $a:ident, $name:ident, $( $bsize:expr ),+) => {
        $( define_aliases![$a, $name, $bsize]; )+
    };

    ($a:ident, $name:ident, $bsize:expr) => {
        devela::paste!{
            #[doc = "Alias of [`" $name $bsize "`]." ]
            pub type [<$a$bsize>] = [<$name $bsize>];
        }
    };
}
pub(crate) use define_aliases;

/// Implements upcasting and downcasting methods.
macro_rules! impl_larger_smaller {
    (
     $name:ident, $bsize:expr, $family:ident,
      larger: $larger:literal, $larger_bsize:literal,
      smaller: $smaller:literal, $smaller_bsize:literal
    ) => {
        devela::paste! {

            /// # Methods for resizing
            impl [<$name $bsize>]  {
                /// Returns the current number as the next larger bit-size.
                ///
                /// This method is not implemented for the largest available bit-size.
                #[devela::compile($larger)]
                pub fn as_larger(&self) -> [<$name $larger_bsize>] {
                    [<$name $larger_bsize>]::from(self)
                }

                /// Returns the current number as the next larger bit-size,
                /// or the same if there's no larger one available.
                #[devela::compile($larger)]
                pub fn as_larger_or_same(&self) -> [<$name $larger_bsize>] {
                    [<$name $larger_bsize>]::from(self)
                }
                /// Returns the current number as the next larger bit-size,
                /// or the same if there's no larger one available.
                #[must_use]
                #[devela::compile(not($larger))]
                pub fn as_larger_or_same(&self) -> [<$name $bsize>] {
                    *self
                }

                /// Tries to return the current number as the next larger bit-size.
                /// # Errors
                /// If there's no larger bit-size available.
                #[devela::compile($larger)]
                pub fn try_as_larger(&self) -> NumeraResult<[<$name $larger_bsize>]> {
                    Ok([<$name $larger_bsize>]::from(self))
                }
                /// Tries to return the current number as the next larger bit-size.
                /// # Errors
                /// If there's no larger bit-size available.
                #[devela::compile(not($larger))]
                pub fn try_as_larger(&self) -> NumeraResult<[<$name $bsize>]> {
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
                    if let Ok(smaller) = [<$name $smaller_bsize>]::try_from(self) {
                        $family::[<$name $smaller_bsize>](smaller)
                    } else {
                        $family::[<$name $bsize>](*self)
                    }
                }
                /// Returns the current number as the next smaller bit-size,
                /// or the same if the value can't fit in the smaller bit-size,
                /// or if there's no smaller bit-size available.
                #[devela::compile(not($smaller))]
                #[cfg(feature = "try_from")]
                #[cfg_attr(feature = "nightly", doc(cfg(feature = "try_from")))]
                pub fn as_smaller_or_same(&self) -> $family {
                    $family::[<$name $bsize>](*self)
                }

                /// Tries to return the current number as the next smaller bit-size.
                /// # Errors
                /// If the value can't fit in the smaller bit-size,
                /// or if there's no smaller bit-size available.
                #[devela::compile($smaller)]
                #[cfg(feature = "try_from")]
                #[cfg_attr(feature = "nightly", doc(cfg(feature = "try_from")))]
                pub fn try_as_smaller(&self) -> NumeraResult<[<$name $smaller_bsize>]> {
                    [<$name $smaller_bsize>]::try_from(self)
                }
                /// Tries to return the current number as the next smaller bit-size.
                /// # Errors
                /// If the value can't fit in the smaller bit-size,
                /// or if there's no smaller bit-size available.
                #[devela::compile(not($smaller))]
                #[cfg(feature = "try_from")]
                #[cfg_attr(feature = "nightly", doc(cfg(feature = "try_from")))]
                pub fn try_as_smaller(&self) -> NumeraResult<[<$name $bsize>]> {
                    Err($crate::error::NumeraError::Conversion)
                }
            }
        }
    };
}
pub(crate) use impl_larger_smaller;
