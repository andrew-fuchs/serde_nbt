struct Parser<R> {
    input: R,
    state: ParserState,
}

#[derive(Debug, PartialEq)]
enum ParserState {
    InvalidState,
    ExpectingTag,
}

impl<R> Parser<R> where R: std::io::Read {
    pub fn new(input: R) -> Self {
        Parser { input, state: ParserState::ExpectingTag }
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use super::*;

    #[test]
    fn test_parser_new() {
        let buffer: Vec<u8> = Vec::new();
        let input = Cursor::new(buffer);
        let parser = Parser::new(input);

        assert_eq!(parser.state, ParserState::ExpectingTag);
    }
}
