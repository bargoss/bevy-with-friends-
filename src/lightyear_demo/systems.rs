
use lightyear::shared::tick_manager::Tick;
use bevy::math::Vec3;
use bevy::prelude::*;
use bevy_vector_shapes::painter::ShapePainter;

use lightyear::prelude::client::{Confirmed, Interpolated, Predicted};
use lightyear::prelude::{ClientId, NetworkIdentity, NetworkTarget, PreSpawnedPlayerObject, ReplicationGroup, TickManager};
use lightyear::shared::events::InputEvent;

use crate::defender_game::utils;
use crate::lightyear_demo::server::Global;
use crate::lightyear_demo::shared::{ClientMut, DirectionInput, Inputs, PawnInputData, PlayerId, Replicate, Simulated};


use super::components::*;

pub fn draw_circle_view(
    circle_views: Query<(Entity,&CircleView, &Transform)>,
    confirmed: Query<&Confirmed>,
    mut painter: ShapePainter
)
{

    circle_views.for_each(|(entity,circle_view, transform)|{
        let mut color = Color::rgb(0.0, 0.0, 0.0);
        if confirmed.get(entity).is_ok() {
            color = Color::rgb(0.0, 1.0, 0.0);
        }
        else{
            color = Color::rgb(1.0, 0.0, 0.0);
        }

        utils::draw_o(
            transform.translation,
            circle_view.radius,
            color,
            &mut painter
        );
    });
}



pub fn handle_pawn_movement(
    mut pawn_query: Query<(&Pawn, &PawnInput, &mut Transform), Or<(With<Predicted>, With<Replicate>)>>,
){
    let speed = 0.25;
    pawn_query.for_each_mut(|(_, pawn_input, mut replicated_position)|{
        replicated_position.translation += pawn_input.movement_direction * speed;
    });
}


// this didnt match on the client and thats why I had prediction problems I think
// Without<Confirmed>, Without<Interpolated>
pub fn handle_projectile(
    //mut projectile_query: Query<(Entity,&mut Projectile, &SimpleVelocity, &mut Transform),With<Simulated>>,
    mut projectile_query: Query<(Entity,&mut Projectile, &SimpleVelocity, &mut Transform),(Without<Confirmed>, Without<Interpolated>)>,
    mut commands: Commands,
    tick_manager: Res<TickManager>
){
    projectile_query.for_each_mut(|(entity, mut projectile, velocity, mut replicated_position)|{
        replicated_position.translation += 0.15 * Vec3::new(0.0, -1.0, 0.0);


        if tick_manager.tick().0 - projectile.start_tick.0 > 100 {
            commands.entity(entity).despawn();
        }
    });
}


pub fn cause_mis_predictions(mut transforms: Query<&mut Transform>, mut counter: Local<i32>) {
    for mut transform in &mut transforms.iter_mut() {
        transform.translation += 0.01 * ((*counter % 10) - 5) as f32;
    }
    *counter += 1;
}
pub fn handle_pawn_shooting(
    mut pawn_query: Query<(Entity,&mut Pawn, &PawnInput, &Transform, &PlayerId), With<Simulated>>,
    tick_manager: Res<TickManager>,
    //global_time: Res<GlobalTime>,
    mut commands: Commands,
    identity: NetworkIdentity,
){
    pawn_query.for_each_mut(|(entity, mut pawn, pawn_input, mut transform, player_id)|{
        let last_shot_tick = pawn.last_attack_time;
        let current_tick = tick_manager.tick();
        let ticks_since_last_shot = current_tick.0 - last_shot_tick.0;
        let cooldown = 50;
        let cooldown_finished = ticks_since_last_shot > cooldown;

        let moving_prev = pawn.moving;
        if(pawn_input.movement_direction != Vec3::ZERO){
            pawn.moving = true;
        }
        else{
            pawn.moving = false;
        }

        let mut shoot_now = false;
        if(moving_prev != pawn.moving){
            shoot_now = true;
        }

        //let shoot_now = pawn_input.attack && cooldown_finished;
        if shoot_now {

            pawn.last_attack_time = current_tick;

            let shoot_dir = Vec3::new(0.0, -1.0, 0.0);


            let owner_client_id = *player_id.clone();
            let start_tick = current_tick;
            let position = transform.translation;
            let velocity = shoot_dir;


            let projectile =ProjectileBundle::new(
                owner_client_id,
                start_tick,
                position,
                velocity
            );

            if identity.is_server() {
                commands.spawn((projectile, PreSpawnedPlayerObject::default(), Replicate{
                    prediction_target: NetworkTarget::Only(vec![owner_client_id]),
                    replication_group : ReplicationGroup::Group(owner_client_id),
                    interpolation_target: NetworkTarget::AllExcept(vec![owner_client_id]),
                    ..Default::default()
                }));
            } else {
                commands.spawn((projectile, PreSpawnedPlayerObject::default()));
            }

            //replicate: Replicate{
            //    prediction_target: NetworkTarget::Only(vec![owner_client_id]),
            //    replication_group : ReplicationGroup::Group(owner_client_id),
            //    interpolation_target: NetworkTarget::AllExcept(vec![owner_client_id]),
            //    ..Default::default()
            //},
            //PreSpawnedPlayerObject
        }
    });
}


pub fn handle_pawn_input_client(
    mut pawn_query: Query<(&Pawn, &mut PawnInput), With<Predicted>>,
    mut input_reader: EventReader<InputEvent<Inputs>>,
){
    //if PlayerPosition::mode() != ComponentSyncMode::Full {
    //    return;
    //}
    for input in input_reader.read() {
        if let Some(input) = input.input() {
            for mut pawn in pawn_query.iter_mut() {
                match input {
                    Inputs::None => {}
                    Inputs::PawnInputData(pawn_input_data) => {
                        pawn.1.movement_direction = pawn_input_data.direction_input.to_vec3();
                        pawn.1.attack = pawn_input_data.attack;
                    }
                }
            }
        }
    }
}
pub fn handle_pawn_input_server(
    mut pawn_query: Query<(&Pawn, &mut PawnInput)>,
    mut input_reader: EventReader<InputEvent<Inputs, ClientId>>,
    global: Res<Global>,
){
    for input in input_reader.read() {
        let client_id = input.context();
        if let Some(input) = input.input() {
            if let Some(player_entity) = global.client_id_to_entity_id.get(client_id) {
                if let Ok(mut data) = pawn_query.get_mut(*player_entity) {
                    if let Inputs::PawnInputData(pawn_input_data) = input {
                        data.1.movement_direction = pawn_input_data.direction_input.to_vec3();
                        data.1.attack = pawn_input_data.attack;
                    }
                }
            }
        }
    }
}

pub(crate) fn buffer_input(mut client: ClientMut, keypress: Res<Input<KeyCode>>) {
    let mut input = PawnInputData {
        direction_input: DirectionInput::None,
        attack: false,
    };
    if keypress.pressed(KeyCode::W) || keypress.pressed(KeyCode::Up) {
        input.direction_input = DirectionInput::Up;
    }
    if keypress.pressed(KeyCode::S) || keypress.pressed(KeyCode::Down) {
        input.direction_input = DirectionInput::Down;
    }
    if keypress.pressed(KeyCode::A) || keypress.pressed(KeyCode::Left) {
        input.direction_input = DirectionInput::Left;
    }
    if keypress.pressed(KeyCode::D) || keypress.pressed(KeyCode::Right) {
        input.direction_input = DirectionInput::Right;
    }
    if keypress.pressed(KeyCode::Space) {
        input.attack = true;
    }

    // always remember to send an input message
    client.add_input(Inputs::PawnInputData(input));
}