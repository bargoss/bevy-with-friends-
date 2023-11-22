use bevy::input::ButtonState::Pressed;
use bevy::input::mouse::MouseButtonInput;
use bevy::math::{Vec2Swizzles, Vec3Swizzles};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_vector_shapes::prelude::*;
use crate::defender_game::components::*;
use crate::defender_game::events::*;
use crate::defender_game::resources::*;
use crate::defender_game::utils;
use crate::utils::{Plane, ray_plane_intersection};

pub fn init(mut commands: Commands,
    mut _meshes: ResMut<Assets<Mesh>>,
    mut _materials: ResMut<Assets<StandardMaterial>>
)
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

pub fn projectile_movement_system(
    mut projectile_query: Query<(&mut Transform, &mut Projectile)>,
    time: Res<Time>
){
    let delta_time= time.delta_seconds();
    projectile_query.for_each_mut(|(mut transform, mut projectile)|{
        let vel_3d = Vec3::new(projectile.velocity.x, projectile.velocity.y, 0.0);
        let movement = vel_3d * delta_time;
        transform.translation += movement;
        projectile.distance_travelled += movement.length();
    });
}

pub fn projectile_collision_system(
    projectile_query: Query<(&Transform, &Projectile, Entity)>,
    rapier_context: Res<RapierContext>,
    mut event_writer: EventWriter<ProjectileCollisionEvent>,
    mut commands: Commands,
){
    projectile_query.for_each(|(transform, projectile, entity)|{
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

            // despawn self
            commands.entity(entity).despawn();

            info!("projectile hit entity: {:?}, distance: {:?}", hit_entity, hit_distance);
        }
    });
}

pub fn projectile_damage_system(
    mut health_query: Query<&mut Health>,
    mut event_reader: EventReader<ProjectileCollisionEvent>,
){
    event_reader.iter().for_each(|event|{
        let projectile = event.projectile.clone();
        let collided_entity = event.collided_entity;

        if let Ok(mut health) = health_query.get_mut(collided_entity) {
            health.hit_points -= projectile.damage;
            info!("enemy hit, health: {:?}", health.hit_points);
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

pub fn draw_projectile(
    projectile_query: Query<(&Projectile, &Transform)>,
    mut painter: ShapePainter,
){
    projectile_query.for_each(|(_projectile, transform)|{
        utils::draw_o(Vec3::new(transform.translation.x, transform.translation.y, 0.0),
                      0.1,
                      Color::rgb(1.0, 0.0, 0.0),
                      &mut painter
        );
    });
}

pub fn draw_player_tower(
    player_tower_query: Query<(&PlayerTower, &Transform)>,
    mut painter: ShapePainter,
){
    player_tower_query.for_each(|(player_tower, transform)|{
        utils::draw_o(
            Vec3::new(transform.translation.x, transform.translation.y, 0.0),
            0.5,
            Color::rgb(0.0, 1.0, 0.0),
            &mut painter
        );

        let position = transform.translation;
        let aim_direction_3d = player_tower.aim_direction_3d();

        utils::draw_line(
            position,
            position + aim_direction_3d,
            0.4,
            Color::rgb(0.0, 1.0, 0.0),
            &mut painter
        );
    });
}

pub fn update_player_tower_input_system(
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

pub fn player_tower_system(
    mut player_tower_query: Query<(&mut PlayerTower, &Transform)>,
    time: Res<Time>,
    mut commands: Commands,
){
    player_tower_query.for_each_mut(|(mut player_tower, transform)|{
        let position = transform.translation;
        let aim_direction_3d = player_tower.aim_direction_3d();

        if player_tower.shoot_input {
            let now = time.elapsed().as_secs_f32();
            if now - player_tower.last_shot_time > 0.5 {
                player_tower.last_shot_time = now;
                shoot_bullet(
                    position,
                    aim_direction_3d * 10.0,
                    1.0,
                    &mut commands
                );
            }
        }
    });
}

pub fn shoot_bullet(
        position : Vec3,
        velocity : Vec3,
        damage : f32,
        mut commands: &mut Commands,
){
    commands.spawn_empty()
        .insert(TransformBundle{
            local: Transform{
                translation: position,
                ..default()
            },
            ..default()
        })
        .insert(Projectile{
            damage: damage,
            velocity: velocity.xy(),
            max_range: 50.0,
            distance_travelled: 0.0,
        })


    ;
}

pub fn take_user_input_system(
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut user_input: ResMut<UserInput>,
    mut click_events: EventReader<MouseButtonInput>,
) {
    let window = windows.single();

    if let Some(screen_pos) = window.cursor_position() {
        if let Ok(cam_entity) = camera_query.get_single() {
            let camera = cam_entity.0;
            let transform = cam_entity.1;

            let plane = Plane {
                normal: Vec3::Z,
                point: Vec3::ZERO,
            };

            user_input.left_click = false;

            if let Some(ray) = camera.viewport_to_world(transform, screen_pos) {
                if let Some(hit) = ray_plane_intersection(&ray, &plane) {
                    user_input.mouse_pos = hit;
                    user_input.mouse_pos_2d = Vec2::new(hit.x, hit.y);
                }
            }

            if click_events.iter().any(|event| event.button == MouseButton::Left && event.state == Pressed) {
                user_input.left_click = true;
            }
        }
        else{
            // log error
            error!("no camera found!");
        }
    }
}