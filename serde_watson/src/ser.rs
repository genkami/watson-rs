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

    /// Unwraps the inner value from this `Serializer`.
    pub fn into_inner(self) -> W {
        self.inner.into_inner()
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

    fn serialize_u8(self, v: u8) -> Result<()> {
        self.inner.serialize(&Value::Uint(v as u64))?;
        Ok(())
    }

    fn serialize_u16(self, v: u16) -> Result<()> {
        self.inner.serialize(&Value::Uint(v as u64))?;
        Ok(())
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
        self.inner.serialize(&Value::Uint(v as u64))?;
        Ok(())
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        self.inner.serialize(&Value::Uint(v as u64))?;
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        self.inner.serialize(&Value::Float(v as f64))?;
        Ok(())
    }

    fn serialize_f64(self, v: f64) -> Result<()> {
        self.inner.serialize(&Value::Float(v as f64))?;
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<()> {
        let mut buf = [0; 4];
        self.serialize_str(v.encode_utf8(&mut buf))
    }

    fn serialize_str(self, v: &str) -> Result<()> {
        self.serialize_bytes(v.as_bytes())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        self.inner.serialize(&Value::String(v.to_vec()))?;
        Ok(())
    }

    fn serialize_none(self) -> Result<()> {
        self.inner.serialize(&Value::Nil)?;
        Ok(())
    }

    fn serialize_some<T>(self, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<()> {
        self.inner.serialize(&Value::Nil)?;
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        self.inner.serialize(&Value::Nil)?;
        Ok(())
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<()> {
        self.serialize_str(variant)
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
        todo!("seq")
    }
    fn serialize_tuple(self, _len: usize) -> Result<Self> {
        todo!("tuple")
    }
    fn serialize_tuple_struct(self, _name: &'static str, _len: usize) -> Result<Self> {
        todo!("tuple_struct")
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
    use serde::ser::Serializer as SerdeSerializer;
    use watson::ToBytes;
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

    #[test]
    fn serialize_u8() {
        assert_encodes(0_u8, Uint(0));
        assert_encodes(1_u8, Uint(1));
        assert_encodes(255_u8, Uint(255));
    }

    #[test]
    fn serialize_u16() {
        assert_encodes(0_u16, Uint(0));
        assert_encodes(1_u16, Uint(1));
        assert_encodes(65535_u16, Uint(65535));
    }

    #[test]
    fn serialize_u32() {
        assert_encodes(0_u32, Uint(0));
        assert_encodes(1_u32, Uint(1));
        assert_encodes(4294967295_u32, Uint(4294967295));
    }

    #[test]
    fn serialize_u64() {
        assert_encodes(0_u64, Uint(0));
        assert_encodes(1_u64, Uint(1));
        assert_encodes(18446744073709551615_u64, Uint(18446744073709551615));
    }

    #[test]
    fn serialize_f32() {
        assert_encodes_to_float_satisfying(f32::NAN, |f| f.is_nan());
        assert_encodes_to_float_satisfying(f32::INFINITY, |f| {
            f.is_sign_positive() && f.is_infinite()
        });
        assert_encodes_to_float_satisfying(f32::NEG_INFINITY, |f| {
            f.is_sign_negative() && f.is_infinite()
        });
        assert_encodes(1.25_f32, Float(1.25));
        assert_encodes(-1.25_f32, Float(-1.25));
    }

    #[test]
    fn serialize_f64() {
        assert_encodes_to_float_satisfying(f64::NAN, |f| f.is_nan());
        assert_encodes_to_float_satisfying(f64::INFINITY, |f| {
            f.is_sign_positive() && f.is_infinite()
        });
        assert_encodes_to_float_satisfying(f64::NEG_INFINITY, |f| {
            f.is_sign_negative() && f.is_infinite()
        });
        assert_encodes(1.25e67_f64, Float(1.25e67));
        assert_encodes(-1.25e-67_f64, Float(-1.25e-67));
    }

    #[test]
    fn serialize_char() {
        assert_encodes('a', String(b"a".to_vec()));
        assert_encodes('あ', String("あ".to_bytes()));
    }

    #[test]
    fn serialize_str() {
        assert_encodes("", String("".to_bytes()));
        assert_encodes("x", String("x".to_bytes()));
        assert_encodes("こんにちは世界", String("こんにちは世界".to_bytes()));
        assert_encodes("Привет, мир!", String("Привет, мир!".to_bytes()));
    }

    #[test]
    fn serialize_bytes() {
        assert_encodes_to_bytes(b"", String("".to_bytes()));
        assert_encodes_to_bytes(b"1", String("1".to_bytes()));
        assert_encodes_to_bytes(b"Hello, world!", String("Hello, world!".to_bytes()));
    }

    #[test]
    fn serialize_none() {
        assert_encodes(Option::<i32>::None, Nil);
    }

    #[test]
    fn serialize_some() {
        assert_encodes(Some(true), Bool(true));
        assert_encodes(Some(123), Int(123));
    }

    #[test]
    fn serialize_unit() {
        assert_encodes((), Nil);
    }

    #[test]
    fn serialize_unit_struct() {
        use serde::Serialize;

        #[derive(Debug, Serialize)]
        struct S;

        assert_encodes(S, Nil);
    }

    #[test]
    fn serialize_unit_variant() {
        use serde::Serialize;

        #[derive(Debug, Serialize)]
        enum E {
            A,
            B,
            C,
        }

        assert_encodes(E::A, String(b"A".to_vec()));
        assert_encodes(E::B, String(b"B".to_vec()));
        assert_encodes(E::C, String(b"C".to_vec()));
    }

    /*
     * Helper functions
     */

    fn assert_encodes<T>(x: T, expected: watson::Value)
    where
        T: fmt::Debug + ser::Serialize,
    {
        let actual = encode_then_decode(x);
        assert_eq!(actual, expected);
    }

    fn assert_encodes_to_float_satisfying<T, F>(x: T, pred: F)
    where
        T: fmt::Debug + ser::Serialize,
        F: FnOnce(f64) -> bool,
    {
        match encode_then_decode(x) {
            Float(f) => {
                assert!(pred(f));
            }
            actual => {
                panic!("expected float but got {:?}", actual);
            }
        }
    }

    fn assert_encodes_to_bytes(s: &[u8], expected: watson::Value) {
        let mut buf = vec![];
        let mut ser = Serializer::new(&mut buf);
        ser.serialize_bytes(s).expect("serialization error");
        assert_eq!(decode(&mut buf.into_iter()), expected);
    }

    fn encode_then_decode<T>(x: T) -> watson::Value
    where
        T: ser::Serialize,
    {
        let mut buf = vec![];
        let mut ser = Serializer::new(&mut buf);

        x.serialize(&mut ser).expect("selialization error");

        decode(&mut buf.into_iter())
    }

    fn decode<It>(it: &mut It) -> watson::Value
    where
        It: Iterator<Item = watson::Insn>,
    {
        let mut vm = watson::VM::new();
        vm.execute_all(watson::vm::SliceTokenReader::new(
            &it.collect::<Vec<watson::Insn>>(),
        ))
        .expect("execution error");
        vm.peek_top().expect("stack should not be empty").clone()
    }
}
