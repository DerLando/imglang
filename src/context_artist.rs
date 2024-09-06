use crate::rhai_plugin::{self, Context};
use piet::{kurbo, RenderContext};

pub fn into_piet(shape: rhai_plugin::Shape) -> impl kurbo::Shape {
    match shape {
        rhai_plugin::Shape::Circle(circle) => kurbo::Circle::new((0.0, 0.0), circle.radius),
    }
}

impl From<rhai_plugin::Color> for piet::Color {
    fn from(value: rhai_plugin::Color) -> Self {
        match value {
            rhai_plugin::Color::BLACK => piet::Color::BLACK,
            rhai_plugin::Color::WHITE => piet::Color::WHITE,
            rhai_plugin::Color::RED => piet::Color::RED,
        }
    }
}

pub fn draw_context(context: rhai_plugin::Context) -> piet_svg::RenderContext {
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

    canvas
}
