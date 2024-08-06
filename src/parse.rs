use winnow::{ascii::alphanumeric1, PResult, Parser};

pub(crate) enum Keyword {
    Extern,
    Let,
}

pub(crate) fn extern_keyword<'a>(input: &mut &'a str) -> PResult<&'a str> {
    let actual = "extern".parse_next(input)?;
    Ok(actual)
}

pub(crate) fn let_keyword<'a>(input: &mut &'a str) -> PResult<&'a str> {
    let actual = "let".parse_next(input)?;
    Ok(actual)
}

pub(crate) fn variable_name<'a>(input: &'a mut &'a str) -> PResult<&'a str> {
    alphanumeric1(input)
}

pub(crate) fn comment<'a>(input: &mut &'a str) -> PResult<()> {
    let comment_start = "//".parse_next(input)?;
    // TODO: Discard everything until newline of the input
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_declaration() {
        let raw = "let extern r = externInt 10 15";
        // TODO: parse the string somehow?
    }
}
