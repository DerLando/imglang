#[derive(Clone, Copy, Debug)]
pub(crate) struct PointPolar {
    radius: f64,
    angle: f64,
}

impl PointPolar {
    pub const fn new(radius: f64, angle: f64) -> Self {
        Self { radius, angle }
    }

    pub fn x(&self) -> f64 {
        self.radius * self.angle.cos()
    }

    pub fn y(&self) -> f64 {
        self.radius * self.angle.sin()
    }
}
