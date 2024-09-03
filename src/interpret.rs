use std::collections::HashMap;

use crate::{
    ast::{Ast, Value},
    parse::{Program, Token},
};

pub(crate) struct Interpreter;

struct TokenStream {
    cursor: usize,
    tokens: Vec<Token>,
}

impl TokenStream {
    pub fn consume(program: Program) -> Self {
        Self {
            cursor: 0,
            tokens: program.tokens,
        }
    }

    pub fn next(&mut self) -> Option<&Token> {
        if self.cursor == self.tokens.len() - 1 {
            None
        } else {
            let value = &self.tokens[self.cursor];
            self.cursor += 1;
            Some(value)
        }
    }

    pub fn peek(&self) -> Option<&Token> {
        if self.cursor == self.tokens.len() - 2 {
            None
        } else {
            Some(&self.tokens[self.cursor + 1])
        }
    }
}

struct Bindings<'a> {
    variables: HashMap<&'a str, Value>,
}

/// TODO: Return type would be a result
/// actually, but I don't want to define
/// the error type yet...
pub fn interpret(ast: Ast) -> Option<()> {
    todo!()
}

pub fn interpret_tokens(tokens: &mut TokenStream) -> Option<()> {
    while let Some(token) = tokens.next() {
        match token {
            Token::Let => todo!(),
            Token::Extern => todo!(),
            Token::Assign => todo!(),
            Token::Ident(_) => todo!(),
            Token::Pipe => todo!(),
            Token::Literal(_) => todo!(),
            Token::Indent => todo!(),
            Token::Comment => todo!(),
        }
    }
    todo!()
}

fn has_token_chain(tokens: &TokenStream, chain: Vec<Token>) -> bool {
    let mut chain = chain.into_iter();
    while let Some(chain_token) = chain.next() {
        if tokens
            .peek()
            .is_some_and(|t| std::mem::discriminant(t) == std::mem::discriminant(&chain_token))
        {
            continue;
        }
        return false;
    }

    return true;
}

fn try_interpret_let_binding(tokens: &mut TokenStream, bindings: &mut Bindings) -> Option<()> {
    if has_token_chain(tokens, vec![Token::Ident(String::new()), Token::Assign]) {
        // TODO: Parse the chain here?
        todo!()
    }
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::parse;

    use super::*;

    #[test]
    fn test_raw_interpret_simple() {
        let mut program = "let r = externInt 10 20";
        let mut program = parse::full_program(&mut program);
        let mut program = program.unwrap();

        interpret_tokens(&mut TokenStream::consume(program));
    }
}
