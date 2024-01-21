use bevy::prelude::{Color, Entity, Query, Transform};
use bevy_vector_shapes::prelude::ShapePainter;
use lightyear::prelude::client::Confirmed;
use crate::defender_game::utils;
use crate::lightyear_demo::components_old::CircleView;

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