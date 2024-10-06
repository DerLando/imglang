use std::collections::HashMap;

use anyhow::Error;
use context_artist::ImageWriter;
use input::InputValue;
use rhai::{exported_module, Engine};
use solver::Solver;

use crate::context_artist::draw_context;

mod ast;
mod color;
mod context_artist;
mod input;
mod interpret;
mod parse;
mod rhai_plugin;
mod solver;
mod stdlib;
fn main() -> Result<(), anyhow::Error> {
    let script = std::fs::read_to_string("test_script.rhai")?;

    // TODO: Rather spawn a thread to watch the file location for changes and re-run the script/drawing

    // TODO: Implement the external inputs and store them in the context

    // TODO: Use the context to somehow create a web UI with sliders for inputs

    // TODO: Probably need to wrap all of that into a simple server that serves a text editor together with a canvas + sliders

    let solver = Solver::new();

    let mut inputs: HashMap<String, InputValue> = HashMap::new();
    inputs.insert("r".to_string(), 75.0.into());
    inputs.insert("t".to_string(), 7.0.into());
    inputs.insert("n".to_string(), 8.into());
    let writer = solver.solve(&script, inputs.into())?;

    let file = std::fs::File::create("test.svg")?;
    writer.write(file)?;

    Ok(())
}
