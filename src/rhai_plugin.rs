use rhai::plugin::*;

#[derive(Clone, Debug)]
pub struct Context {
    canvas_width: i64,
    canvas_height: i64,
    shapes: Vec<(Shape, Stroke)>,
}

#[derive(Clone, Copy, Debug)]
pub enum Color {
    BLACK,
    WHITE,
    RED,
}

#[derive(Clone, Copy, Debug)]
pub struct Stroke {
    width: f64,
    color: Color,
}

#[derive(Clone, Copy, Debug)]
pub enum Shape {
    Circle(Circle),
}

#[derive(Clone, Copy, Debug)]
pub struct Circle {
    radius: f64,
}

#[export_module]
pub mod imgstd {
    pub type Canvas = Context;

    pub fn canvas_width_height(width: i64, height: i64) -> Canvas {
        Canvas {
            canvas_width: width,
            canvas_height: height,
            shapes: Vec::new(),
        }
    }

    pub fn stroke(width: f64, color: Color) -> Stroke {
        Stroke { width, color }
    }

    pub fn circle(radius: f64) -> Shape {
        Shape::Circle(Circle { radius })
    }

    pub fn draw(canvas: &mut Canvas, shape: Shape, stroke: Stroke) {
        canvas.shapes.push((shape, stroke))
    }

    pub const BLACK: Color = Color::BLACK;

    pub fn out(canvas: Canvas) -> Vec<(Shape, Stroke)> {
        canvas.shapes
    }
}
