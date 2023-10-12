use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
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
        .insert(Health{hit_points: 100.0, max_hit_points: 100.0}
    );

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
        let position2d = Vec2::new(position.x, position.y);
        //let ray = Ray{
        //    direction: Vec2::new(projectile.velocity.x, projectile.velocity.y) as Vec3,
        //    origin: Vec3::new(position.x, position.y, 0.0),
        //};

        let direction = Vec3::new(projectile.velocity.x, projectile.velocity.y, 0.0);
        let direction2d = Vec2::new(direction.x, direction.y);
        let origin = Vec3::new(position.x, position.y, 0.0);
        let origin2d = Vec2::new(origin.x, origin.y);

        let raycast_result = rapier_context.cast_ray(origin2d, direction2d, 0.0, false, Default::default());
    });
}

pub fn draw_player_tower(
    player_tower_query: Query<(&PlayerTower, &Transform)>,
    mut painter: ShapePainter,
){
    utils::draw_o(Vec3::new(0.0, 0.0, 0.0), &mut painter);
}