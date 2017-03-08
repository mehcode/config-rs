use serde::de;
use value::{Value, ValueKind};
use error::*;
use std::iter::Peekable;
use std::collections::HashMap;
use std::collections::hash_map::Drain;

impl de::Deserializer for Value {
    type Error = ConfigError;

    #[inline]
    fn deserialize<V: de::Visitor>(self, visitor: V) -> Result<V::Value> {
        // Deserialize based on the underlying type
        match self.kind {
            ValueKind::Boolean(value) => {
                visitor.visit_bool(value)
            }

            ValueKind::Table(map) => {
                visitor.visit_map(MapVisitor::new(map))
            }

            _ => { unimplemented!(); }
        }
    }

    #[inline]
    fn deserialize_bool<V: de::Visitor>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_bool(self.into_bool()?)
    }

    #[inline]
    fn deserialize_u8<V: de::Visitor>(self, visitor: V) -> Result<V::Value> {
        unimplemented!();
    }

    #[inline]
    fn deserialize_u16<V: de::Visitor>(self, visitor: V) -> Result<V::Value> {
        unimplemented!();
    }

    #[inline]
    fn deserialize_u32<V: de::Visitor>(self, visitor: V) -> Result<V::Value> {
        unimplemented!();
    }

    #[inline]
    fn deserialize_u64<V: de::Visitor>(self, visitor: V) -> Result<V::Value> {
        unimplemented!();
    }

    #[inline]
    fn deserialize_i8<V: de::Visitor>(self, visitor: V) -> Result<V::Value> {
        unimplemented!();
    }

    #[inline]
    fn deserialize_i16<V: de::Visitor>(self, visitor: V) -> Result<V::Value> {
        unimplemented!();
    }

    #[inline]
    fn deserialize_i32<V: de::Visitor>(self, visitor: V) -> Result<V::Value> {
        unimplemented!();
    }

    #[inline]
    fn deserialize_i64<V: de::Visitor>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_i64(self.into_int()?)
    }

    #[inline]
    fn deserialize_f32<V: de::Visitor>(self, visitor: V) -> Result<V::Value> {
        unimplemented!();
    }

    #[inline]
    fn deserialize_f64<V: de::Visitor>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_f64(self.into_float()?)
    }

    #[inline]
    fn deserialize_char<V: de::Visitor>(self, visitor: V) -> Result<V::Value> {
        unimplemented!();
    }

    #[inline]
    fn deserialize_str<V: de::Visitor>(self, visitor: V) -> Result<V::Value> {
        unimplemented!();
    }

    #[inline]
    fn deserialize_string<V: de::Visitor>(self, visitor: V) -> Result<V::Value> {
        unimplemented!();
    }

    #[inline]
    fn deserialize_bytes<V: de::Visitor>(self, visitor: V) -> Result<V::Value> {
        unimplemented!();
    }

    #[inline]
    fn deserialize_byte_buf<V: de::Visitor>(self, visitor: V) -> Result<V::Value> {
        unimplemented!();
    }

    #[inline]
    fn deserialize_option<V: de::Visitor>(self, visitor: V) -> Result<V::Value> {
        unimplemented!();
    }

    #[inline]
    fn deserialize_unit<V: de::Visitor>(self, visitor: V) -> Result<V::Value> {
        unimplemented!();
    }

    #[inline]
    fn deserialize_unit_struct<V: de::Visitor>(self,
                                               name: &'static str,
                                               visitor: V)
                                               -> Result<V::Value> {
        unimplemented!();
    }

    #[inline]
    fn deserialize_newtype_struct<V: de::Visitor>(self,
                                                  name: &'static str,
                                                  visitor: V)
                                                  -> Result<V::Value> {
        unimplemented!();
    }

    #[inline]
    fn deserialize_seq<V: de::Visitor>(self, visitor: V) -> Result<V::Value> {
        unimplemented!();
    }

    #[inline]
    fn deserialize_seq_fixed_size<V: de::Visitor>(self,
                                                  len: usize,
                                                  visitor: V)
                                                  -> Result<V::Value> {
        unimplemented!();
    }

    #[inline]
    fn deserialize_tuple<V: de::Visitor>(self, len: usize, visitor: V) -> Result<V::Value> {
        unimplemented!();
    }

    #[inline]
    fn deserialize_tuple_struct<V: de::Visitor>(self,
                                                name: &'static str,
                                                len: usize,
                                                visitor: V)
                                                -> Result<V::Value> {
        unimplemented!();
    }

    #[inline]
    fn deserialize_enum<V: de::Visitor>(self,
                                        name: &'static str,
                                        variants: &'static [&'static str],
                                        visitor: V)
                                        -> Result<V::Value> {
        unimplemented!();
    }

    #[inline]
    fn deserialize_ignored_any<V: de::Visitor>(self, visitor: V) -> Result<V::Value> {
        unimplemented!();
    }

    forward_to_deserialize! {
        map
        struct
        struct_field
    }
}

struct StrDeserializer<'a>(&'a str);

impl<'a> de::Deserializer for StrDeserializer<'a> {
    type Error = ConfigError;

    #[inline]
    fn deserialize<V: de::Visitor>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_str(self.0)
    }

    forward_to_deserialize! {
        bool
        u8
        u16
        u32
        u64
        i8
        i16
        i32
        i64
        f32
        f64
        char
        str
        string
        bytes
        byte_buf
        option
        unit
        unit_struct
        newtype_struct
        seq
        seq_fixed_size
        tuple
        tuple_struct
        map
        struct
        struct_field
        enum
        ignored_any
    }
}

struct MapVisitor {
    elements: Vec<(String, Value)>,
    index: usize,
}

impl MapVisitor {
    fn new(mut table: HashMap<String, Value>) -> MapVisitor {
        MapVisitor { elements: table.drain().collect(), index: 0 }
    }
}

impl de::MapVisitor for MapVisitor {
    type Error = ConfigError;

    fn visit_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
        where K: de::DeserializeSeed,
    {
        if self.index >= self.elements.len() {
            return Ok(None);
        }

        let ref key_s = self.elements[0].0;
        let key_de = StrDeserializer(&key_s);
        let key = de::DeserializeSeed::deserialize(seed, key_de)?;

        Ok(Some(key))
    }

    fn visit_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
        where V: de::DeserializeSeed,
    {
        de::DeserializeSeed::deserialize(seed, self.elements.remove(0).1)
    }
}
