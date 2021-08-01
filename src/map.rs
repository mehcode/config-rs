#[cfg(not(feature = "preserve_order"))]
pub type MapImpl<K, V> = std::collections::HashMap<K, V>;
#[cfg(feature = "preserve_order")]
pub type MapImpl<K, V> = linked_hash_map::LinkedHashMap<K, V>;
