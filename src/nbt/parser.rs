use byteorder::{BigEndian, ReadBytesExt};
use crate::error::{Result, Error};
use crate::nbt;

pub struct Parser<R> {
    input: R,
    state: ParserState,
    stack: Vec<ParserState>,
}

#[derive(Debug, PartialEq)]
pub enum ValueType {
    Invalid,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    String,
    SeqBegin,
    SeqEnd,
    MapBegin,
    MapEnd,
}

#[derive(Debug, PartialEq)]
enum ParserState {
    InvalidState,
    // states for tags
    ExpectingTag,
    TagHeader { value_type: u8, name: String },
    TagEnd,
    TagValueI8 { value: i8 },
    TagValueI16 { value: i16 },
    TagValueI32 { value: i32 },
    TagValueI64 { value: i64 },
    TagValueF32 { value: f32 },
    TagValueF64 { value: f64 },
    TagValueString { value: String },
    // marker state, indicates that the parser will enter a compound type
    Compound,
    // states for array types
    I8Array { len: usize },
    I8ArrayValue { remaining: usize, value: i8 },
    I8ArrayEnd,
    I32Array { len: usize },
    I32ArrayValue { remaining: usize, value: i32 },
    I32ArrayEnd,
    I64Array { len: usize },
    I64ArrayValue { remaining: usize, value: i64 },
    I64ArrayEnd,
    // states for lists and positions within lists
    List { len: usize, elem_type: u8 },
    ListValueI8 { remaining: usize, value: i8 },
    ListValueI16 { remaining: usize, value: i16 },
    ListValueI32 { remaining: usize, value: i32 },
    ListValueI64 { remaining: usize, value: i64 },
    ListValueF32 { remaining: usize, value: f32 },
    ListValueF64 { remaining: usize, value: f64 },
    ListValueI8Array { remaining: usize },
    ListValueString { remaining: usize, value: String },
    ListValueList { remaining: usize },
    ListValueCompound { remaining: usize },
    ListValueI32Array { remaining: usize },
    ListValueI64Array { remaining: usize },
    ListEnd,
}

impl<R> Parser<R> where R: std::io::Read {
    pub fn new(input: R) -> Self {
        Parser { input, state: ParserState::ExpectingTag, stack: Vec::new() }
    }

    pub fn get_value_type(&self) -> ValueType {
        match self.state {
            ParserState::InvalidState => ValueType::Invalid,
            ParserState::ExpectingTag => ValueType::Invalid,
            ParserState::TagHeader { value_type: _, name: _ } => ValueType::String,
            ParserState::TagEnd => ValueType::MapEnd,
            ParserState::TagValueI8 { value: _ } => ValueType::I8,
            ParserState::TagValueI16 { value: _ } => ValueType::I16,
            ParserState::TagValueI32 { value: _ } => ValueType::I32,
            ParserState::TagValueI64 { value: _ } => ValueType::I64,
            ParserState::TagValueF32 { value: _ } => ValueType::F32,
            ParserState::TagValueF64 { value: _ } => ValueType::F64,
            ParserState::TagValueString { value: _ } => ValueType::String,
            ParserState::Compound => ValueType::MapBegin,
            ParserState::I8Array { len: _ } => ValueType::SeqEnd,
            ParserState::I8ArrayValue { remaining: _, value: _ } => ValueType::I8,
            ParserState::I8ArrayEnd => ValueType::SeqEnd,
            ParserState::I32Array { len: _ } => ValueType::SeqBegin,
            ParserState::I32ArrayValue { remaining: _, value: _ } => ValueType::I32,
            ParserState::I32ArrayEnd => ValueType::SeqEnd,
            ParserState::I64Array { len: _ } => ValueType::SeqBegin,
            ParserState::I64ArrayValue { remaining: _, value: _ } => ValueType::I64,
            ParserState::I64ArrayEnd => ValueType::SeqEnd,
            ParserState::List { len: _, elem_type: _ } => ValueType::SeqBegin,
            ParserState::ListValueI8 { remaining: _, value: _ } => ValueType::I8,
            ParserState::ListValueI16 { remaining: _, value: _ } => ValueType::I16,
            ParserState::ListValueI32 { remaining: _, value: _ } => ValueType::I32,
            ParserState::ListValueI64 { remaining: _, value: _ } => ValueType::I64,
            ParserState::ListValueF32 { remaining: _, value: _ } => ValueType::F32,
            ParserState::ListValueF64 { remaining: _, value: _ } => ValueType::F64,
            ParserState::ListValueI8Array { remaining: _ } => ValueType::Invalid,
            ParserState::ListValueString { remaining: _, value: _ } => ValueType::String,
            ParserState::ListValueList { remaining: _ } => ValueType::Invalid,
            ParserState::ListValueCompound { remaining: _ } => ValueType::Invalid,
            ParserState::ListValueI32Array { remaining: _ } => ValueType::Invalid,
            ParserState::ListValueI64Array { remaining: _ } => ValueType::Invalid,
            ParserState::ListEnd => ValueType::SeqEnd,
        }
    }

    pub fn get_i8_value(&self) -> Result<i8> {
        match self.state {
            ParserState::TagValueI8 { value } => Ok(value),
            ParserState::I8ArrayValue { remaining: _, value } => Ok(value),
            ParserState::ListValueI8 { remaining: _, value } => Ok(value),
            _ => Err(Error::InvalidTagTypeError),
        }
    }

    pub fn get_i16_value(&self) -> Result<i16> {
        match self.state {
            ParserState::TagValueI16 { value } => Ok(value),
            ParserState::ListValueI16 { remaining: _, value } => Ok(value),
            _ => Err(Error::InvalidTagTypeError),
        }
    }

    pub fn get_i32_value(&self) -> Result<i32> {
        match self.state {
            ParserState::TagValueI32 { value } => Ok(value),
            ParserState::I32ArrayValue { remaining: _, value } => Ok(value),
            ParserState::ListValueI32 { remaining: _, value } => Ok(value),
            _ => Err(Error::InvalidTagTypeError),
        }
    }

    pub fn get_i64_value(&self) -> Result<i64> {
        match self.state {
            ParserState::TagValueI64 { value } => Ok(value),
            ParserState::I64ArrayValue { remaining: _, value } => Ok(value),
            ParserState::ListValueI64 { remaining: _, value } => Ok(value),
            _ => Err(Error::InvalidTagTypeError),
        }
    }

    pub fn get_f32_value(&self) -> Result<f32> {
        match self.state {
            ParserState::TagValueF32 { value } => Ok(value),
            ParserState::ListValueF32 { remaining: _, value } => Ok(value),
            _ => Err(Error::InvalidTagTypeError),
        }
    }

    pub fn get_f64_value(&self) -> Result<f64> {
        match self.state {
            ParserState::TagValueF64 { value } => Ok(value),
            ParserState::ListValueF64 { remaining: _, value } => Ok(value),
            _ => Err(Error::InvalidTagTypeError),
        }
    }

    pub fn get_string_value(&self) -> Result<String> {
        match &self.state {
            ParserState::TagHeader { value_type: _, name } => Ok(name.clone()),
            ParserState::TagValueString { value } => Ok(value.clone()),
            ParserState::ListValueString { remaining: _, value } => Ok(value.clone()),
            _ => Err(Error::InvalidTagTypeError),
        }
    }

    //pub fn is_compound_value(&mut self) -> Result<bool> {
    //    match self.state {
    //        ParserState::Compound => Ok(true),
    //        ParserState::InvalidState => Err(Error::InvalidParserStateError),
    //        _ => Ok(false),
    //    }
    //}

    /// reads the next value from the parser's input
    pub fn next(&mut self) -> Result<()> {
        match self.state {
            ParserState::InvalidState => Err(Error::InvalidParserStateError),
            // states for tags
            ParserState::ExpectingTag => self.next_tag_header(),
            ParserState::TagHeader { value_type, name: _ } => self.next_tag_value(value_type),
            ParserState::TagEnd => self.next_state_from_stack(),
            ParserState::TagValueI8 { value: _ } => self.next_tag_header(),
            ParserState::TagValueI16 { value: _ } => self.next_tag_header(),
            ParserState::TagValueI32 { value: _ } => self.next_tag_header(),
            ParserState::TagValueI64 { value: _ } => self.next_tag_header(),
            ParserState::TagValueF32 { value: _ } => self.next_tag_header(),
            ParserState::TagValueF64 { value: _ } => self.next_tag_header(),
            ParserState::TagValueString { value: _ } => self.next_tag_header(),
            // states for compound types
            ParserState::Compound => self.next_compound(),
            // states for array types
            ParserState::I8Array { len } => self.next_i8_array_value(len),
            ParserState::I8ArrayValue { remaining, value: _ } => self.next_i8_array_value(remaining),
            ParserState::I8ArrayEnd => self.next_state_from_stack(),
            ParserState::I32Array { len } => self.next_i32_array_value(len),
            ParserState::I32ArrayValue { remaining, value: _ } => self.next_i32_array_value(remaining),
            ParserState::I32ArrayEnd => self.next_state_from_stack(),
            ParserState::I64Array { len } => self.next_i64_array_value(len),
            ParserState::I64ArrayValue { remaining, value: _ } => self.next_i64_array_value(remaining),
            ParserState::I64ArrayEnd => self.next_state_from_stack(),
            // states for lists and positions within lists
            ParserState::List { len, elem_type } => self.next_list(len, elem_type),
            ParserState::ListValueI8 { remaining, value: _ } => self.next_list_value_i8(remaining),
            ParserState::ListValueI16 { remaining, value: _ } => self.next_list_value_i16(remaining),
            ParserState::ListValueI32 { remaining, value: _ } => self.next_list_value_i32(remaining),
            ParserState::ListValueI64 { remaining, value: _ } => self.next_list_value_i64(remaining),
            ParserState::ListValueF32 { remaining, value: _ } => self.next_list_value_f32(remaining),
            ParserState::ListValueF64 { remaining, value: _ } => self.next_list_value_f64(remaining),
            ParserState::ListValueI8Array { remaining } => self.next_list_value_i8_array(remaining),
            ParserState::ListValueString { remaining, value: _ } => self.next_list_value_string(remaining),
            ParserState::ListValueList { remaining } => self.next_list_value_list(remaining),
            ParserState::ListValueCompound { remaining } => self.next_list_value_compound(remaining),
            ParserState::ListValueI32Array { remaining } => self.next_list_value_i32_array(remaining),
            ParserState::ListValueI64Array { remaining } => self.next_list_value_i64_array(remaining),
            ParserState::ListEnd => self.next_state_from_stack(),
        }
    }

    fn next_tag_header(&mut self) -> Result<()> {
        let value_type = self.input.read_u8()?;

        // `TAG_END` is a special case, indicates the end of a compound type
        if value_type == nbt::TAG_END {
            self.state = ParserState::TagEnd;
            return Ok(())
        }

        let name = self.read_nbt_string()?;
        self.state = ParserState::TagHeader { value_type, name };
        Ok(())
    }

    fn next_tag_value(&mut self, value_type: u8) -> Result<()> {
        match value_type {
            // FIXME: `TAG_END` may show up in unexpected places, figure out how to properly handle this case
            nbt::TAG_END => Err(Error::InvalidParserStateError),
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

    fn next_tag_value_string(&mut self) -> Result<()> {
        let value = self.read_nbt_string()?;
        self.state = ParserState::TagValueString { value };
        Ok(())
    }

    fn next_tag_value_list(&mut self) -> Result<()> {
        // return to parsing tags after parsing the list
        self.stack.push(ParserState::ExpectingTag);
        // parse the list value
        self.next_value_list()
    }

    fn next_tag_value_compound(&mut self) -> Result<()> {
        // the parser should expect another tag after this compound type
        self.stack.push(ParserState::ExpectingTag);
        // "marker" to show that the parser will read tags inside a compound value
        self.state = ParserState::Compound;
        Ok(())
    }

    fn next_tag_value_i8_array(&mut self) -> Result<()> {
        // parser should expect more tags after this array ends
        self.stack.push(ParserState::ExpectingTag);

        // start parsing the array
        self.next_value_i8_array()
    }

    fn next_tag_value_i32_array(&mut self) -> Result<()> {
        // parser should expect more tags after this array ends
        self.stack.push(ParserState::ExpectingTag);

        // start parsing the array
        self.next_value_i32_array()
    }

    fn next_tag_value_i64_array(&mut self) -> Result<()> {
        // parser should expect more tags after this array ends
        self.stack.push(ParserState::ExpectingTag);

        // start parsing the array
        self.next_value_i64_array()
    }

    fn next_compound(&mut self) -> Result<()> {
        // a compound is a bunch of tags, followed by a `TAG_END` tag
        // read the header of the first tag
        self.next_tag_header()
    }

    /// helper function, read an TAG_I8_ARRAY as another type's value
    fn next_value_i8_array(&mut self) -> Result<()> {
        let len = self.read_i32_list_len()?;
        self.state = ParserState::I8Array { len };
        Ok(())
    }

    fn next_i8_array_value(&mut self, remaining: usize) -> Result<()> {
        if remaining <= 0 {
            self.state = ParserState::I8ArrayEnd;
            return Ok(());
        }

        let remaining = remaining - 1;
        let value = self.input.read_i8()?;
        self.state = ParserState::I8ArrayValue { remaining, value };
        Ok(())
    }

    /// helper function to read a TAG_I32_ARRAY as another type's value
    fn next_value_i32_array(&mut self) -> Result<()> {
        let len = self.read_i32_list_len()?;
        self.state = ParserState::I32Array { len };
        Ok(())
    }

    fn next_i32_array_value(&mut self, remaining: usize) -> Result<()> {
        if remaining <= 0 {
            self.state = ParserState::I32ArrayEnd;
            return Ok(());
        }

        let remaining = remaining - 1;
        let value = self.input.read_i32::<BigEndian>()?;
        self.state = ParserState::I32ArrayValue { remaining, value };
        Ok(())
    }

    /// helper function to read a TAG_I64_ARRAY as another type's value
    fn next_value_i64_array(&mut self) -> Result<()> {
        let len = self.read_i32_list_len()?;
        self.state = ParserState::I64Array { len };
        Ok(())
    }

    fn next_i64_array_value(&mut self, remaining: usize) -> Result<()> {
        if remaining <= 0 {
            self.state = ParserState::I64ArrayEnd;
            return Ok(());
        }

        let remaining = remaining - 1;
        let value = self.input.read_i64::<BigEndian>()?;
        self.state = ParserState::I64ArrayValue { remaining, value };
        Ok(())
    }

    /// helper function to parse a list as part of another type's value
    fn next_value_list(&mut self) -> Result<()> {
        let elem_type = self.input.read_u8()?;
        let len = self.read_i32_list_len()?;
        self.state = ParserState::List { len, elem_type };
        Ok(())
    }

    fn next_list(&mut self, len: usize, elem_type: u8) -> Result<()> {
        if len <= 0 {
            // ignore `elem_type` to avoid issues:
            // https://minecraft.gamepedia.com/NBT_format#Usage
            self.state = ParserState::ListEnd;
            return Ok(());
        }

        match elem_type {
            nbt::TAG_I8 => self.next_list_value_i8(len),
            nbt::TAG_I16 => self.next_list_value_i16(len),
            nbt::TAG_I32 => self.next_list_value_i32(len),
            nbt::TAG_I64 => self.next_list_value_i64(len),
            nbt::TAG_F32 => self.next_list_value_f32(len),
            nbt::TAG_F64 => self.next_list_value_f64(len),
            nbt::TAG_I8_ARRAY => self.next_list_value_i8_array(len),
            nbt::TAG_STRING => self.next_list_value_string(len),
            nbt::TAG_LIST => self.next_list_value_list(len),
            nbt::TAG_COMPOUND => self.next_list_value_compound(len),
            nbt::TAG_I32_ARRAY => self.next_list_value_i32_array(len),
            nbt::TAG_I64_ARRAY => self.next_list_value_i64_array(len),
            _ => {
                self.state = ParserState::InvalidState;
                // TODO: more descriptive errors for malformed input
                Err(Error::InvalidParserStateError)
            },
        }
    }

    fn next_list_value_i8(&mut self, remaining: usize) -> Result<()> {
        if remaining <= 0 {
            self.state = ParserState::ListEnd;
            return Ok(());
        }

        let remaining = remaining - 1;
        let value = self.input.read_i8()?;
        self.state = ParserState::ListValueI8 { remaining, value };
        Ok(())
    }

    fn next_list_value_i16(&mut self, remaining: usize) -> Result<()> {
        if remaining <= 0 {
            self.state = ParserState::ListEnd;
            return Ok(());
        }

        let remaining = remaining - 1;
        let value = self.input.read_i16::<BigEndian>()?;
        self.state = ParserState::ListValueI16 { remaining, value };
        Ok(())
    }

    fn next_list_value_i32(&mut self, remaining: usize) -> Result<()> {
        if remaining <= 0 {
            self.state = ParserState::ListEnd;
            return Ok(());
        }

        let remaining = remaining - 1;
        let value = self.input.read_i32::<BigEndian>()?;
        self.state = ParserState::ListValueI32 { remaining, value };
        Ok(())
    }

    fn next_list_value_i64(&mut self, remaining: usize) -> Result<()> {
        if remaining <= 0 {
            self.state = ParserState::ListEnd;
            return Ok(());
        }

        let remaining = remaining - 1;
        let value = self.input.read_i64::<BigEndian>()?;
        self.state = ParserState::ListValueI64 { remaining, value };
        Ok(())
    }

    fn next_list_value_f32(&mut self, remaining: usize) -> Result<()> {
        if remaining <= 0 {
            self.state = ParserState::ListEnd;
            return Ok(());
        }

        let remaining = remaining - 1;
        let value = self.input.read_f32::<BigEndian>()?;
        self.state = ParserState::ListValueF32 { remaining, value };
        Ok(())
    }

    fn next_list_value_f64(&mut self, remaining: usize) -> Result<()> {
        if remaining <= 0 {
            self.state = ParserState::ListEnd;
            return Ok(());
        }

        let remaining = remaining - 1;
        let value = self.input.read_f64::<BigEndian>()?;
        self.state = ParserState::ListValueF64 { remaining, value };
        Ok(())
    }

    fn next_list_value_i8_array(&mut self, remaining: usize) -> Result<()> {
        if remaining <= 0 {
            self.state = ParserState::ListEnd;
            return Ok(());
        }

        // return to parsing this list after parsing the TAG_I8_ARRAY
        let remaining = remaining - 1;
        self.state = ParserState::ListValueI8Array { remaining };

        // parse the TAG_I8_ARRAY
        self.next_value_i8_array()
    }

    fn next_list_value_string(&mut self, remaining: usize) -> Result<()> {
        if remaining <= 0 {
            self.state = ParserState::ListEnd;
            return Ok(());
        }

        let remaining = remaining - 1;
        let value = self.read_nbt_string()?;
        self.state = ParserState::ListValueString { remaining, value };
        Ok(())
    }

    fn next_list_value_list(&mut self, remaining: usize) -> Result<()> {
        if remaining <= 0 {
            self.state = ParserState::ListEnd;
            return Ok(());
        }

        // return to current list after parsing the contained list
        let remaining = remaining - 1;
        self.stack.push(ParserState::ListValueList { remaining });

        // parse the list (as the current list's value)
        self.next_value_list()
    }

    fn next_list_value_compound(&mut self, remaining: usize) -> Result<()> {
        if remaining <= 0 {
            self.state = ParserState::ListEnd;
            return Ok(());
        }

        // return to parsing this list after parsing the compound value
        let remaining = remaining - 1;
        self.stack.push(ParserState::ListValueCompound { remaining });

        // parse the compound next
        self.state = ParserState::Compound;
        Ok(())
    }

    fn next_list_value_i32_array(&mut self, remaining: usize) -> Result<()> {
        if remaining <= 0 {
            self.state = ParserState::ListEnd;
            return Ok(());
        }

        // return to parsing this list after parsing the TAG_I32_ARRAY
        let remaining = remaining - 1;
        self.stack.push(ParserState::ListValueI32Array { remaining });

        // parse the TAG_I32_ARRAY
        self.next_value_i32_array()
    }

    fn next_list_value_i64_array(&mut self, remaining: usize) -> Result<()> {
        if remaining <= 0 {
            self.state = ParserState::ListEnd;
            return Ok(());
        }

        // return to parsing this list after parsing the TAG_I64_ARRAY
        let remaining = remaining - 1;
        self.stack.push(ParserState::ListValueI64Array { remaining });

        // parse the TAG_I64_ARRAY
        self.next_value_i64_array()
    }

    /// restore parser's state from the stack and continue parsing
    fn next_state_from_stack(&mut self) -> Result<()> {
        match self.stack.pop() {
            Some(next_state) => {
                // restore previous state from the stack
                self.state = next_state;
                // read the next value
                // FIXME: try to avoid creating new frames on the call stack
                self.next()
            },
            None => {
                self.state = ParserState::InvalidState;
                // TODO: make a more descriptive error for malformed input
                Err(Error::InvalidParserStateError)
            },
        }
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
    
    /// helper function to read NBT i32 length fields
    fn read_i32_list_len(&mut self) -> Result<usize> {
        let len = self.input.read_i32::<BigEndian>()?;
        if len < 0 {
            self.state = ParserState::InvalidState;
            // TODO: create more descriptive errors for malformed input
            return Err(Error::InvalidParserStateError);
        } else if len > nbt::LIST_MAX_LEN as i32 {
            self.state = ParserState::InvalidState;
            // TODO: create more descriptive errors for malformed input
            return Err(Error::InvalidParserStateError);
        }

        Ok(len as usize)
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
        const EXPECTED_VALUE: i8 = 0x11;

        // `"": 17B` (0x11)
        let buffer = b"\x01\x00\x00\x11";
        let input = Cursor::new(buffer);
        let mut parser = Parser::new(input);

        // initial parser state
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
        const EXPECTED_VALUE: i16 = 0x1122;

        // `"": 462S` (0x1616)
        let buffer = b"\x02\x00\x00\x11\x22";
        let input = Cursor::new(buffer);
        let mut parser = Parser::new(input);

        // initial parser state
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
        const EXPECTED_VALUE: i32 = 0x11223344;

        // `"": 287454020` (0x11223344)
        let buffer = b"\x03\x00\x00\x11\x22\x33\x44";
        let input = Cursor::new(buffer);
        let mut parser = Parser::new(input);

        // initial parser state
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

        // initial parser state
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

    #[test]
    fn test_f32_tag() {
        const EXPECTED_VALUE: f32 = 0.0;

        // `"": 0F`
        let buffer = b"\x05\x00\x00\x00\x00\x00\x00";
        let input = Cursor::new(buffer);
        let mut parser = Parser::new(input);

        assert_eq!(parser.state, ParserState::ExpectingTag);

        // read the tag's header
        assert!(parser.next().is_ok());
        assert_eq!(parser.state, ParserState::TagHeader { value_type: nbt::TAG_F32, name: "".to_string() });
        assert_eq!(parser.get_string_value().unwrap(), "");

        // read the tag's value
        assert!(parser.next().is_ok());
        assert_eq!(parser.state, ParserState::TagValueF32 { value: EXPECTED_VALUE });
        assert_eq!(parser.get_f32_value().unwrap(), EXPECTED_VALUE);

        // TODO: check to make sure that the other `get_*` functions return errors
        // TODO: try reading beyond the end of the input
    }

    #[test]
    fn test_f64_tag() {
        const EXPECTED_VALUE: f64 = 0.0;

        // `"": 0D`
        let buffer = b"\x06\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";
        let input = Cursor::new(buffer);
        let mut parser = Parser::new(input);

        // initial parser state
        assert_eq!(parser.state, ParserState::ExpectingTag);

        // read the tag's header
        assert!(parser.next().is_ok());
        assert_eq!(parser.state, ParserState::TagHeader { value_type: nbt::TAG_F64, name: "".to_string() });
        assert_eq!(parser.get_string_value().unwrap(), "");

        // read the tag's value
        assert!(parser.next().is_ok());
        assert_eq!(parser.state, ParserState::TagValueF64 { value: EXPECTED_VALUE });
        assert_eq!(parser.get_f64_value().unwrap(), EXPECTED_VALUE);

        // TODO: check to make sure that the other `get_*` functions return errors
        // TODO: try reading beyond the end of the input
    }

    #[test]
    fn test_string_tag() {
        let tag_name = "tag name".to_string();
        let expected_value = "Hello, World!".to_string();

        // `"tag name": "Hello, World!"`
        let buffer = b"\x08\x00\x08tag name\x00\x0dHello, World!";
        let input = Cursor::new(buffer);
        let mut parser = Parser::new(input);

        // initial parser state
        assert_eq!(parser.state, ParserState::ExpectingTag);

        // read the tag's header
        assert!(parser.next().is_ok());
        assert_eq!(parser.state, ParserState::TagHeader { value_type: nbt::TAG_STRING, name: tag_name.clone() });
        assert_eq!(parser.get_string_value().unwrap(), tag_name);

        // read the tag's value
        assert!(parser.next().is_ok());
        assert_eq!(parser.state, ParserState::TagValueString { value: expected_value.clone() });
        assert_eq!(parser.get_string_value().unwrap(), expected_value);

        // TODO: check to make sure that the other `get_*` functions return errors
        // TODO: try reading beyond the end of the input
    }

    #[test]
    fn test_empty_compound_tag() {
        // `"": {}`
        let buffer = b"\x0a\x00\x00\x00";
        let input = Cursor::new(buffer);
        let mut parser = Parser::new(input);

        // initial parser state
        assert_eq!(parser.state, ParserState::ExpectingTag);

        // read the tag's header
        assert!(parser.next().is_ok());
        assert_eq!(parser.state, ParserState::TagHeader { value_type: nbt::TAG_COMPOUND, name: "".to_string() });
        assert_eq!(parser.get_string_value().unwrap(), "");

        // enter into the `TAG_COMPOUND`
        assert!(parser.next().is_ok());
        assert_eq!(parser.state, ParserState::Compound);

        // read the end marker for the empty tag
        assert!(parser.next().is_ok());
        assert_eq!(parser.state, ParserState::TagEnd);

        // TODO: try reading beyond the end of the input
    }

    #[test]
    fn test_nested_compound_tags() {
        let outer_name = "outer".to_string();
        let inner_name = "inner".to_string();

        // `"outer": {"inner": {}}`
        let buffer = b"\x0a\x00\x05outer\x0a\x00\x05inner\x00\x00";
        let input = Cursor::new(buffer);
        let mut parser = Parser::new(input);

        // initial parser state
        assert_eq!(parser.state, ParserState::ExpectingTag);

        // read the outer tag's header
        assert!(parser.next().is_ok());
        assert_eq!(parser.state, ParserState::TagHeader { value_type: nbt::TAG_COMPOUND, name: outer_name.clone() });
        assert_eq!(parser.get_string_value().unwrap(), outer_name);

        // enter into the outer `TAG_COMPOUND`
        assert!(parser.next().is_ok());

        // read the inner tag
        assert!(parser.next().is_ok());
        assert_eq!(parser.state, ParserState::TagHeader { value_type: nbt::TAG_COMPOUND, name: inner_name.clone() });
        assert_eq!(parser.get_string_value().unwrap(), inner_name);

        // enter into the inner `TAG_COMPOUND`
        assert!(parser.next().is_ok());

        // read the inner tag's end marker
        assert!(parser.next().is_ok());
        assert_eq!(parser.state, ParserState::TagEnd);

        // read the outer tag's `TAG_END`
        assert!(parser.next().is_ok());
        assert_eq!(parser.state, ParserState::TagEnd);

        // TODO: try reading beyond the end of the input
    }

    #[test]
    fn test_triple_nested_compound_tags() {
        let outer_name = "outer".to_string();
        let mid_name = "mid".to_string();
        let inner_name = "inner".to_string();

        // `"outer": {"inner": {}}`
        let buffer = b"\x0a\x00\x05outer\x0a\x00\x03mid\x0a\x00\x05inner\x00\x00\x00";
        let input = Cursor::new(buffer);
        let mut parser = Parser::new(input);

        // initial parser state
        assert_eq!(parser.state, ParserState::ExpectingTag);

        // read the outer tag's header
        assert!(parser.next().is_ok());
        assert_eq!(parser.state, ParserState::TagHeader { value_type: nbt::TAG_COMPOUND, name: outer_name.clone() });
        assert_eq!(parser.get_string_value().unwrap(), outer_name);

        // enter into the outer `TAG_COMPOUND`
        assert!(parser.next().is_ok());

        // read the middle tag
        assert!(parser.next().is_ok());
        assert_eq!(parser.state, ParserState::TagHeader { value_type: nbt::TAG_COMPOUND, name: mid_name.clone() });
        assert_eq!(parser.get_string_value().unwrap(), mid_name);

        // enter into the middle TAG_COMPOUND
        assert!(parser.next().is_ok());

        // read the inner tag
        assert!(parser.next().is_ok());
        assert_eq!(parser.state, ParserState::TagHeader { value_type: nbt::TAG_COMPOUND, name: inner_name.clone() });
        assert_eq!(parser.get_string_value().unwrap(), inner_name);

        // enter into the inner `TAG_COMPOUND`
        assert!(parser.next().is_ok());

        // read the inner tag's end marker
        assert!(parser.next().is_ok());
        assert_eq!(parser.state, ParserState::TagEnd);

        // read the middle tag's end marker
        assert!(parser.next().is_ok());
        assert_eq!(parser.state, ParserState::TagEnd);

        // read the outer tag's `TAG_END`
        assert!(parser.next().is_ok());
        assert_eq!(parser.state, ParserState::TagEnd);

        // TODO: try reading beyond the end of the input
    }

    #[test]
    fn test_empty_i8_array() {
        // `"": [B;]`
        let buffer = b"\x07\x00\x00\x00\x00\x00\x00";
        let input = Cursor::new(buffer);
        let mut parser = Parser::new(input);

        // initial parser state
        assert_eq!(parser.state, ParserState::ExpectingTag);

        // tag header
        assert!(parser.next().is_ok());
        assert_eq!(parser.state, ParserState::TagHeader { value_type: nbt::TAG_I8_ARRAY, name: "".to_string() });

        // begining of the array
        assert!(parser.next().is_ok());
        assert_eq!(parser.state, ParserState::I8Array { len: 0 });

        // end of the array
        assert!(parser.next().is_ok());
        assert_eq!(parser.state, ParserState::I8ArrayEnd);

        // TODO: try reading beyond the end of the input
    }

    #[test]
    fn test_empty_i32_array() {
        // `"": [I;]`
        let buffer = b"\x0b\x00\x00\x00\x00\x00\x00";
        let input = Cursor::new(buffer);
        let mut parser = Parser::new(input);

        // initial parser state
        assert_eq!(parser.state, ParserState::ExpectingTag);

        // tag header
        assert!(parser.next().is_ok());
        assert_eq!(parser.state, ParserState::TagHeader { value_type: nbt::TAG_I32_ARRAY, name: "".to_string() });

        // begining of the array
        assert!(parser.next().is_ok());
        assert_eq!(parser.state, ParserState::I32Array { len: 0 });

        // end of the array
        assert!(parser.next().is_ok());
        assert_eq!(parser.state, ParserState::I32ArrayEnd);

        // TODO: try reading beyond the end of the input
    }

    #[test]
    fn test_empty_i64_array() {
        // `"": [L;]`
        let buffer = b"\x0c\x00\x00\x00\x00\x00\x00";
        let input = Cursor::new(buffer);
        let mut parser = Parser::new(input);

        // initial parser state
        assert_eq!(parser.state, ParserState::ExpectingTag);

        // tag header
        assert!(parser.next().is_ok());
        assert_eq!(parser.state, ParserState::TagHeader { value_type: nbt::TAG_I64_ARRAY, name: "".to_string() });

        // begining of the array
        assert!(parser.next().is_ok());
        assert_eq!(parser.state, ParserState::I64Array { len: 0 });

        // end of the array
        assert!(parser.next().is_ok());
        assert_eq!(parser.state, ParserState::I64ArrayEnd);

        // TODO: try reading beyond the end of the input
    }

    #[test]
    fn test_empty_end_list() {
        // `"": []` (elements are of type `TAG_END`)
        let buffer = b"\x09\x00\x00\x00\x00\x00\x00\x00";
        let input = Cursor::new(buffer);
        let mut parser = Parser::new(input);

        // initial parser state
        assert_eq!(parser.state, ParserState::ExpectingTag);

        // tag header
        assert!(parser.next().is_ok());
        assert_eq!(parser.state, ParserState::TagHeader { value_type: nbt::TAG_LIST, name: "".to_string() });

        // list header
        assert!(parser.next().is_ok());
        assert_eq!(parser.state, ParserState::List { len: 0, elem_type: nbt::TAG_END });

        // list end
        assert!(parser.next().is_ok());
        assert_eq!(parser.state, ParserState::ListEnd);
    }

    #[test]
    fn test_empty_i8_list() {
        // `"": []` (elements are of type `TAG_I8`)
        let buffer = b"\x09\x00\x00\x01\x00\x00\x00\x00";
        let input = Cursor::new(buffer);
        let mut parser = Parser::new(input);

        // initial parser state
        assert_eq!(parser.state, ParserState::ExpectingTag);

        // tag header
        assert!(parser.next().is_ok());
        assert_eq!(parser.state, ParserState::TagHeader { value_type: nbt::TAG_LIST, name: "".to_string() });

        // list header
        assert!(parser.next().is_ok());
        assert_eq!(parser.state, ParserState::List { len: 0, elem_type: nbt::TAG_I8 });

        // list end
        assert!(parser.next().is_ok());
        assert_eq!(parser.state, ParserState::ListEnd);
    }
}
