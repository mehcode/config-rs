use crate::object::ConfigObject;

pub trait ConfigSource: std::fmt::Debug {
    type Error: std::error::Error;

    fn load<'a>(&'a self) -> Result<ConfigObject<'a>, Self::Error>;
}


#[cfg(test)]
pub(crate) mod test_source {
    use crate::source::ConfigSource;
    use crate::object::ConfigObject;
    use crate::element::ConfigElement;
    use crate::description::ConfigSourceDescription;

    pub(crate) struct TestSource<'a, G>(pub(crate) G)
        where G: Fn() -> ConfigElement<'a>;

    impl<'g, G> std::fmt::Debug for TestSource<'g, G>
        where G: Fn() -> ConfigElement<'g>
    {
        fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            Ok(())
        }
    }

    impl<'g, G> ConfigSource for TestSource<'g, G>
        where G: Fn() -> ConfigElement<'g>
    {
        type Error = std::convert::Infallible; // can never happen

        fn load<'a>(&'a self) -> Result<ConfigObject<'a>, Self::Error> {
            Ok(ConfigObject::new(self.0(), ConfigSourceDescription::Unknown))
        }
    }
}
