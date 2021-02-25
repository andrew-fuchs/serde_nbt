use byteorder::{BigEndian, ReadBytesExt};
use crate::error::{Result, Error};
use crate::nbt;

struct Parser<R> {
    input: R,
    state: ParserState,
}

#[derive(Debug, PartialEq)]
enum ParserState {
    InvalidState,
    ExpectingTag,
    TagHeader { value_type: u8, name: String },
    TagValueI8 { value: i8 },
    TagValueI16 { value: i16 },
    TagValueI32 { value: i32 },
    TagValueI64 { value: i64 },
    TagValueF32 { value: f32 },
    TagValueF64 { value: f64 },
    TagValueString { value: String },
}

impl<R> Parser<R> where R: std::io::Read {
    pub fn new(input: R) -> Self {
        Parser { input, state: ParserState::ExpectingTag }
    }

    pub fn get_i8_value(&mut self) -> Result<i8> {
        match self.state {
            ParserState::TagValueI8 { value } => Ok(value),
            _ => Err(Error::InvalidTagTypeError),
        }
    }

    pub fn get_i16_value(&mut self) -> Result<i16> {
        match self.state {
            ParserState::TagValueI16 { value } => Ok(value),
            _ => Err(Error::InvalidTagTypeError),
        }
    }

    pub fn get_i32_value(&mut self) -> Result<i32> {
        match self.state {
            ParserState::TagValueI32 { value } => Ok(value),
            _ => Err(Error::InvalidTagTypeError),
        }
    }

    pub fn get_i64_value(&mut self) -> Result<i64> {
        match self.state {
            ParserState::TagValueI64 { value } => Ok(value),
            _ => Err(Error::InvalidTagTypeError),
        }
    }

    pub fn get_f32_value(&mut self) -> Result<f32> {
        match self.state {
            ParserState::TagValueF32 { value } => Ok(value),
            _ => Err(Error::InvalidTagTypeError),
        }
    }

    pub fn get_f64_value(&mut self) -> Result<f64> {
        match self.state {
            ParserState::TagValueF64 { value } => Ok(value),
            _ => Err(Error::InvalidTagTypeError),
        }
    }

    pub fn get_string_value(&mut self) -> Result<String> {
        match &self.state {
            ParserState::TagHeader { value_type: _, name } => Ok(name.clone()),
            ParserState::TagValueString { value } => Ok(value.clone()),
            _ => Err(Error::InvalidTagTypeError),
        }
    }

    pub fn next(&mut self) -> Result<()> {
        match self.state {
            ParserState::InvalidState => Err(Error::InvalidParserStateError),
            ParserState::ExpectingTag => self.next_tag_header(),
            ParserState::TagHeader { value_type, name: _ } => self.next_tag_value(value_type),
            ParserState::TagValueI8 { value: _ } => self.next_tag_header(),
            ParserState::TagValueI16 { value: _ } => self.next_tag_header(),
            ParserState::TagValueI32 { value: _ } => self.next_tag_header(),
            ParserState::TagValueI64 { value: _ } => self.next_tag_header(),
            ParserState::TagValueF32 { value: _ } => self.next_tag_header(),
            ParserState::TagValueF64 { value: _ } => self.next_tag_header(),
            ParserState::TagValueString { value: _ } => self.next_tag_header(),
        }
    }

    fn next_tag_header(&mut self) -> Result<()> {
        let value_type = self.input.read_u8()?;
        let name = self.read_nbt_string()?;
        self.state = ParserState::TagHeader { value_type, name };
        Ok(())
    }

    fn next_tag_value(&mut self, value_type: u8) -> Result<()> {
        match value_type {
            // FIXME: `TAG_END` may show up in unexpected places, figure out how to properly handle this case
            nbt::TAG_END => Err(Error::InvalidTagTypeError),
            nbt::TAG_I8 => self.next_tag_value_i8(),
            nbt::TAG_I16 => self.next_tag_value_i16(),
            nbt::TAG_I32 => self.next_tag_value_i32(),
            nbt::TAG_I64 => self.next_tag_value_i64(),
            nbt::TAG_F32 => self.next_tag_value_f32(),
            nbt::TAG_F64 => self.next_tag_value_f64(),
            nbt::TAG_I8_ARRAY => self.next_tag_value_i8_array(),
            nbt::TAG_STRING => self.next_tag_value_string(),
            nbt::TAG_LIST => self.next_tag_value_list(),
            nbt::TAG_COMPOUND => self.next_tag_value_compound(),
            nbt::TAG_I32_ARRAY => self.next_tag_value_i32_array(),
            nbt::TAG_I64_ARRAY => self.next_tag_value_i64_array(),
            _ => Err(Error::InvalidTagTypeError),
        }
    }

    fn next_tag_value_i8(&mut self) -> Result<()> {
        let value = self.input.read_i8()?;
        self.state = ParserState::TagValueI8 { value };
        Ok(())
    }

    fn next_tag_value_i16(&mut self) -> Result<()> {
        let value = self.input.read_i16::<BigEndian>()?;
        self.state = ParserState::TagValueI16 { value };
        Ok(())
    }

    fn next_tag_value_i32(&mut self) -> Result<()> {
        let value = self.input.read_i32::<BigEndian>()?;
        self.state = ParserState::TagValueI32 { value };
        Ok(())
    }

    fn next_tag_value_i64(&mut self) -> Result<()> {
        let value = self.input.read_i64::<BigEndian>()?;
        self.state = ParserState::TagValueI64 { value };
        Ok(())
    }

    fn next_tag_value_f32(&mut self) -> Result<()> {
        let value = self.input.read_f32::<BigEndian>()?;
        self.state = ParserState::TagValueF32 { value };
        Ok(())
    }

    fn next_tag_value_f64(&mut self) -> Result<()> {
        let value = self.input.read_f64::<BigEndian>()?;
        self.state = ParserState::TagValueF64 { value };
        Ok(())
    }

    fn next_tag_value_i8_array(&mut self) -> Result<()> {
        todo!()
    }

    fn next_tag_value_string(&mut self) -> Result<()> {
        let value = self.read_nbt_string()?;
        self.state = ParserState::TagValueString { value };
        Ok(())
    }

    fn next_tag_value_list(&mut self) -> Result<()> {
        todo!()
    }

    fn next_tag_value_compound(&mut self) -> Result<()> {
        todo!()
    }

    fn next_tag_value_i32_array(&mut self) -> Result<()> {
        todo!()
    }

    fn next_tag_value_i64_array(&mut self) -> Result<()> {
        todo!()
    }

    /// helper function to read NBT strings
    fn read_nbt_string(&mut self) -> Result<String> {
        // u16 prefixed length
        let len = self.input.read_u16::<BigEndian>()?;

        // read the string's contents
        let mut value: Vec<u8> = vec![0; len as usize];
        self.input.read_exact(value.as_mut_slice())?;
        let value = String::from_utf8(value)?;

        Ok(value)
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


    #[test]
    fn test_i8_tag() {
        // FIXME use something that isn't a byte-order independent palindrome
        const EXPECTED_VALUE: i8 = 0x11;

        // `"": 17B` (0x11)
        let buffer = b"\x01\x00\x00\x11";
        let input = Cursor::new(buffer);
        let mut parser = Parser::new(input);

        // assert_eq!(parser.state, ParserState::StartOfInput);
        assert_eq!(parser.state, ParserState::ExpectingTag);

        // read the tag's header
        assert!(parser.next().is_ok());
        assert_eq!(parser.state, ParserState::TagHeader { value_type: nbt::TAG_I8, name: "".to_string() });
        assert_eq!(parser.get_string_value().unwrap(), "");

        // read the tag's value
        assert!(parser.next().is_ok());
        assert_eq!(parser.state, ParserState::TagValueI8 { value: EXPECTED_VALUE });
        assert_eq!(parser.get_i8_value().unwrap(), EXPECTED_VALUE);

        // TODO: check to make sure that the other `get_*` functions return errors

        // TODO: try reading beyond the end of the input
    }

    #[test]
    fn test_i16_tag() {
        // FIXME use something that isn't a byte-order independent palindrome
        const EXPECTED_VALUE: i16 = 0x1122;

        // `"": 462S` (0x1616)
        let buffer = b"\x02\x00\x00\x11\x22";
        let input = Cursor::new(buffer);
        let mut parser = Parser::new(input);

        // assert_eq!(parser.state, ParserState::StartOfInput);
        assert_eq!(parser.state, ParserState::ExpectingTag);

        // read the tag's header
        assert!(parser.next().is_ok());
        assert_eq!(parser.state, ParserState::TagHeader { value_type: nbt::TAG_I16, name: "".to_string() });
        assert_eq!(parser.get_string_value().unwrap(), "");

        // read the tag's value
        assert!(parser.next().is_ok());
        assert_eq!(parser.state, ParserState::TagValueI16 { value: EXPECTED_VALUE });
        assert_eq!(parser.get_i16_value().unwrap(), EXPECTED_VALUE);

        // TODO: check to make sure that the other `get_*` functions return errors

        // TODO: try reading beyond the end of the input
    }

    #[test]
    fn test_i32_tag() {
        // FIXME use something that isn't a byte-order independent palindrome
        const EXPECTED_VALUE: i32 = 0x11223344;

        // `"": 287454020` (0x11223344)
        let buffer = b"\x03\x00\x00\x11\x22\x33\x44";
        let input = Cursor::new(buffer);
        let mut parser = Parser::new(input);

        // assert_eq!(parser.state, ParserState::StartOfInput);
        assert_eq!(parser.state, ParserState::ExpectingTag);

        // read the tag's header
        assert!(parser.next().is_ok());
        assert_eq!(parser.state, ParserState::TagHeader { value_type: nbt::TAG_I32, name: "".to_string() });
        assert_eq!(parser.get_string_value().unwrap(), "");

        // read the tag's value
        assert!(parser.next().is_ok());
        assert_eq!(parser.state, ParserState::TagValueI32 { value: EXPECTED_VALUE });
        assert_eq!(parser.get_i32_value().unwrap(), EXPECTED_VALUE);

        // TODO: check to make sure that the other `get_*` functions return errors

        // TODO: try reading beyond the end of the input
    }

    #[test]
    fn test_i64_tag() {
        const EXPECTED_VALUE: i64 = 0x1122334455667788;

        // `"": 1234605616436508552L` (0x1122334455667788)
        let buffer = b"\x04\x00\x00\x11\x22\x33\x44\x55\x66\x77\x88";
        let input = Cursor::new(buffer);
        let mut parser = Parser::new(input);

        // assert_eq!(parser.state, ParserState::StartOfInput);
        assert_eq!(parser.state, ParserState::ExpectingTag);

        // read the tag's header
        assert!(parser.next().is_ok());
        assert_eq!(parser.state, ParserState::TagHeader { value_type: nbt::TAG_I64, name: "".to_string() });
        assert_eq!(parser.get_string_value().unwrap(), "");

        // read the tag's value
        assert!(parser.next().is_ok());
        assert_eq!(parser.state, ParserState::TagValueI64 { value: EXPECTED_VALUE });
        assert_eq!(parser.get_i64_value().unwrap(), EXPECTED_VALUE);

        // TODO: check to make sure that the other `get_*` functions return errors

        // TODO: try reading beyond the end of the input
    }

    // TODO: test_f32_tag
    // TODO: test_f64_tag
    // TODO: test_string_tag
}
