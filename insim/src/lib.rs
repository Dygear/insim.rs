#![doc = include_str!("../README.md")]

pub mod builder;
pub mod error;
pub mod insim;
pub mod net;
pub mod packet;
pub mod relay;
pub mod result;

const VERSION: u8 = 9;

use std::net::SocketAddr;

#[doc(hidden)]
/// Rexport insim_core
pub use insim_core as core;

#[cfg(feature = "pth")]
/// Report insim_pth when pth feature is enabled
pub use insim_pth as pth;

#[cfg(feature = "smx")]
/// Report insim_smx when smx feature is enabled
pub use insim_smx as smx;

pub use error::Error;
pub use packet::Packet;
pub use result::Result;

/// Shortcut method to create a TCP connection
///
/// # Examples
/// ```rust
/// let conn = insim::tcp("127.0.0.1:2999").connect().await?;
/// while let Some(packet) = conn.read().await? {
///     println!("{:?}", packet);
/// }
/// ```
pub fn tcp<R: Into<SocketAddr>>(remote_addr: R) -> builder::Builder {
    builder::Builder::default().tcp(remote_addr)
}

/// Shortcut method to create a UDP connection.
/// If local_addr is not provided then we will bind to "0.0.0.0:0" (all addresses, random port).
///
/// # Examples
/// ```rust
/// let conn = insim::udp("127.0.0.1:2999", None).connect().await?;
/// while let Some(packet) = conn.read().await? {
///     println!("{:?}", packet);
/// }
/// ```
pub fn udp<L: Into<Option<SocketAddr>>, R: Into<SocketAddr>>(
    remote_addr: R,
    local_addr: L,
) -> builder::Builder {
    builder::Builder::default().udp(remote_addr, local_addr)
}

/// Shortcut method to create a LFS World Relay connection.
///
/// # Examples
/// ```rust
/// let conn = insim::relay()
///     .relay_select_host("Nubbins AU Demo")
///     .relay_websocket(true)
///     .connect()
///     .await?;
/// while let Some(packet) = conn.read().await? {
///     println!("{:?}", packet);
/// }
/// ```
pub fn relay() -> builder::Builder {
    builder::Builder::default().relay()
}
