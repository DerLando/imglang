use crate::imgstd::{self, Context};
use piet::{kurbo, RenderContext};

pub fn into_piet(shape: imgstd::Shape) -> impl kurbo::Shape {
    match shape.geometry() {
        imgstd::Geometry::Circle(circle) => {
            let circle = kurbo::Circle::new((0.0, 0.0), circle.radius);
            shape.transform().inner * circle
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

pub(crate) fn draw_context_to_svg(context: imgstd::Context) -> SvgImageWriter {
    let width = context.canvas_width as f64;
    let height = context.canvas_height as f64;
    let mut canvas = piet_svg::RenderContext::new(kurbo::Size { width, height });

    let pre = crate::imgstd::Transform::get_pre_transform(width, height);

    canvas.fill(
        piet::kurbo::Rect::new(0.0, 0.0, width, height),
        &piet::Color::WHITE,
    );
    // canvas.clip(piet::kurbo::Rect::new(0.0, 0.0, width / 4.0, height / 4.0));
    for (mut shape, stroke) in context.shapes.into_iter() {
        shape.pre_transform(pre);
        let shape = into_piet(shape);
        canvas.stroke(
            shape,
            &piet::PaintBrush::Color(stroke.color.into()),
            stroke.width,
        )
    }

    SvgImageWriter { rc: canvas }
}

// pub(crate) fn draw_context_to_png(context: rhai_plugin::Context) -> CairoImageWriter {
//     let width = context.canvas_width as f64;
//     let height = context.canvas_height as f64;
//     let mut canvas = piet_cairo::RenderContext::new(kurbo::Size { width, height });

//     canvas.clear(None, piet::Color::WHITE);
//     canvas.clip(piet::kurbo::Rect::new(0.0, 0.0, width / 4.0, height / 4.0));
//     for (shape, stroke) in context.shapes.into_iter() {
//         let shape = into_piet(shape);
//         canvas.stroke(
//             shape,
//             &piet::PaintBrush::Color(stroke.color.into()),
//             stroke.width,
//         )
//     }

//     CairoImageWriter { rc: canvas }
// }

pub trait ImageWriter {
    fn write(&self, writer: &mut impl std::io::Write) -> std::io::Result<()>;
    fn width(&self) -> u64;
    fn height(&self) -> u64;
}

pub(crate) struct SvgImageWriter {
    rc: piet_svg::RenderContext,
}

impl SvgImageWriter {
    const CLIPPING_HEADER: &str = r#"
<svg xmlns="http://www.w3.org/2000/svg" clip-path="url(#cut-off-bottom)">
  <defs>
    <clipPath id="cut-off-bottom">
      <rect x="0" y="0" width="{}" height="{}" />
    </clipPath>
  </defs>
        "#;

    const CLIPPING_FOOTER: &str = "</svg>";

    fn get_clipping_header(&self) -> String {
        format!(
            r#"
<svg xmlns="http://www.w3.org/2000/svg" clip-path="url(#cut-off-bottom)">
  <defs>
    <clipPath id="cut-off-bottom">
      <rect x="0" y="0" width="{}" height="{}" />
    </clipPath>
  </defs>
        "#,
            self.width(),
            self.height()
        )
    }
}

impl ImageWriter for SvgImageWriter {
    fn write(&self, writer: &mut impl std::io::Write) -> std::io::Result<()> {
        writer.write_all(self.get_clipping_header().as_bytes())?;
        self.rc.write(&mut *writer)?; // reborrow to avoid moving into write
        writer.write_all(Self::CLIPPING_FOOTER.as_bytes())?;
        Ok(())
    }

    fn width(&self) -> u64 {
        self.rc.size().width as u64
    }

    fn height(&self) -> u64 {
        self.rc.size().height as u64
    }
}

// pub(crate) struct CairoImageWriter {
//     rc: piet_cairo::RenderContext,
// }

// impl ImageWriter for CairoImageWriter {
//     fn write(&self, writer: impl std::io::Write) -> std::io::Result<()> {
//         self.rc.write(writer)
//     }
// }
