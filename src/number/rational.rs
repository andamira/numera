// numera::number::rational
//
//!
//

use core::ops::Neg;
use min_max_traits::{Max, Min};
use std::fmt;

use num_integer::Integer as NumInt;
use num_traits::{One, Zero};

use super::{Integer, Number, Positive};

/// A `Rational` number
///
///
/// - <https://en.wikipedia.org/wiki/Rational_data_type>
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Rational<INUM, IDEN>
where
    INUM: NumInt + Max + Min + Neg,
    IDEN: NumInt + Max + Min + Neg,
{
    num: Integer<INUM>,
    den: Positive<IDEN>,
}

impl<INUM, IDEN> Rational<INUM, IDEN>
where
    INUM: NumInt + Max + Min + Neg,
    IDEN: NumInt + Max + Min + Neg,
{
    /// Returns a new `Rational`, where a `value` of [`0`][Zero] for the
    /// denominator is changed to [`1`][One].
    ///
    /// By calling the [`new_0to1`] method from the denominator's `Integer`.
    ///
    /// [`new_0to1`]: Integer#method.new_0to1
    pub fn new<IINUM: Into<Integer<INUM>>, IIDEN: Into<Positive<IDEN>>>(
        num: IINUM,
        den: IIDEN,
    ) -> Self {
        Self {
            num: num.into(),
            den: den.into(),
        }
    }
}

impl<INUM, IDEN> fmt::Display for Rational<INUM, IDEN>
where
    INUM: NumInt + Max + Min + Neg + Copy + fmt::Display,
    IDEN: NumInt + Max + Min + Neg + Copy + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.num, self.den)
    }
}

/// The default Rational is $0/1$.
impl<INUM, IDEN> Default for Rational<INUM, IDEN>
where
    INUM: NumInt + Max + Min + Neg + Copy + Zero,
    IDEN: NumInt + Max + Min + Neg + Copy + One,
{
    fn default() -> Self {
        Self {
            num: Integer::new(INUM::zero()),
            den: Positive::new(IDEN::one()),
        }
    }
}
