use bevy::math::Vec3;
use bevy::prelude::{Color, Commands, EventReader, ResMut};
use lightyear::prelude::server::{ConnectEvent, DisconnectEvent};
use crate::lightyear_demo::components_old::PawnBundle;
use crate::lightyear_demo::server::Global;

pub fn handle_connections(
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
        bevy::log::info!("SPAWNED CLIENT ENTITY");
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