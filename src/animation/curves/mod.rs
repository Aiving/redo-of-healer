mod bounce;
mod cubic;
mod decelerate;
mod elastic;
mod interval;
mod linear;
mod saw_tooth;
mod split;
mod threshold;

pub use self::{
    bounce::{BounceInCurve, BounceInOutCurve, BounceOutCurve},
    cubic::Cubic,
    decelerate::DecelerateCurve,
    elastic::{ElasticInCurve, ElasticInOutCurve, ElasticOutCurve},
    interval::Interval,
    linear::Linear,
    saw_tooth::SawTooth,
    split::Split,
    threshold::Threshold,
};

trait ParametricCurve<T> {
    fn transform_internal(&self, t: f32) -> T;
}

pub trait ICurve {
    fn transform(&self, t: f32) -> f32;
}

impl<T: ParametricCurve<f32>> ICurve for T {
    fn transform(&self, t: f32) -> f32
    where
        Self: Sized,
    {
        match t {
            0.0 | 1.0 => t,
            t => self.transform_internal(t),
        }
    }
}

pub enum Curve {
    Linear(Linear),
    Cubic(Cubic),
    Threshold(Threshold),
    SawTooth(SawTooth),
    BounceIn(BounceInCurve),
    BounceOut(BounceOutCurve),
    BounceInOut(BounceInOutCurve),
    ElasticIn(ElasticInCurve),
    ElasticOut(ElasticOutCurve),
    ElasticInOut(ElasticInOutCurve),
    Decelerate(DecelerateCurve),
}

impl Curve {
    pub const LINEAR: Self = Self::Linear(Linear);
    pub const DECELERATE: Self = Self::Decelerate(DecelerateCurve);
    pub const FAST_LINEAR_TO_SLOW_EASE_IN: Self = Self::cubic(0.18, 1.0, 0.04, 1.0);
    //   pub const fastEaseInToSlowEaseOut: Curve = ThreePointCubic(
    //     Offset(0.056, 0.024),
    //     Offset(0.108, 0.3085),
    //     Offset(0.198, 0.541),
    //     Offset(0.3655, 1.0),
    //     Offset(0.5465, 0.989),
    //   );
    pub const EASE: Self = Self::cubic(0.25, 0.1, 0.25, 1.0);
    pub const EASE_IN: Self = Self::cubic(0.42, 0.0, 1.0, 1.0);
    pub const EASE_IN_TO_LINEAR: Self = Self::cubic(0.67, 0.03, 0.65, 0.09);
    pub const EASE_IN_SINE: Self = Self::cubic(0.47, 0.0, 0.745, 0.715);
    pub const EASE_IN_QUAD: Self = Self::cubic(0.55, 0.085, 0.68, 0.53);
    pub const EASE_IN_CUBIC: Self = Self::cubic(0.55, 0.055, 0.675, 0.19);
    pub const EASE_IN_QUART: Self = Self::cubic(0.895, 0.03, 0.685, 0.22);
    pub const EASE_IN_QUINT: Self = Self::cubic(0.755, 0.05, 0.855, 0.06);
    pub const EASE_IN_EXPO: Self = Self::cubic(0.95, 0.05, 0.795, 0.035);
    pub const EASE_IN_CIRC: Self = Self::cubic(0.6, 0.04, 0.98, 0.335);
    pub const EASE_IN_BACK: Self = Self::cubic(0.6, -0.28, 0.735, 0.045);
    pub const EASE_OUT: Self = Self::cubic(0.0, 0.0, 0.58, 1.0);
    pub const LINEAR_TO_EASE_OUT: Self = Self::cubic(0.35, 0.91, 0.33, 0.97);
    pub const EASE_OUT_SINE: Self = Self::cubic(0.39, 0.575, 0.565, 1.0);
    pub const EASE_OUT_QUAD: Self = Self::cubic(0.25, 0.46, 0.45, 0.94);
    pub const EASE_OUT_CUBIC: Self = Self::cubic(0.215, 0.61, 0.355, 1.0);
    pub const EASE_OUT_QUART: Self = Self::cubic(0.165, 0.84, 0.44, 1.0);
    pub const EASE_OUT_QUINT: Self = Self::cubic(0.23, 1.0, 0.32, 1.0);
    pub const EASE_OUT_EXPO: Self = Self::cubic(0.19, 1.0, 0.22, 1.0);
    pub const EASE_OUT_CIRC: Self = Self::cubic(0.075, 0.82, 0.165, 1.0);
    pub const EASE_OUT_BACK: Self = Self::cubic(0.175, 0.885, 0.32, 1.275);
    pub const EASE_IN_OUT: Self = Self::cubic(0.42, 0.0, 0.58, 1.0);
    pub const EASE_IN_OUT_SINE: Self = Self::cubic(0.445, 0.05, 0.55, 0.95);
    pub const EASE_IN_OUT_QUAD: Self = Self::cubic(0.455, 0.03, 0.515, 0.955);
    pub const EASE_IN_OUT_CUBIC: Self = Self::cubic(0.645, 0.045, 0.355, 1.0);
    //   static const ThreePointCubic easeInOutCubicEmphasized = ThreePointCubic(
    //       Offset(0.05, 0), Offset(0.133333, 0.06),
    //       Offset(0.166666, 0.4),
    //       Offset(0.208333, 0.82), Offset(0.25, 1),
    //   );
    pub const EASE_IN_OUT_QUART: Self = Self::cubic(0.77, 0.0, 0.175, 1.0);
    pub const EASE_IN_OUT_QUINT: Self = Self::cubic(0.86, 0.0, 0.07, 1.0);
    pub const EASE_IN_OUT_EXPO: Self = Self::cubic(1.0, 0.0, 0.0, 1.0);
    pub const EASE_IN_OUT_CIRC: Self = Self::cubic(0.785, 0.135, 0.15, 0.86);
    pub const EASE_IN_OUT_BACK: Self = Self::cubic(0.68, -0.55, 0.265, 1.55);
    pub const FAST_OUT_SLOW_IN: Self = Self::cubic(0.4, 0.0, 0.2, 1.0);
    pub const SLOW_MIDDLE: Self = Self::cubic(0.15, 0.85, 0.85, 0.15);
    pub const BOUNCE_IN: Self = Self::BounceIn(BounceInCurve);
    pub const BOUNCE_OUT: Self = Self::BounceOut(BounceOutCurve);
    pub const BOUNCE_IN_OUT: Self = Self::BounceInOut(BounceInOutCurve);
    pub const ELASTIC_IN: Self = Self::ElasticIn(ElasticInCurve::default());
    pub const ELASTIC_OUT: Self = Self::ElasticOut(ElasticOutCurve::default());
    pub const ELASTIC_IN_OUT: Self = Self::ElasticInOut(ElasticInOutCurve::default());

    #[must_use]
    pub const fn cubic(a: f32, b: f32, c: f32, d: f32) -> Self {
        Self::Cubic(Cubic { a, b, c, d })
    }

    #[must_use]
    pub const fn threshold(threshold: f32) -> Self {
        Self::Threshold(Threshold { threshold })
    }

    #[must_use]
    pub const fn saw_tooth(count: f32) -> Self {
        Self::SawTooth(SawTooth { count })
    }
}

impl ParametricCurve<f32> for Curve {
    fn transform_internal(&self, t: f32) -> f32 {
        match self {
            Self::Linear(curve) => curve.transform_internal(t),
            Self::Cubic(curve) => curve.transform_internal(t),
            Self::Threshold(curve) => curve.transform_internal(t),
            Self::SawTooth(curve) => curve.transform_internal(t),
            Self::BounceIn(curve) => curve.transform_internal(t),
            Self::BounceOut(curve) => curve.transform_internal(t),
            Self::BounceInOut(curve) => curve.transform_internal(t),
            Self::ElasticIn(curve) => curve.transform_internal(t),
            Self::ElasticOut(curve) => curve.transform_internal(t),
            Self::ElasticInOut(curve) => curve.transform_internal(t),
            Self::Decelerate(curve) => curve.transform_internal(t),
        }
    }
}
