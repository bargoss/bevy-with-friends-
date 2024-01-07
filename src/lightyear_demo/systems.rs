use bevy::math::Vec3;
use bevy::prelude::*;
use bevy_vector_shapes::painter::ShapePainter;
use crate::defender_game::utils;
use crate::lightyear_demo::shared::ReplicatedPosition;
use super::components::*;

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
    TickUpdate,    /// Main loop (with physics, game logic) during FixedUpda
    Main,
    MainFlush,
    */
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