use std::collections::HashMap;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::time::Duration;
use bevy::log;
use lightyear::prelude::*;
use bevy::prelude::*;
use lightyear::client::resource::Client;
use lightyear::prelude::server::*;
use crate::lightyear_demo::components::PawnBundle;
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
        //let io_config = IoConfig::from_transport(TransportConfig::LocalChannel)
        //    .with_conditioner(link_conditioner);

        app
            .add_plugins(
                ServerPlugin::new(PluginConfig::new(
                    ServerConfig::default(),
                    Io::from_config(&io_config),
                    protocol()
                )
            ))
            .init_resource::<Global>()
            .add_systems(Update, handle_connections)
            .add_systems(Startup, init)
        ;
    }
}



#[derive(Resource, Default)]
struct Global {
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
        // Generate pseudo random color from client id.
        let h = (((client_id * 30) % 360) as f32) / 360.0;
        let s = 0.8;
        let l = 0.5;

        let entity = commands.spawn(PawnBundle::new(
            Vec3::new(0.0, 0.0, 0.0),
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