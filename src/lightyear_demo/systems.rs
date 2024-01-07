use bevy::math::Vec3;
use bevy::prelude::*;
use bevy_vector_shapes::painter::ShapePainter;
use crate::defender_game::utils;
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