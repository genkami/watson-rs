use serde::de;

use crate::error::{Error, ErrorKind, Result};

/// Deserializer implements serde::de::Deserializer for WATSON encoding.
pub struct Deserializer<'de> {
    value: &'de watson::Value,
}

impl<'de> Deserializer<'de> {
    /// Returns a new `Deserializer` that reads from `value`.
    pub fn new(value: &'de watson::Value) -> Self {
        Deserializer { value: value }
    }

    /// Borrows an `str` from `Value::String`.
    fn borrow_str<V>(&self, visitor: &V) -> Result<&'de str>
    where
        V: de::Visitor<'de>,
    {
        match self.value {
            &watson::Value::String(ref bytes) => try_borrow_str(bytes, visitor),
            _ => Err(self.invalid_type(visitor)),
        }
    }

    fn invalid_type(&self, exp: &dyn de::Expected) -> Error {
        invalid_type(self.ty(), exp)
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

impl<'a, 'de> de::Deserializer<'de> for &'a Deserializer<'de> {
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
            None => Err(invalid_value("empty byte sequence", &visitor)),
            Some(c) => {
                if chars.next().is_some() {
                    Err(invalid_value(
                        "string consisting of more than one characters",
                        &visitor,
                    ))
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

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.value {
            &watson::Value::String(ref bytes) => visitor.visit_borrowed_bytes(bytes.as_slice()),
            _ => Err(self.invalid_type(&visitor)),
        }
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.value {
            &watson::Value::String(ref bytes) => visitor.visit_byte_buf(bytes.clone()),
            _ => Err(self.invalid_type(&visitor)),
        }
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.value {
            &watson::Value::Nil => visitor.visit_none(),
            _ => visitor.visit_some(self),
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.value {
            &watson::Value::Nil => visitor.visit_unit(),
            _ => Err(self.invalid_type(&visitor)),
        }
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.value {
            &watson::Value::Array(ref vec) => visitor.visit_seq(SeqAccess::new(&vec)),
            _ => Err(self.invalid_type(&visitor)),
        }
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.value {
            &watson::Value::Object(ref map) => visitor.visit_map(MapAccess::new(&map)),
            _ => Err(self.invalid_type(&visitor)),
        }
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.value {
            &watson::Value::Array(ref vec) => visitor.visit_seq(SeqAccess::new(&vec)),
            &watson::Value::Object(ref map) => visitor.visit_map(MapAccess::new(&map)),
            _ => Err(self.invalid_type(&visitor)),
        }
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.value {
            &watson::Value::String(ref name) => visitor.visit_enum(UnitVariantAccess::new(name)),
            &watson::Value::Object(ref map) => visitor.visit_enum(NonUnitVariantAccess::new(map)),
            _ => Err(self.invalid_type(&visitor)),
        }
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        todo!("ignored_any")
    }
}

struct SeqAccess<'de> {
    arr: &'de Vec<watson::Value>,
    next: usize,
}

impl<'de> SeqAccess<'de> {
    fn new(arr: &'de Vec<watson::Value>) -> Self {
        SeqAccess { arr: arr, next: 0 }
    }
}

impl<'de> de::SeqAccess<'de> for SeqAccess<'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: de::DeserializeSeed<'de>,
    {
        if self.arr.len() <= self.next {
            Ok(None)
        } else {
            let i = self.next;
            self.next += 1;
            let next_elem = seed.deserialize(&Deserializer::new(&self.arr[i]))?;
            Ok(Some(next_elem))
        }
    }
}

struct MapAccess<'de> {
    it: std::collections::hash_map::Iter<'de, watson::Bytes, watson::Value>,
    next_value: Option<&'de watson::Value>,
}

impl<'de> MapAccess<'de> {
    fn new(map: &'de watson::Map) -> Self {
        MapAccess {
            it: map.iter(),
            next_value: None,
        }
    }
}

impl<'de> de::MapAccess<'de> for MapAccess<'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: de::DeserializeSeed<'de>,
    {
        if self.next_value.is_some() {
            return Err(error(ErrorKind::UnexpectedMapValue));
        }
        match self.it.next() {
            None => Ok(None),
            Some((k, v)) => {
                self.next_value = Some(v);
                let next_key = seed.deserialize(MapKeyDeserializer::new(k))?;
                Ok(Some(next_key))
            }
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: de::DeserializeSeed<'de>,
    {
        match self.next_value.take() {
            None => Err(error(ErrorKind::UnexpectedMapKey)),
            Some(v) => seed.deserialize(&Deserializer::new(v)),
        }
    }
}

struct MapKeyDeserializer<'de> {
    key: &'de watson::Bytes,
}

impl<'de> MapKeyDeserializer<'de> {
    fn new(k: &'de watson::Bytes) -> Self {
        MapKeyDeserializer { key: k }
    }
}

impl<'de> MapKeyDeserializer<'de> {
    fn invalid_type(&self, exp: &dyn de::Expected) -> Error {
        invalid_type(de::Unexpected::Bytes(self.key.as_slice()), exp)
    }

    fn to_array<const N: usize>(&self, exp: &dyn de::Expected) -> Result<[u8; N]> {
        self.key
            .as_slice()
            .try_into()
            .map_err(|_| self.invalid_type(exp))
    }
}

impl<'de> de::Deserializer<'de> for MapKeyDeserializer<'de> {
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
        match self.to_array::<1>(&visitor)? {
            [0] => visitor.visit_bool(false),
            [1] => visitor.visit_bool(true),
            _ => Err(self.invalid_type(&visitor)),
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let n = i8::from_be_bytes(self.to_array(&visitor)?);
        visitor.visit_i8(n)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let n = i16::from_be_bytes(self.to_array(&visitor)?);
        visitor.visit_i16(n)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let n = i32::from_be_bytes(self.to_array(&visitor)?);
        visitor.visit_i32(n)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let n = i64::from_be_bytes(self.to_array(&visitor)?);
        visitor.visit_i64(n)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let n = u8::from_be_bytes(self.to_array(&visitor)?);
        visitor.visit_u8(n)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let n = u16::from_be_bytes(self.to_array(&visitor)?);
        visitor.visit_u16(n)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let n = u32::from_be_bytes(self.to_array(&visitor)?);
        visitor.visit_u32(n)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let n = u64::from_be_bytes(self.to_array(&visitor)?);
        visitor.visit_u64(n)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(self.invalid_type(&visitor))
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(self.invalid_type(&visitor))
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let s = try_borrow_str(self.key, &visitor)?;
        let mut chars = s.chars();
        let c = chars
            .next()
            .map(|c| Ok(c))
            .unwrap_or_else(|| Err(self.invalid_type(&visitor)))?;
        match chars.next() {
            None => visitor.visit_char(c),
            Some(_) => Err(self.invalid_type(&visitor)),
        }
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let s = try_borrow_str(self.key, &visitor)?;
        visitor.visit_borrowed_str(s)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let s = try_borrow_str(self.key, &visitor)?;
        visitor.visit_string(s.to_owned())
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_borrowed_bytes(self.key.as_slice())
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_byte_buf(self.key.clone())
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(self.invalid_type(&visitor))
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(self.invalid_type(&visitor))
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(self.invalid_type(&visitor))
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_seq(MapKeySeqAccess::new(self.key))
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(self.invalid_type(&visitor))
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(self.invalid_type(&visitor))
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(self.invalid_type(&visitor))
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(self.invalid_type(&visitor))
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

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        todo!("ignored_any")
    }
}

struct MapKeySeqAccess<'de> {
    key: &'de watson::Bytes,
    next: usize,
}

impl<'de> MapKeySeqAccess<'de> {
    fn new(key: &'de watson::Bytes) -> Self {
        MapKeySeqAccess { key: key, next: 0 }
    }
}

impl<'de> de::SeqAccess<'de> for MapKeySeqAccess<'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: de::DeserializeSeed<'de>,
    {
        if self.key.len() <= self.next {
            Ok(None)
        } else {
            let i = self.next;
            self.next += 1;
            let v = seed.deserialize(MapKeyByteDeserializer::new(self.key[i]))?;
            Ok(Some(v))
        }
    }
}

struct MapKeyByteDeserializer {
    b: u8,
}

impl MapKeyByteDeserializer {
    fn new(b: u8) -> Self {
        MapKeyByteDeserializer { b: b }
    }

    fn invalid_type(&self, exp: &dyn de::Expected) -> Error {
        invalid_type(de::Unexpected::Unsigned(self.b as u64), exp)
    }
}

impl<'de> de::Deserializer<'de> for MapKeyByteDeserializer {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(self.invalid_type(&visitor))
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(self.invalid_type(&visitor))
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(self.invalid_type(&visitor))
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(self.invalid_type(&visitor))
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(self.invalid_type(&visitor))
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(self.invalid_type(&visitor))
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u8(self.b)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(self.invalid_type(&visitor))
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(self.invalid_type(&visitor))
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(self.invalid_type(&visitor))
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(self.invalid_type(&visitor))
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(self.invalid_type(&visitor))
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(self.invalid_type(&visitor))
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(self.invalid_type(&visitor))
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(self.invalid_type(&visitor))
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(self.invalid_type(&visitor))
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(self.invalid_type(&visitor))
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(self.invalid_type(&visitor))
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(self.invalid_type(&visitor))
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(self.invalid_type(&visitor))
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(self.invalid_type(&visitor))
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(self.invalid_type(&visitor))
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(self.invalid_type(&visitor))
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(self.invalid_type(&visitor))
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(self.invalid_type(&visitor))
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(self.invalid_type(&visitor))
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(self.invalid_type(&visitor))
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(self.invalid_type(&visitor))
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(self.invalid_type(&visitor))
    }
}

struct EnumCtorDeserializer<'de> {
    name: &'de watson::Bytes,
}

impl<'de> EnumCtorDeserializer<'de> {
    fn new(name: &'de watson::Bytes) -> Self {
        EnumCtorDeserializer { name: name }
    }

    fn invalid_type(&self, exp: &dyn de::Expected) -> Error {
        invalid_type(de::Unexpected::Bytes(self.name.as_slice()), exp)
    }
}

impl<'de> de::Deserializer<'de> for EnumCtorDeserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(self.invalid_type(&visitor))
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(self.invalid_type(&visitor))
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(self.invalid_type(&visitor))
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(self.invalid_type(&visitor))
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(self.invalid_type(&visitor))
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(self.invalid_type(&visitor))
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(self.invalid_type(&visitor))
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(self.invalid_type(&visitor))
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(self.invalid_type(&visitor))
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(self.invalid_type(&visitor))
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(self.invalid_type(&visitor))
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(self.invalid_type(&visitor))
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(self.invalid_type(&visitor))
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(self.invalid_type(&visitor))
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(self.invalid_type(&visitor))
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(self.invalid_type(&visitor))
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(self.invalid_type(&visitor))
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(self.invalid_type(&visitor))
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(self.invalid_type(&visitor))
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(self.invalid_type(&visitor))
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(self.invalid_type(&visitor))
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(self.invalid_type(&visitor))
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(self.invalid_type(&visitor))
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(self.invalid_type(&visitor))
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(self.invalid_type(&visitor))
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(self.invalid_type(&visitor))
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(self.invalid_type(&visitor))
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let s = try_borrow_str(self.name, &visitor)?;
        visitor.visit_borrowed_str(s)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(self.invalid_type(&visitor))
    }
}

struct UnitVariantAccess<'de> {
    name: &'de watson::Bytes,
}

impl<'de> UnitVariantAccess<'de> {
    fn new(name: &'de watson::Bytes) -> Self {
        UnitVariantAccess { name: name }
    }
}

impl<'de> de::EnumAccess<'de> for UnitVariantAccess<'de> {
    type Error = Error;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant)>
    where
        V: de::DeserializeSeed<'de>,
    {
        let v = seed.deserialize(EnumCtorDeserializer::new(self.name))?;
        Ok((v, self))
    }
}

impl<'de> de::VariantAccess<'de> for UnitVariantAccess<'de> {
    type Error = Error;

    fn unit_variant(self) -> Result<()> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, _seed: T) -> Result<T::Value>
    where
        T: de::DeserializeSeed<'de>,
    {
        Err(de::Error::invalid_type(
            de::Unexpected::UnitVariant,
            &"newtype variant",
        ))
    }

    fn tuple_variant<V>(self, _len: usize, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(de::Error::invalid_type(
            de::Unexpected::UnitVariant,
            &"tuple variant",
        ))
    }

    fn struct_variant<V>(self, _fields: &'static [&'static str], _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(de::Error::invalid_type(
            de::Unexpected::UnitVariant,
            &"struct variant",
        ))
    }
}

struct NonUnitVariantAccess<'de> {
    map: &'de watson::Map,
}

impl<'de> NonUnitVariantAccess<'de> {
    fn new(map: &'de watson::Map) -> Self {
        NonUnitVariantAccess { map: map }
    }
}

impl<'de> de::EnumAccess<'de> for NonUnitVariantAccess<'de> {
    type Error = Error;
    type Variant = VariantFieldAccess<'de>;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant)>
    where
        V: de::DeserializeSeed<'de>,
    {
        if self.map.len() != 1 {
            Err(error(ErrorKind::UnexpectedMap))
        } else {
            let (k, v) = self.map.iter().next().unwrap();
            let ctor = seed.deserialize(EnumCtorDeserializer::new(k))?;
            Ok((ctor, VariantFieldAccess::new(v)))
        }
    }
}

struct VariantFieldAccess<'de> {
    value: &'de watson::Value,
}

impl<'de> VariantFieldAccess<'de> {
    fn new(v: &'de watson::Value) -> Self {
        VariantFieldAccess { value: v }
    }
}

impl<'de> de::VariantAccess<'de> for VariantFieldAccess<'de> {
    type Error = Error;

    fn unit_variant(self) -> Result<()> {
        de::Deserialize::deserialize(&Deserializer::new(self.value))
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value>
    where
        T: de::DeserializeSeed<'de>,
    {
        seed.deserialize(&Deserializer::new(self.value))
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        de::Deserializer::deserialize_seq(&Deserializer::new(self.value), visitor)
    }

    fn struct_variant<V>(self, fields: &'static [&'static str], visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        de::Deserializer::deserialize_struct(&Deserializer::new(self.value), "", fields, visitor)
    }
}

/*
 * Helper functions
 */

fn try_borrow_str<'de, V>(bytes: &'de watson::Bytes, visitor: &V) -> Result<&'de str>
where
    V: de::Visitor<'de>,
{
    std::str::from_utf8(bytes.as_slice()).map_err(|_| invalid_utf8(visitor))
}

fn invalid_type(ty: de::Unexpected, exp: &dyn de::Expected) -> Error {
    de::Error::invalid_type(ty, exp)
}

fn invalid_utf8(exp: &dyn de::Expected) -> Error {
    invalid_value("invalid UTF-8 string", exp)
}

fn invalid_value(desc: &'static str, exp: &dyn de::Expected) -> Error {
    de::Error::invalid_value(de::Unexpected::Other(desc), exp)
}

fn error(k: ErrorKind) -> Error {
    Error {
        kind: k,
        location: None,
        source: None,
    }
}

#[cfg(test)]
mod test {
    use std::fmt;

    use serde::Deserialize;
    use watson::Value::*;
    use watson::{array, object};

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
        assert_decodes("abc".to_string(), &String(b"abc".to_vec()));
    }

    #[test]
    fn deserialize_bytes() {
        let v = String(b"hello".to_vec());
        let b: &[u8] = deserialize(&v);
        assert_eq!(b, &b"hello"[..])
    }

    #[test]
    fn deserialize_byte_buf() {
        assert_decodes(Buf(b"".to_vec()), &String(b"".to_vec()));
        assert_decodes(Buf(b"goodbye".to_vec()), &String(b"goodbye".to_vec()));
    }

    #[test]
    fn deserialize_option() {
        assert_decodes(Some(123), &Int(123));
        assert_decodes(Option::<i32>::None, &Nil);
    }

    #[test]
    fn deserialize_unit() {
        assert_decodes((), &Nil);
    }

    #[test]
    fn deserialize_unit_struct() {
        #[derive(Eq, PartialEq, Deserialize, Debug)]
        struct S;

        assert_decodes(S, &Nil);
    }

    #[test]
    fn deserialize_newtype_struct() {
        #[derive(Eq, PartialEq, Deserialize, Debug)]
        struct S(i64);

        assert_decodes(S(123), &Int(123));
    }

    #[test]
    fn deserialize_seq() {
        assert_decodes(Vec::<bool>::new(), &array![]);
        assert_decodes(vec![1_i32, 2_i32, 3_i32], &array![Int(1), Int(2), Int(3)]);
    }

    #[test]
    fn deserialize_tuple() {
        assert_decodes(
            (1_u32, true, "foo"),
            &array![Uint(1), Bool(true), String(b"foo".to_vec())],
        );
    }

    #[test]
    fn deserialize_tuple_struct() {
        #[derive(Eq, PartialEq, Deserialize, Debug)]
        struct S(i32, (), u16);

        assert_decodes(S(123, (), 45), &array![Int(123), Nil, Uint(45)]);
    }

    #[test]
    fn deserialize_map_key_bool() {
        type HM<T> = std::collections::HashMap<bool, T>;

        assert_decodes(HM::<i32>::new(), &object![]);
        assert_decodes(
            [(true, 123), (false, 456)].into_iter().collect::<HM<i32>>(),
            &object![
                [b"\x01"]: Int(123),
                [b"\x00"]: Int(456),
            ],
        );
    }

    #[test]
    fn deserialize_map_key_i8() {
        type HM<T> = std::collections::HashMap<i8, T>;

        assert_decodes(HM::<i32>::new(), &object![]);
        assert_decodes(
            [(0, 1), (1, 2), (0x7f, 3), (-1, 4)]
                .into_iter()
                .collect::<HM<i32>>(),
            &object![
                [b"\x00"]: Int(1),
                [b"\x01"]: Int(2),
                [b"\x7f"]: Int(3),
                [b"\xff"]: Int(4),
            ],
        );
    }

    #[test]
    fn deserialize_map_key_i16() {
        type HM<T> = std::collections::HashMap<i16, T>;

        assert_decodes(HM::<i32>::new(), &object![]);
        assert_decodes(
            [(0, 1), (1, 2), (0x7fff, 3), (-1, 4)]
                .into_iter()
                .collect::<HM<i32>>(),
            &object![
                [b"\x00\x00"]: Int(1),
                [b"\x00\x01"]: Int(2),
                [b"\x7f\xff"]: Int(3),
                [b"\xff\xff"]: Int(4),
            ],
        );
    }

    #[test]
    fn deserialize_map_key_i32() {
        type HM<T> = std::collections::HashMap<i32, T>;

        assert_decodes(HM::<i32>::new(), &object![]);
        assert_decodes(
            [(0, 1), (1, 2), (0x7fff_ffff, 3), (-1, 4)]
                .into_iter()
                .collect::<HM<i32>>(),
            &object![
                [b"\x00\x00\x00\x00"]: Int(1),
                [b"\x00\x00\x00\x01"]: Int(2),
                [b"\x7f\xff\xff\xff"]: Int(3),
                [b"\xff\xff\xff\xff"]: Int(4),
            ],
        );
    }

    #[test]
    fn deserialize_map_key_i64() {
        type HM<T> = std::collections::HashMap<i64, T>;

        assert_decodes(HM::<i32>::new(), &object![]);
        assert_decodes(
            [(0, 1), (1, 2), (0x7fff_ffff_ffff_ffff, 3), (-1, 4)]
                .into_iter()
                .collect::<HM<i32>>(),
            &object![
                [b"\x00\x00\x00\x00\x00\x00\x00\x00"]: Int(1),
                [b"\x00\x00\x00\x00\x00\x00\x00\x01"]: Int(2),
                [b"\x7f\xff\xff\xff\xff\xff\xff\xff"]: Int(3),
                [b"\xff\xff\xff\xff\xff\xff\xff\xff"]: Int(4),
            ],
        );
    }

    #[test]
    fn deserialize_map_key_u8() {
        type HM<T> = std::collections::HashMap<u8, T>;

        assert_decodes(HM::<i32>::new(), &object![]);
        assert_decodes(
            [(0, 1), (1, 2), (0x7f, 3), (0xff, 4)]
                .into_iter()
                .collect::<HM<i32>>(),
            &object![
                [b"\x00"]: Int(1),
                [b"\x01"]: Int(2),
                [b"\x7f"]: Int(3),
                [b"\xff"]: Int(4),
            ],
        );
    }

    #[test]
    fn deserialize_map_key_u16() {
        type HM<T> = std::collections::HashMap<u16, T>;

        assert_decodes(HM::<i32>::new(), &object![]);
        assert_decodes(
            [(0, 1), (1, 2), (0x7fff, 3), (0xffff, 4)]
                .into_iter()
                .collect::<HM<i32>>(),
            &object![
                [b"\x00\x00"]: Int(1),
                [b"\x00\x01"]: Int(2),
                [b"\x7f\xff"]: Int(3),
                [b"\xff\xff"]: Int(4),
            ],
        );
    }

    #[test]
    fn deserialize_map_key_u32() {
        type HM<T> = std::collections::HashMap<u32, T>;

        assert_decodes(HM::<i32>::new(), &object![]);
        assert_decodes(
            [(0, 1), (1, 2), (0x7fff_ffff, 3), (0xffff_ffff, 4)]
                .into_iter()
                .collect::<HM<i32>>(),
            &object![
                [b"\x00\x00\x00\x00"]: Int(1),
                [b"\x00\x00\x00\x01"]: Int(2),
                [b"\x7f\xff\xff\xff"]: Int(3),
                [b"\xff\xff\xff\xff"]: Int(4),
            ],
        );
    }

    #[test]
    fn deserialize_map_key_u64() {
        type HM<T> = std::collections::HashMap<u64, T>;

        assert_decodes(HM::<i32>::new(), &object![]);
        assert_decodes(
            [
                (0, 1),
                (1, 2),
                (0x7fff_ffff_ffff_ffff, 3),
                (0xffff_ffff_ffff_ffff, 4),
            ]
            .into_iter()
            .collect::<HM<i32>>(),
            &object![
                [b"\x00\x00\x00\x00\x00\x00\x00\x00"]: Int(1),
                [b"\x00\x00\x00\x00\x00\x00\x00\x01"]: Int(2),
                [b"\x7f\xff\xff\xff\xff\xff\xff\xff"]: Int(3),
                [b"\xff\xff\xff\xff\xff\xff\xff\xff"]: Int(4),
            ],
        );
    }

    #[test]
    fn deserialize_map_key_char() {
        type HM<T> = std::collections::HashMap<char, T>;

        assert_decodes(HM::<i32>::new(), &object![]);
        assert_decodes(
            [('A', 1), ('ぽ', 2), ('ꙮ', 8)]
                .into_iter()
                .collect::<HM<i32>>(),
            &object![
                A: Int(1),
                [b"\xe3\x81\xbd"]: Int(2),
                [b"\xea\x99\xae"]: Int(8),
            ],
        );
    }

    #[test]
    fn deserialize_map_key_str() {
        type HM<'a, T> = std::collections::HashMap<&'a str, T>;

        assert_decodes(HM::<'_, i32>::new(), &object![]);
        assert_decodes(
            [("foo", 1), ("bar", 2)]
                .into_iter()
                .collect::<HM<'_, i32>>(),
            &object![
                foo: Int(1),
                bar: Int(2),
            ],
        );
    }

    #[test]
    fn deserialize_map_key_string() {
        type HM<T> = std::collections::HashMap<std::string::String, T>;

        assert_decodes(HM::<i32>::new(), &object![]);
        assert_decodes(
            [
                ("hello".to_owned(), "world".to_owned()),
                ("foo".to_owned(), "bar".to_owned()),
            ]
            .into_iter()
            .collect::<HM<std::string::String>>(),
            &object![
                hello: String(b"world".to_vec()),
                foo: String(b"bar".to_vec()),
            ],
        );
    }

    #[test]
    fn deserialize_map_key_bytes() {
        type HM<'a, T> = std::collections::HashMap<&'a [u8], T>;

        assert_decodes(HM::<'_, i32>::new(), &object![]);
        assert_decodes(
            [(b"foo".as_slice(), 1), (b"bar".as_slice(), 2)]
                .into_iter()
                .collect::<HM<'_, i32>>(),
            &object![
                foo: Int(1),
                bar: Int(2),
            ],
        );
    }

    #[test]
    fn deserialize_map_key_byte_buf() {
        type HM<T> = std::collections::HashMap<Buf, T>;

        assert_decodes(HM::<i32>::new(), &object![]);
        assert_decodes(
            [(Buf(b"foo".to_vec()), 1), (Buf(b"bar".to_vec()), 2)]
                .into_iter()
                .collect::<HM<i32>>(),
            &object![
                foo: Int(1),
                bar: Int(2),
            ],
        );
    }

    #[test]
    fn deserialize_map_key_newtype_struct() {
        #[derive(Eq, PartialEq, Hash, Deserialize, Debug)]
        struct S(std::string::String);

        type HM<T> = std::collections::HashMap<S, T>;

        assert_decodes(HM::<i32>::new(), &object![]);
        assert_decodes(
            [(S("foo".to_owned()), 1), (S("bar".to_owned()), 2)]
                .into_iter()
                .collect::<HM<i32>>(),
            &object![
                foo: Int(1),
                bar: Int(2),
            ],
        );
    }

    #[test]
    fn deserialize_map_key_u8_seq() {
        use watson::ToBytes;
        type HM<T> = std::collections::HashMap<watson::Bytes, T>;

        assert_decodes(HM::<i32>::new(), &object![]);
        assert_decodes(
            [(b"foo".to_bytes(), 1), (b"bar".to_bytes(), 2)]
                .into_iter()
                .collect::<HM<i32>>(),
            &object![
                foo: Int(1),
                bar: Int(2),
            ],
        );
    }

    #[test]
    fn deserialize_struct() {
        #[derive(Eq, PartialEq, Deserialize, Debug)]
        struct S {
            f1: i32,
            f2: bool,
        }

        assert_decodes(
            S { f1: 123, f2: true },
            &object![
                f1: Int(123),
                f2: Bool(true),
            ],
        );
        assert_decodes(S { f1: 456, f2: false }, &array![Int(456), Bool(false)]);
    }

    #[test]
    fn deserialize_enum() {
        #[derive(PartialEq, Deserialize, Debug)]
        enum E {
            A,
            B(i32),
            C(u32, bool),
            D { f1: f64, f2: std::string::String },
        }

        assert_decodes(E::A, &String(b"A".to_vec()));
        assert_decodes(E::A, &object![A: Nil]);
        assert_decodes(E::B(123), &object![B: Int(123)]);
        assert_decodes(E::C(456, true), &object![C: array![Uint(456), Bool(true)]]);
        assert_decodes(
            E::D {
                f1: 1.25,
                f2: "hey".to_owned(),
            },
            &object![D: object![f1: Float(1.25), f2: String(b"hey".to_vec())]],
        );
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
        T::deserialize(&Deserializer::new(v)).expect("deserialization error")
    }

    // The standard `de::Deserialize` implementation for `Vec<u8>` does not use `deserialize_byte_buf`.
    #[derive(Eq, PartialEq, Hash, Debug)]
    struct Buf(Vec<u8>);

    impl<'de> de::Deserialize<'de> for Buf {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: de::Deserializer<'de>,
        {
            deserializer.deserialize_byte_buf(BufVisitor)
        }
    }

    struct BufVisitor;

    impl<'de> de::Visitor<'de> for BufVisitor {
        type Value = Buf;

        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "byte_buf")
        }

        fn visit_byte_buf<E>(self, v: Vec<u8>) -> std::result::Result<Buf, E>
        where
            E: de::Error,
        {
            Ok(Buf(v))
        }
    }
}
