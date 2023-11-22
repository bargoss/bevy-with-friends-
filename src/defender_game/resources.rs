use bevy::prelude::{Resource, Vec2, Vec3};

#[derive(Resource, Default)]
pub struct UserInput{
    pub mouse_pos : Vec3,
    pub mouse_pos_2d : Vec2,
    pub left_click : bool,
}