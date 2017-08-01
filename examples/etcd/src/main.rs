// extern crate etcd;
// extern crate futures;
// extern crate tokio_core;
extern crate config;

use config::{Config, File, remote};

// use etcd::Client;
// use etcd::kv::{self, Action};
// use futures::Future;
// use tokio_core::reactor::Core;

fn main() {
    // let mut core = Core::new().unwrap();
    // let handle = core.handle();

    // let client = Client::new(&handle, &["http://localhost:2379"], None).unwrap();

    // // Set the key "/foo" to the value "bar" with no expiration.
    // let work = kv::set(&client, "/foo", "bar", None).and_then(|_| {
    //     // Once the key has been set, ask for details about it.
    //     let get_request = kv::get(&client, "/", kv::GetOptions {
    //         strong_consistency: false,
    //         sort: false,
    //         recursive: true,
    //     });

    //     get_request.and_then(|response| {
    //         println!("{:?}", response.data);
    //         // The information returned tells you what kind of operation was performed.
    //         // assert_eq!(response.data.action, Action::Get);

    //         // // The value of the key is what we set it to previously.
    //         // assert_eq!(response.data.node.value, Some("bar".to_string()));

    //         // // Each API call also returns information about the etcd cluster extracted from
    //         // // HTTP response headers.
    //         // assert!(response.cluster_info.etcd_index.is_some());

    //         Ok(())
    //     })
    // });

    // core.run(work).unwrap();

    // Create a simple config store that is driven by our local
    // Settings.json file and overridden by etcd (within the /config path)
    let config = Config::default()
        .merge(File::with_name("Settings")).unwrap()
        .merge(remote::Etcd::new(&["http://localhost:2379"], None).with_path("/config")).unwrap()
        .clone();
}
