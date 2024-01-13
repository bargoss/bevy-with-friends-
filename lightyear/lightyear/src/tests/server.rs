use std::default::Default;
use std::net::SocketAddr;
use std::str::FromStr;
use std::time::Duration;

use bevy::app::App;

use crate::prelude::server::*;
use crate::prelude::*;
use crate::tests::protocol::*;

pub fn setup(protocol_id: u64, private_key: Key) -> anyhow::Result<Server> {
    // create udp-socket based io
    let addr = SocketAddr::from_str("127.0.0.1:0")?;
    let netcode_config = NetcodeConfig::default()
        .with_protocol_id(protocol_id)
        .with_key(private_key);
    let io = Io::from_config(IoConfig::from_transport(TransportConfig::UdpSocket(addr)));
    let config = ServerConfig {
        shared: SharedConfig {
            enable_replication: false,
            tick: TickConfig::new(Duration::from_millis(10)),
            ..Default::default()
        },
        netcode: netcode_config,
        ping: PingConfig::default(),
    };

    // create lightyear server
    Ok(Server::new(config, io, protocol()))
}

pub fn bevy_setup(app: &mut App, addr: SocketAddr, protocol_id: u64, private_key: Key) {
    // create udp-socket based io
    let netcode_config = NetcodeConfig::default()
        .with_protocol_id(protocol_id)
        .with_key(private_key);
    let io = Io::from_config(IoConfig::from_transport(TransportConfig::UdpSocket(addr)));
    let config = ServerConfig {
        shared: SharedConfig {
            enable_replication: false,
            tick: TickConfig::new(Duration::from_millis(10)),
            ..Default::default()
        },
        netcode: netcode_config,
        ping: PingConfig::default(),
    };
    let plugin_config = PluginConfig::new(config, io, protocol());
    let plugin = ServerPlugin::new(plugin_config);
    app.add_plugins(plugin);
}
