use winnow::{
    ascii::alphanumeric1,
    error::{ContextError, ErrMode, ParserError},
    token::one_of,
    PResult, Parser,
};

#[derive(Debug, PartialEq)]
pub(crate) enum Token {
    /// let
    Let,
    /// extern
    Extern,
    /// =
    Assign,
    /// any identifier
    Ident(String),
    /// |>
    Pipe,
    /// A literal value
    Literal(Literal),
    /// 4 spaces
    Indent,
    Comment,
}

/// A literal value
#[derive(Debug, PartialEq)]
pub(crate) enum Literal {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
}

pub(crate) fn extern_keyword(input: &mut &str) -> PResult<Token> {
    let actual = "extern".parse_next(input)?;
    Ok(Token::Extern)
}

pub(crate) fn let_keyword(input: &mut &str) -> PResult<Token> {
    let actual = "let".parse_next(input)?;
    Ok(Token::Let)
}

pub(crate) fn assign(input: &mut &str) -> PResult<Token> {
    '='.parse_next(input).map(|_| Token::Assign)
}

pub(crate) fn ident(input: &mut &str) -> PResult<Token> {
    alphanumeric1(input).map(|name| Token::Ident(name.to_string()))
}

pub(crate) fn comment(input: &mut &str) -> PResult<Token> {
    let comment_start = "//".parse_next(input)?;
    winnow::ascii::till_line_ending(input).map(|_| Token::Comment)
}

pub(crate) fn string_literal(input: &mut &str) -> PResult<Token> {
    let start = '"'.take().parse_next(input)?;
    let name = alphanumeric1(input)?;
    let end = '"'.take().parse_next(input)?;
    Ok(Token::Literal(Literal::String(name.to_string())))
}

pub(crate) fn bool_literal(input: &mut &str) -> PResult<Token> {
    winnow::combinator::alt(("true", "false"))
        .parse_next(input)
        .map(|v| match v {
            "true" => Token::Literal(Literal::Bool(true)),
            "false" => Token::Literal(Literal::Bool(false)),
            _ => unreachable!(),
        })
}

pub(crate) fn int_literal(input: &mut &str) -> PResult<Token> {
    let n = winnow::ascii::dec_int(input)?;

    match input.chars().next() {
        Some('.') | Some('e') | Some('E') => Err(ErrMode::Backtrack(ContextError::new())),
        _ => Ok(Token::Literal(Literal::Int(n))),
    }
}

pub(crate) fn float_literal(input: &mut &str) -> PResult<Token> {
    winnow::ascii::float(input).map(|f| Token::Literal(Literal::Float(f)))
}

pub(crate) fn literal(input: &mut &str) -> PResult<Token> {
    winnow::combinator::alt((bool_literal, int_literal, float_literal, string_literal))
        .parse_next(input)
}

pub(crate) fn indent(input: &mut &str) -> PResult<Token> {
    "    ".parse_next(input).map(|_| Token::Indent)
}

pub(crate) fn pipe(input: &mut &str) -> PResult<Token> {
    "|>".parse_next(input).map(|_| Token::Pipe)
}

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace, returning the output of `inner`.
fn ws0<'a, F, O, E: ParserError<&'a str>>(inner: F) -> impl Parser<&'a str, O, E>
where
    F: Parser<&'a str, O, E>,
{
    winnow::combinator::terminated(inner, winnow::ascii::multispace0)
}

fn ws1<'a, F, O, E: ParserError<&'a str>>(inner: F) -> impl Parser<&'a str, O, E>
where
    F: Parser<&'a str, O, E>,
{
    winnow::combinator::terminated(inner, winnow::ascii::multispace1)
}

fn strip_start(input: &mut &str) -> PResult<()> {
    fn stripper<'a>(input: &mut &'a str) -> PResult<&'a str> {
        winnow::combinator::alt((winnow::ascii::line_ending, winnow::ascii::multispace1))
            .parse_next(input)
    }
    while let Ok(_res) = stripper(input) {
        // continue
    }

    Ok(())
}

#[derive(Debug)]
pub(crate) struct Program {
    pub(crate) tokens: Vec<Token>,
}
pub(crate) fn full_program(input: &mut &str) -> PResult<Program> {
    // a line always starts with an indent, a let binding, or pipe (I think)
    let mut tokens = Vec::new();

    strip_start(input);

    while let Ok(token) = ws0(winnow::combinator::alt((
        comment,
        indent,
        ws1(let_keyword),
        ws1(extern_keyword),
        ws1(pipe),
        ws0(literal),
        assign,
        ws0(ident),
    )))
    .parse_next(input)
    {
        tokens.push(token);
    }

    Ok(Program { tokens })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_literals() {
        let mut raw = r#""Hello""#;
        assert_eq!(
            Token::Literal(Literal::String("Hello".to_string())),
            literal(&mut raw).unwrap()
        );

        let mut raw = "0.1234";
        assert_eq!(
            Token::Literal(Literal::Float(0.1234)),
            literal(&mut raw).unwrap()
        );

        let mut raw = "-12";
        assert_eq!(
            Token::Literal(Literal::Int(-12)),
            literal(&mut raw).unwrap()
        );

        let mut raw = "true";
        assert_eq!(
            Token::Literal(Literal::Bool(true)),
            literal(&mut raw).unwrap()
        );
    }

    #[test]
    fn test_input_declaration() {
        let mut raw = "let extern r = externInt 10 15";
        // TODO: parse the string somehow?
        let program = full_program(&mut raw).unwrap();

        assert_eq!(7, program.tokens.len());
        assert_eq!(Token::Let, program.tokens[0]);
        assert_eq!(Token::Extern, program.tokens[1]);
        assert_eq!(Token::Ident("r".to_string()), program.tokens[2]);
        assert_eq!(Token::Assign, program.tokens[3]);
        assert_eq!(Token::Ident("externInt".to_string()), program.tokens[4]);
        assert_eq!(Token::Literal(Literal::Int(10)), program.tokens[5]);
        assert_eq!(Token::Literal(Literal::Int(15)), program.tokens[6]);
    }

    #[test]
    fn test_declaration() {
        let mut raw = "let canvas = canvasWidthHeight 300 400";
        // TODO: parse the string somehow?
        let program = full_program(&mut raw).unwrap();
        let mut tokens = program.tokens.into_iter();

        assert_eq!(6, tokens.len());
        assert_eq!(Token::Let, tokens.next().unwrap());
        assert_eq!(Token::Ident("canvas".to_string()), tokens.next().unwrap());
        assert_eq!(Token::Assign, tokens.next().unwrap());
        assert_eq!(
            Token::Ident("canvasWidthHeight".to_string()),
            tokens.next().unwrap()
        );
        assert_eq!(Token::Literal(Literal::Int(300)), tokens.next().unwrap());
        assert_eq!(Token::Literal(Literal::Int(400)), tokens.next().unwrap());
    }

    #[test]
    fn test_pipe() {
        let mut raw = r#"
  canvas
  |> draw circle circleStroke
  |> out
        "#;
        // TODO: parse the string somehow?
        let program = full_program(&mut raw).unwrap();
        let mut tokens = program.tokens.into_iter();

        assert_eq!(7, tokens.len());
        assert_eq!(Token::Ident("canvas".to_string()), tokens.next().unwrap());
        assert_eq!(Token::Pipe, tokens.next().unwrap());
        assert_eq!(Token::Ident("draw".to_string()), tokens.next().unwrap());
        assert_eq!(Token::Ident("circle".to_string()), tokens.next().unwrap());
        assert_eq!(
            Token::Ident("circleStroke".to_string()),
            tokens.next().unwrap()
        );
        assert_eq!(Token::Pipe, tokens.next().unwrap());
        assert_eq!(Token::Ident("out".to_string()), tokens.next().unwrap());
    }
}
