// numera::number::integer::nnz::convert:tests

use crate::all::*;
use core::num::NonZeroU8;

#[test]
fn nnz_from() -> NumeraResult<()> {
    /* complementary primitive conversions */

    // from smaller or equal sized u
    assert_eq![Nnz8::new(200), 200_u8.into()];
    assert_eq![Nnz16::new(200), 200_u8.into()];

    // from smaller or equal sized NonZeroU
    assert_eq![Nnz8::new(200), NonZeroU8::new(200).unwrap().into()];
    assert_eq![Nnz16::new(200), NonZeroU8::new(200).unwrap().into()];

    /* complementary Integer conversions */

    // from smaller NonNegativeInteger (Self)
    assert_eq![Nnz16::new(200), Nnz8::new(200).into()];

    // from smaller or equal sized PositiveInteger
    assert_eq![Nnz16::new(200), Pz8::new(200)?.into()];
    assert_eq![Nnz8::new(200), Pz8::new(200)?.into()];

    // from smaller or equal sized PRime
    assert_eq![Nnz16::new(251), Prime8::new(251)?.into()];
    assert_eq![Nnz8::new(251), Prime8::new(251)?.into()];

    Ok(())
}

#[test]
#[cfg(feature = "try_from")]
fn nnz_try_from() -> NumeraResult<()> {
    use core::num::{NonZeroI16, NonZeroI8, NonZeroU16};

    /* complementary primitive conversions */

    // try_from bigger u
    assert_eq![Nnz8::new(200), 200_u16.try_into()?];
    assert![TryInto::<Nnz8>::try_into(500u16).is_err()];

    // try_from bigger NonZeroU
    assert_eq![Nnz8::new(200), NonZeroU16::new(200).unwrap().try_into()?];
    assert![TryInto::<Nnz8>::try_into(NonZeroU16::new(500).unwrap()).is_err()];

    /* remaining fallible primitive conversions */

    // try_from i (only the non-negative values)
    assert_eq![Nnz8::new(0), 0_i8.try_into()?];
    assert_eq![Nnz8::new(100), 100_i8.try_into()?];
    assert_eq![Nnz8::new(200), 200_i16.try_into()?];
    assert_eq![Nnz16::new(100), 100_i8.try_into()?];
    assert![TryInto::<Nnz8>::try_into(-100_i8).is_err()];
    assert![TryInto::<Nnz8>::try_into(500_i16).is_err()];

    // try_from NonZeroI (only the non-negative values)
    assert_eq![Nnz8::new(100), NonZeroI8::new(100).unwrap().try_into()?];
    assert_eq![Nnz8::new(200), NonZeroI16::new(200).unwrap().try_into()?];
    assert_eq![Nnz16::new(100), NonZeroI8::new(100).unwrap().try_into()?];
    assert![TryInto::<Nnz8>::try_into(NonZeroI8::new(-100).unwrap()).is_err()];
    assert![TryInto::<Nnz8>::try_into(NonZeroI16::new(500).unwrap()).is_err()];

    /* complementary Integer conversions */

    // try_from bigger NonNegativeInteger (Self)
    assert_eq![Nnz8::new(200), Nnz16::new(200).try_into()?];
    assert_eq![Nnz8::new(200), Nnz8::new(200).try_into()?];
    assert![TryInto::<Nnz8>::try_into(Nnz16::new(500)).is_err()];

    // try_from bigger PositiveInteger
    assert_eq![Nnz8::new(200), Pz16::new(200)?.try_into()?];
    assert![TryInto::<Nnz8>::try_into(Pz16::new(500)?).is_err()];

    // try_from bigger Prime
    assert_eq![Nnz8::new(251), Pz16::new(251)?.try_into()?];
    assert![TryInto::<Nnz8>::try_into(Pz16::new(521)?).is_err()];

    /* remaining fallible integer conversions */

    // try_from Integer (only the non-negative values)
    assert_eq![Nnz8::new(0), Z8::new(0).try_into()?];
    assert_eq![Nnz8::new(100), Z8::new(100).try_into()?];
    assert_eq![Nnz8::new(200), Z16::new(200).try_into()?];
    assert_eq![Nnz16::new(100), Z8::new(100).try_into()?];
    assert![TryInto::<Nnz8>::try_into(Z8::new(-100)).is_err()];
    assert![TryInto::<Nnz8>::try_into(Z16::new(500)).is_err()];

    // try_from NonZeroInteger (only the positive values)
    assert_eq![Nnz8::new(100), N0z8::new(100)?.try_into()?];
    assert_eq![Nnz8::new(200), N0z16::new(200)?.try_into()?];
    assert_eq![Nnz16::new(100), N0z8::new(100)?.try_into()?];
    assert![TryInto::<Nnz8>::try_into(N0z8::new(-100)?).is_err()];
    assert![TryInto::<Nnz8>::try_into(N0z16::new(500)?).is_err()];

    // try_from NonPositiveInteger (only the 0)
    assert_eq![Nnz8::new(0), Npz8::new_neg(0).try_into()?];
    assert![TryInto::<Nnz8>::try_into(Npz8::new_neg(100)).is_err()];

    /* impossible Integer conversions */

    // try_from NegativeInteger (no valid values)
    assert![TryInto::<Nnz8>::try_into(Nz8::new_neg(100)?).is_err()];
    Ok(())
}

#[test]
fn nnz_for() -> NumeraResult<()> {
    // for bigger or equal sized u (Self inner representation)
    assert_eq![0_u8, Nnz8::new(0).into()];
    assert_eq![200_u8, Nnz8::new(200).into()];
    assert_eq![200_u16, Nnz8::new(200).into()];

    Ok(())
}

#[test]
#[cfg(feature = "try_from")]
fn nnz_try_for() -> NumeraResult<()> {
    use core::num::{NonZeroI8, NonZeroU8};

    // try_for smaller u (Self inner representation)
    assert_eq![0_u8, Nnz8::new(0).try_into()?];
    assert_eq![200_u8, Nnz8::new(200).try_into()?];
    assert![TryInto::<u8>::try_into(Nnz16::new(300)).is_err()];

    // try_for i
    assert_eq![0_i8, Nnz8::new(0).try_into()?];
    assert_eq![100_i8, Nnz8::new(100).try_into()?];
    assert![TryInto::<i8>::try_into(Nnz8::new(200)).is_err()];

    // try_for NonZeroI
    assert_eq![NonZeroI8::new(100).unwrap(), Nnz16::new(100).try_into()?];
    assert![TryInto::<NonZeroI8>::try_into(Nnz8::new(0)).is_err()];
    assert![TryInto::<NonZeroI8>::try_into(Nnz16::new(200)).is_err()];

    // try_for NonZeroU
    assert_eq![NonZeroU8::new(200).unwrap(), Nnz16::new(200).try_into()?];
    assert![TryInto::<NonZeroU8>::try_into(Nnz8::new(0)).is_err()];
    assert![TryInto::<NonZeroU8>::try_into(Nnz16::new(300)).is_err()];

    Ok(())
}
