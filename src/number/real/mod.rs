// numera::real
//
//! Real numbers, subsets of $\R$.
//

mod fns;

pub use all::*;
pub(crate) mod all {
    #[doc(inline)]
    // pub use super::{family::AnyReals, f::*, x::*, trait_real::Real};
    pub use super::fns::*;
}
