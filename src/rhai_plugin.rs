use rhai::plugin::*;

use crate::color::Color;
use crate::input::ExternalInput;

#[derive(Clone, Debug)]
pub struct Context {
    pub(crate) canvas_width: i64,
    pub(crate) canvas_height: i64,
    pub(crate) shapes: Vec<(Shape, Stroke)>,
}

#[derive(Clone, Copy, Debug)]
pub struct Stroke {
    pub(crate) width: f64,
    pub(crate) color: Color,
}

#[derive(Clone, Copy, Debug)]
pub enum Shape {
    Circle {
        geometry: Circle,
        transform: Transform,
    },
}

impl Shape {
    // TODO: Split geometry and transform out of the enum
    // so we don't need to match here, it's awkward!
    pub(crate) fn pre_transform(&mut self, pre: Transform) {
        match self {
            Shape::Circle {
                geometry,
                transform,
            } => *transform = pre * *transform,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Circle {
    pub(crate) radius: f64,
}

#[derive(Clone, Copy, Debug)]
pub struct Transform {
    pub(crate) inner: piet::kurbo::Affine,
}

impl Transform {
    pub const IDENTITY: Transform = Self {
        inner: piet::kurbo::Affine::IDENTITY,
    };

    pub(crate) fn get_pre_transform(canvas_width: f64, canvas_height: f64) -> Self {
        let translation = piet::kurbo::Affine::translate((canvas_width / 2.0, canvas_height / 2.0));
        let mirror = piet::kurbo::Affine::FLIP_Y;

        Self {
            inner: translation * mirror,
        }
    }

    pub fn translate(x: f64, y: f64) -> Self {
        Self {
            inner: piet::kurbo::Affine::translate((x, y)),
        }
    }
}

impl std::ops::Mul<Transform> for Transform {
    type Output = Transform;

    fn mul(self, rhs: Transform) -> Self::Output {
        Transform {
            inner: self.inner * rhs.inner,
        }
    }
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
        Shape::Circle {
            geometry: Circle { radius },
            transform: Transform::IDENTITY,
        }
    }

    pub fn circle_at(radius: f64, x: f64, y: f64) -> Shape {
        Shape::Circle {
            geometry: Circle { radius },
            transform: Transform {
                inner: piet::kurbo::Affine::translate((x, y)),
            },
        }
    }

    pub fn translation(x: f64, y: f64) -> Transform {
        Transform::translate(x, y)
    }

    pub fn transform(shape: Shape, transform: Transform) -> Shape {
        match shape {
            Shape::Circle { geometry, .. } => Shape::Circle {
                geometry,
                transform,
            },
        }
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
