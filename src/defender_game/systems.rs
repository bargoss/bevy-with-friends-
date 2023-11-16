use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_vector_shapes::prelude::ShapePainter;
use crate::defender_game::components::*;
use crate::defender_game::events::*;
use crate::defender_game::utils;
use crate::UserInput;

pub fn init(mut commands: Commands,
            mut meshes: ResMut<Assets<Mesh>>,
            mut materials: ResMut<Assets<StandardMaterial>>)
{
    println!("init");

    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 20.0))
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.spawn_empty()
        .insert(Transform{
            ..default()
        })
        .insert(PlayerTower{aim_direction: Vec2::new(0.0, 1.0), shoot_input: false, last_shot_time: 0.0})
        .insert(Health{hit_points: 100.0, max_hit_points: 100.0})
        .insert(Collider::ball(0.5))
    ;

}

pub fn handle_projectile_movement(
    mut projectile_query: Query<(&mut Transform, &mut Projectile)>,
){
    projectile_query.for_each_mut(|(mut transform, mut projectile)|{
        transform.translation.x += projectile.velocity.x;
        transform.translation.y += projectile.velocity.y;
        projectile.distance_travelled += projectile.velocity.length();
    });
}

pub fn projectile_collision_events(
    projectile_query: Query<(&Transform, &Projectile)>,
    rapier_context: Res<RapierContext>,
    mut event_writer: EventWriter<ProjectileCollisionEvent>,
){
    projectile_query.for_each(|(transform, projectile)|{
        let position = transform.translation;

        let direction = Vec2::new(projectile.velocity.x, projectile.velocity.y);
        let origin = Vec2::new(position.x, position.y);

        let raycast_result = rapier_context.cast_ray(origin, direction, 0.0, false, Default::default());
        if let Some(hit) = raycast_result {
            let hit_entity = hit.0;
            let hit_distance = hit.1;

            event_writer.send(ProjectileCollisionEvent{
                projectile: projectile.clone(),
                projectile_position: Vec2::new(position.x, position.y),
                collided_entity: hit_entity,
            });
            info!("projectile hit entity: {:?}, distance: {:?}", hit_entity, hit_distance);
        }
    });
}
pub fn enemy_death_system(
    mut commands: Commands,
    health_query: Query<(&Health, &Enemy, &Transform, Entity)>,
){
    health_query.for_each(|(health, _enemy, _transform, entity)|{
        if health.hit_points <= 0.0 {
            commands.entity(entity).despawn();
        }
    });
}

pub fn handle_projectile_enemy_collisions(
    mut commands: Commands,
    mut enemy_query: Query<(&Enemy, &mut Health, &Transform)>,
    mut projectile_collision_event_reader: EventReader<ProjectileCollisionEvent>,
){
    projectile_collision_event_reader.iter().for_each(|event|{
        let projectile = event.projectile.clone();
        let projectile_position = event.projectile_position;
        let collided_entity = event.collided_entity;

        let _ = enemy_query.get_mut(collided_entity).map(|(enemy, mut health, transform)| {
            health.hit_points -= projectile.damage;
            info!("enemy hit, health: {:?}", health.hit_points);
        });
    });
}

pub fn draw_projectile(

)
{

}

pub fn draw_player_tower(
    player_tower_query: Query<(&PlayerTower, &Transform)>,
    mut painter: ShapePainter,
){
    utils::draw_o(Vec3::new(0.0, 0.0, 0.0), &mut painter);
}

pub fn update_player_tower_input(
    mut player_tower_query: Query<(&mut PlayerTower, &Transform)>,
    user_input : Res<UserInput>,
){
    player_tower_query.for_each_mut(|(mut player_tower, transform)|{
        let mouse_position = user_input.mouse_pos_2d;
        let tower_position = transform.translation.xy();
        let aim_direction = (mouse_position - tower_position).normalize_or_zero();

        player_tower.aim_direction = aim_direction;
        player_tower.shoot_input = user_input.left_click;
    });
}

pub fn update_player_tower(
    mut player_tower_query: Query<(&mut PlayerTower, &Transform)>,
){

}

pub fn shoot_bullet(
    shoot_velocity : Vec2,
    mut commands: Commands,
){

}