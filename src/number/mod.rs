//! Numbers
//!
//! A *number* ([w][1w]/[m][1m]) is a mathematical object used to count, measure,
//! and label. A general term which refers to a member of a given set.
//!
//! [1w]: https://en.wikipedia.org/wiki/Number
//! [1m]: https://mathworld.wolfram.com/Number.html
//!
//! # Number types
//!
//! - [`Natural`]
//! - [`Integer`]
//! - [`Rational`]
//! - [`Real`]
// - [`Imaginary`]
// - [`Complex`]

#![allow(non_camel_case_types)]

use num_traits::{Zero, One};

mod macros;

pub mod naming;

pub mod integer;
// pub mod irrational;
pub mod natural;
pub mod rational;
pub mod real;

// #[doc(inline)]
pub use {
    // complex::Complex,
    // imaginary::{Imaginary},
    integer::Integer,
    // irrational::Irrational,
    natural::Natural,
    rational::{Fraction, FractionType, Rational},
    real::Real,
};

/// A common API for all 
pub trait NumberApi: Zero + One {
    type Value;

    /// Returns a new number.
    fn new(value: Self::Value) -> Self;

    /// Returns a new number, where a `value` of `0` is converted to `1`.
    fn new_0to1(value: Self::Value) -> Self;

    /// Returns a new number, but only if `value` $\ne 0$.
    fn new_nonzero(value: Self::Value) -> Self;
}

///
#[non_exhaustive]
pub enum NumberType {
    /// Positive numbers, without a decimal or fractional part.
    ///
    /// $\naturals = \lbrace 0, 1, 2, 3, 4… \rbrace $
    ///
    /// - <https://mathworld.wolfram.com/Z-Plus.html>
    /// - <https://mathworld.wolfram.com/N.html>
    /// - <https://en.wikipedia.org/wiki/Natural_number>
    /// - <http://www.positiveintegers.org>
    Natural,

    /// Positive and negative [`Natural`] numbers.
    ///
    /// $\z = \lbrace …-4, -3, -2, -1, 0, 1, 2, 3, 4… \rbrace$
    ///
    /// - <https://en.wikipedia.org/wiki/Integer>
    Integer,

    /// Rational numbers ($Q$) can be expressed as a fraction $\frac{a}{b}$
    /// where $a$ and $b$ are [`Integer`][Self::Integer]s and $b \ne 0$.
    ///
    Rational,

    /// Irrational numbers
    ///
    /// [Wikipedia Irrational number](https://en.wikipedia.org/wiki/Irrational_number)
    Irrational,

    ///
    Real,

    ///
    Imaginary,

    ///
    Complex,

    // ///
    // Quaternion,
    //
    // ///
    // Octonion,
}
