// numera::real
//
//! Real numbers
//!
//

use crate::traits::{Number, Signed};

/// A real number, from the set $\R$.
///
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Real<N: Number + Signed>(N);
