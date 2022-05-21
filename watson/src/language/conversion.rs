use super::*;
use Value::*;

macro_rules! impl_from_int_for_value {
    ( $( $t:ty ),* ) => {
        $(
            impl From<$t> for Value {
                fn from(v: $t) -> Value {
                    Int(v as i64)
                }
            }
        )*
    };
}

impl_from_int_for_value!(i8, i16, i32, i64, i128, isize);

macro_rules! impl_from_uint_for_value {
    ( $( $t:ty ),* ) => {
        $(
            impl From<$t> for Value {
                fn from(v: $t) -> Value {
                    Uint(v as u64)
                }
            }
        )*
    };
}

impl_from_uint_for_value!(u8, u16, u32, u64, u128, usize);

macro_rules! impl_from_float_for_value {
    ( $( $t:ty ),* ) => {
        $(
            impl From<$t> for Value {
                fn from(v: $t) -> Value {
                    Float(v as f64)
                }
            }
        )*
    };
}

impl_from_float_for_value!(f32, f64);

impl From<Bytes> for Value {
    fn from(v: Bytes) -> Value {
        String(v)
    }
}

impl From<std::string::String> for Value {
    fn from(v: std::string::String) -> Value {
        String(v.into_bytes())
    }
}

impl From<Map> for Value {
    fn from(v: Map) -> Value {
        Object(v)
    }
}

impl From<Vec<Value>> for Value {
    fn from(v: Vec<Value>) -> Value {
        Array(v)
    }
}

impl From<bool> for Value {
    fn from(v: bool) -> Value {
        Bool(v)
    }
}

impl From<()> for Value {
    fn from(_: ()) -> Value {
        Nil
    }
}

/// A type that can be converted directly from and to `Value`.
/// This is different from From<Value> and Into<Value> in that the values of these these types are "identical" to `Value`.
pub trait IsValue: Into<Value> {
    /// Converts a `Value` into its expected type.
    fn from_value(v: Value) -> Option<Self>;

    /// Converts self into a `Value`.
    fn into_value(self) -> Value {
        self.into()
    }
}

impl IsValue for Value {
    fn from_value(v: Value) -> Option<Value> {
        Some(v)
    }
}

impl IsValue for i64 {
    fn from_value(v: Value) -> Option<i64> {
        match v {
            Int(i) => Some(i),
            _ => None,
        }
    }
}

impl IsValue for u64 {
    fn from_value(v: Value) -> Option<u64> {
        match v {
            Uint(u) => Some(u),
            _ => None,
        }
    }
}

impl IsValue for f64 {
    fn from_value(v: Value) -> Option<f64> {
        match v {
            Float(f) => Some(f),
            _ => None,
        }
    }
}

impl IsValue for Bytes {
    fn from_value(v: Value) -> Option<Bytes> {
        match v {
            String(s) => Some(s),
            _ => None,
        }
    }
}

impl IsValue for Map {
    fn from_value(v: Value) -> Option<Map> {
        match v {
            Object(o) => Some(o),
            _ => None,
        }
    }
}

impl IsValue for Vec<Value> {
    fn from_value(v: Value) -> Option<Vec<Value>> {
        match v {
            Array(a) => Some(a),
            _ => None,
        }
    }
}

impl IsValue for bool {
    fn from_value(v: Value) -> Option<bool> {
        match v {
            Bool(b) => Some(b),
            _ => None,
        }
    }
}

impl IsValue for () {
    fn from_value(v: Value) -> Option<()> {
        match v {
            Nil => Some(()),
            _ => None,
        }
    }
}

/// A type that can be converted to `Bytes`.
pub trait ToBytes {
    /// Converts `self` to `Bytes`.
    fn to_bytes(self) -> Bytes;
}

impl ToBytes for Bytes {
    fn to_bytes(self) -> Bytes {
        self
    }
}

impl<'a> ToBytes for &'a Bytes {
    fn to_bytes(self) -> Bytes {
        self.to_vec()
    }
}

impl ToBytes for char {
    fn to_bytes(self) -> Bytes {
        let mut buf = [0; 4];
        let len = self.encode_utf8(&mut buf).len();
        buf[..len].to_bytes()
    }
}

impl ToBytes for std::string::String {
    fn to_bytes(self) -> Bytes {
        self.to_owned().into_bytes()
    }
}

impl<'a> ToBytes for &'a str {
    fn to_bytes(self) -> Bytes {
        self.as_bytes().to_vec()
    }
}

impl ToBytes for u8 {
    fn to_bytes(self) -> Bytes {
        vec![self]
    }
}

impl<'a> ToBytes for &'a [u8] {
    fn to_bytes(self) -> Bytes {
        self.to_vec()
    }
}

macro_rules! impl_to_bytes_for_array {
    ( $( $n:tt ),* $(,)? ) => {
        $(
            impl<'a> ToBytes for &'a [u8; $n] {
                fn to_bytes(self) -> Bytes {
                    self.to_vec()
                }
            }
        )*
    };
}

impl_to_bytes_for_array! {
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
    17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32,
    33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48,
    49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64,
}
