use std::collections::BTreeMap;

use serde_derive::Deserialize;
use shuttle_secrets::SecretStore;

use config::{Config, ShuttleSecretStore};

#[derive(Deserialize, Debug, PartialEq)]
pub struct MyAppConfiguration {
    pub authentication: AuthenticationSettings,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct AuthenticationSettings {
    pub token_secret: String,
    pub github: GitHubSettings,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct GitHubSettings {
    pub client_id: String,
    pub client_secret: String,
}

#[test]
#[cfg(feature = "shuttle")]
fn test_read_secrets_from_store() {
    let secrets = BTreeMap::from([
        (
            "MY_APP_AUTHENTICATION__TOKEN_SECRET".to_owned(),
            "my_secret".to_owned().into(),
        ),
        (
            "MY_APP_AUTHENTICATION__GITHUB__CLIENT_ID".to_owned(),
            "my_client_id".to_owned().into(),
        ),
        (
            "MY_APP_AUTHENTICATION__GITHUB__CLIENT_SECRET".to_owned(),
            "my_client_secret".to_owned().into(),
        ),
    ]);
    let secret_store = SecretStore::new(secrets);
    let config = Config::builder()
        .add_source(
            ShuttleSecretStore::new(&secret_store)
                .prefix("MY_APP")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()
        .unwrap()
        .try_deserialize::<MyAppConfiguration>()
        .unwrap();

    assert_eq!(
        config,
        MyAppConfiguration {
            authentication: AuthenticationSettings {
                token_secret: "my_secret".to_string(),
                github: GitHubSettings {
                    client_id: "my_client_id".to_string(),
                    client_secret: "my_client_secret".to_string(),
                },
            }
        }
    )
}
