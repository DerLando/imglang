use std::collections::HashMap;

use anyhow::Error;
use context_artist::ImageWriter;
use egui::{Label, Slider};
use egui_code_editor::{CodeEditor, ColorTheme, Syntax};
use input::{InputMap, InputValue, Inputs};
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

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc)))),
    );
}

#[derive(Default)]
struct MyEguiApp {
    code: String,
    solver: Solver,
    input_map: InputMap,
    inputs: Inputs,
}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        let mut initial = Self::default();
        let script = std::fs::read_to_string("test_script.rhai").unwrap();
        initial.code = script;

        initial
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            CodeEditor::default()
                .id_source("code editor")
                .with_rows(12)
                .with_fontsize(14.0)
                .with_theme(ColorTheme::GRUVBOX)
                .with_syntax(Syntax::rust())
                .with_numlines(true)
                .show(ui, &mut self.code);

            egui::SidePanel::right("canvas")
                .show_separator_line(true)
                .show(ctx, |ui| {
                    if let Ok(input_map) = InputMap::try_from(self.code.as_str()) {
                        if input_map != self.input_map {
                            self.inputs = Inputs::init_from(&input_map);
                            self.input_map = input_map;
                        }
                    }
                    for (ident, input) in self.input_map.get_inputs_sorted() {
                        ui.add(Label::new(&ident));
                        match input {
                            input::ExternalInput::Int { min, max } => {
                                ui.add(Slider::new(
                                    self.inputs.get_int_mut(&ident).unwrap(),
                                    min..=max,
                                ));
                            }
                            input::ExternalInput::Float { min, max } => {
                                ui.add(
                                    Slider::new(
                                        self.inputs.get_float_mut(&ident).unwrap(),
                                        min..=max,
                                    )
                                    .fixed_decimals(2),
                                );
                            }
                            input::ExternalInput::Color(_) => todo!(),
                        }
                    }
                });
        });
    }
}

// fn main() -> Result<(), anyhow::Error> {

//     let script = std::fs::read_to_string("test_script.rhai")?;

//     // TODO: Use the input map to somehow create a UI with sliders for inputs

//     // TODO: Probably need to wrap all of that into a simple server that serves a text editor together with a canvas + sliders

//     // TODO: Alternatively an egui solution might work for now, I found
//     // a crate that offers a code-editor widget for it, which was
//     // the main reason not to use a native GUI for now

//     // TODO: Add a new crate for caching script evaluation results
//     // by a hash of input script and Inputs given for solving.
//     // Important for a proper web server in the end, so it does
//     // less work and can just serve results from cache for hot paths

//     let solver = Solver::new();

//     let mut inputs: HashMap<String, InputValue> = HashMap::new();
//     inputs.insert("r".to_string(), 75.0.into());
//     inputs.insert("t".to_string(), 7.0.into());
//     inputs.insert("n".to_string(), 8.into());
//     let writer = solver.solve(&script, inputs.into())?;

//     let file = std::fs::File::create("test.svg")?;
//     writer.write(file)?;

//     Ok(())
// }
