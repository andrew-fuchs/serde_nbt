use crate::error::{Error, Result};
use serde::{ser, Serialize};

pub fn to_writer<W, T>(output: W, value: T) -> Result<()>
where
    W: std::io::Write,
    T: Serialize,
{
    let mut serializer = Serializer::new(output);
    value.serialize(&mut serializer)?;
    Ok(())
}

pub struct Serializer<W> {
    _output: W,
}

impl<W> Serializer<W>
where
    W: std::io::Write,
{
    pub fn new(output: W) -> Self {
        Self { _output: output }
    }
}

impl<'ser, W> ser::Serializer for &'ser mut Serializer<W> {
    type Ok = ();
    type Error = Error;
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, _v: bool) -> Result<Self::Ok> {
        // TODO: convert to byte
        todo!()
    }

    fn serialize_i8(self, _v: i8) -> Result<Self::Ok> {
        todo!()
    }

    fn serialize_i16(self, _v: i16) -> Result<Self::Ok> {
        todo!()
    }

    fn serialize_i32(self, _v: i32) -> Result<Self::Ok> {
        todo!()
    }

    fn serialize_i64(self, _v: i64) -> Result<Self::Ok> {
        todo!()
    }

    fn serialize_u8(self, _v: u8) -> Result<Self::Ok> {
        todo!()
    }

    fn serialize_u16(self, _v: u16) -> Result<Self::Ok> {
        todo!()
    }

    fn serialize_u32(self, _v: u32) -> Result<Self::Ok> {
        todo!()
    }

    fn serialize_u64(self, _v: u64) -> Result<Self::Ok> {
        todo!()
    }

    fn serialize_f32(self, _v: f32) -> Result<Self::Ok> {
        todo!()
    }

    fn serialize_f64(self, _v: f64) -> Result<Self::Ok> {
        todo!()
    }

    fn serialize_char(self, _v: char) -> Result<Self::Ok> {
        todo!()
    }

    fn serialize_str(self, _v: &str) -> Result<Self::Ok> {
        todo!()
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok> {
        todo!()
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        todo!()
    }

    fn serialize_some<T: ?Sized>(self, _value: &T) -> Result<Self::Ok>
    where
        T: Serialize,
    {
        todo!()
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        todo!()
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok> {
        todo!()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok> {
        todo!()
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<Self::Ok>
    where
        T: Serialize,
    {
        todo!()
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok>
    where
        T: Serialize,
    {
        todo!()
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        todo!()
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        todo!()
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        todo!()
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        todo!()
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        todo!()
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        todo!()
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        todo!()
    }
}

impl<'ser, W> ser::SerializeSeq for &'ser mut Serializer<W> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, _value: &T) -> Result<Self::Ok>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok> {
        todo!()
    }
}

impl<'ser, W> ser::SerializeTuple for &'ser mut Serializer<W> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, _value: &T) -> Result<Self::Ok>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok> {
        todo!()
    }
}

impl<'ser, W> ser::SerializeTupleStruct for &'ser mut Serializer<W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _value: &T) -> Result<()>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok> {
        todo!()
    }
}

impl<'ser, W> ser::SerializeTupleVariant for &'ser mut Serializer<W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _value: &T) -> Result<Self::Ok>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok> {
        todo!()
    }
}

impl<'ser, W> ser::SerializeMap for &'ser mut Serializer<W> {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, _key: &T) -> Result<Self::Ok>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn serialize_value<T: ?Sized>(&mut self, _value: &T) -> Result<Self::Ok>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok> {
        todo!()
    }
}

impl<'ser, W> ser::SerializeStruct for &'ser mut Serializer<W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _key: &'static str, _value: &T) -> Result<Self::Ok>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok> {
        todo!()
    }
}

impl<'ser, W> ser::SerializeStructVariant for &'ser mut Serializer<W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _key: &'static str, _value: &T) -> Result<Self::Ok>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok> {
        todo!()
    }
}
