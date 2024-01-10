use bevy::ecs::component::Tick;
use bevy::math::Vec3;
use bevy::prelude::*;
use bevy_vector_shapes::painter::ShapePainter;
use lightyear::client::components::ComponentSyncMode;
use lightyear::prelude::client::{Client, Predicted, SyncComponent};
use lightyear::prelude::{ClientId, NetworkTarget, Replicate};
use lightyear::prelude::server::Server;
use lightyear::shared::events::InputEvent;

use crate::defender_game::utils;
use crate::lightyear_demo::shared::{GlobalTime, Inputs, MyProtocol, PlayerId, PlayerPosition, ReplicatedPosition, Simulated};
use crate::lightyear_demo::shared::ComponentsKind::ShouldBePredicted;

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
pub fn direction_input_to_vec3(dir: &super::shared::PawnInputData) -> Vec3{
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
    //if PlayerPosition::mode() != ComponentSyncMode::Full {
    //    return;
    //}
    for input in input_reader.read() {
        if let Some(input) = input.input() {
            for mut data in pawn_query.iter_mut() {
                match input { Inputs::Move(input_data) => {
                    data.1.movement_direction = direction_input_to_vec3(input_data);
                    data.1.attack = input_data.attack;
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
        let client_id = input.context();
        if let Some(input) = input.input() {
            if let Some(player_entity) = global.client_id_to_entity_id.get(client_id) {
                if let Ok(mut data) = pawn_query.get_mut(*player_entity) {
                    if let Inputs::Move(input_data) = input {
                        let movement_direction = direction_input_to_vec3(input_data);
                        data.1.movement_direction = movement_direction;
                        data.1.attack = input_data.attack;
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
    global_time: Res<GlobalTime>
){

    let current_tick = global_time.simulation_tick;
    let float_from_tick = current_tick.0 as f32 * 0.05;
    let sin = float_from_tick.sin();
    let speed = 0.05 * sin;


    pawn_query.for_each_mut(|(_, pawn_input, mut transform)|{
        let movement_direction = pawn_input.movement_direction;
        transform.translation.x += speed;
        transform.translation.y += movement_direction.y * speed;
    });

}

//pub fn update_time_client(
//    client: Res<Client<MyProtocol>>,
//    tick_manager: Res<TickManager>,
//    mut global_time: ResMut<GlobalTime>,
//){
//    let current_tick = tick_manager.current_tick();
//
//}
pub fn update_time_client(
    client: Res<Client<MyProtocol>>,
    mut global_time: ResMut<GlobalTime>,
){
    let tick = client.tick();
    global_time.simulation_tick = tick;
}

pub fn update_time_server(
    server: Res<Server<MyProtocol>>,
    mut global_time: ResMut<GlobalTime>,
){
    let tick = server.tick();
    global_time.simulation_tick = tick;

}

pub fn handle_projectile(
    mut projectile_query: Query<(&mut Projectile, &SimpleVelocity, &mut Transform),With<Simulated>>,
    mut commands: Commands
){
    projectile_query.for_each_mut(|(mut projectile, velocity, mut transform)|{
        transform.translation += velocity.value * 0.15;

        //projectile.life_time -= 1;
        //if projectile.life_time <= 0 {
        //    commands.entity(projectile.entity).despawn();
        //}
    });
}



pub fn handle_pawn_shooting(
    mut pawn_query: Query<(Entity,&mut Pawn, &PawnInput, &Transform, &PlayerId), With<Simulated>>,
    global_time: Res<GlobalTime>,
    mut commands: Commands,
){
    pawn_query.for_each_mut(|(entity, mut pawn, pawn_input, mut transform, player_id)|{
        let last_shot_tick = pawn.last_attack_time;
        let current_tick = global_time.simulation_tick;
        let ticks_since_last_shot = current_tick.0 - last_shot_tick.0;
        let cooldown = 50;
        let cooldown_finished = ticks_since_last_shot > cooldown;
        if pawn_input.attack && cooldown_finished { // global_time.simulation_tick.0 % 50 == 49
            log::info!("SHOOTING");
            pawn.last_attack_time = current_tick;

            //let shoot_dir = pawn_input.movement_direction;
            let shoot_dir = Vec3::new(0.0, -1.0, 0.0);

            //let projectile = commands.spawn(ProjectileBundle::new(
            //    *player_id.clone(),
            //    global_time.simulation_tick,
            //    Vec3::new(transform.translation.x, transform.translation.y, transform.translation.z),
            //    shoot_dir,
            //));
            //let entity_id = projectile.id();


            // remove Replicate component
            //commands.entity(entity_id).remove::<Replicate>();


            let owner_client_id = *player_id.clone();
            let start_tick = global_time.simulation_tick;
            let position = Vec3::new(transform.translation.x, transform.translation.y, transform.translation.z);
            let velocity = shoot_dir;

            let entity_id = commands.spawn_empty()
                .insert(PlayerId::new(owner_client_id))
                .insert(Projectile{
                    start_tick
                })
                .insert(SimpleVelocity{
                    value: velocity,
                })
                .insert(ReplicatedPosition(position))
                .insert(TransformBundle{
                    local: Transform::from_translation(position),
                    ..Default::default()
                })
                .insert(CircleView{
                    radius: 0.25,
                    color: Color::RED,
                })
                .insert(Replicate{
                    prediction_target: NetworkTarget::Only(vec![owner_client_id]),
                    interpolation_target: NetworkTarget::AllExcept(vec![owner_client_id]),
                    ..Default::default()
                })
                //.insert(SpawnHash{
                //    hash: 0,
                //    spawned_tick: start_tick,
                //}).id()
                ;

            //commands.entity(entity_id).insert(lightyear::_reexport::ShouldBePredicted{
            //    client_entity: Some(entity_id),
            //});

            /*commands.spawn({

                PlayerId::new(owner_client_id);
                Projectile{
                    start_tick
                };
                SimpleVelocity{
                    value: velocity,
                };
                ReplicatedPosition(position);
                TransformBundle{
                    local: Transform::from_translation(position),
                    ..Default::default()
                };
                CircleView{
                    radius: 0.25,
                    color: Color::RED,
                };
                //Replicate{
                //    prediction_target: NetworkTarget::Only(vec![owner_client_id]),
                //    interpolation_target: NetworkTarget::AllExcept(vec![owner_client_id]),
                //    ..Default::default()
                //};
                SpawnHash{
                    hash: 0,
                    spawned_tick: start_tick,
                }
                //should_be_predicted: ShouldBePredicted{client_entity: None},
            });*/


            //projectile.insert(lightyear::_reexport::ShouldBePredicted{
            //    client_entity :  Some(entity_id),
            //});


        }
    });
}