// numera::number::integer::rational_trait
//
//!
//
// TOC
// - define the `Rational` trait

use crate::number::traits::Number;

/// Common trait for all rationals.
pub trait Rational: Number {
    /// Is fraction an integer?
    ///
    /// Retuns `true` if the denominator 1 or the numerator is a multiple of the
    /// denominator.
    fn is_integer(&self) -> bool;

    /// Is this a *proper fraction* ([m][0m]) such that $a/b<1$?
    ///
    /// Retuns `true` if the numerator is *smaller* than the denominator.
    ///
    /// [0m]: https://mathworld.wolfram.com/ProperFraction.html
    fn is_proper(&self) -> bool;

    /// Is this a *reduced fraction* ([m][0m]).
    ///
    /// Returns `true` if the numerator and denominator have no common factors
    /// other than 1.
    ///
    /// [0m]: https://mathworld.wolfram.com/ReducedFraction.html
    fn is_reduced(&self) -> bool;

    /// Reduces the fraction in place to its simplest form by dividing both the
    /// numerator and denominator by their greatest common divisor (GCD).
    fn reduce(&mut self);

    /// Returns the reduced fraction to its simplest form by dividing both the
    /// numerator and denominator by their greatest common divisor (GCD).
    #[must_use]
    fn reduced(&self) -> Self;

    /// Inverts the fraction in place by swapping the numerator and denominator,
    /// except in the case of 0, where it does nothing.
    fn invert(&mut self);

    /// Returns the inverted fraction by swapping the numerator and denominator,
    /// except in the case of 0, where it returns itself unchanged.
    #[must_use]
    fn inverted(&self) -> Self;

    /* auto */

    /// Is this an *improper fraction* ([m][0m]) such that $a/b>1$?
    ///
    /// Retuns `true` if the numerator is *greater* than the denominator.
    ///
    /// E.g.: $ \dfrac{48}{23} , \dfrac{3}{2} ... $
    ///
    /// [0m]: https://mathworld.wolfram.com/ImproperFraction.html
    #[inline]
    fn is_improper(&self) -> bool {
        !self.is_proper()
    }

    /// Is this fraction not in its *reduced* mode ([m][0m])?
    ///
    /// Returns `true` if the numerator and denominator share a common factor
    /// greater than 1, indicating that the fraction can be reduced further.
    ///
    /// [0m]: https://mathworld.wolfram.com/ReducedFraction.html
    #[inline]
    fn is_reducible(&self) -> bool {
        !self.is_reduced()
    }

    /// Inverts and reduces the fraction.
    #[inline]
    fn invert_reduce(&mut self) {
        self.invert();
        self.reduce();
    }

    /// Returns the inverted and reduced fraction.
    #[inline]
    #[must_use]
    fn inverted_reduced(&self) -> Self
    where
        Self: Sized,
    {
        self.inverted().reduced()
    }
}
