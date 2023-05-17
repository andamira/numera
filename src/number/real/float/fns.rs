// numera::number::real::fns
//
//! Floating-point functions alternatives to the ones from the standard library.
//! They are not as efficient or precise, but they are no-std compatible.
//

#![allow(dead_code)]

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

/// Returns the largest integer less than or equal to `n`.
#[inline]
#[must_use]
pub fn floor32(n: f32) -> f32 {
    const EPSILON32: f32 = 1e-6;
    let mut result = trunc32(n);
    if n.is_sign_negative() && abs32(n - result) > EPSILON32 {
        result -= 1.0;
    }
    result
}

/// Returns the largest integer less than or equal to `n`.
#[inline]
#[must_use]
pub fn floor64(n: f64) -> f64 {
    const EPSILON64: f64 = 1e-12;
    let mut result = trunc64(n);
    if n.is_sign_negative() && abs64(n - result) > EPSILON64 {
        result -= 1.0;
    }
    result
}

/// Raises `n` to an integer power `p`.
#[inline]
#[must_use]
pub fn powi32(n: f32, p: i32) -> f32 {
    match p {
        0 => 1.0,
        1.. => {
            let mut result = n;
            for _i in 1..p {
                result *= n;
            }
            result
        }
        _ => {
            let mut result = n;
            for _i in 1..p.abs() {
                result /= n;
            }
            result
        }
    }
}

/// Raises `n` to an integer power `p`.
#[inline]
#[must_use]
pub fn powi64(n: f64, p: i32) -> f64 {
    match p {
        0 => 1.0,
        1.. => {
            let mut result = n;
            for _i in 1..p {
                result *= n;
            }
            result
        }
        _ => {
            let mut result = n;
            for _i in 1..p.abs() {
                result /= n;
            }
            result
        }
    }
}

/// Returns the nearest integer to `n`.
/// If a value is half-way between two integers, round away from `0.0`.
#[inline]
#[must_use]
pub fn round_half_away32(n: f32) -> f32 {
    let bits = n.to_bits();
    #[allow(clippy::cast_possible_wrap)]
    let exponent = ((bits >> 23) & 0xFF) as i32 - 127;
    if exponent < 0 {
        if n.is_sign_positive() {
            0.0
        } else {
            -0.0
        }
    } else if exponent < 23 {
        let mask = (1u32 << (23 - exponent)) - 1;
        let frac_part = bits & mask;
        let half = 1u32 << (22 - exponent);
        if frac_part >= half {
            #[allow(clippy::cast_precision_loss)]
            floor32(n + (half as f32 * powi32(2.0, -23)))
        } else {
            floor32(n)
        }
    } else {
        n
    }
}

/// Returns the nearest integer to `n`.
/// If a value is half-way between two integers, round away from `0.0`.
#[inline]
#[must_use]
pub fn round_half_away64(n: f64) -> f64 {
    let bits = n.to_bits();
    let exponent = ((bits >> 52) & 0x7FF) as i32 - 1023;
    if exponent < 0 {
        if n.is_sign_positive() {
            0.0
        } else {
            -0.0
        }
    } else if exponent < 52 {
        let mask = (1u64 << (52 - exponent)) - 1;
        let frac_part = bits & mask;
        let half = 1u64 << (51 - exponent);
        if frac_part >= half {
            #[allow(clippy::cast_precision_loss)]
            floor64(n + (half as f64 * powi64(2.0, -52)))
        } else {
            floor64(n)
        }
    } else {
        n
    }
}

/// Rounds `n` to the nearest integer, where a fractional part of 0.5 is rounded
/// to the nearest even integer.
#[inline]
#[must_use]
pub fn round_half_even32(n: f32) -> f32 {
    let rounded = round_half_away32(n);
    if rounded % 2.0 == 0.0 || abs32(rounded - n) > 0.5 {
        rounded
    } else {
        rounded - signum32(n)
    }
}

/// Rounds `n` to the nearest integer, where a fractional part of 0.5 is rounded
/// to the nearest even integer.
#[inline]
#[must_use]
pub fn round_half_even64(n: f64) -> f64 {
    let rounded = round_half_away64(n);
    if rounded % 2.0 == 0.0 || abs64(rounded - n) > 0.5 {
        rounded
    } else {
        rounded - signum64(n)
    }
}

/// Returns the integer part of `n`.
/// This means that non-integer numbers are always truncated towards zero.
///
/// This implementation uses bitwise manipulation to remove the fractional part
/// of the floating-point number. The exponent is extracted, and a mask is
/// created to remove the fractional part. The new bits are then used to create
/// the truncated floating-point number.
#[inline]
#[must_use]
pub fn trunc32(n: f32) -> f32 {
    let bits = n.to_bits();
    #[allow(clippy::cast_possible_wrap)]
    let exponent = ((bits >> 23) & 0xFF) as i32 - 127;
    if exponent < 0 {
        if n.is_sign_positive() {
            0.0
        } else {
            -0.0
        }
    } else if exponent < 23 {
        let mask = !((1u32 << (23 - exponent)) - 1);
        let new_bits = bits & mask;
        f32::from_bits(new_bits)
    } else {
        n
    }
}

/// Returns the integer part of `n`.
/// This means that non-integer numbers are always truncated towards zero.
///
/// This implementation uses bitwise manipulation to remove the fractional part
/// of the floating-point number. The exponent is extracted, and a mask is
/// created to remove the fractional part. The new bits are then used to create
/// the truncated floating-point number.
#[inline]
#[must_use]
pub fn trunc64(n: f64) -> f64 {
    let bits = n.to_bits();
    let exponent = ((bits >> 52) & 0x7FF) as i32 - 1023;
    if exponent < 0 {
        if n.is_sign_positive() {
            0.0
        } else {
            -0.0
        }
    } else if exponent < 52 {
        let mask = !((1u64 << (52 - exponent)) - 1);
        let new_bits = bits & mask;
        f64::from_bits(new_bits)
    } else {
        n
    }
}

/// Returns a number that represents the sign of `n`.
///
/// - `1.0` if the number is positive, `+0.0` or `INFINITY`.
/// - `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`.
/// - NaN if the number is NaN.
#[inline]
#[must_use]
pub fn signum32(n: f32) -> f32 {
    let sign_bit = n.to_bits() >> 31;
    if n.is_nan() {
        f32::NAN
    } else if sign_bit == 0 {
        1.0
    } else {
        -1.0
    }
}
/// Returns a number that represents the sign of `n`.
///
/// - `1.0` if the number is positive, `+0.0` or `INFINITY`.
/// - `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`.
/// - NaN if the number is NaN.
#[inline]
#[must_use]
pub fn signum64(n: f64) -> f64 {
    let sign_bit = n.to_bits() >> 63;
    if n.is_nan() {
        f64::NAN
    } else if sign_bit == 0 {
        1.0
    } else {
        -1.0
    }
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
