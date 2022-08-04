// numera::error
//
//! Error types.
//!
//

use core::result;

/// The *numera* result type.
pub type Result<N> = result::Result<N, Error>;

/// The general number error type.
#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    /// An error involving integer numbers.
    Integer(IntegerError),
    /// An error involving rational numbers.
    Rational(RationalError),
    /// An error involving real numbers.
    Real(RealError),
}

// /// Errors related to [`integer`][crate::integer]s.
#[non_exhaustive]
#[derive(Debug)]
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
    // /// An error related to integer primitives.
    // Int(core::num::IntErrorKind),
}

// /// Errors related to [`rational`][crate::rational]s.
#[non_exhaustive]
#[derive(Debug)]
pub enum RationalError {
    /// Invalid value `0`.
    ZeroDenominator,
    // /// Invalid value `>= 0`.
    // ZeroOrMore,
    //
    // /// Invalid value `<= 0`.
    // ZeroOrLess,
    //
    // /// Invalid value `< 0`.
    // LessThanZero,
    //
    // /// Invalid value `> 0`.
    // MoreThanZero,
    // /// An error related to integer primitives.
    // Int(core::num::IntErrorKind),
}

/// Errors related to [`real`][crate::real]s.
#[non_exhaustive]
#[derive(Debug)]
pub enum RealError {
    Other,
}

/// allows converting into `Error` from other error types.
mod core_impls {
    use super::{Error, IntegerError, RationalError, RealError};

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
}

/// impl Display & Error on all types.
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
                // Other(s) => write!(f, "Error::Other: {s}"),
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
                // Int(i) => i.fmt(f),
                // Other(s) => write!(f, "IntegerError::Other: {s}"),
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
            // use RealError::*;
            // match self {
            //     // => write!(f, "RealError::"),
            //     Other(s) => write!(f, "RealError::Other: {s}"),
            // }
        }
    }
}
