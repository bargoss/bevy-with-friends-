//! The transport layer is responsible for sending and receiving raw byte arrays packets through the network.

/// A conditioner is used to simulate network conditions such as latency, jitter and packet loss.
pub(crate) mod conditioner;

/// io is a wrapper around the underlying transport layer
pub mod io;

/// The transport is a local channel
pub(crate) mod local;

/// The transport is a UDP socket
pub(crate) mod udp;

/// The transport is a map of channels (used for server, during testing)
pub(crate) mod channels;

/// The transport is using WebTransport
#[cfg(feature = "webtransport")]
pub mod webtransport;

use std::io::Result;
use std::net::SocketAddr;

pub const LOCAL_SOCKET: SocketAddr = SocketAddr::new(
    std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)),
    0,
);

/// Transport combines a PacketSender and a PacketReceiver
pub trait Transport {
    /// Return the local socket address for this transport
    fn local_addr(&self) -> SocketAddr;
    fn listen(self) -> (Box<dyn PacketSender>, Box<dyn PacketReceiver>);

    // fn split(&mut self) -> (Box<dyn PacketReceiver>, Box<dyn PacketSender>);
}

/// Send data to a remote address
pub trait PacketSender: Send + Sync {
    /// Send data on the socket to the remote address
    fn send(&mut self, payload: &[u8], address: &SocketAddr) -> Result<()>;
}

impl PacketSender for Box<dyn PacketSender> {
    fn send(&mut self, payload: &[u8], address: &SocketAddr) -> Result<()> {
        (**self).send(payload, address)
    }
}

/// Receive data from a remote address
pub trait PacketReceiver: Send + Sync {
    /// Receive a packet from the socket. Returns the data read and the origin.
    ///
    /// Returns Ok(None) if no data is available
    fn recv(&mut self) -> Result<Option<(&mut [u8], SocketAddr)>>;
}

impl PacketReceiver for Box<dyn PacketReceiver> {
    fn recv(&mut self) -> Result<Option<(&mut [u8], SocketAddr)>> {
        (**self).recv()
    }
}
