#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum ConfigSourceDescription {
    Unknown,
    Default,
    Overwrite,
    Path(std::path::PathBuf),
    Uri(url::Url),
    Custom(String),
}
