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
mod family;
mod fns;
mod impl_traits;
mod sized;
mod r#trait;

pub mod data;

pub use all::*;
pub(crate) mod all {
    #[doc(inline)]
    pub use super::{
        abbr::*,
        data::*,
        family::Primes,
        fns::*,
        r#trait::Prime,
        sized::{Prime128, Prime16, Prime32, Prime64, Prime8},
    };
}

mod abbr {
    use super::{family::*, sized::*};

    use crate::number::macros::define_abbreviations;
    define_abbreviations![sized P, Prime, 8, 16, 32, 64, 128];
    define_abbreviations![family P, Primes];
}
