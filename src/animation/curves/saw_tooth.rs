use super::ParametricCurve;

pub struct SawTooth {
    pub(super) count: f32,
}

impl ParametricCurve<f32> for SawTooth {
    fn transform_internal(&self, mut t: f32) -> f32 {
        t *= self.count;

        t - t.trunc()
    }
}
