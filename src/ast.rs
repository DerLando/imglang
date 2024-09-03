use crate::parse::Token;

pub(crate) enum Ast {
    // All functions are unary ops,
    // as they are partially applied
    // for each argument
    // Note: Partial application skipped for now to be more pragramtic
    FunctionCall { name: String, arg: Vec<Value> },
    // Binding a value to a name
    // eg let x = 2
    Binding { name: String, arg: Value },
    // A value name
    Ident(String),
    // A value itself, which must come from an Ast
    Value(Value),
    Const(Const),
}

pub(crate) type Value = Box<Ast>;
pub(crate) enum Const {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    // The name of the function
    Function(String),
}

pub(crate) fn parse(tokens: &mut impl Iterator<Item = Token>) -> Option<Ast> {
    let mut ast: Ast;
    while let Some(token) = tokens.next() {}
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_api() {
        // let circle = circleRadius 10
        let fn_call_ast = Ast::FunctionCall {
            name: "circleRadius".to_string(),
            arg: vec![Box::new(Ast::Const(Const::Int(10)))],
        };
        let ast = Ast::Binding {
            name: "circle".to_string(),
            arg: Box::new(fn_call_ast),
        };

        // TODO: Pass in a function table (circleRadius is built-in)
        // and execute the ast.
    }

    #[test]
    fn test_binary_fn_api() {
        // let r = externInt 10 20
        let fn_partial_ast = Ast::FunctionCall {
            name: "externInt".to_string(),
            arg: vec![
                Box::new(Ast::Const(Const::Int(10))),
                Box::new(Ast::Const(Const::Int(20))),
            ],
        };

        let binding = Ast::Binding {
            name: "r".to_string(),
            arg: Box::new(fn_partial_ast),
        };
    }
}
