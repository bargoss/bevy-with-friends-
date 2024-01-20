use std::collections::HashMap;
use std::net::{Ipv4Addr, SocketAddr};
use std::time::Duration;

use bevy::log;
use bevy::prelude::*;
use lightyear::prelude::*;
use lightyear::prelude::server::*;

use crate::lightyear_demo::{KEY, PROTOCOL_ID, SERVER_PORT};
use crate::lightyear_demo::components::PawnBundle;
use crate::lightyear_demo::systems::*;

use super::shared::*;

// define a bevy plugin

pub struct DemoServerPlugin;

impl Plugin for DemoServerPlugin {
    fn build(&self, app: &mut App) {
        //let server_addr = SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), self.port);
        let server_addr = SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), SERVER_PORT);
        let netcode_config = NetcodeConfig::default()
            .with_protocol_id(PROTOCOL_ID)
            .with_key(KEY);
        let link_conditioner = LinkConditionerConfig {
            incoming_latency: Duration::from_millis(80),
            incoming_jitter: Duration::from_millis(0),
            incoming_loss: 0.00,
        };
        //let transport = match self.transport {
        //    Transports::Udp => TransportConfig::UdpSocket(server_addr),
        //    Transports::Webtransport => TransportConfig::WebTransportServer {
        //        server_addr,
        //        certificate: Certificate::self_signed(&["localhost"]),
        //    },
        //};
        let transport = TransportConfig::UdpSocket(server_addr);
        let io =
            Io::from_config(IoConfig::from_transport(transport).with_conditioner(link_conditioner));
        let config = ServerConfig {
            shared: shared_config().clone(),
            netcode: netcode_config,
            ping: PingConfig::default(),
        };
        let plugin_config = PluginConfig::new(config, io, protocol());

        app
            .add_plugins(ServerPlugin::new(plugin_config))
            .add_plugins(SharedPlugin)

            //.add_systems(
            //    FixedUpdate,
            //    (
            //    ).in_set(FixedUpdateMainSet::Pull)
            //)

            .add_systems(Update, handle_connections)
            .add_systems(Startup, init);
    }
}



#[derive(Resource, Default)]
pub struct Global {
    pub client_id_to_entity_id: HashMap<ClientId, Entity>,
}

fn handle_connections(
    mut connections: EventReader<ConnectEvent>,
    mut disconnections: EventReader<DisconnectEvent>,
    mut global: ResMut<Global>,
    mut commands: Commands,
) {
    for connection in connections.read() {
        let client_id = connection.context();
        let h = (((client_id * 30) % 360) as f32) / 360.0;
        let s = 0.8;
        let l = 0.5;
        let entity = commands.spawn(PawnBundle::new(
            // psuedo random pos
            Vec3::new((client_id % 10) as f32, (client_id / 10) as f32, 0.0),
            0.5,
            Color::hsl(h, s, l),
            *client_id,
        ));
        log::info!("SPAWNED CLIENT ENTITY");
        // Add a mapping from client id to entity id
        global
            .client_id_to_entity_id
            .insert(*client_id, entity.id());
    }
    for disconnection in disconnections.read() {
        let client_id = disconnection.context();
        if let Some(entity) = global.client_id_to_entity_id.remove(client_id) {
            commands.entity(entity).despawn();
        }
    }
}

/*
pub(crate) fn handle_connections(
    mut connections: EventReader<ConnectEvent>,
    mut disconnections: EventReader<DisconnectEvent>,
    mut global: ResMut<Global>,
    mut commands: Commands,
) {
    for connection in connections.read() {
        let client_id = connection.context();
        // Generate pseudo random color from client id.
        let h = (((client_id * 30) % 360) as f32) / 360.0;
        let s = 0.8;
        let l = 0.5;
        let entity = commands.spawn(PlayerBundle::new(
            *client_id,
            Vec2::ZERO,
            Color::hsl(h, s, l),
        ));
        // Add a mapping from client id to entity id
        global
            .client_id_to_entity_id
            .insert(*client_id, entity.id());
    }
    for disconnection in disconnections.read() {
        let client_id = disconnection.context();
        if let Some(entity) = global.client_id_to_entity_id.remove(client_id) {
            commands.entity(entity).despawn();
        }
    }
}
*/

fn init(
    mut commands: Commands
) {
    commands.spawn(Camera3dBundle::default());
    commands.spawn(TextBundle::from_section(
        "Server",
        TextStyle {
            font_size: 30.0,
            color: Color::WHITE,
            ..default()
        },
    ));
}