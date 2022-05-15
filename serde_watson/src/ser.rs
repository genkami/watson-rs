use std::error::Error as StdError;
use std::fmt;

use serde::ser;
use watson::serializer::WriteInsn;

use crate::value::Value;

#[derive(Debug)]
pub struct Error;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl StdError for Error {
    // TODO
}

impl ser::Error for Error {
    fn custom<T>(msg: T) -> Self {
        todo!()
    }
}

type Result<T> = std::result::Result<T, Error>;

/// Serializer implements serde::ser::Serializer for WATSON encoding.
pub struct Serializer<W> {
    writer: W,
}

impl<W> Serializer<W> {
    /// Returns a new `Serializer` that writes to the given writer.
    pub fn new(writer: W) -> Self {
        Serializer { writer: writer }
    }
}

impl<W> ser::Serializer for Serializer<W>
where
    W: WriteInsn,
{
    type Ok = Value;
    type Error = Error;
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<Value> {
        todo!()
    }

    fn serialize_i8(self, v: i8) -> Result<Value> {
        todo!()
    }

    fn serialize_i16(self, v: i16) -> Result<Value> {
        todo!()
    }

    fn serialize_i32(self, v: i32) -> Result<Value> {
        todo!()
    }
    fn serialize_i64(self, v: i64) -> Result<Value> {
        todo!()
    }
    fn serialize_u8(self, v: u8) -> Result<Value> {
        todo!()
    }
    fn serialize_u16(self, v: u16) -> Result<Value> {
        todo!()
    }
    fn serialize_u32(self, v: u32) -> Result<Value> {
        todo!()
    }
    fn serialize_u64(self, v: u64) -> Result<Value> {
        todo!()
    }
    fn serialize_f32(self, v: f32) -> Result<Value> {
        todo!()
    }
    fn serialize_f64(self, v: f64) -> Result<Value> {
        todo!()
    }
    fn serialize_char(self, v: char) -> Result<Value> {
        todo!()
    }
    fn serialize_str(self, v: &str) -> Result<Value> {
        todo!()
    }
    fn serialize_bytes(self, v: &[u8]) -> Result<Value> {
        todo!()
    }
    fn serialize_none(self) -> Result<Value> {
        todo!()
    }
    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Value> {
        todo!()
    }
    fn serialize_unit(self) -> Result<Value> {
        todo!()
    }
    fn serialize_unit_struct(self, name: &'static str) -> Result<Value> {
        todo!()
    }
    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Value> {
        todo!()
    }
    fn serialize_newtype_struct<T: ?Sized>(self, name: &'static str, value: &T) -> Result<Value> {
        todo!()
    }
    fn serialize_newtype_variant<T: ?Sized>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Value> {
        todo!()
    }
    fn serialize_seq(self, len: Option<usize>) -> Result<Self> {
        todo!()
    }
    fn serialize_tuple(self, len: usize) -> Result<Self> {
        todo!()
    }
    fn serialize_tuple_struct(self, name: &'static str, len: usize) -> Result<Self> {
        todo!()
    }
    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self> {
        todo!()
    }
    fn serialize_map(self, len: Option<usize>) -> Result<Self> {
        todo!()
    }
    fn serialize_struct(self, name: &'static str, len: usize) -> Result<Self> {
        todo!()
    }
    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self> {
        todo!()
    }
}

impl<W> ser::SerializeSeq for Serializer<W>
where
    W: WriteInsn,
{
    type Ok = Value;
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: ser::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Value> {
        todo!()
    }
}

impl<W> ser::SerializeTuple for Serializer<W>
where
    W: WriteInsn,
{
    type Ok = Value;
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<()> {
        todo!()
    }

    fn end(self) -> Result<Value> {
        todo!()
    }
}

impl<W> ser::SerializeTupleStruct for Serializer<W>
where
    W: WriteInsn,
{
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<()> {
        todo!()
    }

    fn end(self) -> Result<Value> {
        todo!()
    }
}

impl<W> ser::SerializeTupleVariant for Serializer<W>
where
    W: WriteInsn,
{
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<()> {
        todo!()
    }

    fn end(self) -> Result<Value> {
        todo!()
    }
}

impl<W> ser::SerializeMap for Serializer<W>
where
    W: WriteInsn,
{
    type Ok = Value;
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<()>
    where
        T: ser::Serialize,
    {
        todo!()
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: ser::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Value> {
        todo!()
    }
}

impl<W> ser::SerializeStruct for Serializer<W>
where
    W: WriteInsn,
{
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<()> {
        todo!()
    }

    fn end(self) -> Result<Value> {
        todo!()
    }
}

impl<W> ser::SerializeStructVariant for Serializer<W>
where
    W: WriteInsn,
{
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ser::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Value> {
        todo!()
    }
}
