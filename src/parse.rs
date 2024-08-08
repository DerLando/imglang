use winnow::{ascii::alphanumeric1, token::one_of, PResult, Parser};

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
    Literal(Literal),
}

#[derive(Debug, PartialEq)]
pub(crate) enum Literal {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
}

pub(crate) fn extern_keyword<'a>(input: &mut &'a str) -> PResult<Token> {
    let actual = "extern".parse_next(input)?;
    Ok(Token::Extern)
}

pub(crate) fn let_keyword<'a>(input: &mut &'a str) -> PResult<Token> {
    let actual = "let".parse_next(input)?;
    Ok(Token::Let)
}

pub(crate) fn ident<'a>(input: &'a mut &'a str) -> PResult<Token> {
    alphanumeric1(input).map(|name| Token::Ident(name.to_string()))
}

pub(crate) fn comment<'a>(input: &mut &'a str) -> PResult<()> {
    let comment_start = "//".parse_next(input)?;
    // TODO: Discard everything until newline of the input
    Ok(())
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
    winnow::ascii::dec_int(input).map(|num| Token::Literal(Literal::Int(num)))
}

pub(crate) fn float_literal(input: &mut &str) -> PResult<Token> {
    winnow::ascii::float(input).map(|f| Token::Literal(Literal::Float(f)))
}

pub(crate) fn literal(input: &mut &str) -> PResult<Token> {
    winnow::combinator::alt((bool_literal, int_literal, float_literal, string_literal))
        .parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_literals() {
        let mut raw = r#""Hello""#;
        assert_eq!(
            Token::Literal(Literal::String("Hello".to_string())),
            literal(&mut raw).unwrap()
        )
    }

    #[test]
    fn test_input_declaration() {
        let raw = "let extern r = externInt 10 15";
        // TODO: parse the string somehow?
    }
}
