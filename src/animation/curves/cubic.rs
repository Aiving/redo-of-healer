use super::ParametricCurve;

pub struct Cubic {
    pub(super) a: f32,
    pub(super) b: f32,
    pub(super) c: f32,
    pub(super) d: f32,
}

impl Cubic {
    pub const CUBIC_ERROR_BOUND: f32 = 0.001;

    fn _evaluate_cubic(a: f32, b: f32, m: f32) -> f32 {
        (3.0 * a * (1.0 - m).powi(2)).mul_add(m, 3.0 * b * (1.0 - m) * m.powi(2)) + m.powi(3)
    }
}

impl ParametricCurve<f32> for Cubic {
    fn transform_internal(&self, t: f32) -> f32 {
        let mut start = 0.0;
        let mut end = 1.0;

        loop {
            let midpoint = (start + end) / 2.0;
            let estimate = Self::_evaluate_cubic(self.a, self.c, midpoint);

            if (t - estimate).abs() < Self::CUBIC_ERROR_BOUND {
                return Self::_evaluate_cubic(self.b, self.d, midpoint);
            }

            if estimate < t {
                start = midpoint;
            } else {
                end = midpoint;
            }
        }
    }
}
