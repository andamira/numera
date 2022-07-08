// numera::number::integer
//
//! An *integer* ([w][0w]/[m][0m]) is a number that can be written
//! without a fractional component.
//!
//! For example, 21, 4, 0, and −2048 are integers, while 9.75, 5+1/2, and √2 are not.
//!
//! # Integer subsets
//!
//! *Natural numbers* ([w][1w]/[m][1m]), *Counting numbers* ([m][2m]) and *Whole numbers*
//! ([m][3m]) are tradicitonal ambiguous ways to refer to different subsets of
//! integers, without consensus on whether *zero* ([m][4m]) is included in
//! any of those sets.
//!
//! This is why the integer types defined here are named using a more explicit,
//! unambiguous notation.
//!
//! [0w]: https://en.wikipedia.org/wiki/Integer
//! [0m]: https://mathworld.wolfram.com/Integer.html
//! [1w]: https://en.wikipedia.org/wiki/Natural_number
//! [1m]: https://mathworld.wolfram.com/NaturalNumber.html
//! [2m]: https://mathworld.wolfram.com/NaturalNumber.html
//! [3m]: https://mathworld.wolfram.com/NaturalNumber.html
//! [4m]: https://mathworld.wolfram.com/Zero.html
//

mod impl_bounded;
mod impl_number;
mod impl_std;

use crate::number::traits::InnerNumber;

/// An integer number ([w][w0]/[m][m0]), from the set $\Z$.
///
/// $ \Z = \lbrace …, -2, -1, 0, 1, 2, … \rbrace $
///
/// This type perfectly encapsulates the signed primitives ([`i8`] … [`i128`]).
///
/// [w0]: https://en.wikipedia.org/wiki/Integer
/// [m0]: https://mathworld.wolfram.com/Integer.html
#[repr(transparent)]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Integer<I: InnerNumber>(I);

/// An only *non-negative* integer number ([m][0m]/[o][0o]), from the set $\Z^*$.
///
/// $ \Z^* = \lbrace 0, 1, 2, … \rbrace $
///
/// Sometimes called *Natural number* ([w][1w]) or *counting number*,
/// but in that case it can be confounded with [`Positive`].
///
/// This type exactly corresponds to the unsigned primitives (u8…u128).
///
/// [0m]: https://mathworld.wolfram.com/NonnegativeInteger.html
/// [0o]: http://oeis.org/A001477
/// [1w]: https://en.wikipedia.org/wiki/Natural_number
#[repr(transparent)]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct NonNegative<I: InnerNumber>(I);

/// An only *positive* integer number ([m][0m]), from the set $\Z^+$.
///
/// $ \Z^+ = \lbrace 1, 2, … \rbrace $
///
/// Doesn't include 0.
///
/// Sometimes called *Natural number* ([w][1w]) or *counting number*,
/// but in that case it can be confounded with [`NonNegative`].
///
/// [0m]: https://mathworld.wolfram.com/PositiveInteger.html
/// [1w]: https://en.wikipedia.org/wiki/Natural_number
#[repr(transparent)]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Positive<I: InnerNumber>(I);

/// An only *negative* integer number ([m][0m]/[o][0o]), from the set $\Z^-$.
///
/// $ \Z^- = \lbrace …, -2, -1 \rbrace $
///
/// Doesn't include 0.
///
/// [0m]: https://mathworld.wolfram.com/NegativeInteger.html
/// [0o]: http://oeis.org/A001478
#[repr(transparent)]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Negative<I: InnerNumber>(I);

/// An only *non-positive* integer number ([m][0m]), from the set ${0} \cup \Z^-$.
///
/// $ {0} \cup Z^- = \lbrace …, -2, -1, 0 \rbrace $
///
/// [0m]: https://mathworld.wolfram.com/NonpositiveInteger.html
#[repr(transparent)]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct NonPositive<I: InnerNumber>(I);
