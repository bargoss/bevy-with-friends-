use bevy::math::Vec3;
use bevy::prelude::*;
use bevy_vector_shapes::painter::ShapePainter;
use lightyear::prelude::client::Predicted;
use lightyear::prelude::{ClientId, Replicate};
use lightyear::prelude::server::Server;
use lightyear::shared::events::InputEvent;
use log::log;
use crate::defender_game::utils;
use crate::lightyear_demo::shared::{Inputs, MyProtocol, ReplicatedPosition, Simulated};
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
app.add_systems(FixedUpdate, movement.in_set(FixedUpdateSet::Main));
*/
pub fn direction_input_to_vec3(dir: &super::shared::Direction) -> Vec3{
    let mut movement_direction = Vec3::ZERO;
    if dir.up {
        movement_direction.y += 1.0;
    }
    if dir.down {
        movement_direction.y -= 1.0;
    }
    if dir.left {
        movement_direction.x -= 1.0;
    }
    if dir.right {
        movement_direction.x += 1.0;
    }

    movement_direction
}

pub fn handle_pawn_input_client(
    mut pawn_query: Query<(&Pawn, &mut PawnInput), With<Predicted>>,
    mut input_reader: EventReader<InputEvent<Inputs>>,
){
    for input in input_reader.read() {
        if let Some(input) = input.input() {
            for mut data in pawn_query.iter_mut() {
                match input { Inputs::Move(direction) => {
                    data.1.movement_direction = direction_input_to_vec3(direction);
                }}
            }
        }
    }
}
pub fn handle_pawn_input_server(
    mut pawn_query: Query<(&Pawn, &mut PawnInput)>,
    mut input_reader: EventReader<InputEvent<Inputs, ClientId>>,
    //mut input_reader2: EventReader<InputEvent<Inputs>>,
    global: Res<Global>,
    //server: Res<Server<MyProtocol>>, not necessary
){
    //input_reader2.read().for_each(|input|{
    //    log::info!("input_reader2");
    //});

    for input in input_reader.read() {
        log::info!("a");
        let client_id = input.context();
        if let Some(input) = input.input() {
            log::info!("b");
            if let Some(player_entity) = global.client_id_to_entity_id.get(client_id) {
                log::info!("c");
                if let Ok(mut data) = pawn_query.get_mut(*player_entity) {
                    log::info!("d");
                    if let Inputs::Move(dir) = input {
                        log::info!("e");
                        let input = direction_input_to_vec3(dir);
                        log::info!("client input on server: {:?}", input);
                        data.1.movement_direction = input;
                    }
                }
            }
        }
    }
}

pub fn create_replicated_transforms(
    mut commands: Commands,
    // if it doesnt have transform
    query: Query<(Entity, &ReplicatedPosition), Without<Transform>>,
){
    query.for_each(|(entity, replicated_position)|{
        commands.entity(entity).insert(TransformBundle{
            local: Transform::from_translation(Vec3::new(replicated_position.0.x, replicated_position.0.y, replicated_position.0.z)),
            ..Default::default()
        });
    });
}
pub fn pull_replicated_positions(
    mut query: Query<(&ReplicatedPosition, &mut Transform)>,
){
    query.for_each_mut(|(replicated_position, mut transform)|{
        transform.translation.x = replicated_position.0.x;
        transform.translation.y = replicated_position.0.y;
        transform.translation.z = replicated_position.0.z;
    });
}
pub fn push_replicated_positions(
    mut query: Query<(&mut ReplicatedPosition, &Transform)>,
){
    query.for_each_mut(|(mut replicated_position, transform)|{
        replicated_position.0.x = transform.translation.x;
        replicated_position.0.y = transform.translation.y;
        replicated_position.0.z = transform.translation.z;
    });
}

pub fn handle_pawn_movement(
    mut pawn_query: Query<(&Pawn, &PawnInput, &mut Transform), With<Simulated>>,
){
    let speed = 0.05;

    pawn_query.for_each_mut(|(_, pawn_input, mut transform)|{
        let movement_direction = pawn_input.movement_direction;
        transform.translation.x += movement_direction.x * speed;
        transform.translation.y += movement_direction.y * speed;
    });

}