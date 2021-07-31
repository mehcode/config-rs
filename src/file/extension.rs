/// Describes an entity that can return file extensions.
///
/// Meant primarily for [`Format`](crate::Format) trait.
/// Since [`Format`](crate::Format) only describes certain internal encoding, for instance JSON or Yaml
/// it is not necessarily bound to file extension name.
///
///In networking context JSONs are used without file extensions.
/// One can also imagine some encodings that do not necessarily have file extension associated, for instance
/// MessagePack or bincode.
/// Hence the decision to have extensions separated from [`Format`](crate::Format).
pub trait FileExtensions {
    /// Returns a vector of file extensions, for instance `[yml, yaml]`.
    fn extensions(&self) -> &Vec<&'static str>;
}
