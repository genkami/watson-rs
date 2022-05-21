use std::error::Error as StdError;
use std::fmt;

use serde::ser;
use watson::serializer;
use watson::serializer::WriteInsn;
use watson::{Insn, Value};

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
    type SerializeSeq = SerializeSeq<'a, W>;
    type SerializeTuple = SerializeTuple<'a, W>;
    type SerializeTupleStruct = SerializeTupleStruct<'a, W>;
    type SerializeTupleVariant = SerializeTupleVariant<'a, W>;
    type SerializeMap = SerializeMap<'a, W>;
    type SerializeStruct = SerializeStruct<'a, W>;
    type SerializeStructVariant = SerializeStructVariant<'a, W>;

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

    fn serialize_newtype_struct<T>(self, name: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        self.inner.write(Insn::Onew)?;
        self.serialize_str(name)?;
        value.serialize(&mut *self)?;
        self.inner.write(Insn::Oadd)?;
        Ok(())
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        self.inner.write(Insn::Onew)?;
        self.serialize_str(variant)?;
        value.serialize(&mut *self)?;
        self.inner.write(Insn::Oadd)?;
        Ok(())
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        self.inner.write(Insn::Anew)?;
        Ok(SerializeSeq { ser: self })
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        self.serialize_seq(None)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        self.serialize_seq(None)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        self.inner.write(Insn::Onew)?;
        self.serialize_str(variant)?;
        self.serialize_seq(None)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        self.inner.write(Insn::Onew)?;
        Ok(SerializeMap { ser: self })
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        self.serialize_map(None)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        self.inner.write(Insn::Onew)?;
        self.serialize_str(variant)?;
        self.serialize_map(None)
    }
}

pub struct SerializeSeq<'a, W> {
    ser: &'a mut Serializer<W>,
}

impl<'a, W> ser::SerializeSeq for SerializeSeq<'a, W>
where
    W: WriteInsn,
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        value.serialize(&mut *self.ser)?;
        self.ser.inner.write(Insn::Aadd)?;
        Ok(())
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

type SerializeTuple<'a, W> = SerializeSeq<'a, W>;

impl<'a, W> ser::SerializeTuple for SerializeTuple<'a, W>
where
    W: WriteInsn,
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<()> {
        ser::SerializeSeq::end(self)
    }
}

type SerializeTupleStruct<'a, W> = SerializeSeq<'a, W>;

impl<'a, W> ser::SerializeTupleStruct for SerializeTupleStruct<'a, W>
where
    W: WriteInsn,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<()> {
        ser::SerializeSeq::end(self)
    }
}

type SerializeTupleVariant<'a, W> = SerializeSeq<'a, W>;

impl<'a, W> ser::SerializeTupleVariant for SerializeTupleVariant<'a, W>
where
    W: WriteInsn,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<()> {
        self.ser.inner.write(Insn::Oadd)?;
        ser::SerializeSeq::end(self)
    }
}

pub struct SerializeMap<'a, W> {
    ser: &'a mut Serializer<W>,
}

impl<'a, W> ser::SerializeMap for SerializeMap<'a, W>
where
    W: WriteInsn,
{
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        key.serialize(MapKeySerializer {
            ser: &mut *self.ser,
        })
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        value.serialize(&mut *self.ser)?;
        self.ser.inner.write(Insn::Oadd)?;
        Ok(())
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

type SerializeStruct<'a, W> = SerializeMap<'a, W>;

impl<'a, W> ser::SerializeStruct for SerializeStruct<'a, W>
where
    W: WriteInsn,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        ser::SerializeMap::serialize_key(&mut *self, key)?;
        ser::SerializeMap::serialize_value(&mut *self, value)?;
        Ok(())
    }

    fn end(self) -> Result<()> {
        ser::SerializeMap::end(self)
    }
}

type SerializeStructVariant<'a, W> = SerializeMap<'a, W>;

impl<'a, W> ser::SerializeStructVariant for SerializeStructVariant<'a, W>
where
    W: WriteInsn,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        ser::SerializeMap::serialize_entry(self, key, value)
    }

    fn end(self) -> Result<()> {
        self.ser.inner.write(Insn::Oadd)?;
        ser::SerializeMap::end(self)
    }
}

struct MapKeySerializer<'a, W> {
    ser: &'a mut Serializer<W>,
}

impl<'a, W> ser::Serializer for MapKeySerializer<'a, W>
where
    W: WriteInsn,
{
    type Ok = ();
    type Error = Error;
    type SerializeSeq = SerializeSeq<'a, W>;
    type SerializeTuple = SerializeTuple<'a, W>;
    type SerializeTupleStruct = SerializeTupleStruct<'a, W>;
    type SerializeTupleVariant = SerializeTupleVariant<'a, W>;
    type SerializeMap = SerializeMap<'a, W>;
    type SerializeStruct = SerializeStruct<'a, W>;
    type SerializeStructVariant = SerializeStructVariant<'a, W>;

    fn serialize_bool(self, v: bool) -> Result<()> {
        if v {
            self.ser.serialize_bytes(&[1])
        } else {
            self.ser.serialize_bytes(&[0])
        }
    }

    fn serialize_i8(self, v: i8) -> Result<()> {
        self.ser.serialize_bytes(&v.to_be_bytes())
    }

    fn serialize_i16(self, v: i16) -> Result<()> {
        self.ser.serialize_bytes(&v.to_be_bytes())
    }

    fn serialize_i32(self, v: i32) -> Result<()> {
        self.ser.serialize_bytes(&v.to_be_bytes())
    }

    fn serialize_i64(self, v: i64) -> Result<()> {
        self.ser.serialize_bytes(&v.to_be_bytes())
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        self.ser.serialize_bytes(&v.to_be_bytes())
    }

    fn serialize_u16(self, v: u16) -> Result<()> {
        self.ser.serialize_bytes(&v.to_be_bytes())
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
        self.ser.serialize_bytes(&v.to_be_bytes())
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        self.ser.serialize_bytes(&v.to_be_bytes())
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

    fn serialize_str(self, v: &str) -> Result<()> {
        self.ser.serialize_str(v)
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<()> {
        todo!()
    }

    fn serialize_none(self) -> Result<()> {
        todo!()
    }

    fn serialize_some<T>(self, _value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
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
        variant: &'static str,
    ) -> Result<()> {
        self.ser.serialize_str(variant)
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
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

#[cfg(test)]
mod test {
    use super::*;
    use serde::ser::Serializer as SerdeSerializer;
    use serde::Serialize;
    use watson::ToBytes;
    use watson::Value::*;
    use watson::{array, object};

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
        #[derive(Debug, Serialize)]
        struct S;

        assert_encodes(S, Nil);
    }

    #[test]
    fn serialize_unit_variant() {
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

    #[test]
    fn serialize_newtype_struct() {
        #[derive(Debug, Serialize)]
        struct S(i64);

        assert_encodes(S(123), object![S: Int(123)])
    }

    #[test]
    fn serialize_newtype_variant() {
        #[derive(Debug, Serialize)]
        enum E {
            A(bool),
        }

        assert_encodes(E::A(false), object![A: Bool(false)]);
    }

    #[test]
    fn serialize_seq() {
        assert_encodes(vec![1, 2, 3], array![Int(1), Int(2), Int(3)]);
    }

    #[test]
    fn serialize_tuple() {
        assert_encodes(
            (1, true, vec![2_u8, 3_u8]),
            array![Int(1), Bool(true), array![Uint(2), Uint(3)]],
        );
    }

    #[test]
    fn serialize_tuple_struct() {
        #[derive(Debug, Serialize)]
        struct T(i32, bool, &'static str);

        assert_encodes(
            T(123, true, "foo"),
            array![Int(123), Bool(true), String(b"foo".to_vec())],
        );
    }

    #[test]
    fn serialize_tuple_variant() {
        #[derive(Debug, Serialize)]
        enum E {
            A(i32, bool),
            B(u64, ()),
        }

        assert_encodes(E::A(123, true), object![A: array![Int(123), Bool(true)]]);
        assert_encodes(E::B(456, ()), object![B: array![Uint(456), Nil]]);
    }

    #[test]
    fn serialize_map_key_bool() {
        type HM<T> = std::collections::HashMap<bool, T>;

        assert_encodes(HM::<i32>::new(), object![]);
        assert_encodes(
            [(true, "true"), (false, "false")]
                .into_iter()
                .collect::<HM<&'static str>>(),
            object![
                [b"\x01"]: String(b"true".to_vec()),
                [b"\x00"]: String(b"false".to_vec()),
            ],
        )
    }

    #[test]
    fn serialize_map_key_i8() {
        type HM<T> = std::collections::HashMap<i8, T>;

        assert_encodes(HM::<i32>::new(), object![]);
        assert_encodes(
            [(0, "A"), (0x7f, "B"), (-0x80, "C")]
                .into_iter()
                .collect::<HM<&'static str>>(),
            object![
                [b"\x00"]: String(b"A".to_vec()),
                [b"\x7f"]: String(b"B".to_vec()),
                [b"\x80"]: String(b"C".to_vec()),
            ],
        )
    }

    #[test]
    fn serialize_map_key_i16() {
        type HM<T> = std::collections::HashMap<i16, T>;

        assert_encodes(HM::<i32>::new(), object![]);
        assert_encodes(
            [(0, "A"), (0x7fff, "B"), (-0x8000, "C")]
                .into_iter()
                .collect::<HM<&'static str>>(),
            object![
                [b"\x00\x00"]: String(b"A".to_vec()),
                [b"\x7f\xff"]: String(b"B".to_vec()),
                [b"\x80\x00"]: String(b"C".to_vec()),
            ],
        )
    }

    #[test]
    fn serialize_map_key_i32() {
        type HM<T> = std::collections::HashMap<i32, T>;

        assert_encodes(HM::<i32>::new(), object![]);
        assert_encodes(
            [(0, "A"), (0x7fff_ffff, "B"), (-0x8000_0000, "C")]
                .into_iter()
                .collect::<HM<&'static str>>(),
            object![
                [b"\x00\x00\x00\x00"]: String(b"A".to_vec()),
                [b"\x7f\xff\xff\xff"]: String(b"B".to_vec()),
                [b"\x80\x00\x00\x00"]: String(b"C".to_vec()),
            ],
        )
    }

    #[test]
    fn serialize_map_key_i64() {
        type HM<T> = std::collections::HashMap<i64, T>;

        assert_encodes(HM::<i32>::new(), object![]);
        assert_encodes(
            [
                (0, "A"),
                (0x7fff_ffff_ffff_ffff, "B"),
                (-0x8000_0000_0000_0000, "C"),
            ]
            .into_iter()
            .collect::<HM<&'static str>>(),
            object![
                [b"\x00\x00\x00\x00\x00\x00\x00\x00"]: String(b"A".to_vec()),
                [b"\x7f\xff\xff\xff\xff\xff\xff\xff"]: String(b"B".to_vec()),
                [b"\x80\x00\x00\x00\x00\x00\x00\x00"]: String(b"C".to_vec()),
            ],
        )
    }

    #[test]
    fn serialize_map_key_u8() {
        type HM<T> = std::collections::HashMap<u8, T>;

        assert_encodes(HM::<i32>::new(), object![]);
        assert_encodes(
            [(0, "A"), (0x7f, "B"), (0xff, "C")]
                .into_iter()
                .collect::<HM<&'static str>>(),
            object![
                [b"\x00"]: String(b"A".to_vec()),
                [b"\x7f"]: String(b"B".to_vec()),
                [b"\xff"]: String(b"C".to_vec()),
            ],
        )
    }

    #[test]
    fn serialize_map_key_u16() {
        type HM<T> = std::collections::HashMap<u16, T>;

        assert_encodes(HM::<i32>::new(), object![]);
        assert_encodes(
            [(0, "A"), (0x7fff, "B"), (0xffff, "C")]
                .into_iter()
                .collect::<HM<&'static str>>(),
            object![
                [b"\x00\x00"]: String(b"A".to_vec()),
                [b"\x7f\xff"]: String(b"B".to_vec()),
                [b"\xff\xff"]: String(b"C".to_vec()),
            ],
        )
    }

    #[test]
    fn serialize_map_key_u32() {
        type HM<T> = std::collections::HashMap<u32, T>;

        assert_encodes(HM::<i32>::new(), object![]);
        assert_encodes(
            [(0, "A"), (0x7fff_ffff, "B"), (0xffff_ffff, "C")]
                .into_iter()
                .collect::<HM<&'static str>>(),
            object![
                [b"\x00\x00\x00\x00"]: String(b"A".to_vec()),
                [b"\x7f\xff\xff\xff"]: String(b"B".to_vec()),
                [b"\xff\xff\xff\xff"]: String(b"C".to_vec()),
            ],
        )
    }

    #[test]
    fn serialize_map_key_u64() {
        type HM<T> = std::collections::HashMap<u64, T>;

        assert_encodes(HM::<i32>::new(), object![]);
        assert_encodes(
            [
                (0, "A"),
                (0x7fff_ffff_ffff_ffff, "B"),
                (0xffff_ffff_ffff_ffff, "C"),
            ]
            .into_iter()
            .collect::<HM<&'static str>>(),
            object![
                [b"\x00\x00\x00\x00\x00\x00\x00\x00"]: String(b"A".to_vec()),
                [b"\x7f\xff\xff\xff\xff\xff\xff\xff"]: String(b"B".to_vec()),
                [b"\xff\xff\xff\xff\xff\xff\xff\xff"]: String(b"C".to_vec()),
            ],
        )
    }

    #[test]
    fn serialize_map_key_str() {
        type HM<T> = std::collections::HashMap<&'static str, T>;

        assert_encodes(HM::<i32>::new(), object![]);
        assert_encodes(
            [("foo", "bar")].into_iter().collect::<HM<&'static str>>(),
            object![foo: String(b"bar".to_vec())],
        );
        assert_encodes(
            [("foo", 123), ("bar", 456)]
                .into_iter()
                .collect::<HM<i32>>(),
            object![foo: Int(123), bar: Int(456)],
        );
    }

    #[test]
    fn serialize_map_key_unit_variant() {
        #[derive(Eq, PartialEq, Hash, Debug, Serialize)]
        enum Key {
            A,
            B,
        }
        type HM<T> = std::collections::HashMap<Key, T>;

        assert_encodes(
            [(Key::A, 123), (Key::B, 456)]
                .into_iter()
                .collect::<HM<i32>>(),
            object![A: Int(123), B: Int(456)],
        );
    }

    #[test]
    fn serialize_map_key_newtype_struct() {
        #[derive(Eq, PartialEq, Hash, Debug, Serialize)]
        struct Key(&'static str);
        type HM<T> = std::collections::HashMap<Key, T>;

        assert_encodes(
            [(Key("foo"), 123), (Key("bar"), 456)]
                .into_iter()
                .collect::<HM<i32>>(),
            object![foo: Int(123), bar: Int(456)],
        );
    }

    #[test]
    fn serialize_struct() {
        #[derive(Debug, Serialize)]
        struct S {
            f1: i32,
            f2: &'static str,
            f3: bool,
        }

        assert_encodes(
            S {
                f1: 123,
                f2: "abc",
                f3: true,
            },
            object![f1: Int(123), f2: String(b"abc".to_vec()), f3: Bool(true)],
        )
    }

    #[test]
    fn serialize_struct_variant() {
        #[derive(Debug, Serialize)]
        enum E {
            S { f1: u32, f2: i32 },
        }

        assert_encodes(
            E::S { f1: 123, f2: 456 },
            object![S: object![f1: Uint(123), f2: Int(456)]],
        );
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
