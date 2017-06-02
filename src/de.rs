use serde::de;
use value::{Value, ValueKind};
use error::*;
use std::borrow::Cow;
use std::iter::Peekable;
use std::collections::HashMap;
use std::collections::hash_map::Drain;

impl de::Deserializer for Value {
    type Error = ConfigError;

    #[inline]
    fn deserialize<V>(self, visitor: V) -> Result<V::Value>
        where V: de::Visitor
    {
        // Deserialize based on the underlying type
        match self.kind {
            ValueKind::Integer(i) => visitor.visit_i64(i),
            ValueKind::Boolean(b) => visitor.visit_bool(b),
            ValueKind::Float(f) => visitor.visit_f64(f),
            ValueKind::String(s) => visitor.visit_string(s),
            ValueKind::Array(values) => unimplemented!(),
            ValueKind::Table(map) => visitor.visit_map(MapVisitor::new(map)),
            _ => {
                unimplemented!();
            }
        }
    }

    #[inline]
    fn deserialize_bool<V: de::Visitor>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_bool(self.into_bool()?)
    }

    #[inline]
    fn deserialize_i8<V: de::Visitor>(self, visitor: V) -> Result<V::Value> {
        // FIXME: This should *fail* if the value does not fit in the requets integer type
        visitor.visit_i8(self.into_int()? as i8)
    }

    #[inline]
    fn deserialize_i16<V: de::Visitor>(self, visitor: V) -> Result<V::Value> {
        // FIXME: This should *fail* if the value does not fit in the requets integer type
        visitor.visit_i16(self.into_int()? as i16)
    }

    #[inline]
    fn deserialize_i32<V: de::Visitor>(self, visitor: V) -> Result<V::Value> {
        // FIXME: This should *fail* if the value does not fit in the requets integer type
        visitor.visit_i32(self.into_int()? as i32)
    }

    #[inline]
    fn deserialize_i64<V: de::Visitor>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_i64(self.into_int()?)
    }

    #[inline]
    fn deserialize_u8<V: de::Visitor>(self, visitor: V) -> Result<V::Value> {
        // FIXME: This should *fail* if the value does not fit in the requets integer type
        visitor.visit_u8(self.into_int()? as u8)
    }

    #[inline]
    fn deserialize_u16<V: de::Visitor>(self, visitor: V) -> Result<V::Value> {
        // FIXME: This should *fail* if the value does not fit in the requets integer type
        visitor.visit_u16(self.into_int()? as u16)
    }

    #[inline]
    fn deserialize_u32<V: de::Visitor>(self, visitor: V) -> Result<V::Value> {
        // FIXME: This should *fail* if the value does not fit in the requets integer type
        visitor.visit_u32(self.into_int()? as u32)
    }

    #[inline]
    fn deserialize_u64<V: de::Visitor>(self, visitor: V) -> Result<V::Value> {
        // FIXME: This should *fail* if the value does not fit in the requets integer type
        visitor.visit_u64(self.into_int()? as u64)
    }

    #[inline]
    fn deserialize_f32<V: de::Visitor>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_f32(self.into_float()? as f32)
    }

    #[inline]
    fn deserialize_f64<V: de::Visitor>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_f64(self.into_float()?)
    }

    #[inline]
    fn deserialize_str<V: de::Visitor>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_string(self.into_str()?)
    }

    #[inline]
    fn deserialize_string<V: de::Visitor>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_string(self.into_str()?)
    }

    #[inline]
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
        where V: de::Visitor
    {
        // Match an explicit nil as None and everything else as Some
        match self.kind {
            ValueKind::Nil => visitor.visit_none(),
            _ => visitor.visit_some(self),
        }
    }

    forward_to_deserialize! {
        char seq
        seq_fixed_size bytes byte_buf map struct unit enum newtype_struct
        struct_field ignored_any unit_struct tuple_struct tuple
    }
}

struct StrDeserializer<'a>(&'a str);

impl<'a> StrDeserializer<'a> {
    fn new(key: &'a str) -> Self {
        StrDeserializer(key)
    }
}

impl<'a> de::Deserializer for StrDeserializer<'a> {
    type Error = ConfigError;

    #[inline]
    fn deserialize<V: de::Visitor>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_str(self.0)
    }

    forward_to_deserialize! {
        bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string seq
        seq_fixed_size bytes byte_buf map struct unit enum newtype_struct
        struct_field ignored_any unit_struct tuple_struct tuple option
    }
}

struct MapVisitor {
    elements: Vec<(String, Value)>,
    index: usize,
}

impl MapVisitor {
    fn new(mut table: HashMap<String, Value>) -> Self {
        MapVisitor {
            elements: table.drain().collect(),
            index: 0,
        }
    }
}

impl de::MapVisitor for MapVisitor {
    type Error = ConfigError;

    fn visit_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
        where K: de::DeserializeSeed
    {
        if self.index >= self.elements.len() {
            return Ok(None);
        }

        let key_s = &self.elements[0].0;
        let key_de = StrDeserializer(key_s);
        let key = de::DeserializeSeed::deserialize(seed, key_de)?;

        Ok(Some(key))
    }

    fn visit_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
        where V: de::DeserializeSeed
    {
        de::DeserializeSeed::deserialize(seed, self.elements.remove(0).1)
    }
}
