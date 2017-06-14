use serde::de;
use value::{Value, ValueWithKey, ValueKind};
use error::*;
use std::borrow::Cow;
use std::iter::Peekable;
use std::collections::HashMap;
use std::collections::hash_map::Drain;

// TODO: Use a macro or some other magic to reduce the code duplication here

impl<'de> de::Deserializer<'de> for ValueWithKey<'de> {
    type Error = ConfigError;

    #[inline]
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
        where V: de::Visitor<'de>
    {
        // Deserialize based on the underlying type
        match self.0.kind {
            ValueKind::Nil => visitor.visit_unit(),
            ValueKind::Integer(i) => visitor.visit_i64(i),
            ValueKind::Boolean(b) => visitor.visit_bool(b),
            ValueKind::Float(f) => visitor.visit_f64(f),
            ValueKind::String(s) => visitor.visit_string(s),
            ValueKind::Array(values) => visitor.visit_seq(SeqAccess::new(values)),
            ValueKind::Table(map) => visitor.visit_map(MapAccess::new(map)),
        }
    }

    #[inline]
    fn deserialize_bool<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_bool(self.into_bool()?)
    }

    #[inline]
    fn deserialize_i8<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        // FIXME: This should *fail* if the value does not fit in the requets integer type
        visitor.visit_i8(self.into_int()? as i8)
    }

    #[inline]
    fn deserialize_i16<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        // FIXME: This should *fail* if the value does not fit in the requets integer type
        visitor.visit_i16(self.into_int()? as i16)
    }

    #[inline]
    fn deserialize_i32<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        // FIXME: This should *fail* if the value does not fit in the requets integer type
        visitor.visit_i32(self.into_int()? as i32)
    }

    #[inline]
    fn deserialize_i64<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_i64(self.into_int()?)
    }

    #[inline]
    fn deserialize_u8<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        // FIXME: This should *fail* if the value does not fit in the requets integer type
        visitor.visit_u8(self.into_int()? as u8)
    }

    #[inline]
    fn deserialize_u16<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        // FIXME: This should *fail* if the value does not fit in the requets integer type
        visitor.visit_u16(self.into_int()? as u16)
    }

    #[inline]
    fn deserialize_u32<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        // FIXME: This should *fail* if the value does not fit in the requets integer type
        visitor.visit_u32(self.into_int()? as u32)
    }

    #[inline]
    fn deserialize_u64<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        // FIXME: This should *fail* if the value does not fit in the requets integer type
        visitor.visit_u64(self.into_int()? as u64)
    }

    #[inline]
    fn deserialize_f32<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_f32(self.into_float()? as f32)
    }

    #[inline]
    fn deserialize_f64<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_f64(self.into_float()?)
    }

    #[inline]
    fn deserialize_str<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_string(self.into_str()?)
    }

    #[inline]
    fn deserialize_string<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_string(self.into_str()?)
    }

    #[inline]
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
        where V: de::Visitor<'de>
    {
        // Match an explicit nil as None and everything else as Some
        match self.0.kind {
            ValueKind::Nil => visitor.visit_none(),
            _ => visitor.visit_some(self),
        }
    }

    forward_to_deserialize_any! {
        char seq
        bytes byte_buf map struct unit enum newtype_struct
        identifier ignored_any unit_struct tuple_struct tuple
    }
}

impl<'de> de::Deserializer<'de> for Value {
    type Error = ConfigError;

    #[inline]
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
        where V: de::Visitor<'de>
    {
        // Deserialize based on the underlying type
        match self.kind {
            ValueKind::Nil => visitor.visit_unit(),
            ValueKind::Integer(i) => visitor.visit_i64(i),
            ValueKind::Boolean(b) => visitor.visit_bool(b),
            ValueKind::Float(f) => visitor.visit_f64(f),
            ValueKind::String(s) => visitor.visit_string(s),
            ValueKind::Array(values) => visitor.visit_seq(SeqAccess::new(values)),
            ValueKind::Table(map) => visitor.visit_map(MapAccess::new(map)),
        }
    }

    #[inline]
    fn deserialize_bool<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_bool(self.into_bool()?)
    }

    #[inline]
    fn deserialize_i8<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        // FIXME: This should *fail* if the value does not fit in the requets integer type
        visitor.visit_i8(self.into_int()? as i8)
    }

    #[inline]
    fn deserialize_i16<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        // FIXME: This should *fail* if the value does not fit in the requets integer type
        visitor.visit_i16(self.into_int()? as i16)
    }

    #[inline]
    fn deserialize_i32<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        // FIXME: This should *fail* if the value does not fit in the requets integer type
        visitor.visit_i32(self.into_int()? as i32)
    }

    #[inline]
    fn deserialize_i64<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_i64(self.into_int()?)
    }

    #[inline]
    fn deserialize_u8<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        // FIXME: This should *fail* if the value does not fit in the requets integer type
        visitor.visit_u8(self.into_int()? as u8)
    }

    #[inline]
    fn deserialize_u16<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        // FIXME: This should *fail* if the value does not fit in the requets integer type
        visitor.visit_u16(self.into_int()? as u16)
    }

    #[inline]
    fn deserialize_u32<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        // FIXME: This should *fail* if the value does not fit in the requets integer type
        visitor.visit_u32(self.into_int()? as u32)
    }

    #[inline]
    fn deserialize_u64<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        // FIXME: This should *fail* if the value does not fit in the requets integer type
        visitor.visit_u64(self.into_int()? as u64)
    }

    #[inline]
    fn deserialize_f32<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_f32(self.into_float()? as f32)
    }

    #[inline]
    fn deserialize_f64<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_f64(self.into_float()?)
    }

    #[inline]
    fn deserialize_str<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_string(self.into_str()?)
    }

    #[inline]
    fn deserialize_string<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_string(self.into_str()?)
    }

    #[inline]
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
        where V: de::Visitor<'de>
    {
        // Match an explicit nil as None and everything else as Some
        match self.kind {
            ValueKind::Nil => visitor.visit_none(),
            _ => visitor.visit_some(self),
        }
    }

    forward_to_deserialize_any! {
        char seq
        bytes byte_buf map struct unit enum newtype_struct
        identifier ignored_any unit_struct tuple_struct tuple
    }
}

struct StrDeserializer<'a>(&'a str);

impl<'a> StrDeserializer<'a> {
    fn new(key: &'a str) -> Self {
        StrDeserializer(key)
    }
}

impl<'de, 'a> de::Deserializer<'de> for StrDeserializer<'a> {
    type Error = ConfigError;

    #[inline]
    fn deserialize_any<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_str(self.0)
    }

    forward_to_deserialize_any! {
        bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string seq
        bytes byte_buf map struct unit enum newtype_struct
        identifier ignored_any unit_struct tuple_struct tuple option
    }
}

struct SeqAccess {
    elements: ::std::vec::IntoIter<Value>,
}

impl SeqAccess {
    fn new(elements: Vec<Value>) -> Self {
        SeqAccess {
            elements: elements.into_iter(),
        }
    }
}

impl<'de> de::SeqAccess<'de> for SeqAccess {
    type Error = ConfigError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
        where T: de::DeserializeSeed<'de>
    {
        match self.elements.next() {
            Some(value) => seed.deserialize(value).map(Some),
            None => Ok(None),
        }
    }

    fn size_hint(&self) -> Option<usize> {
        match self.elements.size_hint() {
            (lower, Some(upper)) if lower == upper => Some(upper),
            _ => None,
        }
    }
}

struct MapAccess {
    elements: Vec<(String, Value)>,
    index: usize,
}

impl MapAccess {
    fn new(mut table: HashMap<String, Value>) -> Self {
        MapAccess {
            elements: table.drain().collect(),
            index: 0,
        }
    }
}

impl<'de> de::MapAccess<'de> for MapAccess {
    type Error = ConfigError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
        where K: de::DeserializeSeed<'de>
    {
        if self.index >= self.elements.len() {
            return Ok(None);
        }

        let key_s = &(self.elements[0].0);
        let key_de = StrDeserializer(key_s);
        let key = de::DeserializeSeed::deserialize(seed, key_de)?;

        Ok(Some(key))
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
        where V: de::DeserializeSeed<'de>
    {
        de::DeserializeSeed::deserialize(seed, self.elements.remove(0).1)
    }
}
