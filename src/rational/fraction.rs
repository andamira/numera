// numera::rational::fraction
//
//!
//

/// A fraction ([m][0m])
///
/// [0m]: https://mathworld.wolfram.com/Fraction.html
pub trait Fraction {
    /// Is this a *proper fraction* ([m][0m]) such that $a/b<1$?
    ///
    /// Retuns true if the numerator is *smaller* than the denominator.
    ///
    /// [0m]: https://mathworld.wolfram.com/ProperFraction.html
    fn is_proper(&self) -> bool;

    /// Is this an *improper fraction* ([m][0m]) such that $a/b>1$?
    ///
    /// Retuns true if the numerator is *greater* than the denominator.
    ///
    /// E.g.: $ \dfrac{48}{23} , \dfrac{3}{2} ... $
    ///
    /// [0m]: https://mathworld.wolfram.com/ImproperFraction.html
    fn is_improper(&self) -> bool;
}
