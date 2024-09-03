use rhai::{exported_module, Engine};

mod ast;
mod interpret;
mod parse;
mod rhai_plugin;
mod stdlib;
fn main() {
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

    println!("{:?}", engine.eval::<rhai_plugin::Context>(&script));
}
