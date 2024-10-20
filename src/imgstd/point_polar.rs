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

#[derive(Clone, Copy, Debug)]
pub(crate) struct PointCarthesian {
    x: f64,
    y: f64,
}

impl From<PointPolar> for PointCarthesian {
    fn from(value: PointPolar) -> Self {
        Self::new(value.x(), value.y())
    }
}

impl From<(f64, f64)> for PointCarthesian {
    fn from(value: (f64, f64)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl PointCarthesian {
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }
}
