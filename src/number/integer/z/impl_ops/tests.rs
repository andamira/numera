// numera::number::integer::z::impl_ops::tests

use crate::all::*;

#[test]
fn z_ops_add() -> NumeraResult<()> {
    // op
    assert_eq![Z8::new(120) + Z8::new(7), Z8::new(127)];

    // checked
    assert_eq![Z8::new(120).checked_add(Z8::new(7)), Some(Z8::new(127))];
    assert_eq![Z8::new(120).checked_add(Z8::new(8)), None];

    // saturating
    assert_eq![Z8::new(120).saturating_add(Z8::new(8)), Z8::new(127)];

    // wrapping
    assert_eq![Z8::new(120).wrapping_add(Z8::new(8)), Z8::MIN];

    // modular
    // assert_eq![Z8::new(120).modular_add(Z8::new(8), Z8::MAX), Z8::MIN];
    // assert_eq![Z8::new(5).modular_add(Z8::new(3), Z8::new(7)), Z8::new(1)];
    // assert_eq![Z8::new(-5).modular_add(Z8::new(3), Z8::new(7)), Z8::new(-2)];
    // assert_eq![Z8::new(127).modular_add(Z8::new(2), Z8::new(50)), Z8::new(-21)]; // BAD

    // CHECK negative numbers
    // assert_eq![Z8::new(-5).modular_add(Z8::new(-3), Z8::new(-7)), Z8::new(-1)];

    // modulo_count CHECK

    // overflowing

    // PANICS
    #[cfg(feature = "std")]
    {
        use std::panic::catch_unwind;
        // basic overflow
        assert![catch_unwind(|| Z8::new(120) + Z8::new(8)).is_err()];
    }

    Ok(())
}

#[test]
fn z_ops_rem() -> NumeraResult<()> {
    // rem_trunc
    assert_eq![Z16::new(-347) % Z16::new(6), Z16::new(-5)];
    // rem_euclid
    assert_eq![Z16::new(-347).rem_euclid(Z16::new(6)), Z16::new(1)];

    Ok(())
}
