use bevy::app::App;
use bevy::prelude::{Plugin, Startup, Update};
use crate::defender_game::systems::*;

pub struct DefenderGamePlugin;

impl Plugin for DefenderGamePlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init)
        .add_systems(Update, handle_projectile_movement)
        .add_systems(Update, handle_projectile_collision)
        .add_systems(Update, draw_player_tower);
    }
}