use std::error::Error as StdError;
use std::fmt;

use serde::ser;
use watson::serializer;
use watson::serializer::WriteInsn;
use watson::Value;

#[derive(Debug)]
pub struct Error;

impl fmt::Display for Error {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl StdError for Error {
    // TODO
}

impl ser::Error for Error {
    fn custom<T>(_msg: T) -> Self {
        todo!()
    }
}

impl From<watson::Error> for Error {
    fn from(_v: watson::Error) -> Self {
        todo!()
    }
}

type Result<T> = std::result::Result<T, Error>;

/// Serializer implements serde::ser::Serializer for WATSON encoding.
pub struct Serializer<W> {
    inner: serializer::Serializer<W>,
}

impl<W> Serializer<W> {
    /// Returns a new `Serializer` that writes to the given writer.
    pub fn new(writer: W) -> Self {
        Serializer {
            inner: serializer::Serializer::new(writer),
        }
    }
}

impl<'a, W> ser::Serializer for &'a mut Serializer<W>
where
    W: WriteInsn,
{
    type Ok = ();
    type Error = Error;
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<()> {
        self.inner.serialize(&Value::Bool(v))?;
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<()> {
        self.inner.serialize(&Value::Int(v as i64))?;
        Ok(())
    }

    fn serialize_i16(self, v: i16) -> Result<()> {
        self.inner.serialize(&Value::Int(v as i64))?;
        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<()> {
        self.inner.serialize(&Value::Int(v as i64))?;
        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<()> {
        self.inner.serialize(&Value::Int(v as i64))?;
        Ok(())
    }

    fn serialize_u8(self, _v: u8) -> Result<()> {
        todo!()
    }
    fn serialize_u16(self, _v: u16) -> Result<()> {
        todo!()
    }
    fn serialize_u32(self, _v: u32) -> Result<()> {
        todo!()
    }
    fn serialize_u64(self, _v: u64) -> Result<()> {
        todo!()
    }
    fn serialize_f32(self, _v: f32) -> Result<()> {
        todo!()
    }
    fn serialize_f64(self, _v: f64) -> Result<()> {
        todo!()
    }
    fn serialize_char(self, _v: char) -> Result<()> {
        todo!()
    }
    fn serialize_str(self, _v: &str) -> Result<()> {
        todo!()
    }
    fn serialize_bytes(self, _v: &[u8]) -> Result<()> {
        todo!()
    }
    fn serialize_none(self) -> Result<()> {
        todo!()
    }
    fn serialize_some<T: ?Sized>(self, _value: &T) -> Result<()> {
        todo!()
    }
    fn serialize_unit(self) -> Result<()> {
        todo!()
    }
    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        todo!()
    }
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<()> {
        todo!()
    }
    fn serialize_newtype_struct<T: ?Sized>(self, _name: &'static str, _value: &T) -> Result<()> {
        todo!()
    }
    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<()> {
        todo!()
    }
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self> {
        todo!()
    }
    fn serialize_tuple(self, _len: usize) -> Result<Self> {
        todo!()
    }
    fn serialize_tuple_struct(self, _name: &'static str, _len: usize) -> Result<Self> {
        todo!()
    }
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self> {
        todo!()
    }
    fn serialize_map(self, _len: Option<usize>) -> Result<Self> {
        todo!()
    }
    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self> {
        todo!()
    }
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self> {
        todo!()
    }
}

impl<'a, W> ser::SerializeSeq for &'a mut Serializer<W>
where
    W: WriteInsn,
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, _value: &T) -> Result<()>
    where
        T: ser::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<()> {
        todo!()
    }
}

impl<'a, W> ser::SerializeTuple for &'a mut Serializer<W>
where
    W: WriteInsn,
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, _value: &T) -> Result<()> {
        todo!()
    }

    fn end(self) -> Result<()> {
        todo!()
    }
}

impl<'a, W> ser::SerializeTupleStruct for &'a mut Serializer<W>
where
    W: WriteInsn,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _value: &T) -> Result<()> {
        todo!()
    }

    fn end(self) -> Result<()> {
        todo!()
    }
}

impl<'a, W> ser::SerializeTupleVariant for &'a mut Serializer<W>
where
    W: WriteInsn,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _value: &T) -> Result<()> {
        todo!()
    }

    fn end(self) -> Result<()> {
        todo!()
    }
}

impl<'a, W> ser::SerializeMap for &'a mut Serializer<W>
where
    W: WriteInsn,
{
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, _key: &T) -> Result<()>
    where
        T: ser::Serialize,
    {
        todo!()
    }

    fn serialize_value<T: ?Sized>(&mut self, _value: &T) -> Result<()>
    where
        T: ser::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<()> {
        todo!()
    }
}

impl<'a, W> ser::SerializeStruct for &'a mut Serializer<W>
where
    W: WriteInsn,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _key: &'static str, _value: &T) -> Result<()> {
        todo!()
    }

    fn end(self) -> Result<()> {
        todo!()
    }
}

impl<'a, W> ser::SerializeStructVariant for &'a mut Serializer<W>
where
    W: WriteInsn,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _key: &'static str, _value: &T) -> Result<()>
    where
        T: ser::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<()> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use watson::Value::*;

    #[test]
    fn serialize_bool() {
        assert_encodes(true, Bool(true));
        assert_encodes(false, Bool(false));
    }

    #[test]
    fn serialize_i8() {
        assert_encodes(0_i8, Int(0));
        assert_encodes(1_i8, Int(1));
        assert_encodes(127_i8, Int(127));
        assert_encodes(-1_i8, Int(-1));
        assert_encodes(-128_i8, Int(-128));
    }

    #[test]
    fn serialize_i16() {
        assert_encodes(0_i16, Int(0));
        assert_encodes(1_i16, Int(1));
        assert_encodes(32767_i16, Int(32767));
        assert_encodes(-1_i16, Int(-1));
        assert_encodes(-32768_i16, Int(-32768));
    }

    #[test]
    fn serialize_i32() {
        assert_encodes(0_i32, Int(0));
        assert_encodes(1_i32, Int(1));
        assert_encodes(2147483647_i32, Int(2147483647));
        assert_encodes(-1_i32, Int(-1));
        assert_encodes(-2147483647_i32, Int(-2147483647));
    }

    #[test]
    fn serialize_i64() {
        assert_encodes(0_i64, Int(0));
        assert_encodes(1_i64, Int(1));
        assert_encodes(9223372036854775807_i64, Int(9223372036854775807_i64));
        assert_encodes(-1_i64, Int(-1));
        assert_encodes(-9223372036854775808_i64, Int(-9223372036854775808_i64));
    }

    /*
     * Helper functions
     */

    fn assert_encodes<T>(x: T, expected: watson::Value)
    where
        T: fmt::Debug + ser::Serialize,
    {
        let mut buf = vec![];
        let mut ser = Serializer::new(&mut buf);

        x.serialize(&mut ser).expect("selialization error");

        let mut vm = watson::VM::new();
        for insn in buf.into_iter() {
            let token = watson::Token {
                insn: insn,
                location: watson::Location::unknown(),
            };
            vm.execute(token).expect("execution error");
        }
        let actual = vm.peek_top().expect("stack should not be empty");
        assert_eq!(actual, &expected);
    }
}
