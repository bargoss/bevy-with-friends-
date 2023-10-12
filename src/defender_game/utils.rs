use bevy::prelude::*;
use bevy_rapier2d::prelude::Collider;
use bevy_vector_shapes::prelude::{DiscPainter, LinePainter, ShapePainter};
use crate::defender_game::components::*;

pub fn spawn_enemy(pos : Vec2, commands: &mut Commands){
    commands.spawn_empty()
        .insert(Transform{
            translation: Vec3::new(pos.x, pos.y, 0.0),
            ..default()
        })
        .insert(Enemy::default())
        .insert(Health::new(10.0))
        .insert(Collider::ball(0.5))
    ;
}

pub fn shoot_projectile(pos : Vec2, vel: Vec2, commands: &mut Commands){
    commands.spawn_empty()
        .insert(Transform{
            translation: Vec3::new(pos.x, pos.y, 0.0),
            ..default()
        })
        .insert(Projectile{
            damage: 1.0,
            velocity: vel,
            max_range: 50.0,
            distance_travelled: 0.0,
        })
    ;
}


pub fn draw_o(position : Vec3, painter: &mut ShapePainter){
    painter.thickness = 0.5;
    painter.hollow = true;
    painter.color = Color::rgb(0.0, 1.0, 0.0);
    painter.transform = Transform::from_translation(position);
    painter.circle(1.0);
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