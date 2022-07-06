//!
//

use core::ops::Neg;
use std::fmt;

use num_integer::Integer as NumInt;
use num_traits::{One, Zero};

use super::Integer;

/// A `Rational` number
///
///
/// - <https://en.wikipedia.org/wiki/Rational_data_type>
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Rational<INUM, IDEN>
where
    INUM: NumInt + Neg,
    IDEN: NumInt + Neg,
{
    num: Integer<INUM>,
    den: Integer<IDEN>,
}

impl<INUM, IDEN> Rational<INUM, IDEN>
where
    INUM: NumInt + Neg,
    IDEN: NumInt + Neg,
{
    /// Returns a new `Rational`, where a `value` of [`0`][Zero] for the
    /// denominator is changed to [`1`][One].
    ///
    /// By calling the [`new_0to1`] method from the denominator's `Integer`.
    ///
    /// [`new_0to1`]: Integer#method.new_0to1
    pub fn new<IINUM: Into<Integer<INUM>>, IIDEN: Into<Integer<IDEN>>>(
        num: IINUM,
        den: IIDEN,
    ) -> Self {
        // let i = den.into();
        Self {
            num: num.into(),
            den: den.into(),
        }

        // TEMP
        // Self {
        //     num: num.into(),
        //     den: den.into(),
        // }
    }
}

impl<INUM, IDEN> fmt::Display for Rational<INUM, IDEN>
where
    INUM: NumInt + Neg + Copy + fmt::Display,
    IDEN: NumInt + Neg + Copy + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.num, self.den)
    }
}

/// The default Rational is $0/1$.
impl<INUM, IDEN> Default for Rational<INUM, IDEN>
where
    INUM: NumInt + Neg + Copy + Zero,
    IDEN: NumInt + Neg + Copy + One,
{
    fn default() -> Self {
        Self {
            num: Integer::new(INUM::zero()),
            den: Integer::new(IDEN::one()),
        }
    }
}
