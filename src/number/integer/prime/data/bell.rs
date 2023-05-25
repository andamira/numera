// numera::number::integer::prime::data::bell
//
//! The bell primes.
//

/// The first 5 Bell primes.
// #[cfg(not(feature = "ruint"))]
pub const PRIMES_BELL: [u128; 5] = [
    2,
    5,
    877,
    27_644_437,
    35_742_549_198_872_617_291_353_508_656_626_642_567,
];
// /// The first 6 Bell primes.
// #[cfg(feature = "ruint")]
// pub const BELL_PRIMES: [Uint<256, 4>; 6] = uint!([
//     2_U256,
//     5_U256,
//     877_U256,
//     27_644_437_U256,
//     35_742_549_198_872_617_291_353_508_656_626_642_567_U256,
//     359_334_085_968_622_831_041_960_188_598_043_661_065_388_726_959_079_837_U256,
// ]);
