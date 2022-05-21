use serde::de;

use crate::error::{Error, Result};

/// Deserializer implements serde::de::Deserializer for WATSON encoding.
pub struct Deserializer<'de> {
    value: &'de watson::Value,
}

impl<'de> Deserializer<'de> {
    /// Returns a new `Deeserializer` that reads from `value`.
    pub fn new(value: &'de watson::Value) -> Self {
        Deserializer { value: value }
    }

    /// Borrows an `str` from `Value::String`.
    fn borrow_str<V>(&self, visitor: &V) -> Result<&'de str>
    where
        V: de::Visitor<'de>,
    {
        match self.value {
            &watson::Value::String(ref bytes) => {
                std::str::from_utf8(bytes.as_slice()).map_err(|_| self.invalid_utf8(visitor))
            }
            _ => Err(self.invalid_type(visitor)),
        }
    }

    fn invalid_type(&self, exp: &dyn de::Expected) -> Error {
        de::Error::invalid_type(self.ty(), exp)
    }

    fn invalid_utf8(&self, exp: &dyn de::Expected) -> Error {
        de::Error::invalid_value(de::Unexpected::Other("UTF-8 encoded string"), exp)
    }

    fn invalid_value(&self, desc: &'static str, exp: &dyn de::Expected) -> Error {
        de::Error::invalid_value(de::Unexpected::Other(desc), exp)
    }

    fn ty(&self) -> de::Unexpected<'_> {
        use watson::Value::*;
        match self.value {
            &Int(n) => de::Unexpected::Signed(n),
            &Uint(n) => de::Unexpected::Unsigned(n),
            &Float(f) => de::Unexpected::Float(f),
            &String(ref bs) => de::Unexpected::Bytes(bs.as_slice()),
            &Object(_) => de::Unexpected::Map,
            &Array(_) => de::Unexpected::Seq,
            &Bool(b) => de::Unexpected::Bool(b),
            &Nil => de::Unexpected::Unit,
        }
    }
}

impl<'a, 'de> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        todo!("any")
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.value {
            &watson::Value::Bool(b) => visitor.visit_bool(b),
            _ => Err(self.invalid_type(&visitor)),
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.value {
            &watson::Value::Int(n) => visitor.visit_i64(n),
            _ => Err(self.invalid_type(&visitor)),
        }
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.value {
            &watson::Value::Int(n) => visitor.visit_i64(n),
            _ => Err(self.invalid_type(&visitor)),
        }
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.value {
            &watson::Value::Int(n) => visitor.visit_i64(n),
            _ => Err(self.invalid_type(&visitor)),
        }
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.value {
            &watson::Value::Int(n) => visitor.visit_i64(n),
            _ => Err(self.invalid_type(&visitor)),
        }
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.value {
            &watson::Value::Uint(n) => visitor.visit_u64(n),
            _ => Err(self.invalid_type(&visitor)),
        }
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.value {
            &watson::Value::Uint(n) => visitor.visit_u64(n),
            _ => Err(self.invalid_type(&visitor)),
        }
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.value {
            &watson::Value::Uint(n) => visitor.visit_u64(n),
            _ => Err(self.invalid_type(&visitor)),
        }
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.value {
            &watson::Value::Uint(n) => visitor.visit_u64(n),
            _ => Err(self.invalid_type(&visitor)),
        }
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.value {
            &watson::Value::Float(f) => visitor.visit_f64(f),
            _ => Err(self.invalid_type(&visitor)),
        }
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.value {
            &watson::Value::Float(f) => visitor.visit_f64(f),
            _ => Err(self.invalid_type(&visitor)),
        }
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut chars = self.borrow_str(&visitor)?.chars();
        match chars.next() {
            None => Err(self.invalid_value("empty byte sequence", &visitor)),
            Some(c) => {
                if chars.next().is_some() {
                    Err(self
                        .invalid_value("string consisting of more than one characters", &visitor))
                } else {
                    visitor.visit_char(c)
                }
            }
        }
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let s = self.borrow_str(&visitor)?;
        visitor.visit_borrowed_str(s)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let s = self.borrow_str(&visitor)?;
        visitor.visit_string(s.to_owned())
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        todo!("bytes")
    }
    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        todo!("byte_buf")
    }
    fn deserialize_option<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        todo!("option")
    }
    fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        todo!("unit")
    }
    fn deserialize_unit_struct<V>(self, _name: &'static str, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        todo!("unit_struct")
    }
    fn deserialize_newtype_struct<V>(self, _name: &'static str, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        todo!("newtype_struct")
    }
    fn deserialize_seq<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        todo!("seq")
    }
    fn deserialize_tuple<V>(self, _len: usize, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        todo!("tuple")
    }
    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        todo!("tuple_struct")
    }
    fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        todo!("map")
    }
    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        todo!("struct")
    }
    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        todo!("enum")
    }
    fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        todo!("identifier")
    }
    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        todo!("ignored_any")
    }
}

#[cfg(test)]
mod test {
    use std::fmt;

    use watson::Value::*;

    use super::*;

    #[test]
    fn deserialize_bool() {
        assert_decodes(true, &Bool(true));
        assert_decodes(false, &Bool(false));
    }

    #[test]
    fn deserialize_i8() {
        assert_decodes(0_i8, &Int(0));
        assert_decodes(127_i8, &Int(127));
        assert_decodes(-128_i8, &Int(-128));
    }

    #[test]
    fn deserialize_i16() {
        assert_decodes(0_i16, &Int(0));
        assert_decodes(1_i16, &Int(1));
        assert_decodes(32767_i16, &Int(32767));
        assert_decodes(-1_i16, &Int(-1));
        assert_decodes(-32768_i16, &Int(-32768));
    }

    #[test]
    fn deserialize_i32() {
        assert_decodes(0_i32, &Int(0));
        assert_decodes(1_i32, &Int(1));
        assert_decodes(2147483647_i32, &Int(2147483647));
        assert_decodes(-1_i32, &Int(-1));
        assert_decodes(-2147483647_i32, &Int(-2147483647));
    }

    #[test]
    fn deserialize_i64() {
        assert_decodes(0_i64, &Int(0));
        assert_decodes(1_i64, &Int(1));
        assert_decodes(9223372036854775807_i64, &Int(9223372036854775807_i64));
        assert_decodes(-1_i64, &Int(-1));
        assert_decodes(-9223372036854775808_i64, &Int(-9223372036854775808_i64));
    }

    #[test]
    fn deserialize_u8() {
        assert_decodes(0_u8, &Uint(0));
        assert_decodes(1_u8, &Uint(1));
        assert_decodes(255_u8, &Uint(255));
    }

    #[test]
    fn deserialize_u16() {
        assert_decodes(0_u16, &Uint(0));
        assert_decodes(1_u16, &Uint(1));
        assert_decodes(65535_u16, &Uint(65535));
    }

    #[test]
    fn deserialize_u32() {
        assert_decodes(0_u32, &Uint(0));
        assert_decodes(1_u32, &Uint(1));
        assert_decodes(4294967295_u32, &Uint(4294967295));
    }

    #[test]
    fn deserialize_u64() {
        assert_decodes(0_u64, &Uint(0));
        assert_decodes(1_u64, &Uint(1));
        assert_decodes(18446744073709551615_u64, &Uint(18446744073709551615));
    }

    #[test]
    fn deserialize_f32() {
        assert_decoded_value_satisfies(|f: f32| f.is_nan(), &Float(f64::NAN));
        assert_decoded_value_satisfies(
            |f: f32| f.is_sign_positive() && f.is_infinite(),
            &Float(f64::INFINITY),
        );
        assert_decoded_value_satisfies(
            |f: f32| f.is_sign_negative() && f.is_infinite(),
            &Float(f64::NEG_INFINITY),
        );
        assert_decodes(1.25_f32, &Float(1.25));
        assert_decodes(-1.25_f32, &Float(-1.25));
    }

    #[test]
    fn deserialize_f64() {
        assert_decoded_value_satisfies(|f: f64| f.is_nan(), &Float(f64::NAN));
        assert_decoded_value_satisfies(
            |f: f64| f.is_sign_positive() && f.is_infinite(),
            &Float(f64::INFINITY),
        );
        assert_decoded_value_satisfies(
            |f: f64| f.is_sign_negative() && f.is_infinite(),
            &Float(f64::NEG_INFINITY),
        );
        assert_decodes(1.25_f64, &Float(1.25));
        assert_decodes(-1.25_f64, &Float(-1.25));
    }

    #[test]
    fn deserialize_char() {
        assert_decodes('a', &String(b"a".to_vec()));
        assert_decodes('あ', &String("あ".as_bytes().to_owned()));
    }

    #[test]
    fn deserialize_str() {
        let v = String(b"foobar".to_vec());
        let s: &str = deserialize(&v);
        assert_eq!(s, "foobar");
    }

    #[test]
    fn deserialize_string() {
        assert_decodes("".to_string(), &String(b"".to_vec()));
    }

    /*
     * Helper functions
     */

    fn assert_decoded_value_satisfies<'de, T, F>(f: F, v: &'de watson::Value)
    where
        T: fmt::Debug + de::Deserialize<'de>,
        F: FnOnce(T) -> bool,
    {
        assert!(f(deserialize(v)));
    }

    fn assert_decodes<'de, T>(expected: T, v: &'de watson::Value)
    where
        T: PartialEq + fmt::Debug + de::Deserialize<'de>,
    {
        assert_eq!(expected, deserialize(v));
    }

    fn deserialize<'de, T>(v: &'de watson::Value) -> T
    where
        T: fmt::Debug + de::Deserialize<'de>,
    {
        T::deserialize(&mut Deserializer::new(v)).expect("deserialization error")
    }
}
