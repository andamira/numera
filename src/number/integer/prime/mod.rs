// numera::number::integer::prime
//
//! Prime numbers ([m][0m]/[o][0o]/[w][0w]), from the set $\Bbb{P}$.
//!
//! $ \Bbb{P} = \lbrace 2, 3, 5, 7, 11, 13, 17, 19, 23, … \rbrace $ ([oeis])
//!
//! [0m]: https://mathworld.wolfram.com/PrimeNumber.html
//! [0o]: https://oeis.org/wiki/Prime_numbers
//! [0w]: https://en.wikipedia.org/wiki/Prime_number
//! [oeis]: https://oeis.org/A000040
//

mod convert;
pub mod data;
mod family;
mod fns;
mod impl_traits;
mod primes;
mod sized;

pub use all::*;
pub(crate) mod all {
    #[doc(inline)]
    pub use super::{
        data::*,
        family::Prime,
        fns::{is_prime, is_prime_brute, nth_prime, prime_pi},
        primes::Primes,
        sized::{Prime128, Prime16, Prime32, Prime64, Prime8},
    };

    #[doc(inline)]
    #[cfg(feature = "std")]
    pub use super::fns::{is_prime_sieve, nth_prime_sieve, prime_number_theorem, prime_pi_sieve};

    #[doc(inline)]
    #[cfg(feature = "ibig")]
    pub use super::fns::{largest_prime_pow2_doublings, ten_primes_less_pow2};
}
