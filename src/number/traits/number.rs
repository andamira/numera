// numera::number::traits::number
//
//!
//

use super::{Identities, MaxMin, Sign};

/// The inner representation for the [`Number`] trait.
pub trait InnerNumber: PartialOrd + Identities + MaxMin + Sign {}

impl<T> InnerNumber for T where T: PartialOrd + Identities + MaxMin + Sign {}

/// The main number API trait.
pub trait Number {
    /// The inner numeric value that underlies this number.
    type Inner: InnerNumber;

    /// Returns a new number of the current type.
    ///
    /// This method must ensure the inner value is in a correct format.
    fn new(value: Self::Inner) -> Self;

    /// Returns the largest number that can be represented with the current type.
    fn new_max() -> Self;

    /// Returns the smallest number that can be represented with the current type.
    fn new_min() -> Self;

    //

    /// Returns true if the number is negative.
    ///
    /// `0` is not considered positive or negative and always returns false.
    fn is_negative(&self) -> bool;

    /// Returns true if the number is positive.
    ///
    /// `0` is not considered positive or negative and always returns false.
    fn is_positive(&self) -> bool;

    /// Returns true if the number is the additive identity `0`.
    fn is_zero(&self) -> bool;

    /// Returns true if the number is the multiplicative identity `1`.
    fn is_one(&self) -> bool;

    //

    /// Returns true if the number can be made negative.
    fn can_negative() -> bool;

    /// Returns true if the number can be made positive.
    fn can_positive() -> bool;

    /// Returns true if the number can be made the additive identity `0`.
    fn can_zero() -> bool;

    /// Returns true if the number can be made the multiplicative identity `1`.
    fn can_one() -> bool;
}
