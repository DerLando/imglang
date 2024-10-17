use crate::color::Color;

#[derive(Clone, Copy, Debug)]
pub struct Stroke {
    pub(crate) width: f64,
    pub(crate) color: Color,
}
