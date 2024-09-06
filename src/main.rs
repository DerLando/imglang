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

    let script = std::fs::read_to_string("test_script.rhai")?;

    // TODO: Rather spawn a thread to watch the file location for changes and re-run the script/drawing

    // TODO: Implement the external inputs and store them in the context

    // TODO: Use the context to somehow create a web UI with sliders for inputs

    // TODO: Probably need to wrap all of that into a simple server that serves a text editor together with a canvas + sliders

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
