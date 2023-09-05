// numera::error
//
//! Error types.
//

/// The *numera* common result type.
pub type NumeraResult<N> = core::result::Result<N, NumeraErrors>;

/// The *numera* common error type.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NumeraErrors {
    /// An error involving integer numbers.
    Integer(IntegerErrors),

    /// An error involving rational numbers.
    Rational(RationalErrors),

    /// An error involving real numbers.
    Real(RealErrors),

    /// Couldn't convert between two kinds of numbers.
    Conversion,

    /// Not implemented.
    NotImplemented,

    /// A miscellaneous error message.
    Other(&'static str),
}

/// Errors related to [`integer`][crate::number::integer]s.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntegerErrors {
    /// Invalid value `0`.
    Zero,

    /// Invalid value `>= 0`.
    ZeroOrMore,

    /// Invalid value `<= 0`.
    ZeroOrLess,

    /// Invalid value `< 0`.
    LessThanZero,

    /// Invalid value `> 0`.
    MoreThanZero,

    /// The value is too large to store in the current representation.
    Overflow,

    /// The value is too small to store in the current representation.
    Underflow,

    /// The integer is not a prime.
    NotPrime,
}

/// Errors related to `rational`s.
// Errors related to [`rational`][crate::number::rational]s.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RationalErrors {
    /// Invalid value `0` in the denominator.
    ZeroDenominator,

    /// The value is too large to store in the current representation of the
    /// numerator.
    NumeratorOverflow,

    /// The value is too small to store in the current representation of the
    /// numerator
    NumeratorUnderflow,

    /// The value is too large to store in the current representation of the
    /// denominator.
    DenominatorOverflow,

    /// The value is too small to store in the current representation of the
    /// denominator
    DenominatorUnderflow,
}

/// Errors related to `real`s.
// Errors related to [`real`][crate::number::real]s.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RealErrors {
    /// Invalid value Nan.
    NaN,

    Other, // TEMP
}

mod core_impls {
    use super::{IntegerErrors, NumeraErrors, RationalErrors, RealErrors};
    use core::{
        convert::Infallible,
        fmt::{self, Debug},
        num::{IntErrorKind, TryFromIntError},
    };

    impl fmt::Display for NumeraErrors {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                NumeraErrors::Integer(z) => Debug::fmt(z, f),
                NumeraErrors::Rational(q) => Debug::fmt(q, f),
                NumeraErrors::Real(r) => Debug::fmt(r, f),
                NumeraErrors::Conversion => write!(f, "Couldn't convert the number."),
                NumeraErrors::NotImplemented => write!(f, "Not implemented."),
                NumeraErrors::Other(s) => write!(f, "{s}"),
            }
        }
    }
    impl fmt::Display for IntegerErrors {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                IntegerErrors::Zero => write!(f, "Zero"),
                IntegerErrors::ZeroOrMore => write!(f, "ZeroOrMore"),
                IntegerErrors::ZeroOrLess => write!(f, "ZeroOrLess"),
                IntegerErrors::LessThanZero => write!(f, "LessThanZero"),
                IntegerErrors::MoreThanZero => write!(f, "MoreThanZero"),
                IntegerErrors::Overflow => write!(f, "Overflow"),
                IntegerErrors::Underflow => write!(f, "Underflow"),
                IntegerErrors::NotPrime => write!(f, "NotPrime"),
            }
        }
    }
    impl fmt::Display for RationalErrors {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                RationalErrors::ZeroDenominator => write!(f, "ZeroDenominator"),
                RationalErrors::NumeratorOverflow => write!(f, "NumeratorOverflow"),
                RationalErrors::NumeratorUnderflow => write!(f, "NumeratorUnderflow"),
                RationalErrors::DenominatorOverflow => write!(f, "DenominatorOverflow"),
                RationalErrors::DenominatorUnderflow => write!(f, "DenominatorUnderflow"),
            }
        }
    }
    impl fmt::Display for RealErrors {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                RealErrors::NaN => write!(f, "Nan"),
                _ => write!(f, "Other"),
            }
        }
    }

    impl PartialEq<IntegerErrors> for NumeraErrors {
        #[inline]
        fn eq(&self, other: &IntegerErrors) -> bool {
            match self {
                NumeraErrors::Integer(err) => err == other,
                _ => false,
            }
        }
    }
    impl PartialEq<RationalErrors> for NumeraErrors {
        #[inline]
        fn eq(&self, other: &RationalErrors) -> bool {
            match self {
                NumeraErrors::Rational(err) => err == other,
                _ => false,
            }
        }
    }
    impl PartialEq<RealErrors> for NumeraErrors {
        #[inline]
        fn eq(&self, other: &RealErrors) -> bool {
            match self {
                NumeraErrors::Real(err) => err == other,
                _ => false,
            }
        }
    }

    impl From<IntegerErrors> for NumeraErrors {
        fn from(err: IntegerErrors) -> Self {
            NumeraErrors::Integer(err)
        }
    }

    impl From<RationalErrors> for NumeraErrors {
        fn from(err: RationalErrors) -> Self {
            NumeraErrors::Rational(err)
        }
    }
    impl From<RealErrors> for NumeraErrors {
        fn from(err: RealErrors) -> Self {
            NumeraErrors::Real(err)
        }
    }

    impl From<IntErrorKind> for NumeraErrors {
        fn from(err: IntErrorKind) -> Self {
            match err {
                IntErrorKind::PosOverflow => NumeraErrors::Integer(IntegerErrors::Overflow),
                IntErrorKind::NegOverflow => NumeraErrors::Integer(IntegerErrors::Underflow),
                IntErrorKind::Zero => NumeraErrors::Integer(IntegerErrors::Zero),
                //
                IntErrorKind::Empty => NumeraErrors::Other("IntErrorKind::Empty"),
                IntErrorKind::InvalidDigit => NumeraErrors::Other("IntErrorKind::InvalidDigit"),
                _ => NumeraErrors::Other("IntErrorKind::_"),
            }
        }
    }
    impl From<TryFromIntError> for NumeraErrors {
        fn from(_err: TryFromIntError) -> Self {
            IntegerErrors::Overflow.into()
        }
    }

    impl From<Infallible> for NumeraErrors {
        fn from(_err: Infallible) -> Self {
            NumeraErrors::Conversion
        }
    }
}

#[cfg(feature = "dashu-base")]
mod dashu_base {
    use super::NumeraErrors;
    use dashu_base::error::{ConversionError, ParseError};

    impl From<ConversionError> for NumeraErrors {
        #[inline]
        fn from(_err: ConversionError) -> Self {
            NumeraErrors::Conversion
        }
    }
    // ParseError { NoDigits, InvalidDigit }
    impl From<ParseError> for NumeraErrors {
        #[inline]
        fn from(_err: ParseError) -> Self {
            NumeraErrors::Conversion
        }
    }
}

#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
mod std_impls {
    use super::{IntegerErrors, NumeraErrors, RationalErrors, RealErrors};
    use std::error::Error;

    impl Error for NumeraErrors {}
    impl Error for IntegerErrors {}
    impl Error for RationalErrors {}
    impl Error for RealErrors {}
}
