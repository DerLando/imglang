use rhai::export_module;
use rhai::plugin::*;
use style::Stroke;
mod geometry;
mod shape;
mod style;
mod transform;

pub(crate) use geometry::Geometry;
pub(crate) use shape::Shape;
pub(crate) use transform::Transform;

#[derive(Clone, Debug)]
pub struct Context {
    pub(crate) canvas_width: i64,
    pub(crate) canvas_height: i64,
    pub(crate) shapes: Vec<(Shape, Stroke)>,
}

#[export_module]
pub mod stdexport {
    use geometry::{Circle, Geometry};
    use shape::Shape;
    use style::Stroke;
    use transform::Transform;

    use crate::{color::Color, input::ExternalInput};

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
        Shape::with_geometry(Geometry::Circle(Circle { radius }))
    }

    pub fn circle_at(radius: f64, x: f64, y: f64) -> Shape {
        Shape::new(
            Geometry::Circle(Circle { radius }),
            Transform {
                inner: piet::kurbo::Affine::translate((x, y)),
            },
        )
    }

    pub fn translation(x: f64, y: f64) -> Transform {
        Transform::translate(x, y)
    }

    pub fn transform(shape: Shape, transform: Transform) -> Shape {
        Shape::new(shape.geometry, transform)
    }

    pub fn draw(canvas: &mut Canvas, shape: Shape, stroke: Stroke) {
        canvas.shapes.push((shape, stroke))
    }

    pub const BLACK: Color = Color::BLACK;

    pub fn out(canvas: Canvas) -> Vec<(Shape, Stroke)> {
        canvas.shapes
    }

    #[rhai_fn(name = "extern")]
    pub fn extern_int(min: i64, max: i64) -> ExternalInput {
        ExternalInput::Int { min, max }
    }

    #[rhai_fn(name = "extern")]
    pub fn extern_float(min: f64, max: f64) -> ExternalInput {
        ExternalInput::Float { min, max }
    }
}
