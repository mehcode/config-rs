extern crate config;

use config::{remote, Config, File, FileFormat, Table};

// NOTE: This example expects an etcd note at localhost:2379
//  - etcdctl set /config/key/value 26
//  - etcdctl set /config/debug false
//  - etcdctl set /config/name "Remote"
//  - etcdctl set /jsonfile '{"debug": false, "name": "Remote"}'

fn main() {
    let mut config = Config::default();

    // Read etcd configuration from remote; starting at the /config path
    config
        .merge(
            remote::Etcd::new(&["http://localhost:2379"]).with_prefix("/config"),
        )
        .unwrap();

    println!("{:?}", config.clone().try_into::<Table>().unwrap());

    assert_eq!(config.get("key.value").ok(), Some(26));
    assert_eq!(config.get("debug").ok(), Some(false));
    assert_eq!(config.get("name").ok(), Some("Remote".to_string()));

    // Read etcd configuration from remote; but, only the /key/value and /name paths
    config.clear().unwrap();
    config
        .merge(
            remote::Etcd::new(&["http://localhost:2379"])
                .with_prefix("/config")
                .with_paths(&["/key/value", "/name"]),
        )
        .unwrap();

    println!("{:?}", config.clone().try_into::<Table>().unwrap());

    assert_eq!(config.get("key.value").ok(), Some(26));
    assert_eq!(config.get::<bool>("debug").ok(), None);
    assert_eq!(config.get("name").ok(), Some("Remote".to_string()));

    // Read etcd configuration from remote but parse as a JSON file
    config.clear().unwrap();
    config
        .merge(File::from_remote(
            remote::Etcd::new(&["http://localhost:2379"]),
            "/jsonfile",
            FileFormat::Json,
        ))
        .unwrap();

    println!("{:?}", config.clone().try_into::<Table>().unwrap());

    assert_eq!(config.get("debug").ok(), Some(false));
    assert_eq!(config.get("name").ok(), Some("Remote".to_string()));
}
