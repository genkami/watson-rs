use std::fmt;
use std::ops;

use serde::de;
use serde::de::{Deserialize, Deserializer, MapAccess, SeqAccess, Visitor};
use serde::ser::{Serialize, SerializeMap, SerializeSeq, Serializer};
use watson::language;
use watson::language::Value::*;

/// Value implements Serialize and Deserialize for `value::Value`.
#[derive(PartialEq, Clone, Debug)]
pub struct Value {
    value: language::Value,
}

impl Value {
    /// Returns a new `Value`.
    pub fn new(v: language::Value) -> Self {
        Value { value: v }
    }

    /// Returns underlying `language::Value`.
    pub fn into_watson(self) -> language::Value {
        self.value
    }
}

impl From<language::Value> for Value {
    fn from(v: language::Value) -> Self {
        Value::new(v)
    }
}

impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        ValueRef::new(&self.value).serialize(serializer)
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

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut map = language::Map::with_capacity(access.size_hint().unwrap_or(0));
        while let Some((key, value)) = access.next_entry::<ObjectKey, Value>()? {
            map.insert(key.into_bytes(), value.into_watson());
        }
        Ok(Object(map).into())
    }

    fn visit_seq<S>(self, mut access: S) -> Result<Self::Value, S::Error>
    where
        S: SeqAccess<'de>,
    {
        let mut arr = Vec::with_capacity(access.size_hint().unwrap_or(0));
        while let Some(elem) = access.next_element::<Value>()? {
            arr.push(elem.into_watson());
        }
        Ok(Array(arr).into())
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Bool(v).into())
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Nil.into())
    }
}

pub struct ValueRef<'a> {
    value: &'a language::Value,
}

impl<'a> ValueRef<'a> {
    /// Returns a new `ValueRef` that points to the given `Value`.
    pub fn new(v: &'a language::Value) -> Self {
        ValueRef { value: v }
    }
}

impl<'a> AsRef<language::Value> for ValueRef<'a> {
    fn as_ref(&self) -> &language::Value {
        self.value
    }
}

impl<'a> ops::Deref for ValueRef<'a> {
    type Target = language::Value;

    fn deref(&self) -> &language::Value {
        self.value
    }
}

impl<'a> Serialize for ValueRef<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.value {
            &Int(n) => serializer.serialize_i64(n),
            &Uint(n) => serializer.serialize_u64(n),
            &Float(f) => serializer.serialize_f64(f),
            &String(ref s) => serializer.serialize_bytes(s),
            &Object(ref map) => {
                let mut map_ser = serializer.serialize_map(Some(map.len()))?;
                for (k, v) in map {
                    map_ser.serialize_entry(&ObjectKeyRef(k), &ValueRef::new(v))?;
                }
                map_ser.end()
            }
            &Array(ref arr) => {
                let mut seq_ser = serializer.serialize_seq(Some(arr.len()))?;
                for i in arr {
                    seq_ser.serialize_element(&ValueRef::new(i))?;
                }
                seq_ser.end()
            }
            &Bool(b) => serializer.serialize_bool(b),
            &Nil => serializer.serialize_none(),
        }
    }
}

struct ObjectKeyRef<'a>(&'a language::ObjectKey);

impl<'a> Serialize for ObjectKeyRef<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(self.0)
    }
}

struct ObjectKey(language::ObjectKey);

impl ObjectKey {
    fn into_bytes(self) -> language::ObjectKey {
        self.0
    }
}

impl<'de> Deserialize<'de> for ObjectKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_bytes(BytesVisitor)
    }
}

struct BytesVisitor;

impl<'de> Visitor<'de> for BytesVisitor {
    type Value = ObjectKey;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("bytes")
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
        Ok(ObjectKey(v.to_vec()))
    }

    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(ObjectKey(v))
    }
}

#[cfg(test)]
mod test {
    use serde_test::{assert_tokens, Token};
    use watson::language::Map;
    use watson::language::Value::*;

    use super::*;

    #[test]
    fn ser_de_int() {
        assert_tokens(&Value::new(Int(0)), &[Token::I64(0)]);
        assert_tokens(&Value::new(Int(123)), &[Token::I64(123)]);
        assert_tokens(&Value::new(Int(-123)), &[Token::I64(-123)]);
    }

    #[test]
    fn ser_de_uint() {
        assert_tokens(&Value::new(Uint(0)), &[Token::U64(0)]);
        assert_tokens(&Value::new(Uint(123)), &[Token::U64(123)]);
        assert_tokens(
            &Value::new(Uint(0xdead_beef_fefe_aaaa)),
            &[Token::U64(0xdead_beef_fefe_aaaa)],
        );
    }

    #[test]
    fn ser_de_float() {
        assert_tokens(&Value::new(Float(0.0)), &[Token::F64(0.0)]);
        assert_tokens(&Value::new(Float(1.23e45)), &[Token::F64(1.23e45)]);
        assert_tokens(&Value::new(Float(6.78e-91)), &[Token::F64(6.78e-91)]);
    }

    #[test]
    fn ser_de_string() {
        assert_tokens(&Value::new(String(b"".to_vec())), &[Token::Bytes(b"")]);
        assert_tokens(&Value::new(String(b"a".to_vec())), &[Token::Bytes(b"a")]);
        assert_tokens(
            &Value::new(String(b"hello world!".to_vec())),
            &[Token::Bytes(b"hello world!")],
        );
    }

    #[test]
    fn ser_de_object() {
        assert_tokens(
            &Value::new(Object(Map::new())),
            &[Token::Map { len: Some(0) }, Token::MapEnd],
        );
        assert_tokens(
            &Value::new(Object(
                vec![(b"value".to_vec(), Int(123))].into_iter().collect(),
            )),
            &[
                Token::Map { len: Some(1) },
                Token::Bytes(b"value"),
                Token::I64(123),
                Token::MapEnd,
            ],
        );
    }

    #[test]
    fn ser_de_array() {
        assert_tokens(
            &Value::new(Array(vec![])),
            &[Token::Seq { len: Some(0) }, Token::SeqEnd],
        );
        assert_tokens(
            &Value::new(Array(vec![Int(123)])),
            &[Token::Seq { len: Some(1) }, Token::I64(123), Token::SeqEnd],
        );
        assert_tokens(
            &Value::new(Array(vec![Int(123), String(b"hello".to_vec())])),
            &[
                Token::Seq { len: Some(2) },
                Token::I64(123),
                Token::Bytes(b"hello"),
                Token::SeqEnd,
            ],
        );
    }

    #[test]
    fn ser_de_bool() {
        assert_tokens(&Value::new(Bool(true)), &[Token::Bool(true)]);
        assert_tokens(&Value::new(Bool(false)), &[Token::Bool(false)]);
    }

    #[test]
    fn ser_de_nil() {
        assert_tokens(&Value::new(Nil), &[Token::None]);
    }
}
