use std::net::{Ipv4Addr, SocketAddr};
use std::time::Duration;

use bevy::prelude::*;
use lightyear::prelude::*;
use lightyear::prelude::client::*;

use crate::lightyear_demo::{CLIENT_PORT, KEY, PROTOCOL_ID, SERVER_PORT};
use crate::lightyear_demo::systems::*;

use super::shared::*;

// define a bevy plugin

pub struct DemoClientPlugin{
    pub headless: bool
}

impl Plugin for DemoClientPlugin {
    fn build(&self, app: &mut App) {
        //let server_addr = SocketAddr::new(self.server_addr.into(), self.server_port);
        // localhost
        let server_addr = SocketAddr::new(Ipv4Addr::new(127, 0, 0, 1).into(), SERVER_PORT);
        let auth = Authentication::Manual {
            server_addr,
            client_id: 69,
            private_key: KEY,
            protocol_id: PROTOCOL_ID,
        };
        //let client_addr = SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), self.client_port);
        let client_addr = SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), CLIENT_PORT);
        let link_conditioner = LinkConditionerConfig {
            incoming_latency: Duration::from_millis(0), //incoming_latency: Duration::from_millis(200),
            incoming_jitter: Duration::from_millis(0),
            incoming_loss: 0.0,
        };
        //let transport = match self.transport {
        //    Transports::Udp => TransportConfig::UdpSocket(client_addr),
        //    Transports::Webtransport => TransportConfig::WebTransportClient {
        //        client_addr,
        //        server_addr,
        //    },
        //};
        let transport = TransportConfig::UdpSocket(client_addr);
        let io =
            Io::from_config(&IoConfig::from_transport(transport).with_conditioner(link_conditioner));
        let config = ClientConfig {
            shared: shared_config().clone(),
            input: InputConfig::default(),
            netcode: Default::default(),
            ping: PingConfig::default(),
            sync: SyncConfig::default(),
            prediction: PredictionConfig::default(),
            // we are sending updates every frame (60fps), let's add a delay of 6 network-ticks
            interpolation: InterpolationConfig::default()
                .with_delay(InterpolationDelay::default().with_send_interval_ratio(2.0)),
        };
        let plugin_config = PluginConfig::new(config, io, protocol(), auth);
        //app.add_plugins(ClientPlugin::new(plugin_config));


        app
            .add_plugins(SharedPlugin)
            .add_plugins(ClientPlugin::new(plugin_config))
            .add_systems(Startup, init)

            .add_systems(
                FixedUpdate,
                (
                    handle_simulated_tag_client,
                    update_time_client,
                ).in_set(FixedUpdateMainSet::Pull)
            )


            .add_systems(FixedUpdate, handle_pawn_input_client
                .in_set(FixedUpdateMainSet::Pull))
        ;

        if !self.headless{
            app.add_systems(FixedUpdate, buffer_input.in_set(InputSystemSet::BufferInputs));
        }
    }
}

fn init(
    mut commands: Commands,
    mut client: ResMut<Client<MyProtocol>>,
) {
    //commands.spawn(Camera3dBundle::default());
    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 20.0))
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
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