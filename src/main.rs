use std::hash::{BuildHasher, BuildHasherDefault};

use context_artist::ImageWriter;
use document::Document;
use egui::{CollapsingHeader, Label, Slider};
use egui_code_editor::{CodeEditor, ColorTheme, Syntax};
use input::{InputMap, Inputs, InputsHasher};
use solver::Solver;

mod app;
mod ast;
mod color;
mod context_artist;
mod document;
mod imgstd;
mod input;
mod interpret;
mod parse;
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

/// TODO: I need to abstract here to be able to handle view-specific
/// concerns, like f.e. hiding the side panel, maximizing a window, etc
#[derive(Default)]
struct MyEguiApp {
    document: Document,
    solver: Solver,
    input_map: InputMap,
    inputs: Inputs,
    last_inputs_hash: u64,
}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        let mut initial = Self::default();
        let script = std::fs::read_to_string("test_script.rhai").unwrap();
        *initial.document.content_mut() = script;

        initial
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui_extras::install_image_loaders(ctx);
        egui::TopBottomPanel::top("menu").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open").clicked() {
                        if let Some(path) = rfd::FileDialog::new()
                            .add_filter("imglang file", &["rhai"])
                            .pick_file()
                        {
                            if let Ok(document) = Document::open(&path) {
                                self.document = document;
                            } else {
                                // Do nothing
                            }
                        } else {
                            // Do nothing :)
                        }
                    };
                    ui.add_enabled_ui(self.document.can_save(), |ui| {
                        if ui.button("Save").clicked() {
                            self.document.save();
                        }
                    });
                    if ui.button("Save As").clicked() {
                        if let Some(path) = rfd::FileDialog::new()
                            .add_filter("imglang file", &["rhai"])
                            .save_file()
                        {
                            self.document.set_path(&path);
                            self.document.save();
                        } else {
                            // Do nothing :)
                        }
                    }
                    if ui.button("Save Screenshot").clicked() {
                        if let Ok(writer) = self
                            .solver
                            .solve(self.document.content(), self.inputs.clone())
                        {
                            if let Some(path) = rfd::FileDialog::new()
                                .add_filter("svg file", &["svg"])
                                .save_file()
                            {
                                if let Ok(mut file) = std::fs::File::create(path) {
                                    writer.write(&mut file);
                                }
                            } else {
                                // Do nothing :)
                            }
                        } else {
                            // Do nothing :)
                        }
                    }
                })
            })
        });

        egui::SidePanel::right("canvas")
            .show_separator_line(true)
            .show(ctx, |ui| {
                if let Ok(input_map) = InputMap::try_from(self.document.content()) {
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
                                Slider::new(self.inputs.get_float_mut(&ident).unwrap(), min..=max)
                                    .fixed_decimals(2),
                            );
                        }
                        input::ExternalInput::Color(_) => todo!(),
                    }
                }
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            CollapsingHeader::new("code")
                .default_open(true)
                .show(ui, |ui| {
                    CodeEditor::default()
                        .id_source("code editor")
                        .with_rows(12)
                        .with_fontsize(14.0)
                        .with_theme(ColorTheme::GRUVBOX)
                        .with_syntax(Syntax::rust())
                        .with_numlines(true)
                        .show(ui, self.document.content_mut());
                });
            let hasher = BuildHasherDefault::<egui::ahash::AHasher>::default();
            let hash = InputsHasher::make_hash(
                self.document.content(),
                &self.inputs,
                &mut hasher.build_hasher(),
            );
            if self.last_inputs_hash == hash {
                self.solver.advance_time();
                // TODO: Figure out if repaint only the image is possib
                ctx.request_repaint();
            } else {
                self.solver.reset_time();
                self.last_inputs_hash = hash;
            }
            match self
                .solver
                .solve(self.document.content(), self.inputs.clone())
            {
                Ok(writer) => {
                    let mut buffer: Vec<u8> = Vec::new();
                    if let Ok(_) = writer.write(&mut buffer) {
                        let image = egui::Image::from_bytes(
                            format!("bytes://{}_{}.svg", hash, self.solver.get_time()),
                            buffer,
                        )
                        .max_width((writer.width() * 2) as f32)
                        .max_height((writer.height() * 2) as f32);
                        ui.add(image);
                    }
                }
                Err(e) => {
                    ui.add(Label::new(format!("{:?}", e)));
                }
            }
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
