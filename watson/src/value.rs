/// A value that is defined in WATSON specification.
/// See [the specification](https://github.com/genkami/watson/blob/main/doc/spec.md) for more details.
#[derive(PartialEq, Clone, Debug)]
pub enum Value {
    Int(i64),
    Uint(u64),
    Float(f64),
    String(Vec<u8>),
    Object(Map),
    Array(Vec<Value>),
    Bool(bool),
    Nil,
}

pub type Map = std::collections::HashMap<Vec<u8>, Value>;

pub use Value::*;

/// A type that can be converted directly from and to `Value`.
pub trait IsValue: Sized {
    /// Converts a `Value` into its expected type.
    fn from_value(v: Value) -> Option<Self>;

    /// Converts self into a `Value`.
    fn into_value(self) -> Value;
}

impl IsValue for Value {
    fn from_value(v: Value) -> Option<Value> {
        Some(v)
    }

    fn into_value(self) -> Value {
        self
    }
}

impl IsValue for i64 {
    fn from_value(v: Value) -> Option<i64> {
        match v {
            Int(i) => Some(i),
            _ => None,
        }
    }

    fn into_value(self) -> Value {
        Int(self)
    }
}

impl IsValue for u64 {
    fn from_value(v: Value) -> Option<u64> {
        match v {
            Uint(u) => Some(u),
            _ => None,
        }
    }

    fn into_value(self) -> Value {
        Uint(self)
    }
}

impl IsValue for f64 {
    fn from_value(v: Value) -> Option<f64> {
        match v {
            Float(f) => Some(f),
            _ => None,
        }
    }

    fn into_value(self) -> Value {
        Float(self)
    }
}

impl IsValue for Vec<u8> {
    fn from_value(v: Value) -> Option<Vec<u8>> {
        match v {
            String(s) => Some(s),
            _ => None,
        }
    }

    fn into_value(self) -> Value {
        String(self)
    }
}

impl IsValue for Map {
    fn from_value(v: Value) -> Option<Map> {
        match v {
            Object(o) => Some(o),
            _ => None,
        }
    }

    fn into_value(self) -> Value {
        Object(self)
    }
}

impl IsValue for Vec<Value> {
    fn from_value(v: Value) -> Option<Vec<Value>> {
        match v {
            Array(a) => Some(a),
            _ => None,
        }
    }

    fn into_value(self) -> Value {
        Array(self)
    }
}

impl IsValue for bool {
    fn from_value(v: Value) -> Option<bool> {
        match v {
            Bool(b) => Some(b),
            _ => None,
        }
    }

    fn into_value(self) -> Value {
        Bool(self)
    }
}

impl IsValue for () {
    fn from_value(v: Value) -> Option<()> {
        match v {
            Nil => Some(()),
            _ => None,
        }
    }

    fn into_value(self) -> Value {
        Nil
    }
}
