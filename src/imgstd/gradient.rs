#[derive(Clone, Copy)]
pub(crate) enum StepFunction {
    Linear,
}

impl StepFunction {
    pub fn evaluate(&self, t: f64) -> f64 {
        match self {
            StepFunction::Linear => t,
        }
    }

    pub fn evaluate_bounded(&self, min: f64, max: f64, t: f64) -> f64 {
        min + self.evaluate(t) * (max - min)
    }
}
