use crate::color::Color;
use std::collections::HashMap;

#[derive(Debug)]
pub struct InputMap {
    inputs: HashMap<String, ExternalInput>,
}

impl InputMap {
    pub fn are_valid_inputs(&self, inputs: &Inputs) -> anyhow::Result<()> {
        // TODO: Do exhaustive checking here :)
        Ok(())
    }
}

fn input_from_stmt(stmt: &rhai::Stmt) -> Option<(String, ExternalInput)> {
    if let rhai::Stmt::Var(info, _, _) = stmt {
        let ident = info.0.clone();
        let expr = info.1.clone();

        // TODO: Too many unwraps here, properly check invariants!
        if let rhai::Expr::FnCall { 0: inner, 1: _ } = expr {
            if inner.name == "extern" {
                Some((
                    ident.name.into(),
                    input_from_arg_exprs([&inner.args[0], &inner.args[1]]),
                ))
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    }
}

fn input_from_arg_exprs(args: [&rhai::Expr; 2]) -> ExternalInput {
    let left_arg = args[0].get_literal_value().unwrap();
    let right_arg = args[1].get_literal_value().unwrap();

    // TODO: More rigid type checking!
    if left_arg.is_int() {
        return ExternalInput::Int {
            min: left_arg.as_int().unwrap(),
            max: right_arg.as_int().unwrap(),
        };
    }
    if left_arg.is_float() {
        return ExternalInput::Float {
            min: left_arg.as_float().unwrap(),
            max: right_arg.as_float().unwrap(),
        };
    }

    unimplemented!("Only int and float inputs supported yet!");
}

impl From<rhai::AST> for InputMap {
    fn from(value: rhai::AST) -> Self {
        let mut inputs = HashMap::new();
        for (ident, input) in value
            .statements()
            .iter()
            .filter_map(|stmt| input_from_stmt(stmt))
        {
            inputs.insert(ident, input);
        }

        Self { inputs }
    }
}

impl TryFrom<&str> for InputMap {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let engine = rhai::Engine::new();
        let ast = engine.compile(value)?;
        Ok(ast.into())
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
