// numera::number::traits::maxmin
//
//!
//

/// The minimal and maximal representations of the current number.
pub trait MaxMin {
    const MIN: Self;
    const MAX: Self;
}
macro_rules! impl_maxmin {
    (all: $($ty:ty),+) => {
        $( impl_maxmin![$ty]; )+
    };
    ($ty:ty) => {
        impl MaxMin for $ty {
            const MAX: Self = <$ty>::MAX;
            const MIN: Self = <$ty>::MIN;
        }
    };
}
#[rustfmt::skip]
impl_maxmin![all: f32, f64, i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize];

#[cfg(feature = "twofloat")]
impl_maxmin![twofloat::TwoFloat];

#[cfg(feature = "half")]
impl_maxmin![all: half::bf16, half::f16];
