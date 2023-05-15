// numera::number::integer::prime::fns
//
//! Alternative implementations for finding primes.
//

use core::num::NonZeroU32;
#[cfg(feature = "std")]
use {core::num::NonZeroUsize, primal_sieve::Sieve};

/// Checks whether a `number` is prime, by brute force.
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

/// Finds the `nth` prime number using [`is_prime_brute`].
pub fn nth_prime_brute(nth: NonZeroU32) -> u32 {
    let mut count = 0;
    let mut i = 2;
    let nth = nth.get();
    loop {
        if is_prime_brute(i) {
            count += 1;
        }
        if count == nth {
            return i;
        }
        i += 1;
    }
}

/// Counts the number of primes upto and including `n`.
///
/// # Notation
/// $\pi(x)$
///
/// # Links
/// - <https://mathworld.wolfram.com/PrimeCountingFunction.html>.
/// - <https://en.wikipedia.org/wiki/Prime-counting_function>.
pub fn prime_pi_brute(n: NonZeroU32) -> usize {
    let n = n.get();
    let mut prime_count = 0;
    for i in 1..=n {
        if is_prime_brute(i) {
            prime_count += 1;
        }
    }
    prime_count
}

/// Checks wheter a `number` is prime, using an optimized [`Sieve`].
#[inline]
#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
pub fn is_prime_sieve(number: usize) -> bool {
    Sieve::new(number).is_prime(number)
}

/// Finds the `nth` prime number, using an optimize [`Sieve`]
#[inline]
#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
pub fn nth_prime_sieve(nth: NonZeroUsize) -> usize {
    Sieve::new(nth.get()).nth_prime(nth.get())
}
