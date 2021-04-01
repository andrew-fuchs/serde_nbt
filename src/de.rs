use log::trace;
use serde::de::{self, Deserialize, DeserializeSeed, MapAccess, SeqAccess, Visitor};
use crate::error::{Error, Result};
use crate::nbt::parser::{Parser, ValueType};

pub fn from_reader<'de, R, T>(input: R) -> Result<T>
where
    R: std::io::Read,
    T: Deserialize<'de>,
{
    let mut deserializer = Deserializer::new(input);
    let value = T::deserialize(&mut deserializer)?;
    Ok(value)
}

pub struct Deserializer<R> {
    parser: Parser<R>,
}

impl<'de, R> Deserializer<R>
where
    R: std::io::Read,
{
    pub fn new(input: R) -> Self {
        #[cfg(debug_assertions)]
        trace!("Deserializer::new");

        let mut parser = Parser::new(input);

        // advance the parser into the intial TAG_COMPOUND tag
        // FIXME: this can panic, needs better error handling
        parser.next().expect("");

        // read/parse before values are retrieved so we can check their types
        // FIXME: this can panic, needs better error handling
        parser.next().expect("");

        Deserializer { parser }
    }
}

impl<'de, 'a, R> de::Deserializer<'de> for &'a mut Deserializer<R>
where
    R: std::io::Read,
{
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        #[cfg(debug_assertions)]
        trace!("Deserializer::deserialize_any");

        // FIXME: use the deserialize_* functions instead of reimplementing them
        match self.parser.get_value_type() {
            ValueType::I8 => {
                let value = self.parser.get_i8_value()?;
                self.parser.next()?;
                visitor.visit_i8(value)
            }
            ValueType::I16 => {
                let value = self.parser.get_i16_value()?;
                self.parser.next()?;
                visitor.visit_i16(value)
            }
            ValueType::I32 => {
                let value = self.parser.get_i32_value()?;
                self.parser.next()?;
                visitor.visit_i32(value)
            }
            ValueType::I64 => {
                let value = self.parser.get_i64_value()?;
                self.parser.next()?;
                visitor.visit_i64(value)
            }
            ValueType::F32 => {
                let value = self.parser.get_f32_value()?;
                self.parser.next()?;
                visitor.visit_f32(value)
            }
            ValueType::F64 => {
                let value = self.parser.get_f64_value()?;
                self.parser.next()?;
                visitor.visit_f64(value)
            }
            ValueType::String => {
                let value = self.parser.get_string_value()?;
                self.parser.next()?;
                visitor.visit_string(value)
            }
            ValueType::SeqBegin => {
                self.parser.next()?;
                visitor.visit_seq(self)
            }
            ValueType::MapBegin => {
                self.parser.next()?;
                visitor.visit_map(self)
            }
            ValueType::Invalid => Err(Error::InvalidParserStateError),
            ValueType::SeqEnd => Err(Error::InvalidParserStateError),
            ValueType::MapEnd => Err(Error::InvalidParserStateError),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        #[cfg(debug_assertions)]
        trace!("Deserializer::deserialize_bool");

        let value = match self.parser.get_i8_value()? {
            0 => false,
            _ => true,
        };

        #[cfg(debug_assertions)]
        trace!("Deserializer::deserialize_bool(visitor) -> {:?}", value);

        self.parser.next()?;

        visitor.visit_bool(value)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        #[cfg(debug_assertions)]
        trace!("Deserializer::deserialize_i8");

        let value = self.parser.get_i8_value()?;

        #[cfg(debug_assertions)]
        trace!("Deserializer::deserialize_i8(visitor) -> {:?}", value);

        self.parser.next()?;

        visitor.visit_i8(value)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        #[cfg(debug_assertions)]
        trace!("Deserializer::deserialize_i16");

        let value = self.parser.get_i16_value()?;

        #[cfg(debug_assertions)]
        trace!("Deserializer::deserialize_i16(visitor) -> {:?}", value);

        self.parser.next()?;

        visitor.visit_i16(value)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        #[cfg(debug_assertions)]
        trace!("Deserializer::deserialize_i32");

        let value = self.parser.get_i32_value()?;

        #[cfg(debug_assertions)]
        trace!("Deserializer::deserialize_i32(visitor) -> {:?}", value);

        self.parser.next()?;

        visitor.visit_i32(value)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        #[cfg(debug_assertions)]
        trace!("Deserializer::deserialize_i64");

        let value = self.parser.get_i64_value()?;

        #[cfg(debug_assertions)]
        trace!("Deserializer::deserialize_i64(visitor) -> {:?}", value);

        self.parser.next()?;

        visitor.visit_i64(value)
    }

    fn deserialize_u8<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u16<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u32<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u64<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        #[cfg(debug_assertions)]
        trace!("Deserializer::deserialize_f32");

        let value = self.parser.get_f32_value()?;

        #[cfg(debug_assertions)]
        trace!("Deserializer::deserialize_f32(visitor) -> {:?}", value);

        self.parser.next()?;

        visitor.visit_f32(value)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        #[cfg(debug_assertions)]
        trace!("Deserializer::deserialize_f64");

        let value = self.parser.get_f64_value()?;

        #[cfg(debug_assertions)]
        trace!("Deserializer::deserialize_f64(visitor) -> {:?}", value);

        self.parser.next()?;

        visitor.visit_f64(value)
    }

    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_str<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        #[cfg(debug_assertions)]
        trace!("Deserializer::deserialize_string");

        let value = self.parser.get_string_value()?;

        #[cfg(debug_assertions)]
        trace!("Deserializer::deserialize_f64(visitor) -> {:?}", value);

        self.parser.next()?;

        visitor.visit_string(value)
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_option<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        #[cfg(debug_assertions)]
        trace!("Deserializer::deserialize_seq");

        self.parser.next()?;

        // FIXME: assert that the parser is reading a sequence type

        visitor.visit_seq(self)
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        #[cfg(debug_assertions)]
        trace!("Deserializer::deserialize_tuple");

        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        #[cfg(debug_assertions)]
        trace!("Deserializer::deserialize_map");

        self.parser.next()?;

        // FIXME: assert that the parser is reading a map type

        visitor.visit_map(self)
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        #[cfg(debug_assertions)]
        trace!("Deserializer::deserialize_struct");

        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        #[cfg(debug_assertions)]
        trace!("Deserializer::deserialize_identifier");

        self.deserialize_string(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        #[cfg(debug_assertions)]
        trace!("Deserializer::deserialize_ignored_any");

        self.deserialize_any(visitor)
    }
}

impl<'de, 'a, R> SeqAccess<'de> for &'a mut Deserializer<R>
where
    R: std::io::Read,
{
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        #[cfg(debug_assertions)]
        trace!("SeqAccess::next_element_seed");

        if let ValueType::SeqEnd = self.parser.get_value_type() {
            self.parser.next()?;
            return Ok(None);
        }

        seed.deserialize(&mut **self).map(Some)
    }

    // fn size_hint(&self) -> Option<usize> {
    //     self.parser.get_size_hint()
    // }
}

impl<'de, 'a, R> MapAccess<'de> for &'a mut Deserializer<R>
where
    R: std::io::Read,
{
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        #[cfg(debug_assertions)]
        trace!("MapAccess::next_key_seed");

        if let ValueType::MapEnd = self.parser.get_value_type() {
            match self.parser.next() {
                Ok(()) => {},
                Err(Error::Eof) => {
                    return Ok(None);
                },
                Err(e) => {
                    return Err(From::from(e));
                },
            }
            return Ok(None);
        }

        seed.deserialize(&mut **self).map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        #[cfg(debug_assertions)]
        trace!("MapAccess::next_value_seed");

        seed.deserialize(&mut **self)
    }
}
