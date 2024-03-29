// numera::number::integer::n0z::ops::tests

use crate::all::*;

#[test]
fn n0z_ops() -> NumeraResult<()> {
    let _5 = N0z8::new(5)?;
    let _7 = N0z8::new(7)?;

    assert_eq![_7 + _5, N0z8::new(12)?];
    assert_eq![_7 - _5, N0z8::new(2)?];
    assert_eq![_5 - _7, N0z8::new(-2)?];
    assert_eq![_7 * _5, N0z8::new(35)?];
    assert_eq![_7 / _5, N0z8::new(1)?];
    assert_eq![-_7, N0z8::new(-7)?];

    #[cfg(feature = "std")]
    {
        use std::panic::catch_unwind;
        // overflow
        assert![catch_unwind(|| _7 * _7 * _7).is_err()];
        // underflow
        assert![catch_unwind(|| N0z8::MIN - _5).is_err()];
        // zero
        assert![catch_unwind(|| _5 / _7).is_err()];
    }
    Ok(())
}
