use std::fmt::Debug;
use error::*;

#[cfg(any(feature = "remote-etcd", feature = "remote-etcd-tls"))]
mod etcd;

// #[cfg(feature = "remote-consul")]
// mod consul;

#[cfg(any(feature = "remote-etcd", feature = "remote-etcd-tls"))]
pub use self::etcd::Etcd;

// #[cfg(feature = "remote-consul")]
// pub use self::consul::Consul;

/// Represents a remote client that can accept a key
/// and return a string
pub trait Remote: Debug {
    fn clone_into_box(&self) -> Box<Remote + Send + Sync>;
    fn get(&self, key: &str) -> Result<String>;
    fn uri(&self) -> String;
}

impl Clone for Box<Remote + Send + Sync> {
    fn clone(&self) -> Self {
        self.clone_into_box()
    }
}
