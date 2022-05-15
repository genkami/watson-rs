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
}

#[cfg(test)]
mod test {
    use serde_test::{assert_tokens, Token};
    use watson::language::Value::*;

    use super::*;

    #[test]
    fn test_ser_de_int() {
        assert_tokens(&Value(Int(0)), &[Token::I64(0)]);
        assert_tokens(&Value(Int(123)), &[Token::I64(123)]);
    }
}
