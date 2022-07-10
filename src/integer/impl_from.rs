// numera::integer::impl_from
//
//! implements From between integer types.
//

use crate::integer::{
    a::*, Integer, NegativeInteger, NonNegativeInteger, NonPositiveInteger, NonZeroInteger,
    PositiveInteger,
};
use crate::traits::{sign::*, Number};
use core::{convert::TryFrom, num::IntErrorKind};

// # From more strict types for less strict ones
// -----------------------------------------------------------------------------

// From any integer `<N>` for `Integer<N>`,
// as long as the inner value is of the same type.

impl<N: Number + Signed> From<NegativeInteger<N>> for Integer<N> {
    #[inline]
    fn from(v: Nz<N>) -> Z<N> {
        Self(v.0)
    }
}
impl<N: Number + Signed> From<NonNegativeInteger<N>> for Integer<N> {
    #[inline]
    fn from(v: Nnz<N>) -> Z<N> {
        Self(v.0)
    }
}
impl<N: Number + Signed> From<NonPositiveInteger<N>> for Integer<N> {
    #[inline]
    fn from(v: Npz<N>) -> Z<N> {
        Self(v.0)
    }
}
impl<N: Number + Signed> From<PositiveInteger<N>> for Integer<N> {
    #[inline]
    fn from(v: Pz<N>) -> Z<N> {
        Self(v.0)
    }
}
impl<N: Number + Signed> From<NonZeroInteger<N>> for Integer<N> {
    #[inline]
    fn from(v: N0z<N>) -> Z<N> {
        Self(v.0)
    }
}

// From NonZero

// From `< 0` for `!= 0`
impl<N: Number + Signed> From<NegativeInteger<N>> for NonZeroInteger<N> {
    #[inline]
    fn from(v: Nz<N>) -> N0z<N> {
        Self(v.0)
    }
}

/// From `> 0` for `!= 0`
impl<N: Number + Signed> From<PositiveInteger<N>> for NonZeroInteger<N> {
    #[inline]
    fn from(v: Pz<N>) -> N0z<N> {
        Self(v.0)
    }
}

/// From `< 0` for `<= 0`
impl<N: Number> From<NegativeInteger<N>> for NonPositiveInteger<N> {
    #[inline]
    fn from(v: Nz<N>) -> Npz<N> {
        Self(v.0)
    }
}

/// From `> 0` for `>= 0`
impl<N: Number> From<PositiveInteger<N>> for NonNegativeInteger<N> {
    #[inline]
    fn from(v: Pz<N>) -> Nnz<N> {
        Self(v.0)
    }
}

// # TryFrom Zero integers to NonZero integers
// -----------------------------------------------------------------------------

/// TryFrom `<= 0` for `!= 0`
impl<N: Number + Signed> TryFrom<NonPositiveInteger<N>> for NonZeroInteger<N> {
    type Error = IntErrorKind;

    /// Errors if the value is `0`.
    #[inline]
    fn try_from(v: Npz<N>) -> Result<N0z<N>, Self::Error> {
        if v.is_zero() {
            Err(IntErrorKind::Zero)
        } else {
            Ok(Self(v.0))
        }
    }
}
// WAIT:specialization to be able to do things like:
// impl<NIN, NOUT> TryFrom<NonPositiveInteger<NIN>> for NonZeroInteger<NOUT>
// where
//     NIN: Number + Unsigned, //
//     NOUT: Number + Signed,
// {
//     type Error = IntErrorKind;
//     #[inline]
//     fn try_from(v: Npz<NIN>) -> Result<N0z<NOUT>, Self::Error> {
//         todo![]
//     }
// }

/// TryFrom `>= 0` for `!= 0`
impl<N: Number + Signed> TryFrom<NonNegativeInteger<N>> for NonZeroInteger<N> {
    type Error = IntErrorKind;

    /// Errors if the value is `0`.
    #[inline]
    fn try_from(v: Nnz<N>) -> Result<N0z<N>, Self::Error> {
        if v.is_zero() {
            Err(IntErrorKind::Zero)
        } else {
            Ok(Self(v.0))
        }
    }
}

/// TryFrom `<= 0` for `< 0`
impl<N: Number> TryFrom<NonPositiveInteger<N>> for NegativeInteger<N> {
    type Error = IntErrorKind;

    /// Errors if the value is `0`.
    #[inline]
    fn try_from(v: Npz<N>) -> Result<Nz<N>, Self::Error> {
        if v.is_zero() {
            Err(IntErrorKind::Zero)
        } else {
            Ok(Self(v.0))
        }
    }
}

/// TryFrom `>= 0` for `> 0`
impl<N: Number> TryFrom<NonNegativeInteger<N>> for PositiveInteger<N> {
    type Error = IntErrorKind;

    /// Errors if the value is `0`.
    #[inline]
    fn try_from(v: Nnz<N>) -> Result<Pz<N>, Self::Error> {
        if v.is_zero() {
            Err(IntErrorKind::Zero)
        } else {
            Ok(Self(v.0))
        }
    }
}
