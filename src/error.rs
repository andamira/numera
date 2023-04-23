// numera::error
//
//! Error types.
//

use core::result;

/// The *numera* common result type.
pub type NumeraResult<N> = result::Result<N, NumeraError>;

/// The *numera* common error type.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NumeraError {
    /// An error involving integer numbers.
    Integer(IntegerError),

    /// An error involving rational numbers.
    Rational(RationalError),

    /// An error involving real numbers.
    Real(RealError),

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

    /// The integer is not a prime.
    NotPrime,
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

mod core_impls {
    use super::{IntegerError, NumeraError, RationalError, RealError};
    use core::{
        fmt::{self, Debug},
        num::{IntErrorKind, TryFromIntError},
    };

    impl fmt::Display for NumeraError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            use NumeraError::*;
            match self {
                Integer(z) => Debug::fmt(z, f),
                Rational(q) => Debug::fmt(q, f),
                Real(r) => Debug::fmt(r, f),
                Conversion => write!(f, "Couldn't convert the number."),
                NotImplemented => write!(f, "Not implemented."),
                Other(s) => write!(f, "{s}"),
            }
        }
    }
    impl fmt::Display for IntegerError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            use IntegerError::*;
            match self {
                Zero => write!(f, "Zero"),
                ZeroOrMore => write!(f, "ZeroOrMore"),
                ZeroOrLess => write!(f, "ZeroOrLess"),
                LessThanZero => write!(f, "LessThanZero"),
                MoreThanZero => write!(f, "MoreThanZero"),
                Overflow => write!(f, "Overflow"),
                Underflow => write!(f, "Underflow"),
                NotPrime => write!(f, "NotPrime"),
            }
        }
    }
    impl fmt::Display for RationalError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            use RationalError::*;
            match self {
                ZeroDenominator => write!(f, "ZeroDenominator"),
            }
        }
    }
    impl fmt::Display for RealError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "RealError")
        }
    }

    impl PartialEq<IntegerError> for NumeraError {
        fn eq(&self, other: &IntegerError) -> bool {
            match self {
                NumeraError::Integer(err) => err == other,
                _ => false,
            }
        }
    }
    impl PartialEq<RationalError> for NumeraError {
        fn eq(&self, other: &RationalError) -> bool {
            match self {
                NumeraError::Rational(err) => err == other,
                _ => false,
            }
        }
    }
    impl PartialEq<RealError> for NumeraError {
        fn eq(&self, other: &RealError) -> bool {
            match self {
                NumeraError::Real(err) => err == other,
                _ => false,
            }
        }
    }

    impl From<IntegerError> for NumeraError {
        fn from(err: IntegerError) -> Self {
            NumeraError::Integer(err)
        }
    }

    impl From<RationalError> for NumeraError {
        fn from(err: RationalError) -> Self {
            NumeraError::Rational(err)
        }
    }
    impl From<RealError> for NumeraError {
        fn from(err: RealError) -> Self {
            NumeraError::Real(err)
        }
    }

    impl From<IntErrorKind> for NumeraError {
        fn from(err: IntErrorKind) -> Self {
            use {IntErrorKind::*, NumeraError::Integer};
            match err {
                PosOverflow => Integer(IntegerError::Overflow),
                NegOverflow => Integer(IntegerError::Underflow),
                Zero => Integer(IntegerError::Zero),
                //
                Empty => NumeraError::Other("IntErrorKind::Empty"),
                InvalidDigit => NumeraError::Other("IntErrorKind::InvalidDigit"),
                _ => NumeraError::Other("IntErrorKind::_"),
            }
        }
    }
    impl From<TryFromIntError> for NumeraError {
        fn from(_err: TryFromIntError) -> Self {
            IntegerError::Overflow.into()
        }
    }
}

#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
mod std_impls {
    use super::{IntegerError, NumeraError, RationalError, RealError};
    use std::error::Error as StdError;

    impl StdError for NumeraError {}
    impl StdError for IntegerError {}
    impl StdError for RationalError {}
    impl StdError for RealError {}
}
