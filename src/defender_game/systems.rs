use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::rapier::prelude::ColliderBuilder;
use bevy_vector_shapes::prelude::ShapePainter;
use crate::defender_game::components::*;
use crate::defender_game::utils;

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
        .insert(PlayerTower{aim_direction: Vec2::new(0.0, 1.0), last_shot_time: 0.0})
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

pub fn handle_projectile_collision(
    projectile_query: Query<(&Transform, &Projectile)>,
    mut enemy_query: Query<(&mut Health, &Transform, &mut Enemy)>,
    rapier_context: Res<RapierContext>,
){
    projectile_query.for_each(|(transform, projectile)|{
        let position = transform.translation;

        let direction = Vec2::new(projectile.velocity.x, projectile.velocity.y);
        let origin = Vec2::new(position.x, position.y);

        let raycast_result = rapier_context.cast_ray(origin, direction, 0.0, false, Default::default());
        if let Some(hit) = raycast_result {
            let hit_entity = hit.0;
            let hit_distance = hit.1;

            info!("projectile hit entity: {:?}, distance: {:?}", hit_entity, hit_distance);
        }
    });
}

pub fn draw_player_tower(
    player_tower_query: Query<(&PlayerTower, &Transform)>,
    mut painter: ShapePainter,
){
    utils::draw_o(Vec3::new(0.0, 0.0, 0.0), &mut painter);
}