use rhai::{exported_module, Engine};

use crate::{
    context_artist::{draw_context_to_svg, ImageWriter},
    imgstd,
    input::Inputs,
};

pub struct Solver {
    engine: rhai::Engine,
}

impl Default for Solver {
    fn default() -> Self {
        Self::new()
    }
}

impl Solver {
    fn init_engine() -> rhai::Engine {
        let mut engine = Engine::new();
        let std_module = exported_module!(imgstd::stdexport);
        engine.register_global_module(std_module.into());
        engine
    }

    pub fn new() -> Self {
        Self {
            engine: Self::init_engine(),
        }
    }

    pub fn solve(&mut self, script: &str, inputs: Inputs) -> anyhow::Result<impl ImageWriter> {
        // TODO: Get InputMap for script and check if the
        // inputs are valid and in bounds. Error out if not
        // let input_map = InputMap::try_from(script)?;
        // let _ = input_map.are_valid_inputs(&inputs)?;

        // register resolver for inputs
        self.engine.on_var(move |name, _index, _context| {
            if let Some(value) = inputs.get(name) {
                match value {
                    crate::input::InputValue::Int(v) => Ok(Some((*v).into())),
                    crate::input::InputValue::Float(v) => Ok(Some((*v).into())),
                    crate::input::InputValue::Color(_) => todo!(),
                }
            } else {
                Ok(None)
            }
        });

        let context = self.engine.eval::<imgstd::Context>(script)?;
        let rc = draw_context_to_svg(context);
        Ok(rc)
    }
}
