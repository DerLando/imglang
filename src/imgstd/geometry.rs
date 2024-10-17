#[derive(Clone, Copy, Debug)]
pub enum Geometry {
    Circle(Circle),
}

#[derive(Clone, Copy, Debug)]
pub struct Circle {
    pub(crate) radius: f64,
}
