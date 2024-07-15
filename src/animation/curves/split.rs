use super::{Curve, ICurve};

pub struct Split {
    split: f32,
    begin: Curve,
    end: Curve,
}

impl ICurve for Split {
    fn transform(&self, t: f32) -> f32 {
        assert!((0.0..=1.0).contains(&t));
        assert!((0.0..=1.0).contains(&self.split));

        match t {
            0.0 | 1.0 => t,
            t if (t - self.split).abs() < f32::EPSILON => self.split,
            t if t < self.split => {
                let curve_progress = t / self.split;
                let transformed = ICurve::transform(&self.begin, curve_progress);

                0.0f32.mul_add(1.0 - transformed, self.split * transformed)
            }
            t => {
                let curve_progress = (t - self.split) / (1.0 - self.split);
                let transformed = ICurve::transform(&self.end, curve_progress);

                self.split.mul_add(1.0 - transformed, 1.0 * transformed)
            }
        }
    }
}
