// numera::number::integer::pz::convert::tests

use crate::all::*;

#[test]
fn pz_from() -> NumeraResult<()> {
    use core::num::NonZeroU8;

    /* complementary primitive conversions */

    // from smaller or equal sized NonZeroU
    assert_eq![Pz8::new(100)?, NonZeroU8::new(100).unwrap().into()];
    assert_eq![Pz16::new(100)?, NonZeroU8::new(100).unwrap().into()];

    /* complementary Integer conversions */

    // from smaller PositiveInteger (Self)
    assert_eq![Pz16::new(200)?, Pz8::new(200)?.into()];

    // from smaller or equal sized Prime
    assert_eq![Pz8::new(251)?, Prime8::new(251)?.into()];
    assert_eq![Pz16::new(251)?, Prime8::new(251)?.into()];

    Ok(())
}

#[test]
#[cfg(feature = "try_from")]
fn pz_try_from() -> NumeraResult<()> {
    use core::num::{NonZeroI16, NonZeroI8, NonZeroU16};

    /* complementary primitive conversions */

    // try_from bigger NonZeroU
    assert_eq![Pz8::new(100)?, NonZeroU16::new(100).unwrap().try_into()?];
    assert![TryInto::<Pz8>::try_into(NonZeroU16::new(500).unwrap()).is_err()];

    /* remaining fallible primitive conversions */

    // try_from i (only the positive values)
    assert_eq![Pz8::new(100)?, 100_i8.try_into()?];
    assert_eq![Pz8::new(200)?, 200_i16.try_into()?];
    assert_eq![Pz16::new(100)?, 100_i8.try_into()?];
    assert![TryInto::<Pz8>::try_into(0_i8).is_err()];
    assert![TryInto::<Pz8>::try_into(-100_i8).is_err()];
    assert![TryInto::<Pz8>::try_into(500_i16).is_err()];

    // try_from u (only the positive values)
    assert_eq![Pz8::new(200)?, 200_u8.try_into()?];
    assert_eq![Pz8::new(200)?, 200_u16.try_into()?];
    assert_eq![Pz16::new(200)?, 200_u8.try_into()?];
    assert![TryInto::<Pz8>::try_into(0_u8).is_err()];
    assert![TryInto::<Pz8>::try_into(500_u16).is_err()];

    // try_from NonZeroI (only the positive values)
    assert_eq![Pz8::new(100)?, NonZeroI8::new(100).unwrap().try_into()?];
    assert_eq![Pz8::new(200)?, NonZeroI16::new(200).unwrap().try_into()?];
    assert_eq![Pz16::new(100)?, NonZeroI8::new(100).unwrap().try_into()?];
    assert![TryInto::<Pz8>::try_into(NonZeroI8::new(-100).unwrap()).is_err()];
    assert![TryInto::<Pz8>::try_into(NonZeroI16::new(500).unwrap()).is_err()];

    /* complementary Integer conversions */

    // try_from bigger PositiveInteger (Self)
    assert_eq![Pz8::new(200)?, Pz16::new(200)?.try_into()?];
    assert![TryInto::<Pz8>::try_into(Pz16::new(500)?).is_err()];

    // try_from bigger Prime
    assert_eq![Pz8::new(251)?, Pz16::new(251)?.try_into()?];
    assert![TryInto::<Pz8>::try_into(Pz16::new(521)?).is_err()];

    /* remaining fallible integer conversions */

    // try_from Integer (only the positive values)
    assert_eq![Pz8::new(100)?, Z8::new(100).try_into()?];
    assert_eq![Pz8::new(100)?, Z16::new(100).try_into()?];
    assert_eq![Pz16::new(100)?, Z8::new(100).try_into()?];
    assert![TryInto::<Pz8>::try_into(Z8::new(0)).is_err()];
    assert![TryInto::<Pz8>::try_into(Z8::new(-100)).is_err()];
    assert![TryInto::<Pz8>::try_into(Z16::new(500)).is_err()];

    // try_from NonZeroInteger (only the positive values)
    assert_eq![Pz8::new(100)?, N0z8::new(100)?.try_into()?];
    assert_eq![Pz8::new(100)?, N0z16::new(100)?.try_into()?];
    assert_eq![Pz16::new(100)?, N0z8::new(100)?.try_into()?];
    assert![TryInto::<Pz8>::try_into(N0z8::new(-100)?).is_err()];
    assert![TryInto::<Pz8>::try_into(N0z16::new(500)?).is_err()];

    /* impossible conversions */

    // try_from NegativeInteger (no valid values)
    assert![TryInto::<Pz8>::try_into(Nz8::new_neg(100)?).is_err()];

    // try_from NonPositiveInteger (no valid values)
    assert![TryInto::<Pz8>::try_into(Npz8::new_neg(100)).is_err()];

    Ok(())
}

#[test]
fn pz_for() -> NumeraResult<()> {
    use core::num::{NonZeroU16, NonZeroU8};

    // for bigger or equal sized NonZeroU (Self inner representation)
    assert_eq![NonZeroU8::new(200).unwrap(), Pz8::new(200)?.into()];
    assert_eq![NonZeroU16::new(200).unwrap(), Pz8::new(200)?.into()];

    Ok(())
}

#[test]
#[cfg(feature = "try_from")]
fn pz_try_for() -> NumeraResult<()> {
    use core::num::{NonZeroI8, NonZeroU8};

    // try_for smaller NonZeroU (Self inner representation)
    assert_eq![NonZeroU8::new(200).unwrap(), Pz16::new(200)?.try_into()?];
    assert![TryInto::<NonZeroU8>::try_into(Pz16::new(300)?).is_err()];

    // try_for u
    assert_eq![200_u8, Pz8::new(200)?.try_into()?];
    assert![TryInto::<u8>::try_into(Pz16::new(300)?).is_err()];

    // try_for NonZeroI
    assert_eq![NonZeroI8::new(100).unwrap(), Pz16::new(100)?.try_into()?];
    assert![TryInto::<NonZeroI8>::try_into(Pz16::new(200)?).is_err()];

    // try_for i
    assert_eq![100_i8, Pz8::new(100)?.try_into()?];
    assert![TryInto::<i8>::try_into(Pz8::new(200)?).is_err()];

    Ok(())
}
