// numera::traits_continuity
//
//! defines the `Continous` & `Discrete` traits
//! and implements them for primitive and supported external types.
//

/// A *discrete* value ([w][0w]).
///
/// [0w]: https://en.wikipedia.org/wiki/Continuous_or_discrete_variable#Discrete_variable
pub trait Discrete {}

/// A *continuous* value ([w][0w]).
///
/// [0w]: https://en.wikipedia.org/wiki/Continuous_or_discrete_variable#Continuous_variable
pub trait Continuous {}

// macro impls

macro_rules! impl_discrete {
    (all: $($t:ty),+) => { $( impl_discrete![$t]; )+ };
    ($t:ty) => {
        impl Discrete for $t { }
    };
}
macro_rules! impl_continuous {
    (all: $($t:ty),+) => { $( impl_continuous![$t]; )+ };
    ($t:ty) => {
        impl Continuous for $t { }
    };
}

#[rustfmt::skip]
impl_discrete![all: f32, f64, i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize];
impl_continuous![all: f32, f64];

#[cfg(feature = "twofloat")]
impl_continuous![twofloat::TwoFloat];

#[cfg(feature = "half")]
impl_continuous![all: half::bf16, half::f16];

#[cfg(feature = "ibig")]
impl_discrete![all: ibig::IBig, ibig::UBig];

#[cfg(test)]
mod tests {
    use super::*;
    use static_assertions::*;

    macro_rules! assert_impl {
        (continuous: $($t:ty),+) => {
            $( assert_impl_all![$t: Continuous];)+
        };
        (discrete: $($t:ty),+) => {
            $( assert_impl_all![$t: Discrete];)+
        };
        // BUG:static_assertions
        // (not_continuous: $($t:ty),+) => {
        //     $( assert_not_impl_all![$t: Continuous];)+
        // };
        // (not_discrete: $($t:ty),+) => {
        //     $( assert_not_impl_all![$t: Discrete];)+
        // };
    }

    #[rustfmt::skip]
    #[test]
    fn discrete() {
        assert_impl![discrete:
            i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize];

        #[cfg(feature = "ibig")]
        assert_impl![discrete: ibig::IBig, ibig::UBig];
    }

    #[test]
    fn continuous() {
        assert_impl![continuous: f32, f64];
        // BUG:static_assertions
        // assert_not_impl_all![f32: Discrete];

        #[cfg(feature = "half")]
        assert_impl![continuous: half::bf16, half::f16];

        #[cfg(feature = "twofloat")]
        assert_impl![continuous: twofloat::TwoFloat];
    }
}
