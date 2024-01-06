use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::time::Duration;
use lightyear::prelude::*;
use bevy::prelude::*;
use lightyear::prelude::server::*;
use crate::lightyear_demo::SERVER_PORT;
use super::shared::*;


// define a bevy plugin

pub struct DemoServerPlugin;

impl Plugin for DemoServerPlugin {
    fn build(&self, app: &mut App) {
        let addr = SocketAddrV4::new(Ipv4Addr::LOCALHOST, SERVER_PORT);
        // You can add a link conditioner to simulate network conditions
        let link_conditioner = LinkConditionerConfig {
            incoming_latency: Duration::from_millis(100),
            incoming_jitter: Duration::from_millis(0),
            incoming_loss: 0.00,
        };
        let io_config = IoConfig::from_transport(TransportConfig::UdpSocket(SocketAddr::V4(addr)))
            .with_conditioner(link_conditioner);

        app
            .add_plugins(
                ServerPlugin::new(PluginConfig::new(
                    ServerConfig::default(),
                    Io::from_config(&io_config),
                    protocol()
                )
            ));
    }
}