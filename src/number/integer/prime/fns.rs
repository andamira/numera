// numera::number::integer::prime::fns
//
//! Alternative implementations for finding primes.
//

use crate::number::real::sqrt_fisr64;
use core::num::NonZeroU32;
#[cfg(feature = "std")]
use {core::num::NonZeroUsize, primal_sieve::Sieve};

// /// Checks whether a `number` is prime, using basic trial division.
// ///
// /// This naive approach checks all numbers from 2 to number/2.
// pub fn is_prime_brute(number: u32) -> bool {
//     if number <= 1 {
//         return false;
//     }
//     for i in 2..=number / 2 {
//         if number % i == 0 {
//             return false;
//         }
//     }
//     true
// }

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
