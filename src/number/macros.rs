// numera::number::macros
//
//!
//
// TOC
// - define-abbreviations!

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
        paste::paste!{
            #[doc = "Abbreviation of [`" $name $bsize "`]." ]
            pub type [<$a$bsize>] = [<$name$bsize>];
        }
    };
}
pub(crate) use define_abbreviations;
