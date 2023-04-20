// numera::number::integer::prime::fns
//
//!
//

/// Checks if a `number` is prime, by brute force.
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
