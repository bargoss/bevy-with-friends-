use bevy::math::Vec3;
use bevy::prelude::{Commands, Entity, Query, Res, Transform, With};
use lightyear::prelude::{NetworkIdentity, NetworkTarget, PreSpawnedPlayerObject, ReplicationGroup, TickManager};
use crate::lightyear_demo::components_old::{Pawn, PawnInput, ProjectileBundle};
use crate::lightyear_demo::shared_old::{PlayerId, Replicate, Simulated};

pub fn handle_pawn_shooting(
    mut pawn_query: Query<(Entity,&mut Pawn, &PawnInput, &Transform, &PlayerId), With<Simulated>>,
    tick_manager: Res<TickManager>,
    mut commands: Commands,
    identity: NetworkIdentity,
){
    pawn_query.for_each_mut(|(entity, mut pawn, pawn_input, mut transform, player_id)|{
        let last_shot_tick = pawn.last_attack_time;
        let current_tick = tick_manager.tick();
        let ticks_since_last_shot = current_tick.0 - last_shot_tick.0;
        let cooldown = 50;
        let cooldown_finished = ticks_since_last_shot > cooldown;

        let shoot_now = pawn_input.attack && cooldown_finished;
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
        }
    });
}