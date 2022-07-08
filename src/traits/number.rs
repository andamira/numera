// numera::::traits::number
//
//!
//

use super::Sign;

/// The inner representation for the [`Number`] trait.
pub trait NumberAble: PartialOrd + Sign {}

impl<T> NumberAble for T where T: PartialOrd + Sign {}

/// The main number API trait.
pub trait Number {
    /// The inner numeric value that underlies this number.
    type Inner: NumberAble;

    /// Returns a new number of the current type.
    ///
    /// This method must ensure the inner value is in a correct format.
    fn new(value: Self::Inner) -> Self;

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

    /// Returns true if the number is the negative multiplicative identity `-1`.
    fn is_neg_one(&self) -> bool;

    //

    /// Returns true if the number can represent negative numbers.
    fn can_negative() -> bool;

    /// Returns true if the number can represent positive numbers.
    fn can_positive() -> bool;

    /// Returns true if the number can represent the additive identity `0`.
    fn can_zero() -> bool;

    /// Returns true if the number can represent the multiplicative identity `1`.
    fn can_one() -> bool;

    /// Returns true if the number can represent the negative multiplicative identity `-1`.
    fn can_neg_one() -> bool;
}
