// numera::number::integer::fns
//
//! Integer related standalone functions.
//

#[cfg(feature = "dashu-int")]
use dashu_int::{ops::BitTest, UBig};

/// Returns the minimum binary bits necessary to represent the given `integer`
/// in the given `base`.
///
/// This is known as the bit length function ([m][0m]).
///
/// [0m]: https://mathworld.wolfram.com/BitLength.html
///
///
/// The `base` must be between 2 and 36, inclusive.
/// Digits 10-35 are represented by a-z or A-Z
///
/// # Examples
/// ```
/// use numera::all::bit_len;
///
/// assert_eq![bit_len(10, "255"), Some(8)];
/// assert_eq![bit_len(16, "FFFFFFFFFFFF"), Some(48)];
/// ```
#[inline]
#[cfg(feature = "dashu-int")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "big")))]
pub fn bit_len(base: u8, integer: &str) -> Option<usize> {
    Some(UBig::from_str_radix(integer, base.into()).ok()?.bit_len())
}

/// Returns the result of the [`bit_len`] function and the next power of 2.
///
/// From the given `integer` in the given `base`, returns a tuple with the
/// minimum bit-size and the minimum power of two bit-size needed to represent
/// that number.
///
/// The `base` must be between 2 and 36, inclusive.
/// Digits 10-35 are represented by a-z or A-Z
///
/// Returns `None` if the string is not a valid number for the base, which must be
///
/// # Examples
/// ```
/// use numera::all::bit_len_next_power;
///
/// assert_eq![bit_len_next_power(10, "255"), Some((8, 8))];
/// assert_eq![bit_len_next_power(16, "FFFFFFFFFFFF"), Some((48, 64))];
/// ```
#[inline]
#[cfg(feature = "dashu-int")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "big")))]
pub fn bit_len_next_power(base: u8, integer: &str) -> Option<(usize, usize)> {
    let min = bit_len(base, integer)?;
    Some((min, min.checked_next_power_of_two()?))
}
