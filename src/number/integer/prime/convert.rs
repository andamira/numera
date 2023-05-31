// numera::number::integer::prime::convert
//
//!
//

use crate::{
    error::{IntegerError, NumeraError, NumeraResult},
    number::{integer::prime::all::*, traits::Numbers},
};
use devela::az::CheckedAs;

/* conversions between primes */

#[rustfmt::skip]
mod between_primes {
    use super::*;

    /* infallible */

    impl From<Prime8> for Prime16 { fn from(p: Prime8) -> Prime16 { Prime16(p.0.into()) } }
    impl From<Prime8> for Prime32 { fn from(p: Prime8) -> Prime32 { Prime32(p.0.into()) } }
    impl From<Prime8> for Prime64 { fn from(p: Prime8) -> Prime64 { Prime64(p.0.into()) } }
    impl From<Prime8> for Prime128 { fn from(p: Prime8) -> Prime128 { Prime128(p.0.into()) } }
    impl From<Prime8> for Prime { fn from(p: Prime8) -> Prime { Prime::Prime8(p) } }

    impl From<Prime16> for Prime32 { fn from(p: Prime16) -> Prime32 { Prime32(p.0.into()) } }
    impl From<Prime16> for Prime64 { fn from(p: Prime16) -> Prime64 { Prime64(p.0.into()) } }
    impl From<Prime16> for Prime128 { fn from(p: Prime16) -> Prime128 { Prime128(p.0.into()) } }
    impl From<Prime16> for Prime { fn from(p: Prime16) -> Prime { Prime::Prime16(p) } }

    impl From<Prime32> for Prime64 { fn from(p: Prime32) -> Prime64 { Prime64(p.0.into()) } }
    impl From<Prime32> for Prime128 { fn from(p: Prime32) -> Prime128 { Prime128(p.0.into()) } }
    impl From<Prime32> for Prime { fn from(p: Prime32) -> Prime { Prime::Prime32(p) } }

    impl From<Prime64> for Prime128 { fn from(p: Prime64) -> Prime128 { Prime128(p.0.into()) } }
    impl From<Prime64> for Prime { fn from(p: Prime64) -> Prime { Prime::Prime64(p) } }

    impl From<Prime128> for Prime { fn from(p: Prime128) -> Prime { Prime::Prime128(p) } }

    /* fallible */

    impl TryFrom<Prime16> for Prime8 {
        type Error = NumeraError;
        fn try_from(value: Prime16) -> NumeraResult<Prime8> {
            Ok(Prime8(u8::try_from(value.0)?))
        }
    }
    impl TryFrom<Prime32> for Prime8 {
        type Error = NumeraError;
        fn try_from(value: Prime32) -> NumeraResult<Prime8> {
            Ok(Prime8(u8::try_from(value.0)?))
        }
    }
    impl TryFrom<Prime64> for Prime8 {
        type Error = NumeraError;
        fn try_from(value: Prime64) -> NumeraResult<Prime8> {
            Ok(Prime8(u8::try_from(value.0)?))
        }
    }
    impl TryFrom<Prime128> for Prime8 {
        type Error = NumeraError;
        fn try_from(value: Prime128) -> NumeraResult<Prime8> {
            Ok(Prime8(u8::try_from(value.0)?))
        }
    }

    impl TryFrom<Prime32> for Prime16 {
        type Error = NumeraError;
        fn try_from(value: Prime32) -> NumeraResult<Prime16> {
            Ok(Prime16(u16::try_from(value.0)?))
        }
    }
    impl TryFrom<Prime64> for Prime16 {
        type Error = NumeraError;
        fn try_from(value: Prime64) -> NumeraResult<Prime16> {
            Ok(Prime16(u16::try_from(value.0)?))
        }
    }
    impl TryFrom<Prime128> for Prime16 {
        type Error = NumeraError;
        fn try_from(value: Prime128) -> NumeraResult<Prime16> {
            Ok(Prime16(u16::try_from(value.0)?))
        }
    }

    impl TryFrom<Prime64> for Prime32 {
        type Error = NumeraError;
        fn try_from(value: Prime64) -> NumeraResult<Prime32> {
            Ok(Prime32(u32::try_from(value.0)?))
        }
    }
    impl TryFrom<Prime128> for Prime32 {
        type Error = NumeraError;
        fn try_from(value: Prime128) -> NumeraResult<Prime32> {
            Ok(Prime32(u32::try_from(value.0)?))
        }
    }

    impl TryFrom<Prime128> for Prime64 {
        type Error = NumeraError;
        fn try_from(value: Prime128) -> NumeraResult<Prime64> {
            Ok(Prime64(u64::try_from(value.0)?))
        }
    }

    impl TryFrom<Prime> for Prime8 {
        type Error = NumeraError;
        fn try_from(value: Prime) -> NumeraResult<Prime8> {
            use Prime::*;
            match value {
                Prime8(p) => Ok(p),
                Prime16(p) => Ok(p.try_into()?),
                Prime32(p) => Ok(p.try_into()?),
                Prime64(p) => Ok(p.try_into()?),
                Prime128(p) => Ok(p.try_into()?),
            }
        }
    }
}

/* conversions to primitives */

// Conversions that can't fail.
macro_rules! from_prime_to_primitive {
    ($Prime:ident, $($primitive:ty),+) => {
        $( from_prime_to_primitive![@ $Prime, $primitive]; )*
    };
    (@ $Prime:ident, $primitive:ty) => {
        impl From<$Prime> for $primitive {
            fn from(p: $Prime) -> $primitive {
                p.0.into()
            }
        }
    };
}
from_prime_to_primitive![Prime8, u8, u16, u32, u64, u128, usize, i16, i32, i64, i128, isize];
from_prime_to_primitive![Prime16, u16, u32, u64, u128, usize, i32, i64, i128];
from_prime_to_primitive![Prime32, u32, u64, u128, i64, i128];
from_prime_to_primitive![Prime64, u64, u128, i128];
from_prime_to_primitive![Prime128, u128];

// Conversions that can fail.
macro_rules! try_from_prime_to_primitive {
    ($Prime:ident, $($primitive:ty),+) => {
        $( try_from_prime_to_primitive![@ $Prime, $primitive]; )*
    };
    (@ $Prime:ident, $primitive:ty) => {
        impl TryFrom<$Prime> for $primitive {
            type Error = NumeraError;
            fn try_from(p: $Prime) -> NumeraResult<$primitive> {
                Ok(<$primitive>::try_from(p.0)?)
            }
        }
    };
}
try_from_prime_to_primitive![Prime8, i8];
try_from_prime_to_primitive![Prime16, u8, i8, i16, isize];
try_from_prime_to_primitive![Prime32, u8, u16, usize, i8, i16, i32, isize];
try_from_prime_to_primitive![Prime64, u8, u16, u32, usize, i8, i16, i32, i64, isize];
try_from_prime_to_primitive![Prime128, u8, u16, u32, u64, usize, i8, i16, i32, i64, i128, isize];

/* conversions from primitives */

// tries to convert a primitive into a prime.
macro_rules! try_from_primitive_to_prime {
    ($Prime:ident, $PrimeParts:ty; $($primitive:ty),+) => {
        $( try_from_primitive_to_prime![@ $Prime, $PrimeParts; $primitive]; )*
    };
    (@ $Prime:ident, $PrimeParts:ty; $primitive:ty) => {
        impl TryFrom<$primitive> for $Prime {
            type Error = NumeraError;
            fn try_from(p: $primitive) -> NumeraResult<$Prime> {
                let arg = p.checked_as::<$PrimeParts>().ok_or(IntegerError::Overflow)?;
                $Prime::from_parts(arg)
            }
        }
    };
}
try_from_primitive_to_prime![Prime8,
    u8; u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize];
try_from_primitive_to_prime![Prime16,
    u16; u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize];
try_from_primitive_to_prime![Prime32,
    u32; u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize];
try_from_primitive_to_prime![Prime64,
    u64; u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize];
try_from_primitive_to_prime![Prime128,
    u128; u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize];
