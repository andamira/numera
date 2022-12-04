// numera::error
//
//! Error types.
//

use core::result;

/// The *numera* common result type.
pub type Result<N> = result::Result<N, Error>;

/// The *numera* common error type.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    /// An error involving integer numbers.
    Integer(IntegerError),
    /// An error involving rational numbers.
    Rational(RationalError),
    /// An error involving real numbers.
    Real(RealError),

    /// Couldn't convert between two kinds of numbers.
    Conversion,

    /// A miscellaneous error message.
    Other(&'static str),
}

/// Errors related to [`integer`][crate::number::integer]s.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntegerError {
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
}

/// Errors related to [`rational`][crate::number::rational]s.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RationalError {
    /// Invalid value `0`.
    ZeroDenominator,
}

/// Errors related to [`real`][crate::number::real]s.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RealError {
    Other,
}

/// allows converting into `Error` from other error types.
mod core_impls {
    use super::{Error, IntegerError, RationalError, RealError};
    use core::num::IntErrorKind;

    impl From<IntegerError> for Error {
        fn from(err: IntegerError) -> Self {
            Error::Integer(err)
        }
    }
    impl From<RationalError> for Error {
        fn from(err: RationalError) -> Self {
            Error::Rational(err)
        }
    }
    impl From<RealError> for Error {
        fn from(err: RealError) -> Self {
            Error::Real(err)
        }
    }

    impl From<IntErrorKind> for Error {
        fn from(err: IntErrorKind) -> Self {
            use Error::Integer;
            use IntErrorKind::*;
            match err {
                PosOverflow => Integer(IntegerError::Overflow),
                NegOverflow => Integer(IntegerError::Underflow),
                Zero => Integer(IntegerError::Zero),
                //
                Empty => Error::Other("IntErrorKind::Empty"),
                InvalidDigit => Error::Other("IntErrorKind::InvalidDigit"),
                _ => Error::Other("IntErrorKind::_"),
            }
        }
    }
}

/// impl `Display`, `Error` & `PartialEq`.
#[cfg(feature = "std")]
mod std_impls {
    use super::{Error, IntegerError, RationalError, RealError};
    use std::{
        error::Error as StdError,
        fmt::{self, Debug},
    };

    impl StdError for Error {}
    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            use Error::*;
            match self {
                Integer(z) => Debug::fmt(z, f),
                Rational(q) => Debug::fmt(q, f),
                Real(r) => Debug::fmt(r, f),
                Conversion => write!(f, "Couldn't convert the number."),
                Other(s) => write!(f, "{s}"),
            }
        }
    }

    impl StdError for IntegerError {}
    impl fmt::Display for IntegerError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            use IntegerError::*;
            match self {
                Zero => write!(f, "IntegerError::Zero"),
                ZeroOrMore => write!(f, "IntegerError::ZeroOrMore"),
                ZeroOrLess => write!(f, "IntegerError::ZeroOrLess"),
                LessThanZero => write!(f, "IntegerError::LessThanZero"),
                MoreThanZero => write!(f, "IntegerError::MoreThanZero"),
                Overflow => write!(f, "IntegerError::Overflow"),
                Underflow => write!(f, "IntegerError::Underflow"),
            }
        }
    }

    impl StdError for RationalError {}
    impl fmt::Display for RationalError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            use RationalError::*;
            match self {
                ZeroDenominator => write!(f, "RationalError::ZeroDenominator"),
            }
        }
    }

    impl StdError for RealError {}
    impl fmt::Display for RealError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "RealError")
        }
    }

    impl PartialEq<IntegerError> for Error {
        fn eq(&self, other: &IntegerError) -> bool {
            match self {
                Error::Integer(err) => err == other,
                _ => false,
            }
        }
    }

    impl PartialEq<RationalError> for Error {
        fn eq(&self, other: &RationalError) -> bool {
            match self {
                Error::Rational(err) => err == other,
                _ => false,
            }
        }
    }

    impl PartialEq<RealError> for Error {
        fn eq(&self, other: &RealError) -> bool {
            match self {
                Error::Real(err) => err == other,
                _ => false,
            }
        }
    }
}
