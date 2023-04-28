// numera::number::integer::z
//
//! Integer numbers ([w][w0]/[m][m0]), from the set $\Z$.
//!
//! $ \Z = \lbrace …, -2, -1, 0, 1, 2, … \rbrace $
//!
//! This type corresponds with the signed primitives ([`i8`]…[`i128`]).
//!
//! [w0]: https://en.wikipedia.org/wiki/Integer
//! [m0]: https://mathworld.wolfram.com/Integer.html
//!
//! # Homogeneous relations
//! - [*Symmetric*][s] + [*Transitive*][t] (== [`Partial Equivalence`][pe]),
//!   + [*Reflexive*][r] (== [`Equivalence`][e]).
//! - [*Transitive*][t] + [*Antisymmetric*][as] ([`Partial Order`][po])
//!   + [*Reflexive*][r] + [*Strongly Connected*][sc] (== [`Total Order`][to]).
//!
//! [pe]: https://en.wikipedia.org/wiki/Partial_equivalence_relation
//! [e]: https://en.wikipedia.org/wiki/Equivalence_relation
//! [s]: https://en.wikipedia.org/wiki/Symmetric_relation
//! [t]: https://en.wikipedia.org/wiki/Transitive_relation
//! [r]: https://en.wikipedia.org/wiki/Reflexive_relation
//! [as]: https://en.wikipedia.org/wiki/Antisymmetric_relation
//! [sc]: https://en.wikipedia.org/wiki/Connected_relation
//! [po]: https://en.wikipedia.org/wiki/Partially_ordered_set#Partial_order
//! [to]: https://en.wikipedia.org/wiki/Total_order
//

mod define_sized;
mod family;
mod impl_from;
mod impl_integer;
mod impl_ops;

#[cfg(feature = "ibig")]
mod define_big;

pub use define_sized::{Integer128, Integer16, Integer32, Integer64, Integer8};
pub use family::Integers;

#[cfg(feature = "ibig")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "ibig")))]
pub use define_big::IntegerBig;

/// Abbreviations for integers.
pub mod abbr {
    use super::*;
    use crate::number::macros::define_abbreviations;

    define_abbreviations![many Z, Integer, 8, 16, 32, 64, 128];

    /// Abbreviation of [`IntegerBig`].
    #[cfg(feature = "ibig")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "ibig")))]
    pub type ZBig = IntegerBig;
}
