// numera::number::traits::bounded
//
//! implements the `[Const][Lower|Upper]Bounded` traits.
//

/// Indicates this type as both const lower and upper bounds.
pub trait ConstBounded: ConstLowerBounded + ConstUpperBounded {}

/// Indicates this type has a const lower bound.
pub trait ConstLowerBounded {
    /// The smallest value that can be represented with this type.
    const MIN: Self;
}

/// Indicates this type has a const upper bound.
pub trait ConstUpperBounded {
    /// The smallest value that can be represented with this type.
    const MAX: Self;
}

/// Indicates this type as both lower and upper bounds.
pub trait Bounded: LowerBounded + UpperBounded {}

/// Indicates this type has a lower bound.
pub trait LowerBounded {
    /// The smallest value that can be represented with this type.
    fn new_min() -> Self;
}

/// Indicates this type has an upper bound.
pub trait UpperBounded {
    /// The largest value that can be represented with this type.
    fn new_max() -> Self;
}

// auto impls

/// Auto implements ConstBounded.
impl<T: ConstLowerBounded + ConstUpperBounded> ConstBounded for T {}

/// Auto implements Bounded.
impl<T: LowerBounded + UpperBounded> Bounded for T {}

// Can't do this. It's necessary to implement these manually to avoid conflicts:
// impl<T: ConstLowerBounded> LowerBounded for T { fn new_min() -> Self { Self::MIN } }
// impl<T: ConstUpperBounded> UpperBounded for T { fn new_max() -> Self { Self::MAX } }

// macro impls

/// Implements *both* const & non-const Bounded traits.
macro_rules! impl_const_bounded {
    (all: $($ty:ty),+) => {
        $( impl_const_bounded![both: $ty]; )+
    };
    (both: $ty:ty) => {
        impl_const_bounded![lower: $ty];
        impl_const_bounded![upper: $ty];
    };
    (all_lower: $($ty:ty),+) => {
        $( impl_const_bounded![lower: $ty]; )+
    };
    (lower: $ty:ty) => {
        impl ConstLowerBounded for $ty { const MIN: Self = <$ty>::MIN; }
        impl LowerBounded for $ty { fn new_min() -> Self { <$ty>::MIN }}
    };
    (all_upper: $($ty:ty),+) => {
        $( impl_const_bounded![upper: $ty]; )+
    };
    (upper: $ty:ty) => {
        impl ConstUpperBounded for $ty { const MAX: Self = <$ty>::MAX; }
        impl UpperBounded for $ty { fn new_max() -> Self { <$ty>::MAX }}
    }
}

#[rustfmt::skip]
impl_const_bounded![all:
    f32, f64, i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize];

#[cfg(feature = "twofloat")]
impl_const_bounded![both: twofloat::TwoFloat];

#[cfg(feature = "half")]
impl_const_bounded![all: half::bf16, half::f16];

/// Tests
#[cfg(test)]
mod tests {
    use super::*;
    use static_assertions::assert_impl_all;

    /// Checks the `[[Const][Lower|Upper]Bounded]` traits for primitives.
    #[test]
    fn bounded_primitives() {
        macro_rules! assert_impl_bounded {
            (both: $($ty:ty),+) => {
                assert_impl_bounded![@const: $($ty),+];
                assert_impl_bounded![@nonconst: $($ty),+];
            };
            (@const: $($ty:ty),+) => {
                $( assert_impl_all![$ty: ConstLowerBounded, ConstUpperBounded, ConstBounded];)+
            };
            (@nonconst: $($ty:ty),+) => {
                $( assert_impl_all![$ty: LowerBounded, UpperBounded, Bounded];)+
            };
        }
        assert_impl_bounded![both: i8, i16, i32, i64, i128, isize];
        assert_impl_bounded![both: u8, u16, u32, u64, u128, usize];
        assert_impl_bounded![both: f32, f64];

        #[cfg(feature = "twofloat")]
        assert_impl_bounded![both: twofloat::TwoFloat];

        #[cfg(feature = "half")]
        assert_impl_bounded![both: half::f16, half::bf16];
    }
}
