use std::fmt;

use serde::de;
use serde::de::{Deserialize, Deserializer, Visitor};
use serde::ser::{Serialize, Serializer};
use watson::language;
use watson::language::Value::*;

/// Value implements Serialize and Deserialize for `value::Value`.
#[derive(PartialEq, Clone, Debug)]
pub struct Value(language::Value);

impl Value {
    /// Returns underlying `language::Value`.
    pub fn into_watson(self) -> language::Value {
        self.0
    }
}

impl From<language::Value> for Value {
    fn from(v: language::Value) -> Self {
        Value(v)
    }
}

impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match &self.0 {
            &Int(n) => serializer.serialize_i64(n),
            &Uint(n) => serializer.serialize_u64(n),
            &Float(f) => serializer.serialize_f64(f),
            &String(ref s) => serializer.serialize_bytes(s),
            _ => todo!(),
        }
    }
}

impl<'de> Deserialize<'de> for Value {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(ValueVisitor)
    }
}

struct ValueVisitor;

impl<'de> Visitor<'de> for ValueVisitor {
    type Value = Value;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("bool, integer, float, string, bytes, seq, or map")
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Int(v).into())
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Uint(v).into())
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Float(v).into())
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_bytes(v.as_bytes())
    }

    fn visit_string<E>(self, v: std::string::String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_byte_buf(v.into_bytes())
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(String(v.to_owned()).into())
    }

    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(String(v).into())
    }
}

#[cfg(test)]
mod test {
    use serde_test::{assert_tokens, Token};
    use watson::language::Value::*;

    use super::*;

    #[test]
    fn ser_de_int() {
        assert_tokens(&Value(Int(0)), &[Token::I64(0)]);
        assert_tokens(&Value(Int(123)), &[Token::I64(123)]);
        assert_tokens(&Value(Int(-123)), &[Token::I64(-123)]);
    }

    #[test]
    fn ser_de_uint() {
        assert_tokens(&Value(Uint(0)), &[Token::U64(0)]);
        assert_tokens(&Value(Uint(123)), &[Token::U64(123)]);
        assert_tokens(
            &Value(Uint(0xdead_beef_fefe_aaaa)),
            &[Token::U64(0xdead_beef_fefe_aaaa)],
        );
    }

    #[test]
    fn ser_de_float() {
        assert_tokens(&Value(Float(0.0)), &[Token::F64(0.0)]);
        assert_tokens(&Value(Float(1.23e45)), &[Token::F64(1.23e45)]);
        assert_tokens(&Value(Float(6.78e-91)), &[Token::F64(6.78e-91)]);
    }

    #[test]
    fn ser_de_string() {
        assert_tokens(&Value(String(b"".to_vec())), &[Token::Bytes(b"")]);
        assert_tokens(&Value(String(b"a".to_vec())), &[Token::Bytes(b"a")]);
        assert_tokens(
            &Value(String(b"hello world!".to_vec())),
            &[Token::Bytes(b"hello world!")],
        );
    }
}
