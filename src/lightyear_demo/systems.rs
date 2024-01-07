use bevy::math::Vec3;
use bevy::prelude::*;
use bevy_vector_shapes::painter::ShapePainter;
use lightyear::prelude::client::Predicted;
use lightyear::prelude::Replicate;
use lightyear::prelude::server::Server;
use lightyear::shared::events::InputEvent;
use crate::defender_game::utils;
use crate::lightyear_demo::shared::{Inputs, MyProtocol, ReplicatedPosition};
use super::components::*;
use super::server::Global;

pub fn draw_circle_view(
    circle_views: Query<(&CircleView, &Transform)>,
    mut painter: ShapePainter
)
{
    circle_views.for_each(|(circle_view, transform)|{
        utils::draw_o(
            Vec3::new(transform.translation.x, transform.translation.y, 0.0),
            circle_view.radius,
            circle_view.color,
            &mut painter
        );
    });
}

/*

//app.add_systems(FixedUpdate, movement.in_set(FixedUpdateSet::Main));
pub fn pawn_movement(
    mut query: Query<(Entity, &mut Transform), With<Predicted>>,
    mut input_reader: EventReader<InputEvent<Inputs>>,
    global: Res<Global>,
    server: Res<Server<MyProtocol>>,
){
    for input in input_reader.read() {
        let client_id = input.context();
        if let Some(input) = input.input() {
            if let Some(player_entity) = global.client_id_to_entity_id.get(client_id) {
                if let Ok(position) = query.get_mut(*player_entity) {
                    shared_movement_behaviour(position, input);
                }
            }
        }
    }
}

// app.add_systems(FixedUpdate, movement.in_set(FixedUpdateSet::Main));
pub(crate) fn movement(
    mut position_query: Query<&mut PlayerPosition, With<Predicted>>,
    mut input_reader: EventReader<InputEvent<Inputs>>,
) {
    if PlayerPosition::mode() != ComponentSyncMode::Full {
        return;
    }
    for input in input_reader.read() {
        if let Some(input) = input.input() {
            for mut position in position_query.iter_mut() {
                shared_movement_behaviour(position, input);
            }
        }
    }
}
*/