use crate::color::Color;
use std::collections::HashMap;

#[derive(Debug)]
pub struct InputMap {
    inputs: HashMap<String, ExternalInput>,
}

impl From<&rhai::AST> for InputMap {
    fn from(value: &rhai::AST) -> Self {
        todo!()
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ExternalInput {
    Int { min: i64, max: i64 },
    Float { min: f64, max: f64 },
    Color(Color),
}

#[derive(Clone, Copy, Debug)]
pub enum InputValue {
    Int(i64),
    Float(f64),
    Color(Color),
}

impl From<i64> for InputValue {
    fn from(value: i64) -> Self {
        Self::Int(value)
    }
}

impl From<f64> for InputValue {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}

#[derive(Debug)]
pub struct Inputs {
    pub(crate) inputs: HashMap<String, InputValue>,
}

impl From<HashMap<String, InputValue>> for Inputs {
    fn from(value: HashMap<String, InputValue>) -> Self {
        Self { inputs: value }
    }
}
