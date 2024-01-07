use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::time::Duration;
use lightyear::prelude::*;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_vector_shapes::ShapePlugin;
use lightyear::prelude::client::*;
use crate::lightyear_demo::{CLIENT_PORT, KEY, PROTOCOL_ID, SERVER_PORT};
use super::shared::*;


// define a bevy plugin

pub struct DemoClientPlugin;

impl Plugin for DemoClientPlugin {
    fn build(&self, app: &mut App) {
        let server_addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), SERVER_PORT);
        //let server_addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), CLIENT_PORT);
        let auth = Authentication::Manual {
            // server's IP address
            server_addr,
            // ID to uniquely identify the client
            client_id: 23141,
            // private key shared between the client and server
            private_key: KEY,
            // PROTOCOL_ID identifies the version of the protocol
            protocol_id: PROTOCOL_ID
        };

        // You can add a link conditioner to simulate network conditions
        let link_conditioner = LinkConditionerConfig {
            incoming_latency: Duration::from_millis(100),
            incoming_jitter: Duration::from_millis(0),
            incoming_loss: 0.00,
        };

        let addr = SocketAddrV4::new(Ipv4Addr::LOCALHOST, CLIENT_PORT);

        let io_config = IoConfig::from_transport(TransportConfig::UdpSocket(SocketAddr::V4(addr)))
            .with_conditioner(link_conditioner);


        app
            .add_plugins(
                ClientPlugin::new(PluginConfig::new(
                    ClientConfig::default(),
                    Io::from_config(&io_config),
                    protocol(),
                    auth
                ))
            )
            .add_systems(Startup, init)
        ;
    }
}

fn init(
    mut commands: Commands,
    mut client: ResMut<Client<MyProtocol>>,
) {
    commands.spawn(Camera3dBundle::default());
    commands.spawn(TextBundle::from_section(
        "Client",
        TextStyle {
            font_size: 30.0,
            color: Color::WHITE,
            ..default()
        },
    ));
    client.connect();
}