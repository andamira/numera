// numera::number::integer::prime
//
//! Prime numbers.
//

mod arrays;
mod fns;
mod impl_traits;
mod trait_prime;

pub use arrays::{PRIMES_BELL, PRIMES_U16, PRIMES_U8};
pub use fns::{is_prime_brute, nth_prime_brute};
pub use trait_prime::Prime;

/// Can represent the first `54` prime numbers.
///
// pub struct Prime8(PositiveInteger8);
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Prime8(u8);

/// Can represent the first `6_542` prime numbers.
///
// pub struct Prime16(PositiveInteger16);
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Prime16(u16);

/// Can represent the first `203_280_219` prime numbers.
///
// pub struct Prime32(PositiveInteger32);
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Prime32(u32);
