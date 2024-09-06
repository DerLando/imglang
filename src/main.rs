use anyhow::Error;
use rhai::{exported_module, Engine};

use crate::context_artist::draw_context;

mod ast;
mod context_artist;
mod interpret;
mod parse;
mod rhai_plugin;
mod stdlib;
fn main() -> Result<(), anyhow::Error> {
    println!("Hello, world!");
    let mut engine = Engine::new();
    let std_module = exported_module!(rhai_plugin::imgstd);
    engine.register_global_module(std_module.into());

    println!("{:?}", engine.eval::<rhai_plugin::Shape>("circle(50.0)"));

    let script = r#"
    let canvas = canvas_width_height(400, 300);
    let stroke = stroke(2.0, BLACK);
    let circle = circle(50.0);

    canvas.draw(circle, stroke);

    canvas
"#;

    let context;
    match engine.eval::<rhai_plugin::Context>(&script) {
        Ok(c) => context = c,
        Err(e) => {
            println!("{:?}", e);
            anyhow::bail!("Failed to evaluate script")
        }
    }

    let rc = draw_context(context);
    let file = std::fs::File::create("test.svg")?;
    rc.write(file)?;

    Ok(())
}
