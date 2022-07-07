// numera::number::traits
//
//!
//

#[cfg(test)]
mod tests;

mod identities;
mod maxmin;
mod number;
mod sign;

pub use identities::{One, Zero};
pub use maxmin::MaxMin;
pub use number::{InnerNumber, Number};
pub use sign::Sign;
