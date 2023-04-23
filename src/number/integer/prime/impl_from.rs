// numera::number::integer::prime::impl_from
//
//!
//

use super::{NumeraError, NumeraResult, Prime16, Prime32, Prime8, Primes};

/* conversions between primes */

impl From<Prime8> for Prime16 {
    fn from(p: Prime8) -> Prime16 {
        Prime16(p.0.into())
    }
}
impl From<Prime8> for Prime32 {
    fn from(p: Prime8) -> Prime32 {
        Prime32(p.0.into())
    }
}
impl From<Prime16> for Prime32 {
    fn from(p: Prime16) -> Prime32 {
        Prime32(p.0.into())
    }
}

impl From<Prime8> for Primes {
    fn from(p: Prime8) -> Primes {
        Primes::Prime8(p)
    }
}
impl From<Prime16> for Primes {
    fn from(p: Prime16) -> Primes {
        Primes::Prime16(p)
    }
}
impl From<Prime32> for Primes {
    fn from(p: Prime32) -> Primes {
        Primes::Prime32(p)
    }
}

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
impl TryFrom<Prime32> for Prime16 {
    type Error = NumeraError;
    fn try_from(value: Prime32) -> NumeraResult<Prime16> {
        Ok(Prime16(u16::try_from(value.0)?))
    }
}

impl TryFrom<Primes> for Prime8 {
    type Error = NumeraError;
    fn try_from(value: Primes) -> NumeraResult<Prime8> {
        use Primes::*;
        match value {
            Prime8(p) => Ok(p),
            Prime16(p) => Ok(p.try_into()?),
            Prime32(p) => Ok(p.try_into()?),
        }
    }
}

/* conversions to primitives */

impl From<Prime8> for u8 {
    fn from(p: Prime8) -> u8 {
        p.0
    }
}
impl From<Prime8> for u16 {
    fn from(p: Prime8) -> u16 {
        p.0.into()
    }
}
impl From<Prime8> for u32 {
    fn from(p: Prime8) -> u32 {
        p.0.into()
    }
}
impl From<Prime8> for u64 {
    fn from(p: Prime8) -> u64 {
        p.0.into()
    }
}
impl From<Prime8> for u128 {
    fn from(p: Prime8) -> u128 {
        p.0.into()
    }
}
impl From<Prime8> for usize {
    fn from(p: Prime8) -> usize {
        p.0.into()
    }
}

impl From<Prime16> for u16 {
    fn from(p: Prime16) -> u16 {
        p.0
    }
}
impl From<Prime16> for u32 {
    fn from(p: Prime16) -> u32 {
        p.0.into()
    }
}
impl From<Prime16> for u64 {
    fn from(p: Prime16) -> u64 {
        p.0.into()
    }
}
impl From<Prime16> for u128 {
    fn from(p: Prime16) -> u128 {
        p.0.into()
    }
}
impl From<Prime16> for usize {
    fn from(p: Prime16) -> usize {
        p.0.into()
    }
}

impl From<Prime32> for u32 {
    fn from(p: Prime32) -> u32 {
        p.0
    }
}
impl From<Prime32> for u64 {
    fn from(p: Prime32) -> u64 {
        p.0.into()
    }
}
impl From<Prime32> for u128 {
    fn from(p: Prime32) -> u128 {
        p.0.into()
    }
}
