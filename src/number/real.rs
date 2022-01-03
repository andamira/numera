//! Real numbers

use num_traits::float::{Float, FloatConst};

/// A `Real` number ([w][w1]/[m][m1]),
/// from the set $\real$.
///
/// [w1]: https://en.wikipedia.org/wiki/Real_number
/// [m1]: https://mathworld.wolfram.com/RealNumber.html
#[repr(transparent)]
#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct Real<F>(F)
    // Float already includes:
    // Copy + PartialEq + PartialOrd + Zero + One + Neg + NumOps + NumCast
    where F: Float;

impl<F: Float> Real<F> {
    /// Returns a new `Real` with any value supported by the underlying type.
    pub fn new(value: F) -> Self {
        Self(value)
    }

    /// Returns a copy of the underlying data type.
    pub fn get(&self) -> F {
        self.0
    }

    /// Returns the epsilon value of the raw inner data.
    pub fn epsilon() -> F {
        F::epsilon()
    }
}

use float_eq::{FloatEq, FloatEqUlpsTol, UlpsTol};

pub struct RealUlps<F: Float + FloatConst + FloatEqUlpsTol>(UlpsTol<F>);

impl<F: Float + FloatConst + FloatEqUlpsTol> FloatEqUlpsTol for Real<F> {
    type UlpsTol = RealUlps<F>;
}

impl<F: Float + FloatConst + FloatEqUlpsTol + FloatEq<Tol = F>> FloatEq for Real<F> {
    type Tol = Real<F>;
    fn eq_abs(&self, other: &Self, tol: &Real<F>) -> bool {
        self.0.eq_abs(&other.0, &tol.0)
    }
    fn eq_rmax(&self, other: &Self, tol: &Real<F>) -> bool {
        self.0.eq_rmax(&other.0, &tol.0)
    }
    fn eq_rmin(&self, other: &Self, tol: &Real<F>) -> bool {
        self.0.eq_rmin(&other.0, &tol.0)
    }
    fn eq_r1st(&self, other: &Self, tol: &Real<F>) -> bool {
        self.0.eq_r1st(&other.0, &tol.0)
    }
    fn eq_r2nd(&self, other: &Self, tol: &Real<F>) -> bool {
        self.0.eq_r2nd(&other.0, &tol.0)
    }
    fn eq_ulps(&self, other: &Self, tol: &UlpsTol<Real<F>>) -> bool {
        self.0.eq_ulps(&other.0, &tol.0)
    }
}

crate::impl_ops![F, Real, Real<F>, Self, Float];

mod impl_from {
    use super::Real;
    use half::{bf16, f16};
    // use twofloat::TwoFloat; // WIP

    impl From<Real<f16>> for Real<f32> {
        fn from(f: Real<f16>) -> Self {
            Self(f.0.to_f32())
        }
    }

    impl From<Real<bf16>> for Real<f32> {
        fn from(f: Real<bf16>) -> Self {
            Self(f.0.to_f32())
        }
    }

    impl From<Real<f16>> for Real<f64> {
        fn from(f: Real<f16>) -> Self {
            Self(f.0.to_f64())
        }
    }

    impl From<Real<bf16>> for Real<f64> {
        fn from(f: Real<bf16>) -> Self {
            Self(f.0.to_f64())
        }
    }

    impl From<Real<f32>> for Real<f64> {
        fn from(f: Real<f32>) -> Self {
            Self(f.0.into())
        }
    }

    // impl From<Real<f32>> for Real<TwoFloat> {
    //     fn from(f: Real<f32>) -> Self {
    //         Self(TwoFloat::from(f.0))
    //     }
    // }

    // impl From<Real<f64>> for Real<TwoFloat> {
    //     fn from(f: Real<f64>) -> Self {
    //         Self(TwoFloat::from(f.0))
    //     }
    // }

    // /// From `f64` to `f32`.
    // impl TryFrom<Real<f64>> for Real<f32> {
    //     type Error = &'static str;
    //
    //     fn try_from(f: Real<f64>) -> Result<Self, Self::Error> {
    //         f.0.try_into()
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use super::Real;
    // use half::{bf16, f16};
    // use twofloat::TwoFloat; // WIP
    use float_eq::assert_float_eq;

    #[test]
    fn ops() {
        // MAYBE FIXABLE?
        // assert_float_eq![2.0, Real(4.0) - Real(4.0), r2nd <= Real::epsilon()];
        // assert_float_eq![Real(2.0), Real(4.0) - Real(4.0), r2nd <= Real::epsilon()];

        // WORKS:
        assert_float_eq![3.0, (Real(5.0) - Real(2.0)).0, r2nd <= Real::epsilon()];
        assert_float_eq![7.0, (Real(5.0) + Real(2.0)).0, r2nd <= Real::epsilon()];
        assert_float_eq![10.0, (Real(5.0) * Real(2.0)).0, r2nd <= Real::epsilon()];
        assert_float_eq![2.5, (Real(5.0) / Real(2.0)).0, r2nd <= Real::epsilon()];

        // WIP
        // let mut r = Real(2.0);
        // r += Real(4.0);
        // assert_float_eq![6.0, r.0, r2nd <= Real::epsilon()];
        // r -= Real(1.0);
        // assert_float_eq![5.0, r.0, r2nd <= Real::epsilon()];
        // r *= Real(3.0);
        // assert_float_eq![15.0, r.0, r2nd <= Real::epsilon()];
        // r /= Real(2.0);
        // assert_float_eq![7.5, r.0, r2nd <= Real::epsilon()];
    }
}
