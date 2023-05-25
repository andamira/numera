// numera::number::integer::n0z::from::tests

use crate::all::*;
use core::num::{NonZeroI8, NonZeroU8};

#[test]
fn n0z_from() -> NumeraResult<()> {
    /* complementary primitive conversions */

    // from smaller NonZeroU
    assert_eq![N0z16::new(100)?, NonZeroU8::new(100).unwrap().into()];

    // from smaller or equal sized NonZeroI
    assert_eq![N0z8::new(100)?, NonZeroI8::new(100).unwrap().into()];
    assert_eq![N0z16::new(100)?, NonZeroI8::new(100).unwrap().into()];

    /* complementary Integer conversions */

    // from smaller NonZeroInteger (Self)
    assert_eq![N0z16::new(100)?, N0z8::new(100)?.into()];

    // from smaller PositiveInteger
    assert_eq![N0z16::new(100)?, Pz8::new(100)?.into()];

    // from smaller NegativeInteger
    assert_eq![N0z16::new(-100)?, Nz8::new_neg(100)?.into()];

    // from smaller Prime
    assert_eq![N0z16::new(101)?, P8::new(101)?.into()];

    Ok(())
}

#[test]
#[cfg(feature = "try_from")]
fn n0z_try_from() -> NumeraResult<()> {
    use core::num::{NonZeroI16, NonZeroU16};

    /* complementary primitive conversions */

    // try_from bigger or equal sized NonZeroU
    assert_eq![N0z8::new(100)?, NonZeroU8::new(100).unwrap().try_into()?];
    assert_eq![N0z8::new(100)?, NonZeroU16::new(100).unwrap().try_into()?];
    assert![TryInto::<N0z8>::try_into(NonZeroU16::new(200).unwrap()).is_err()];

    // try_from bigger NonZeroI
    assert_eq![N0z8::new(100)?, NonZeroI16::new(100).unwrap().try_into()?];
    assert![TryInto::<N0z8>::try_into(NonZeroI16::new(200).unwrap()).is_err()];

    /* remaining fallible primitive conversions */

    // try_from u (only the non-zero values)
    assert_eq![N0z8::new(100)?, 100_u8.try_into()?];
    assert_eq![N0z8::new(100)?, 100_u16.try_into()?];
    assert_eq![N0z16::new(200)?, 200_u8.try_into()?];
    assert![TryInto::<N0z8>::try_into(0_u8).is_err()];
    assert![TryInto::<N0z8>::try_into(200_u16).is_err()];

    // try_from i (only the non-zero values)
    assert_eq![N0z8::new(100)?, 100_i8.try_into()?];
    assert_eq![N0z8::new(-100)?, (-100_i8).try_into()?];
    assert_eq![N0z8::new(100)?, 100_i16.try_into()?];
    assert_eq![N0z16::new(100)?, 100_i8.try_into()?];
    assert![TryInto::<N0z8>::try_into(0_i16).is_err()];
    assert![TryInto::<N0z8>::try_into(200_i16).is_err()];
    assert![TryInto::<N0z8>::try_into(-200_i16).is_err()];

    /* complementary Integer conversions */

    // try_from bigger NonZeroInteger
    assert_eq![N0z8::new(100)?, N0z16::new(100)?.try_into()?];
    assert_eq![N0z8::new(-100)?, N0z16::new(-100)?.try_into()?];
    assert![TryInto::<N0z8>::try_into(N0z16::new(200)?).is_err()];
    assert![TryInto::<N0z8>::try_into(N0z16::new(-200)?).is_err()];

    // from bigger or equal sized PositiveInteger
    assert_eq![N0z8::new(100)?, Pz16::new(100)?.try_into()?];
    assert_eq![N0z8::new(100)?, Pz8::new(100)?.try_into()?];
    assert![TryInto::<N0z8>::try_into(Pz16::new(200)?).is_err()];

    // from bigger or equal sized NegativeInteger
    assert_eq![N0z8::new(-100)?, Nz16::new_neg(100)?.try_into()?];
    assert_eq![N0z8::new(-100)?, Nz8::new_neg(100)?.try_into()?];
    assert![TryInto::<N0z8>::try_into(Nz16::new_neg(200)?).is_err()];

    // try_from bigger Prime
    assert_eq![N0z8::new(101)?, P16::new(101)?.try_into()?];
    assert![TryInto::<N0z8>::try_into(P16::new(251)?).is_err()];

    /* remaining fallible integer conversions */

    // try_from Integer
    assert_eq![N0z8::new(100)?, Z8::new(100).try_into()?];
    assert_eq![N0z8::new(-100)?, Z8::new(-100).try_into()?];
    assert_eq![N0z8::new(100)?, Z16::new(100).try_into()?];
    assert_eq![N0z16::new(100)?, Z8::new(100).try_into()?];
    assert![TryInto::<N0z8>::try_into(Z16::new(200)).is_err()];

    // try_from NonNegativeInteger
    assert_eq![N0z8::new(100)?, Nnz8::new(100).try_into()?];
    assert_eq![N0z8::new(100)?, Nnz16::new(100).try_into()?];
    assert_eq![N0z16::new(100)?, Nnz8::new(100).try_into()?];
    assert![TryInto::<N0z8>::try_into(Nnz16::new(200)).is_err()];

    // try_from NonPositiveInteger
    assert_eq![N0z8::new(-100)?, Npz8::new_neg(100).try_into()?];
    assert_eq![N0z8::new(-100)?, Npz16::new_neg(100).try_into()?];
    assert_eq![N0z16::new(-100)?, Npz8::new_neg(100).try_into()?];
    assert![TryInto::<N0z8>::try_into(Npz16::new_neg(200)).is_err()];

    Ok(())
}

#[test]
fn n0z_for() -> NumeraResult<()> {
    use core::num::{NonZeroI8, NonZeroI16};

    // for bigger or equal sized NonZeroI (Self inner representation)
    assert_eq![NonZeroI8::new(100).unwrap(), N0z8::new(100)?.into()];
    assert_eq![NonZeroI8::new(-100).unwrap(), N0z8::new(-100)?.into()];
    assert_eq![NonZeroI16::new(100).unwrap(), N0z8::new(100)?.into()];

    // for bigger or equal sized i
    assert_eq![100_i8, N0z8::new(100)?.into()];
    assert_eq![-100_i8, N0z8::new(-100)?.into()];
    assert_eq![100_i16, N0z8::new(100)?.into()];
    
    Ok(())
}

#[test]
#[cfg(feature = "try_from")]
fn n0z_try_for() -> NumeraResult<()> {
    use core::num::{NonZeroI8, NonZeroU8};

    // try_for smaller NonZeroI  (Self inner representation)
    assert_eq![NonZeroI8::new(100).unwrap(), N0z16::new(100)?.try_into()?];
    assert![TryInto::<NonZeroI8>::try_into(N0z16::new(200)?).is_err()];

    // try_for smaller i
    assert_eq![100_i8, N0z16::new(100)?.try_into()?];
    assert![TryInto::<i8>::try_into(N0z16::new(200)?).is_err()];

    // try_for u
    assert_eq![200_u8, N0z16::new(200)?.try_into()?];
    assert![TryInto::<u8>::try_into(N0z16::new(300)?).is_err()];

    // try_for NonZeroU
    assert_eq![NonZeroU8::new(200).unwrap(), N0z16::new(200)?.try_into()?];
    assert![TryInto::<NonZeroU8>::try_into(N0z16::new(300)?).is_err()];

    Ok(())
}
