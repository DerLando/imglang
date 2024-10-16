use crate::color::Color;
use std::{
    collections::{BTreeMap, HashMap},
    hash::Hash,
};

#[derive(Debug, Default, PartialEq)]
pub struct InputMap {
    inputs: BTreeMap<String, ExternalInput>,
}

impl InputMap {
    pub fn are_valid_inputs(&self, _inputs: &Inputs) -> anyhow::Result<()> {
        // TODO: Do exhaustive checking here :)
        Ok(())
    }

    pub fn get_inputs_sorted(&self) -> Vec<(String, ExternalInput)> {
        self.inputs
            .iter()
            .map(|(i, e)| (i.clone(), *e))
            .collect::<Vec<_>>()
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
        let mut inputs = BTreeMap::new();
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

#[derive(Clone, Copy, Debug, PartialEq)]
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

impl Hash for InputValue {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write(&format!("{:?}", self).into_bytes())
    }
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

#[derive(Debug, Default, Clone)]
pub struct Inputs {
    inputs: BTreeMap<String, InputValue>,
}

impl Inputs {
    pub(crate) fn init_from(map: &InputMap) -> Self {
        let mut inner = BTreeMap::new();
        for (ident, input) in &map.inputs {
            match input {
                ExternalInput::Int { min, max } => {
                    inner.insert(ident.clone(), InputValue::Int(*min));
                }
                ExternalInput::Float { min, max } => {
                    inner.insert(ident.clone(), InputValue::Float(*min));
                }
                ExternalInput::Color(_) => todo!(),
            }
        }
        Self { inputs: inner }
    }
    pub(crate) fn get(&self, key: &str) -> Option<&InputValue> {
        self.inputs.get(key)
    }
    pub(crate) fn get_int_mut(&mut self, key: &str) -> Option<&mut i64> {
        self.inputs
            .get_mut(key)
            .filter(|v| matches!(v, InputValue::Int(_)))
            .map(|v| match v {
                InputValue::Int(n) => n,
                _ => unreachable!(),
            })
    }
    pub(crate) fn get_float_mut(&mut self, key: &str) -> Option<&mut f64> {
        self.inputs
            .get_mut(key)
            .filter(|v| matches!(v, InputValue::Float(_)))
            .map(|v| match v {
                InputValue::Float(f) => f,
                _ => unreachable!(),
            })
    }
}

impl Hash for Inputs {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for value in self.inputs.values() {
            value.hash(state);
        }
    }
}

pub(crate) struct InputsHasher<'a> {
    code: &'a str,
    inputs: &'a Inputs,
}

impl<'a> InputsHasher<'a> {
    pub fn make_hash<H: std::hash::Hasher>(
        code: &'a str,
        inputs: &'a Inputs,
        state: &mut H,
    ) -> u64 {
        let hasher = Self { code, inputs };
        hasher.hash(state);
        state.finish()
    }
}

impl<'a> Hash for InputsHasher<'a> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.code.hash(state);
        self.inputs.hash(state);
    }
}
