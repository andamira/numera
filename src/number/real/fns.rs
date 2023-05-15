// numera::number::real::fns
//
//!
//

/// Calculates the absolute value of `n`.
#[inline]
#[must_use]
pub fn abs32(n: f32) -> f32 {
    let mask: u32 = 0x7FFF_FFFF;
    let bits: u32 = n.to_bits() & mask;
    f32::from_bits(bits)
}

/// Calculates the absolute value of `n`.
#[inline]
#[must_use]
pub fn abs64(n: f64) -> f64 {
    let mask: u64 = 0x7FFF_FFFF_FFFF_FFFF;
    let bits: u64 = n.to_bits() & mask;
    f64::from_bits(bits)
}

/// Calculates the square root of `n` using the Newton-Raphson method.
///
/// # Links
/// - <https://en.wikipedia.org/wiki/Newton%27s_method>.
#[must_use]
pub fn sqrt_nr32(n: f32) -> f32 {
    // the tolerance for the difference between successive guesses
    const EPSILON: f32 = 1e-7;

    let mut x = n;
    let mut x_next = 0.5 * (x + n / x);

    while abs32(x - x_next) > EPSILON {
        x = x_next;
        x_next = 0.5 * (x + n / x);
    }

    x_next
}

/// Calculates the square root of `n` using the Newton-Raphson method.
///
/// # Links
/// - <https://en.wikipedia.org/wiki/Newton%27s_method>.
#[must_use]
pub fn sqrt_nr64(n: f64) -> f64 {
    // the tolerance for the difference between successive guesses
    const EPSILON: f64 = 1e-15;

    let mut x = n;
    let mut x_next = 0.5 * (x + n / x);

    while abs64(x - x_next) > EPSILON {
        x = x_next;
        x_next = 0.5 * (x + n / x);
    }

    x_next
}

/// Calculates the square root of `n` using the Fast Inverse Square Root
/// algorithm.
///
/// # Links
/// - <https://en.wikipedia.org/wiki/Fast_inverse_square_root>.
#[inline]
#[must_use]
#[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
pub fn sqrt_fisr32(n: f32) -> f32 {
    let mut i: i32 = n.to_bits() as i32;
    let threehalfs: f32 = 1.5;
    let x2 = n * 0.5;
    let mut y: f32;

    i = 0x5f37_59df - (i >> 1); // Lomont's single precision magic number
    y = f32::from_bits(i as u32);
    y = y * (threehalfs - (x2 * y * y));

    1.0 / y
}

/// Calculates the square root of `n` using the Fast Inverse Square Root
/// algorithm.
///
/// # Links
/// - <https://en.wikipedia.org/wiki/Fast_inverse_square_root>.
//
// Use 0x5ffe_6eb5_0c7b_537a_9cd9_f02e_504f_cfbf for quadruple precision.
#[inline]
#[must_use]
#[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
pub fn sqrt_fisr64(n: f64) -> f64 {
    let mut i: i64 = n.to_bits() as i64;
    let three_halfs: f64 = 1.5;
    let x2 = n * 0.5;
    let mut y: f64;

    // i = 0x5fe6_ec85_e7de_30da - (i >> 1); // Lomont's double precision magic number
    i = 0x5fe6_eb50_c7b5_37a9 - (i >> 1); // Matthew Robertson's double precision magic number
    y = f64::from_bits(i as u64);
    y = y * (three_halfs - (x2 * y * y));

    1.0 / y
}
