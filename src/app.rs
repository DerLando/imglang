use crate::{
    document::Document,
    input::{InputMap, Inputs},
    solver::Solver,
};

pub(crate) struct App {
    model: Model,
}

#[derive(Default)]
pub(crate) struct Model {
    document: Document,
    solver: Solver,
    input_map: InputMap,
    inputs: Inputs,
}

// TODO: I'm not sure MVU makes sense in egui, since
// changes can be triggered directly from the UI.
// I definitely need to abstract somehow between the actual
// functionality of the backend and the UI concerns in the frontend.
// Maybe MVVM might work, but I'm not sure how to do the bindings,
// since egui is not reactive. Probably MVC would be a better fit
// since ALL of the view is replaced every frame
enum Message {}
