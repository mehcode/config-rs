#[cfg(not(feature = "preserve_order"))]
pub type Map<K, V> = std::collections::HashMap<K, V>;
#[cfg(feature = "preserve_order")]
pub type Map<K, V> = indexmap::IndexMap<K, V>;
