// numera::integer::macros
//
//! shared macros
//
// # TOC
// - define_acronyms
// - define_integer_prim
// - impl_into_primitive

/// Creates acronym type aliases.
///
/// # Args
/// - `$a`: the acronym of the type.
/// - `$name`: the name of the type.
/// - `$bsize`: the size in bits.
macro_rules! define_acronyms {
    // generic & non-generic for all sizes.
    ($a:ident, $name:ident, $( $bsize:expr ),+) => {
        define_acronyms![single $a, $name];
        $( define_acronyms![single $a, $name, $bsize]; )+
    };

    // generic version.
    (single $a:ident, $name:ident) => {
        paste::paste!{
            #[doc = "Acronym for [`" $name "`]." ]
            pub type $a<N> = $name<N>;
        }
    };
    // non-generic version.
    (single $a:ident, $name:ident, $bsize:expr) => {
        paste::paste!{
            #[doc = "Acronym for [`" $name $bsize "`]." ]
            pub type [< $a $bsize >] = [< $name $bsize >];
        }
    };
}
pub(crate) use define_acronyms;

/// - defines an integer type of a concrete size.
/// - implements constructors for that type.
///
/// # Args
/// - `$name`: the name of the integer e.g. `NonZeroInteger`.
/// - `$p`: the primitive prefix (i or u), also used for `NonZero` suffix, uppercased.
///
/// - `$doc_num`: the type of number.
/// - `$doc_type`: adds to the type doc-comment.
/// - `$doc_new`: adds to the `new` constructor doc-comment.
///
/// - `$sign`: an optional negative sign
/// - `$lower`: the lower range of the number type.
/// - `$lower`: the upper range of the number type.
///
/// - `$det`: the determinant before the bit size. e.g. "An" (8-bit) or "A" 16-bit.
/// - `$bsize`: the size in bits of the primitive used.
//
// MAYBE improve the range documentation after Bounded traits are implemented,
// so that it can automatically link to the real values.
macro_rules! define_integer_prim {
    // defines multiple integer types, with an inner primitive.
    ($name:ident, $p:ident,
     $doc_num:literal, $doc_type:literal, $doc_new:literal,
     $sign:literal, $lower:expr, $upper:expr,
     $(($det:literal, $bsize:expr)),+) => {
        $( define_integer_prim![single $name, $p,
               $doc_num, $doc_type, $doc_new,
               $sign, $lower, $upper,
               ($det, $bsize)]; )+
    };
    // defines a single integer type, with an inner primitive.
    (single $name:ident, $p:ident,
     $doc_num:literal, $doc_type:literal, $doc_new:literal,
     $sign:literal, $lower:expr, $upper:expr,
     ($det:literal, $bsize:expr)) => {
        paste::paste! {
            #[doc = $det " " $bsize "-bit " $doc_num " " $doc_type]
            #[doc = "\n\nThe range of valid numeric values is $\\lbrack"
                $sign "$[`" $p $bsize "::" $lower "`] $\\dots$ [`" $p $bsize
                "::" $upper "`] $\\rbrack$."]
            #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
            pub struct [< $name $bsize >](pub(crate) [< $p $bsize >]);

            impl [< $name $bsize >] {
                #[doc = "Returns a new " $bsize "-bit " $doc_num "."]
                #[doc = "\n\n" $doc_new]
                pub fn new(value: [< $p $bsize >]) -> Self {
                    Self(value)
                }
            }
        }
    };

    // defines multiple integer types, with an inner NonZero* primitive.
    (nonzero $value:ident, $name:ident, $p:ident, // $assert:stmt;
     $doc_num:literal, $doc_type:literal, $doc_new:literal,
     $sign:literal, $lower:expr, $upper:expr,
     $(($det:literal, $bsize:expr)),+) => {
        $(
            define_integer_prim![single_nonzero
                $value, $name, $p, /* $assert; */
                $doc_num, $doc_type, $doc_new,
                $sign, $lower, $upper,
                ($det, $bsize)];
        )+
    };
    // defines a single integer type, with an inner NonZero* primitive.
    //
    // NOTE: $value & $assert are infrautilized, but could be useful for future constructors
    //
    // WIP range MIN MAX
    (single_nonzero $value: ident, $name:ident, $p:ident, // $assert:stmt;
     $doc_num:literal, $doc_type:literal, $doc_new:literal,
     $sign:literal, $lower:expr, $upper:expr,
     ($det:literal, $bsize:expr)) => {
        paste::paste! {
            #[doc = $det " " $bsize "-bit " $doc_num " " $doc_type]
            #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
            #[doc = "\n\nThe range of valid numeric values is $\\lbrack"
                $sign "$[`" $p $bsize "::" $lower "`] $\\dots$ [`" $p $bsize "::"
                $upper "`] $\\rbrack \\setminus 0$."]
            pub struct [< $name $bsize >](
                pub(crate) core::num::[< NonZero $p:upper $bsize >]
            );

            impl [< $name $bsize >] {
                #[doc = "Returns a new " $bsize "-bit " $doc_num "."]
                #[doc = "\n\n" $doc_new]
                /// # Panics
                /// Panics if `value == 0`.
                pub fn new($value: [< $p $bsize >]) -> Self {
                    // $assert;
                    Self(
                        core::num::[< NonZero $p:upper $bsize >]::new($value).expect("value == 0")
                        // SAFETY: previous manual assertion
                        // unsafe { core::num::[< NonZero $p:upper $bsize >]::new_unchecked($value) }
                    )
                }

                // MAYBE: add constructor that accepts types that implement some trait:
                //
                // pub fn from_az(value: impl az::UnwrappedCast<$prim>) -> Self {
                //     Self(value.unwrapped_cast())
                // }
                //
                // pub fn from_number($value: Impl Number) -> Self {
                //     $assert;
                //     unsafe { core::num::[< NonZero $p:upper $bsize >]::new_unchecked($value) }
                //     //
                //     //match overflowing_cast(value) {(v, false) => v, (_, true) => panic!("overflow")}
                // }
            }
        }
    };
}
pub(crate) use define_integer_prim;
