// numera::real
//
//! Real numbers, subsets of $\R$.
//

pub mod float;

pub use all::*;
pub(crate) mod all {
    #[doc(inline)]
    // pub use super::{family::AllReals, trait_real::Real};
    pub use super::float::all::*;
}
