// numera::number::integer::q::from

use crate::all::*;
use core::num::{NonZeroI8, NonZeroU8};

#[test]
fn q_from() -> NumeraResult<()> {
    /* complementary primitive conversions */

    // from smaller u
    assert_eq![Q16::new(100, 1)?, 100_u8.into()];

    // from smaller or equal sized i
    assert_eq![Q8::new(100, 1)?, 100_i8.into()];
    assert_eq![Q16::new(100, 1)?, 100_i8.into()];
    // from smaller NonZeroU
    assert_eq![Q16::new(100, 1)?, NonZeroU8::new(100).unwrap().into()];

    // from smaller or equal sized NonZeroI
    assert_eq![Q8::new(100, 1)?, NonZeroI8::new(100).unwrap().into()];
    assert_eq![Q16::new(100, 1)?, NonZeroI8::new(100).unwrap().into()];

    /* complementary Integer conversions */

    // from smaller or equal sized Integer
    assert_eq![Q16::new(100, 1)?, Z8::new(100).into()];
    assert_eq![Q8::new(100, 1)?, Z8::new(100).into()];

    // from smaller or equal sized NonZeroInteger
    assert_eq![Q16::new(100, 1)?, N0z8::new(100)?.into()];
    assert_eq![Q8::new(100, 1)?, N0z8::new(100)?.into()];

    // from smaller NonNegativeInteger
    assert_eq![Q16::new(100, 1)?, Nnz8::new(100).into()];

    // from smaller PositiveInteger
    assert_eq![Q16::new(100, 1)?, Pz8::new(100)?.into()];

    // from smaller NonPositiveInteger
    assert_eq![Q16::new(-100, 1)?, Npz8::new_neg(100).into()];

    // from smaller NegativeInteger
    assert_eq![Q16::new(-100, 1)?, Nz8::new_neg(100)?.into()];

    /* complementary Rational conversions */

    // from smaller sized Rational (Self)
    assert_eq![Q16::new(41, 107)?, Q16::new(41, 107)?.into()];

    // ...

    Ok(())
}

#[test]
#[cfg(feature = "try_from")]
fn q_try_from() -> NumeraResult<()> {
    use core::num::{NonZeroI16, NonZeroU16};
    /* complementary primitive conversions */

    // try_from bigger or equal sized u
    assert_eq![Q8::new(100, 1)?, 100_u8.try_into()?];
    assert_eq![Q8::new(100, 1)?, 100_u16.try_into()?];
    assert![TryInto::<Q8>::try_into(200_u16).is_err()];

    // try_from bigger i
    assert_eq![Q8::new(100, 1)?, 100_i16.try_into()?];
    assert![TryInto::<Q8>::try_into(200i16).is_err()];

    // try_from bigger or equal sized NonZeroU
    assert_eq![Q8::new(100, 1)?, NonZeroU8::new(100).unwrap().try_into()?];
    assert_eq![Q8::new(100, 1)?, NonZeroU16::new(100).unwrap().try_into()?];
    assert![TryInto::<Q8>::try_into(NonZeroU16::new(200).unwrap()).is_err()];

    // try_from bigger NonZeroI
    assert_eq![Q8::new(100, 1)?, NonZeroI16::new(100).unwrap().try_into()?];
    assert![TryInto::<Q8>::try_into(NonZeroI16::new(200).unwrap()).is_err()];

    /* complementary Integer conversions */

    // try_from bigger Integer
    assert_eq![Q8::new(100, 1)?, Z16::new(100).try_into()?];
    assert![TryInto::<Q8>::try_into(Z16::new(200)).is_err()];

    // try_from bigger NonZeroInteger
    assert_eq![Q8::new(100, 1)?, N0z16::new(100)?.try_into()?];
    assert![TryInto::<Q8>::try_into(N0z16::new(200)?).is_err()];

    // from bigger or equal sized NonNegativeInteger
    assert_eq![Q8::new(100, 1)?, Nnz16::new(100).try_into()?];
    assert_eq![Q8::new(100, 1)?, Nnz8::new(100).try_into()?];
    assert![TryInto::<Q8>::try_into(Nnz16::new(200)).is_err()];

    // from bigger or equal sized PositiveInteger
    assert_eq![Q8::new(100, 1)?, Pz16::new(100)?.try_into()?];
    assert_eq![Q8::new(100, 1)?, Pz8::new(100)?.try_into()?];
    assert![TryInto::<Q8>::try_into(Pz16::new(200)?).is_err()];

    // from bigger or equal sized NonPositiveInteger
    assert_eq![Q8::new(-100, 1)?, Npz16::new_neg(100).try_into()?];
    assert_eq![Q8::new(-100, 1)?, Npz8::new_neg(100).try_into()?];
    assert![TryInto::<Q8>::try_into(Npz16::new_neg(200)).is_err()];

    // from bigger or equal sized NegativeInteger
    assert_eq![Q8::new(-100, 1)?, Nz16::new_neg(100)?.try_into()?];
    assert_eq![Q8::new(-100, 1)?, Nz8::new_neg(100)?.try_into()?];
    assert![TryInto::<Q8>::try_into(Nz16::new_neg(200)?).is_err()];

    /* complementary Rational conversions */

    // try_from bigger Rational (Self)
    assert_eq![Q8::new(41, 107)?, Q16::new(41, 107)?.try_into()?];

    // ...

    Ok(())
}
