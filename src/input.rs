use crate::color::Color;

pub struct InputMap {}

#[derive(Clone, Copy, Debug)]
pub enum ExternalInput {
    Int { min: i64, max: i64 },
    Float { min: f64, max: f64 },
    Color(Color),
}
