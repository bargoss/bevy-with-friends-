use bevy::prelude::*;
use bevy_vector_shapes::prelude::{DiscPainter, LinePainter, ShapePainter};
use rand::Rng;

pub fn draw_o(position : Vec3, radius: f32, color: Color, painter: &mut ShapePainter){
    painter.thickness = 0.5;
    painter.hollow = true;
    painter.color = color;
    painter.transform = Transform::from_translation(position);
    painter.circle(radius);
}
pub fn draw_x(position : Vec3, painter: &mut ShapePainter){
    painter.thickness = 0.5;
    painter.hollow = true;
    painter.color = Color::rgb(0.0, 1.0, 0.0);
    painter.transform = Transform::from_translation(position);

    let line_len = 1.0;
    painter.line(Vec3::new(-line_len, line_len, 0.0), Vec3::new(line_len, -line_len, 0.0));
    painter.line(Vec3::new(line_len, line_len, 0.0), Vec3::new(-line_len, -line_len, 0.0));
}

pub fn draw_line(start : Vec3, end: Vec3, thickness: f32, color: Color, painter: &mut ShapePainter){
    painter.thickness = thickness;
    painter.hollow = false;
    painter.color = color;
    painter.transform = Transform::from_translation(Vec3::ZERO);
    painter.line(start, end);
}
//let spawn_position = utils::random_point_in_circle(transform.translation.xy(), spawn_radius);
pub fn random_point_in_circle_2d(center: Vec2, radius: f32) -> Vec2{
    let mut rng = rand::thread_rng();
    let r = rng.gen_range(0.0..radius);
    let theta = rng.gen_range(0.0..2.0 * std::f32::consts::PI);
    Vec2::new(
        center.x + r * theta.cos(),
        center.y + r * theta.sin(),
    )
}
pub fn random_point_in_circle(center: Vec3, radius: f32) -> Vec3{
    let mut rng = rand::thread_rng();
    let r = rng.gen_range(0.0..radius);
    let theta = rng.gen_range(0.0..2.0 * std::f32::consts::PI);
    Vec3::new(
        center.x + r * theta.cos(),
        center.y + r * theta.sin(),
        center.z,
    )
}