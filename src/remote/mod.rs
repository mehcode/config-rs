#[cfg(any(feature = "remote-etcd", feature = "remote-etcd-tls"))]
mod etcd;

#[cfg(feature = "remote-consul")]
mod consul;

#[cfg(any(feature = "remote-etcd", feature = "remote-etcd-tls"))]
pub use self::etcd::Etcd;

#[cfg(feature = "remote-consul")]
pub use self::consul::Consul;
