// numera::number::integer::npz::convert::tests

use crate::all::*;

#[test]
fn npz_from() -> NumeraResult<()> {
    /* complementary Integer conversions */

    // from smaller NonPositiveInteger (Self)
    assert_eq![Npz16::new_neg(200), Npz8::new_neg(200).into()];

    // from smaller or equal sized NegativeInteger
    assert_eq![Npz16::new_neg(200), Nz8::new_neg(200)?.into()];
    assert_eq![Npz8::new_neg(200), Nz8::new_neg(200)?.into()];

    Ok(())
}

#[test]
#[cfg(feature = "try_from")]
fn npz_try_from() -> NumeraResult<()> {
    use core::num::{NonZeroI16, NonZeroI8, NonZeroU8};

    /* fallible primitive conversions */

    // try_from i (only the non-positive values)
    assert_eq![Npz8::new_neg(0), (0_i8).try_into()?];
    assert_eq![Npz8::new_neg(100), (-100_i8).try_into()?];
    assert_eq![Npz8::new_neg(200), (-200_i16).try_into()?];
    assert_eq![Npz16::new_neg(100), (-100_i8).try_into()?];
    assert![TryInto::<Npz8>::try_into(-500_i16).is_err()];

    // try_from u (only the 0)
    assert_eq![Npz8::new_neg(0), 0_u8.try_into()?];
    assert![TryInto::<Npz8>::try_into(100_u8).is_err()];

    // try_from NonZeroI (only the negative values)
    assert_eq![
        Npz8::new_neg(100),
        NonZeroI8::new(-100).unwrap().try_into()?
    ];
    assert_eq![
        Npz8::new_neg(200),
        NonZeroI16::new(-200).unwrap().try_into()?
    ];
    assert_eq![
        Npz16::new_neg(100),
        NonZeroI8::new(-100).unwrap().try_into()?
    ];
    assert![TryInto::<Npz8>::try_into(NonZeroI8::new(100).unwrap()).is_err()];
    assert![TryInto::<Npz8>::try_into(NonZeroI16::new(-500).unwrap()).is_err()];

    /* impossible primitive conversions */

    // try_from NonZeroU (no valid values)
    assert![TryInto::<Npz8>::try_into(NonZeroU8::new(100).unwrap()).is_err()];

    /* complementary Integer conversions */

    // try_from bigger NonPositiveInteger (Self)
    assert_eq![Npz8::new_neg(200), Npz16::new_neg(200).try_into()?];
    assert_eq![Npz8::new_neg(200), Npz8::new_neg(200).try_into()?];
    assert![TryInto::<Npz8>::try_into(Npz16::new_neg(500)).is_err()];

    // try_from bigger NegativeInteger
    assert_eq![Npz8::new_neg(200), Nz16::new_neg(200)?.try_into()?];
    assert![TryInto::<Npz8>::try_into(Nz16::new_neg(500)?).is_err()];

    /* fallible Integer conversions */

    // try_from Integer (only the non-positive values)
    assert_eq![Npz8::new_neg(0), Z8::new(0).try_into()?];
    assert_eq![Npz8::new_neg(100), Z8::new(-100).try_into()?];
    assert_eq![Npz8::new_neg(200), Z16::new(-200).try_into()?];
    assert_eq![Npz16::new_neg(100), Z8::new(-100).try_into()?];
    assert![TryInto::<Npz8>::try_into(Z8::new(100)).is_err()];
    assert![TryInto::<Npz8>::try_into(Z16::new(-500)).is_err()];

    // try_from NonZeroInteger (only the negative values)
    assert_eq![Npz8::new_neg(100), N0z8::new(-100)?.try_into()?];
    assert_eq![Npz8::new_neg(200), N0z16::new(-200)?.try_into()?];
    assert_eq![Npz16::new_neg(100), N0z8::new(-100)?.try_into()?];
    assert![TryInto::<Npz8>::try_into(N0z8::new(100)?).is_err()];
    assert![TryInto::<Npz8>::try_into(N0z16::new(-500)?).is_err()];

    // try_from NonNegativeInteger (only the 0)
    assert_eq![Npz8::new_neg(0), Nnz8::new(0).try_into()?];
    assert![TryInto::<Npz8>::try_into(Nnz8::new(100)).is_err()];

    /* impossible Integer conversions */

    // try_from PositiveInteger (no valid values)
    assert![TryInto::<Npz8>::try_into(Pz8::new(100)?).is_err()];

    // try_from Prime (no valid values)
    assert![TryInto::<Npz8>::try_into(Prime8::new(101)?).is_err()];

    Ok(())
}

#[test]
#[cfg(feature = "try_from")]
fn npz_try_for() -> NumeraResult<()> {
    use core::num::{NonZeroI8, NonZeroU8};

    // try_for i
    assert_eq![0_i8, Npz16::new_neg(0).try_into()?];
    assert_eq![-100_i8, Npz16::new_neg(100).try_into()?];
    assert![TryInto::<i8>::try_into(Npz8::new_neg(200)).is_err()];

    // try_for NonZeroI
    assert_eq![
        NonZeroI8::new(-100).unwrap(),
        Npz8::new_neg(100).try_into()?
    ];
    assert![TryInto::<NonZeroI8>::try_into(Npz8::new_neg(0)).is_err()];
    assert![TryInto::<NonZeroI8>::try_into(Npz8::new_neg(200)).is_err()];

    // try_for u
    assert![TryInto::<u8>::try_into(Npz8::new_neg(1)).is_err()];

    // try_for NonZeroU
    assert![TryInto::<NonZeroU8>::try_into(Npz8::new_neg(1)).is_err()];

    Ok(())
}
