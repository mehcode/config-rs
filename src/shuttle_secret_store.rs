use convert_case::Case;

use shuttle_secrets::SecretStore;

use crate::{ConfigError, Environment, Map, Source, Value};

/// A source for the [SecretStore](https://docs.rs/shuttle-secrets/0.39.0/shuttle_secrets/struct.SecretStore.html)
/// of [shuttle.rs](https://www.shuttle.rs/). It is based on the [Environment] source and offers all
/// the features that Environment provides.
/// # Example
/// ```ignore
/// #[derive(Deserialize, Debug, PartialEq)]
/// pub struct MyAppConfiguration {
///     pub authentication: AuthenticationSettings,
/// }
///
/// #[derive(Deserialize, Debug, PartialEq)]
/// pub struct AuthenticationSettings {
///     pub token_secret: String,
/// }
///
/// #[shuttle_runtime::main]
/// async fn main(
///     #[shuttle_secrets::Secrets] secret_store: SecretStore, // includes MY_APP_AUTHENTICATION__TOKEN_SECRET=my_secret
/// ) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
///     let service_config = move |cfg: &mut ServiceConfig| {
///         let my_config = Config::builder()
///             .add_source(
///                 ShuttleSecretStore::new(&secret_store)
///                     .prefix("MY_APP")
///                     .prefix_separator("_")
///                     .separator("__"),
///             )
///             .build()
///             .unwrap()
///             .try_deserialize::<MyAppConfiguration>()
///             .unwrap();
///         cfg.app_data(Data::new(my_config));
///     };
///
///     Ok(service_config.into())
/// }
/// ```
#[derive(Clone, Debug, Default)]
pub struct ShuttleSecretStore {
    environment: Environment,
}

impl ShuttleSecretStore {
    pub fn new(secret_store: &SecretStore) -> Self {
        let mut secrets_map: Map<String, String> = Map::new();
        for (key, value) in secret_store.clone().into_iter() {
            secrets_map.insert(key, value);
        }

        Self {
            environment: Environment::default().source(Some(secrets_map)),
        }
    }

    /// See [Environment::prefix]
    pub fn prefix(mut self, prefix: &str) -> Self {
        self.environment = self.environment.prefix(prefix);
        self
    }

    /// See [Environment::convert_case]
    pub fn convert_case(mut self, case: Case) -> Self {
        self.environment = self.environment.convert_case(case);
        self
    }

    /// See [Environment::prefix_separator]
    pub fn prefix_separator(mut self, separator: &str) -> Self {
        self.environment = self.environment.prefix_separator(separator);
        self
    }

    /// See [Environment::separator]
    pub fn separator(mut self, separator: &str) -> Self {
        self.environment = self.environment.separator(separator);
        self
    }

    /// See [Environment::list_separator]
    pub fn list_separator(mut self, separator: &str) -> Self {
        self.environment = self.environment.list_separator(separator);
        self
    }

    /// See [Environment::with_list_parse_key]
    pub fn with_list_parse_key(mut self, key: &str) -> Self {
        self.environment = self.environment.with_list_parse_key(key);
        self
    }

    /// See [Environment::ignore_empty]
    pub fn ignore_empty(mut self, ignore: bool) -> Self {
        self.environment = self.environment.ignore_empty(ignore);
        self
    }

    /// See [Environment::try_parsing]
    pub fn try_parsing(mut self, try_parsing: bool) -> Self {
        self.environment = self.environment.try_parsing(try_parsing);
        self
    }

    /// See [Environment::keep_prefix]
    pub fn keep_prefix(mut self, keep: bool) -> Self {
        self.environment = self.environment.keep_prefix(keep);
        self
    }
}

impl Source for ShuttleSecretStore {
    fn clone_into_box(&self) -> Box<dyn Source + Send + Sync> {
        Box::new((*self).clone())
    }

    fn collect(&self) -> Result<Map<String, Value>, ConfigError> {
        self.environment.collect()
    }
}