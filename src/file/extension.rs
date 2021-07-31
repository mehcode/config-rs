pub trait FileExtensions {
    fn extensions(&self) -> &Vec<&'static str>;
}
