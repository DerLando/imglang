use super::point_polar::PointCarthesian;

#[derive(Clone, Copy, Debug)]
pub enum Geometry {
    Circle(Circle),
    Line(Line),
}

#[derive(Clone, Copy, Debug)]
pub struct Circle {
    pub(crate) radius: f64,
}

#[derive(Clone, Copy, Debug)]
pub struct Line {
    pub(crate) x0: f64,
    pub(crate) x1: f64,
    pub(crate) y0: f64,
    pub(crate) y1: f64,
}

impl Line {
    pub const fn new(x0: f64, y0: f64, x1: f64, y1: f64) -> Self {
        Self { x0, x1, y0, y1 }
    }
}

impl<T, U> From<(T, U)> for Line
where
    T: Into<PointCarthesian>,
    U: Into<PointCarthesian>,
{
    fn from(value: (T, U)) -> Self {
        let start = value.0.into();
        let end = value.1.into();

        Self::new(start.x(), start.y(), end.x(), end.y())
    }
}
