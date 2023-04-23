// numera::number::integer::prime::fns
//
//! Alternative implementations for finding primes.
//

use core::num::NonZeroUsize;
use primal_sieve::Sieve;

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
pub fn nth_prime_brute(nth: u32) -> u32 {
    let mut count = 0;
    let mut i = 2;
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

/// Checks wheter a `number` is prime, using an optimized [`Sieve`].
#[inline]
pub fn is_prime_sieve(number: usize) -> bool {
    Sieve::new(number).is_prime(number)
}

/// Finds the `nth` prime number, using an optimize [`Sieve`]
#[inline]
pub fn nth_prime_sieve(nth: NonZeroUsize) -> usize {
    Sieve::new(nth.get()).nth_prime(nth.get())
}
