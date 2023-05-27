// numera::number::integer::prime::fns
//
//! Prime related standalone functions.
//
// TOC
//
// - prime_number_theorem
//
// - is_prime
// - is_prime_brute
// - nth_prime
// - prime_pi
//
// - is_prime_sieve
// - nth_prime_sieve
// - prime_pi_sieve
//
// - largest_prime_pow2_doublings
// - ten_primes_less_pow2

#[cfg(feature = "std")]
use {core::num::NonZeroUsize, primal_sieve::Sieve};

#[cfg(feature = "big")]
use {
    super::data::{LARGEST_PRIME_POW2_DOUBLINGS, TEN_PRIMES_LESS_POW2},
    crate::all::IntegerBig,
};

use crate::number::real::float::fns::sqrt_fisr64;
use core::num::NonZeroU32;

/// The prime number theorem ([m][0m]/[w][0w]) formula.
///
/// $ \pi(x) \sim \frac{x}{\ln(x)} $
///
/// Returns the approximate count of primes less than the given `n`.
///
/// [0m]: https://mathworld.wolfram.com/PrimeNumberTheorem.html
/// [0w]: https://en.wikipedia.org/wiki/Prime_number_theorem
//
// IMPROVE: use big int and big float.
#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
pub fn prime_number_theorem(n: u128) -> u128 {
    #[allow(clippy::cast_precision_loss)]
    let float_n = n as f64;
    let ln_n = float_n.ln();
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    return (float_n / ln_n).round() as u128;
}

/// Checks whether a `number` is prime, using optimized trial division.
///
/// This approach checks only odd numbers starting from 3 and up to the square
/// root of the given number. This is based on the fact that if a number is
/// divisible by a number larger than its square root, the result of the
/// division will be smaller than the square root, and it would have already
/// been checked in previous iterations.
pub fn is_prime(number: u32) -> bool {
    match number {
        0..=1 => false,
        2..=3 => true,
        _ => {
            if number % 2 == 0 {
                return false;
            }

            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let limit = sqrt_fisr64(f64::from(number)) as u32;

            for i in (3..=limit).step_by(2) {
                if number % i == 0 {
                    return false;
                }
            }
            true
        }
    }
}

/// Checks whether a `number` is prime, using basic trial division.
///
/// This naive approach checks all numbers from 2 to number/2.
pub fn is_prime_brute(number: u32) -> bool {
    if number <= 1 {
        return false;
    }
    for i in 2..=number / 2 {
        if number % i == 0 {
            return false;
        }
    }
    true
}

/// Finds the `nth` prime number using [`is_prime`].
pub fn nth_prime(nth: NonZeroU32) -> u32 {
    let mut count = 0;
    let mut i = 2;
    let nth = nth.get();
    loop {
        if is_prime(i) {
            count += 1;
        }
        if count == nth {
            return i;
        }
        i += 1;
    }
}

/// Counts the number of primes upto and including `n`, using [`is_prime`].
///
/// # Notation
/// $\pi(x)$
///
/// # Links
/// - <https://mathworld.wolfram.com/PrimeCountingFunction.html>.
/// - <https://en.wikipedia.org/wiki/Prime-counting_function>.
pub fn prime_pi(n: u32) -> usize {
    let mut prime_count = 0;
    for i in 0..=n {
        if is_prime(i) {
            prime_count += 1;
        }
    }
    prime_count
}

/// Checks whether a `number` is prime, using an optimized [`Sieve`].
#[inline]
#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
pub fn is_prime_sieve(number: usize) -> bool {
    Sieve::new(number).is_prime(number)
}

/// Finds the `nth` prime number, using [`is_prime_sieve`].
#[inline]
#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
pub fn nth_prime_sieve(nth: NonZeroUsize) -> usize {
    Sieve::new(nth.get()).nth_prime(nth.get())
}

/// Counts the number of primes upto and including `n`, using [`is_prime_sieve`].
#[inline]
#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
pub fn prime_pi_sieve(n: usize) -> usize {
    Sieve::new(n).prime_pi(n)
}

/* data extraction */

/// Returns a big integer containing the largest prime just less the power of
/// two represented by $2^{3+i)}$.
///
/// Valid `i` values are between 0 and 13 inclusive, which corresponds to
/// bit-sizes between 8 and 65,536.
///
/// Returns `None` if `index` is > 13.
///
/// It uses the [`LARGEST_PRIME_POW2_DOUBLINGS`] table.
///
/// # Examples
/// ```
/// use numera::all::{largest_prime_pow2_doublings, ConstUpperBounded, Nnz128, Prime64};
///
/// assert_eq![largest_prime_pow2_doublings(3).unwrap(), Prime64::MAX.into()];
/// assert_eq![largest_prime_pow2_doublings(4).unwrap(), (Nnz128::MAX - Nnz128::new(159 -1)).into()];
/// ```
#[inline]
#[cfg(feature = "big")]
#[cfg_attr(feature = "nightly", doc(cfg(any(feature = "big", feature = "ibig"))))]
pub fn largest_prime_pow2_doublings(i: usize) -> Option<IntegerBig> {
    if i > 13 {
        None
    } else {
        #[allow(clippy::cast_possible_truncation)]
        return Some(
            IntegerBig::new(2).pow(2_usize.pow(3 + i as u32))
                - IntegerBig::new(LARGEST_PRIME_POW2_DOUBLINGS[i].into()),
        );
    }
}

/// Returns a big integer containing one of the ten largest primes just less
/// than a power of two, between 8 and 400 bits.
///
/// - Valid `bitsize` values are between 8 and 400 inclusive.
/// - Valid `index` values are from 0 to 9 inclusive, and indexes from the end.
///   E.g. index=0 is the largest prime for the given bitsize.
///
/// Returns `None` if `bitsize` is < 8 or > 400 or if `index` is more than 9.
///
/// It uses the [`TEN_PRIMES_LESS_POW2`] table.
///
/// # Examples
/// ```
/// use numera::number::{integer::prime::*, traits::ConstUpperBounded};
///
/// assert_eq![ten_primes_less_pow2(8, 0).unwrap(), Prime8::MAX.into()];
/// assert_eq![ten_primes_less_pow2(16, 0).unwrap(), Prime16::MAX.into()];
/// assert_eq![ten_primes_less_pow2(32, 0).unwrap(), Prime32::MAX.into()];
/// assert_eq![ten_primes_less_pow2(64, 0).unwrap(), Prime64::MAX.into()];
/// assert_eq![ten_primes_less_pow2(128, 0).unwrap(), Prime128::MAX.into()];
///
/// assert_eq![ten_primes_less_pow2(8, 0).unwrap(), PRIMES_U8[53].into()];
/// assert_eq![ten_primes_less_pow2(8, 1).unwrap(), PRIMES_U8[53-1].into()];
/// assert_eq![ten_primes_less_pow2(8, 9).unwrap(), PRIMES_U8[53-9].into()];
/// assert_eq![ten_primes_less_pow2(16, 0).unwrap(), PRIMES_U16[6_487].into()];
/// assert_eq![ten_primes_less_pow2(16, 1).unwrap(), PRIMES_U16[6_487-1].into()];
/// assert_eq![ten_primes_less_pow2(16, 9).unwrap(), PRIMES_U16[6_487-9].into()];
/// ```
#[inline]
#[cfg(feature = "big")]
#[cfg_attr(feature = "nightly", doc(cfg(any(feature = "big", feature = "ibig"))))]
pub fn ten_primes_less_pow2(bitsize: usize, index: usize) -> Option<IntegerBig> {
    if (8..=400).contains(&bitsize) && index < 10 {
        Some(
            IntegerBig::new(2).pow(bitsize)
                - IntegerBig::new(TEN_PRIMES_LESS_POW2[bitsize - 8][index].into()),
        )
    } else {
        None
    }
}
