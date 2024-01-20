
use lightyear::shared::tick_manager::Tick;
use bevy::math::Vec3;
use bevy::prelude::*;
use bevy_vector_shapes::painter::ShapePainter;
use leafwing_input_manager::action_state::ActionState;
use leafwing_input_manager::axislike::DualAxisData;
use leafwing_input_manager::buttonlike::ButtonState::Pressed;

use lightyear::prelude::client::{Confirmed, Predicted};

use crate::defender_game::utils;
use crate::lightyear_demo::shared::{PlayerActions, PlayerId, Replicate, Simulated};


use super::components::*;

pub fn draw_circle_view(
    circle_views: Query<(Entity,&CircleView, &Transform)>,
    confirmed: Query<&Confirmed>,
    mut painter: ShapePainter
)
{


    circle_views.for_each(|(entity,circle_view, transform)|{
        let mut offset = Vec3::new(0.0, 0.0, 0.0);
        if confirmed.get(entity).is_ok() {
            offset = Vec3::new(0.0, -1.0, 0.0);
        }

        if !confirmed.get(entity).is_ok(){
            utils::draw_o(
                transform.translation + offset,
                circle_view.radius,
                circle_view.color,
                &mut painter
            );
        }
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
pub fn handle_projectile(
    mut projectile_query: Query<(Entity,&mut Projectile, &SimpleVelocity, &mut Transform),With<Simulated>>,
    mut commands: Commands
){
    projectile_query.for_each_mut(|(entity, mut projectile, velocity, mut replicated_position)|{
        replicated_position.translation += 0.15 * Vec3::new(0.0, -1.0, 0.0);

        //if global_time.simulation_tick.0 - projectile.start_tick.0 > 100 {
        //    commands.entity(entity).despawn();
        //}
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
    //global_time: Res<GlobalTime>,
    mut commands: Commands,
){
    /*
    pawn_query.for_each_mut(|(entity, mut pawn, pawn_input, mut transform, player_id)|{
        let last_shot_tick = pawn.last_attack_time;
        //let current_tick = global_time.simulation_tick;
        let current_tick = Tick(0);
        let ticks_since_last_shot = current_tick.0 - last_shot_tick.0;
        let cooldown = 50;
        let cooldown_finished = ticks_since_last_shot > cooldown;
        if pawn_input.attack && cooldown_finished {

            log::info!("SHOOTING: current_tick: {}, is_server: {}", current_tick.0, global_time.is_server);
            pawn.last_attack_time = current_tick;

            let shoot_dir = Vec3::new(0.0, -1.0, 0.0);


            let owner_client_id = *player_id.clone();
            let start_tick = global_time.simulation_tick;
            let position = transform.0;
            let velocity = shoot_dir;

            commands.spawn(ProjectileBundle::new(
                owner_client_id,
                start_tick,
                position,
                velocity
            ));
        }
    });
    */
}


pub fn update_cursor_state_from_window(
    window_query: Query<&Window>,
    mut action_state_query: Query<&mut ActionState<PlayerActions>, With<Predicted>>,
) {
    // Update the action-state with the mouse position from the window
    for window in window_query.iter() {
        for mut action_state in action_state_query.iter_mut() {


            //if let Some(val) = window_relative_mouse_position(window) {
            //    action_state
            //        .action_data_mut(PlayerActions::MoveCursor)
            //        .axis_pair = Some(DualAxisData::from_xy(val));
            //    action_state
            //        .action_data_mut(PlayerActions::MoveCursor)
            //        .state = Pressed;
            //}
        }

    }
}