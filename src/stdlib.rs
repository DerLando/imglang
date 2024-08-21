pub fn circle_origin_radius(radius: f64) -> piet::kurbo::Circle {
    todo!()
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct ExternIntRange {
    min: i64,
    max: i64,
}

pub fn extern_int(min: i64, max: i64) -> ExternIntRange {
    return ExternIntRange { min, max };
}

/// This is partially applied, I think
pub fn extern_int_test(min: i64) -> impl FnOnce(i64) -> ExternIntRange {
    move |max| ExternIntRange { min, max }
}

pub struct Canvas {
    width: u64,
    height: u64,
}

impl Canvas {
    pub fn new(width: u64, height: u64) -> Self {
        Self { width, height }
    }
}

pub enum Color {
    Black,
    White,
}
pub fn canvas_width_height(width: u64, height: u64) -> Canvas {
    Canvas::new(width, height)
}

pub struct Stroke {
    color: Color,
    stroke_width: f64,
}

impl Stroke {
    pub fn from_color_and_width(color: Color, stroke_width: f64) -> Self {
        Self {
            color,
            stroke_width,
        }
    }
}
pub fn stroke(color: Color, stroke_width: f64) -> Stroke {
    Stroke::from_color_and_width(color, stroke_width)
}

pub fn draw(path: &piet::kurbo::Circle, stroke: &Stroke, canvas: &Canvas) -> Canvas {
    // Need to create a new canvas everytime
    todo!()
}

pub fn out(canvas: Canvas) {
    todo!()
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_partial_application() {
        let partial = extern_int_test(-10);
        let result = partial(10);

        assert_eq!(ExternIntRange { min: -10, max: 10 }, result);
    }
}
