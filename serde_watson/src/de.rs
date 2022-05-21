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

    fn unexpected(&self, exp: &dyn de::Expected) -> Error {
        de::Error::invalid_type(self.ty(), exp)
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
            _ => Err(self.unexpected(&visitor)),
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.value {
            &watson::Value::Int(n) => visitor.visit_i64(n),
            _ => Err(self.unexpected(&visitor)),
        }
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.value {
            &watson::Value::Int(n) => visitor.visit_i64(n),
            _ => Err(self.unexpected(&visitor)),
        }
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.value {
            &watson::Value::Int(n) => visitor.visit_i64(n),
            _ => Err(self.unexpected(&visitor)),
        }
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.value {
            &watson::Value::Int(n) => visitor.visit_i64(n),
            _ => Err(self.unexpected(&visitor)),
        }
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.value {
            &watson::Value::Uint(n) => visitor.visit_u64(n),
            _ => Err(self.unexpected(&visitor)),
        }
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.value {
            &watson::Value::Uint(n) => visitor.visit_u64(n),
            _ => Err(self.unexpected(&visitor)),
        }
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.value {
            &watson::Value::Uint(n) => visitor.visit_u64(n),
            _ => Err(self.unexpected(&visitor)),
        }
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.value {
            &watson::Value::Uint(n) => visitor.visit_u64(n),
            _ => Err(self.unexpected(&visitor)),
        }
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.value {
            &watson::Value::Float(f) => visitor.visit_f64(f),
            _ => Err(self.unexpected(&visitor)),
        }
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.value {
            &watson::Value::Float(f) => visitor.visit_f64(f),
            _ => Err(self.unexpected(&visitor)),
        }
    }

    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        todo!("char")
    }
    fn deserialize_str<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        todo!("str")
    }
    fn deserialize_string<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        todo!("bytes")
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
