#[cfg(feature = "json")]
mod example {
    use std::error::Error;

    use config::{builder::AsyncState, AsyncSource, ConfigBuilder, ConfigError, FileFormat, Map};

    use async_trait::async_trait;
    use warp::Filter;

    // Example below presents sample configuration server and client.
    //
    // Server serves simple configuration on HTTP endpoint.
    // Client consumes it using custom HTTP AsyncSource built on top of reqwest.


    pub async fn run_server() -> Result<(), Box<dyn Error>> {
        let service = warp::path("configuration").map(|| r#"{ "value" : 123 }"#);

        println!("Running server on localhost:5001");

        warp::serve(service).bind(([127, 0, 0, 1], 5001)).await;

        Ok(())
    }

    pub async fn run_client() -> Result<(), Box<dyn Error>> {
        // Good enough for an example to allow server to start
        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

        let config = ConfigBuilder::<AsyncState>::default()
            .add_async_source(HttpSource {
                uri: "http://localhost:5001/configuration".into(),
                format: FileFormat::Json,
            })
            .build()
            .await?;

        println!("Config value is {}", config.get::<String>("value")?);

        Ok(())
    }

    // Actual implementation of AsyncSource can be found below

    #[derive(Debug)]
    struct HttpSource {
        uri: String,
        format: FileFormat,
    }

    #[async_trait]
    impl AsyncSource for HttpSource {
        async fn collect(&self) -> Result<Map<String, config::Value>, ConfigError> {
            reqwest::get(&self.uri)
                .await
                .map_err(|e| ConfigError::Foreign(Box::new(e)))? // error conversion is possible from custom AsyncSource impls
                .text()
                .await
                .map_err(|e| ConfigError::Foreign(Box::new(e)))
                .and_then(|text| {
                    self.format
                        .parse(Some(&self.uri), &text)
                        .map_err(|e| ConfigError::Foreign(e))
                })
        }
    }

}

#[cfg(feature = "json")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use futures::{select, FutureExt};
    select! {
        r = example::run_server().fuse() => r,
        r = example::run_client().fuse() => r
    }
}

#[cfg(not(feature = "json"))]
fn main() {
    println!("This example needs the 'json' feature enabled");
}

