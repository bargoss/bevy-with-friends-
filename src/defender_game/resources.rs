use bevy::prelude::{Resource, Vec2, Vec3};

#[derive(Resource, Default)]
pub struct UserInput{
    pub mouse_pos : Vec3,
    pub mouse_pos_2d : Vec2,
    pub left_button: bool,
    pub left_button_up: bool,
    pub left_button_down: bool,
}

#[derive(Resource)]
pub struct DefenderGameConfig{
    pub enemy_spawn_interval : f32,
    pub spawn_radius : f32,
}
impl Default for DefenderGameConfig{
    fn default() -> Self{
        DefenderGameConfig{
            enemy_spawn_interval : 2.0,
            spawn_radius : 10.0,
        }
    }
}