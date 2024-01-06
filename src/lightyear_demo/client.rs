use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::time::Duration;
use lightyear::prelude::*;
use bevy::prelude::*;
use lightyear::prelude::client::{Authentication, ClientConfig, ClientPlugin, PluginConfig};
use crate::lightyear_demo::SERVER_PORT;
use super::shared::*;


// define a bevy plugin

pub struct DemoClientPlugin;

impl Plugin for DemoClientPlugin {
    fn build(&self, app: &mut App) {
        let server_addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), SERVER_PORT);
        let auth = Authentication::Manual {
            // server's IP address
            server_addr,
            // ID to uniquely identify the client
            client_id: 23141,
            // private key shared between the client and server
            private_key: lightyear::prelude::Key::default(),
            // PROTOCOL_ID identifies the version of the protocol
            protocol_id: 0
        };

        // You can add a link conditioner to simulate network conditions
        let link_conditioner = LinkConditionerConfig {
            incoming_latency: Duration::from_millis(100),
            incoming_jitter: Duration::from_millis(0),
            incoming_loss: 0.00,
        };

        let addr = SocketAddrV4::new(Ipv4Addr::LOCALHOST, SERVER_PORT);

        let io_config = IoConfig::from_transport(TransportConfig::UdpSocket(SocketAddr::V4(addr)))
            .with_conditioner(link_conditioner);



        //app
        //    .add_plugin(
        //        ClientPlugin::new(
        //            PluginConfig::new(
        //                ClientConfig::default(),
        //                io_config
        //            )
        //        )
        //    );


        app
            .add_plugins(MinimalPlugins)
            .add_plugins(
                ClientPlugin::new(PluginConfig::new(
                    ClientConfig::default(),
                    Io::from_config(&io_config),
                    protocol(),
                    auth
                ))
            );
    }
}