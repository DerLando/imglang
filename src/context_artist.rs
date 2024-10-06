use crate::rhai_plugin::{self, Context};
use piet::{kurbo, RenderContext};

pub fn into_piet(shape: rhai_plugin::Shape) -> impl kurbo::Shape {
    match shape {
        rhai_plugin::Shape::Circle {
            geometry,
            transform,
        } => {
            let circle = kurbo::Circle::new((0.0, 0.0), geometry.radius);
            transform.inner * circle
        }
    }
}

impl From<crate::color::Color> for piet::Color {
    fn from(value: crate::color::Color) -> Self {
        match value {
            crate::color::Color::BLACK => piet::Color::BLACK,
            crate::color::Color::WHITE => piet::Color::WHITE,
            crate::color::Color::RED => piet::Color::RED,
        }
    }
}

pub(crate) fn draw_context(context: rhai_plugin::Context) -> ContextImageWriter {
    let mut canvas = piet_svg::RenderContext::new(kurbo::Size {
        width: context.canvas_width as f64,
        height: context.canvas_height as f64,
    });

    canvas.clear(None, piet::Color::WHITE);
    for (shape, stroke) in context.shapes.into_iter() {
        let shape = into_piet(shape);
        canvas.stroke(
            shape,
            &piet::PaintBrush::Color(stroke.color.into()),
            stroke.width,
        )
    }

    ContextImageWriter { rc: canvas }
}

pub trait ImageWriter {
    fn write(&self, writer: impl std::io::Write) -> std::io::Result<()>;
}

pub(crate) struct ContextImageWriter {
    rc: piet_svg::RenderContext,
}

impl ImageWriter for ContextImageWriter {
    fn write(&self, writer: impl std::io::Write) -> std::io::Result<()> {
        self.rc.write(writer)
    }
}
